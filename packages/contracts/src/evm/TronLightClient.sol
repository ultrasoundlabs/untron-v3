// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IBlockRangeProver} from "./blockRangeProvers/interfaces/IBlockRangeProver.sol";

/// @title TronLightClient
/// @notice Stores and verifies a sparse set of Tron block checkpoints on an EVM chain.
/// @dev
/// This contract supports two verification paths:
/// 1) `proveBlocks`: verify a contiguous sequence of Tron blocks using per-block witness signatures.
/// 2) `proveBlockRange`: verify an entire range using an external prover (e.g., ZK), anchored at `latestProvenBlock`.
///
/// Glossary / conventions used in this contract:
/// - Tron `blockHash`: `sha256(BlockHeader_raw)` of the protobuf-encoded raw header.
/// - Tron `blockId`: `uint64(blockNumber) || sha256(BlockHeader_raw)[8:]` (height in the top 8 bytes + 24-byte hash tail).
/// - `srs`: SR owner accounts that appear in Tron `BlockHeader_raw.witnessAddress` (0x41 prefix + 20 bytes).
/// - `witnessDelegatees`: the actual secp256k1 signing keys for each SR index (may be delegated from the owner).
/// @author Ultrasound Labs
contract TronLightClient {
    uint256 internal constant _TRON_BLOCK_METADATA_SIZE = 69; // bytes per packed TronBlockMetadata
    uint256 internal constant _SIGNATURE_SIZE = 65; // bytes per secp256k1 signature (r,s,v)

    // secp256k1 half-order (used to enforce "low-s" signatures and prevent malleability).
    // Matches OpenZeppelin's ECDSA upper-bound for `s`.
    uint256 internal constant _SECP256K1N_HALF = 0x7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0;

    uint256 internal constant _TRON_BLOCK_VERSION = 32; // current observed Tron BlockHeader_raw.version
    uint256 internal constant _RECENT_BLOCK_CREATOR_WINDOW = 18; // sliding window size for block creator uniqueness

    /// @notice Verifier used to validate succinct proofs for block ranges.
    IBlockRangeProver public immutable BLOCK_RANGE_PROVER;
    /// @notice EVM addresses of the elected Super Representatives (witness accounts) for this epoch.
    /// These are the owner accounts that appear (with 0x41 prefix) in `BlockHeader_raw.witnessAddress`.
    /// It's not immutable because arrays cannot be immutable in Solidity for some reason.
    bytes20[27] public srs;
    /// @notice EVM addresses of the actual signing keys (may be delegated keys) for each SR index.
    /// A given SR may delegate its witness permission to a separate key; those delegatees live here.
    /// It's not immutable because arrays cannot be immutable in Solidity for some reason.
    bytes20[27] public witnessDelegatees;

    /// @notice Packed mapping witnessIndex -> delegateeGroup (5 bits per index, 27 indices total).
    /// @dev Indices that share the same `witnessDelegatees[i]` share the same group id.
    ///      This lets us do "recently produced" checks by group without assuming delegatees are unique.
    uint256 internal immutable _DELEGATEE_GROUPS_PACKED;

    /// @notice ZK-friendly hash of public keys (not addresses!) of SRs and their delegatees.
    /// @dev This hash is used as a public input to the block range prover, to save
    ///      proving cycles compared to passing two arrays of Keccak-hashed arrays (as above).
    bytes32 public immutable SR_DATA_HASH;
    /// @notice Highest stored/proven Tron `blockId` by embedded block number.
    /// @dev This is monotonic w.r.t. the embedded height (`uint64(blockNumber)` in the top 8 bytes).
    bytes32 public latestProvenBlock;
    mapping(uint256 blockNumber => bytes32 blockId) internal _blockIds;
    mapping(uint256 blockNumber => bytes32 txTrieRoot) internal _txTrieRoots;
    mapping(uint256 blockNumber => uint32 timestamp) internal _blockTimestamps;

    /// @notice Creates a Tron light client anchored at an initial checkpoint.
    /// @param blockRangeProver External verifier for `proveBlockRange`.
    /// @param initialBlockHash Initial Tron `blockId` checkpoint (despite the name, this must include the height prefix).
    /// @param initialTxTrieRoot Transaction trie root corresponding to `initialBlockHash`.
    /// @param initialTimestamp Block timestamp in seconds (Tron raw header is milliseconds; this contract stores seconds).
    /// @param _srs SR owner accounts for the epoch (used for `witnessAddress` in header encoding).
    /// @param _witnessDelegatees SR signing keys for the epoch (used for signature recovery checks).
    /// @param srDataHash Hash of the SR data for the epoch.
    constructor(
        IBlockRangeProver blockRangeProver,
        bytes32 initialBlockHash,
        bytes32 initialTxTrieRoot,
        uint32 initialTimestamp,
        bytes20[27] memory _srs,
        bytes20[27] memory _witnessDelegatees,
        bytes32 srDataHash
    ) {
        BLOCK_RANGE_PROVER = blockRangeProver;
        _appendBlockId(initialBlockHash, initialTxTrieRoot, initialTimestamp);

        // Build a packed mapping witnessIndex -> delegateeGroup.
        // If multiple witness indices share the same delegatee key, they share a group id.
        // We use this group id for the "recently produced" uniqueness window.
        uint8[27] memory groups;
        uint8 nextGroup = 0;

        for (uint256 i = 0; i < 27; ++i) {
            uint8 g = type(uint8).max;

            // Find a prior index with the same delegatee; if found, reuse its group id.
            for (uint256 j = 0; j < i; ++j) {
                if (_witnessDelegatees[i] == _witnessDelegatees[j]) {
                    g = groups[j];
                    break;
                }
            }

            // Otherwise assign a fresh group id.
            if (g == type(uint8).max) {
                g = nextGroup;
                unchecked {
                    ++nextGroup;
                }
            }

            groups[i] = g;
        }

        uint256 packed;
        for (uint256 i = 0; i < 27; ++i) {
            packed |= uint256(groups[i]) << (i * 5);
        }
        _DELEGATEE_GROUPS_PACKED = packed;

        srs = _srs;
        witnessDelegatees = _witnessDelegatees;
        SR_DATA_HASH = srDataHash;
    }

    struct TronBlockMetadata {
        bytes32 parentHash;
        bytes32 txTrieRoot;
        // In Tron protocol, it's expressed in milliseconds, but block time is 3s and actual time is never
        // more granular than .......000. So we can /1000 it and store efficiently in 32-bit ints (seconds).
        uint32 timestamp;
        uint8 witnessAddressIndex;
    }

    struct ProveBlocksCtx {
        bytes32 blockId;
        uint256 blockNumber;
        bytes32 lastTxTrieRoot;

        // ring buffer (last 18 creators) tracked by delegatee-group id
        // (indices with the same delegatee key share the same group id).
        uint8[18] recentGroups;
        uint32 recentMask;

        bool intersectedExisting;
        uint32 lastTimestamp;
        uint8 recentCount;
        uint8 recentPos;
    }

    error NotEnoughBlocksOrSignatures();
    error InvalidParentBlockId(bytes32 yours, bytes32 real);
    error BlockTooOld();
    error InvalidChain();
    error BlockNotRelayed();
    error InvalidCompressedTronBlockMetadataLength();
    error InvalidCompressedSignaturesLength();
    error InvalidWitnessSigner();
    error WitnessProducedRecently(bytes20 signer);
    error UnanchoredBlockRange();

    /// @notice Relays and verifies a contiguous sequence of Tron blocks using per-block witness signatures.
    /// @dev
    /// The proof range must be anchored to already-stored history: either `startingBlock` is stored, or some
    /// block within the provided range intersects an existing stored block number with matching `blockId`.
    ///
    /// `compressedTronBlockMetadata` is `N * 69` bytes, where each 69-byte chunk is:
    /// `[parentBlockId(32) | txTrieRoot(32) | timestampSeconds(uint32 big-endian) | witnessIndex(uint8)]`.
    ///
    /// `compressedSignatures` is `N * 65` bytes, where each signature is `[r(32) | s(32) | v(1)]`.
    /// Tron typically uses `v` as 0/1 (sometimes 27/28); this method normalizes to 27/28 before recovery.
    /// @param startingBlock Parent/anchor Tron `blockId` for the first provided block.
    /// @param compressedTronBlockMetadata Packed metadata for each block in the sequence.
    /// @param compressedSignatures Packed witness signatures for each block in the sequence.
    function proveBlocks(
        bytes32 startingBlock,
        bytes calldata compressedTronBlockMetadata,
        bytes calldata compressedSignatures
    ) external {
        uint256 tronBlocksLength = compressedTronBlockMetadata.length;
        if (tronBlocksLength == 0 || tronBlocksLength % _TRON_BLOCK_METADATA_SIZE != 0) {
            revert InvalidCompressedTronBlockMetadataLength();
        }

        uint256 numBlocks = tronBlocksLength / _TRON_BLOCK_METADATA_SIZE;

        if (compressedSignatures.length != numBlocks * _SIGNATURE_SIZE) {
            revert InvalidCompressedSignaturesLength();
        }

        ProveBlocksCtx memory ctx;
        ctx.blockId = startingBlock;
        ctx.blockNumber = _blockIdToNumber(startingBlock);
        ctx.intersectedExisting = _isStoredAnchor(ctx.blockNumber, startingBlock);

        _proveBlocksLoop(ctx, compressedTronBlockMetadata, compressedSignatures, numBlocks);

        if (!ctx.intersectedExisting) revert UnanchoredBlockRange();
        _appendBlockId(ctx.blockId, ctx.lastTxTrieRoot, ctx.lastTimestamp);
    }

    /// @notice Verifies a block range proof and appends the ending block as the new checkpoint.
    /// @dev Reverts unless `startingBlock` matches `latestProvenBlock` to ensure a single advancing chain of checkpoints.
    /// @param startingBlock Current checkpoint `blockId` that this proof range must start from.
    /// @param endingBlock New checkpoint `blockId` proven by `zkProof`.
    /// @param endingBlockTxTrieRoot Transaction trie root for `endingBlock`.
    /// @param endingBlockTimestamp Block timestamp for `endingBlock` (seconds).
    /// @param zkProof Succinct proof blob understood by `BLOCK_RANGE_PROVER`.
    function proveBlockRange(
        bytes32 startingBlock,
        bytes32 endingBlock,
        bytes32 endingBlockTxTrieRoot,
        uint32 endingBlockTimestamp,
        bytes calldata zkProof
    ) external {
        if (startingBlock != latestProvenBlock) revert BlockTooOld();
        BLOCK_RANGE_PROVER.proveBlockRange(
            SR_DATA_HASH, startingBlock, endingBlock, endingBlockTxTrieRoot, endingBlockTimestamp, zkProof
        );
        _appendBlockId(endingBlock, endingBlockTxTrieRoot, endingBlockTimestamp);
    }

    /// @notice Returns the stored Tron `blockId` for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return blockId Stored Tron `blockId` at that height.
    function getBlockId(uint256 blockNumber) public view returns (bytes32) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return stored;
    }

    /// @notice Returns the stored transaction trie root for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return txTrieRoot Stored transaction trie root at that height.
    function getTxTrieRoot(uint256 blockNumber) external view returns (bytes32) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return _txTrieRoots[blockNumber];
    }

    /// @notice Returns the stored block timestamp (seconds) for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return timestamp Stored block timestamp in seconds.
    function getBlockTimestamp(uint256 blockNumber) external view returns (uint32) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return _blockTimestamps[blockNumber];
    }

    /// @notice Stores a checkpoint for `blockId` and updates `latestProvenBlock` if it is newer.
    /// @dev Callers must ensure the provided checkpoint is consistent with the intended chain.
    /// @param blockId Tron `blockId` to store (height embedded in the top 8 bytes).
    /// @param txTrieRoot Transaction trie root for `blockId`.
    /// @param timestamp Block timestamp in seconds.
    function _appendBlockId(bytes32 blockId, bytes32 txTrieRoot, uint32 timestamp) internal {
        uint256 blockNumber = _blockIdToNumber(blockId);
        _blockIds[blockNumber] = blockId;
        _txTrieRoots[blockNumber] = txTrieRoot;
        _blockTimestamps[blockNumber] = timestamp;
        if (_blockIdToNumber(latestProvenBlock) < blockNumber) {
            latestProvenBlock = blockId;
        }
    }

    /// @dev Encode a minimal Tron `BlockHeader_raw` protobuf message from `TronBlockMetadata`
    /// and return its SHA256 hash.
    ///
    /// The fields we encode are:
    /// - field 1 (varint):  timestamp in milliseconds (Tron stores ms; we keep seconds in metadata)
    /// - field 2 (bytes):   txTrieRoot (32 bytes)
    /// - field 3 (bytes):   parentHash (32 bytes)
    /// - field 7 (varint): number
    /// - field 9 (bytes):   witnessAddress (21 bytes: 0x41 prefix + 20-byte address from `srs`)
    /// - field 10 (varint): version (currently always 32 on Tron mainnet)
    ///
    /// @return blockHash The SHA256 hash of the encoded block header (NOT blockId!!!)
    /// @notice Computes the Tron `blockHash` (SHA256 of encoded `BlockHeader_raw`) for a metadata entry.
    /// @param tronBlock Decoded metadata for the block being hashed.
    /// @param blockNumber Tron block height of the block being hashed.
    /// @param scratch Scratch memory region to use for encoding.
    function _hashBlockScratch(TronBlockMetadata memory tronBlock, uint256 blockNumber, bytes memory scratch)
        internal
        view
        returns (bytes32 blockHash)
    {
        _encodeTronBlockHeaderInto(scratch, tronBlock, blockNumber);

        // Call SHA256 precompile (0x02) directly on the scratch region.
        // Output is written to memory slot 0x00 to avoid moving the free memory pointer.
        // solhint-disable-next-line no-inline-assembly
        assembly {
            if iszero(staticcall(gas(), 0x02, add(scratch, 32), mload(scratch), 0x00, 32)) { revert(0, 0) }
            blockHash := mload(0x00)
        }
    }

    /* solhint-disable function-max-lines */
    // IMO it's quite tight and optimized already

    /// @notice Encode a minimal Tron `BlockHeader_raw` protobuf message into an existing bytes buffer.
    /// @dev Mutates `buf` length to the number of bytes written.
    /// @param buf The buffer to write the encoded header into.
    /// @param tronBlock The Tron block metadata to encode.
    /// @param blockNumber The block number of the Tron block.
    function _encodeTronBlockHeaderInto(bytes memory buf, TronBlockMetadata memory tronBlock, uint256 blockNumber)
        internal
        view
    {
        bytes32 parentHash = tronBlock.parentHash;
        bytes32 txTrieRoot = tronBlock.txTrieRoot;

        // Tron raw header uses timestamp in milliseconds.
        uint256 tsMillis = uint256(tronBlock.timestamp) * 1000;

        // Resolve the Tron witness account address (owner) from the static SR set and convert it into
        // a Tron-style witness address (0x41 prefix + 20-byte EVM address).
        // Note: `srs` encodes the owner accounts that appear in `BlockHeader_raw.witnessAddress`,
        // while `witnessDelegatees` holds the actual signing keys (which may be delegated to that account).
        bytes20 witness = srs[tronBlock.witnessAddressIndex];
        // Treat the witness address as a 160-bit integer so we can explicitly
        // write its 20 bytes in big-endian order without relying on Solidity's
        // internal memory layout for bytes20.
        uint160 witness160 = uint160(witness);

        // without assembly, this parsing logic would be incredibly complex and expensive
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let base := add(buf, 32)
            let ptr := base

            // -----------------------------------------------------------------
            // field 1: timestamp (varint, key = (1 << 3) | 0 = 0x08)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x08)
            ptr := add(ptr, 1)

            // Varint-encode tsMillis (always non-negative, fits in uint64).
            let v := tsMillis
            for {} gt(v, 0x7f) {} {
                mstore8(ptr, or(and(v, 0x7f), 0x80))
                ptr := add(ptr, 1)
                v := shr(7, v)
            }
            mstore8(ptr, and(v, 0x7f))
            ptr := add(ptr, 1)

            // -----------------------------------------------------------------
            // field 2: txTrieRoot (bytes, key = (2 << 3) | 2 = 0x12)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x12) // key
            ptr := add(ptr, 1)
            mstore8(ptr, 32) // length
            ptr := add(ptr, 1)
            mstore(ptr, txTrieRoot)
            ptr := add(ptr, 32)

            // -----------------------------------------------------------------
            // field 3: parentHash (bytes, key = (3 << 3) | 2 = 0x1a)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x1a) // key
            ptr := add(ptr, 1)
            mstore8(ptr, 32) // length
            ptr := add(ptr, 1)
            mstore(ptr, parentHash)
            ptr := add(ptr, 32)

            // -----------------------------------------------------------------
            // field 7: number (varint, key = (7 << 3) | 0 = 0x38)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x38)
            ptr := add(ptr, 1)
            {
                let vNum := blockNumber
                for {} gt(vNum, 0x7f) {} {
                    mstore8(ptr, or(and(vNum, 0x7f), 0x80))
                    ptr := add(ptr, 1)
                    vNum := shr(7, vNum)
                }
                mstore8(ptr, and(vNum, 0x7f))
                ptr := add(ptr, 1)
            }

            // -----------------------------------------------------------------
            // field 9: witnessAddress (bytes, key = (9 << 3) | 2 = 0x4a)
            // Tron witness address is 21 bytes: 0x41 prefix + 20-byte EVM addr.
            // -----------------------------------------------------------------
            mstore8(ptr, 0x4a) // key
            ptr := add(ptr, 1)
            mstore8(ptr, 21) // length
            ptr := add(ptr, 1)

            // 0x41 prefix
            mstore8(ptr, 0x41)
            ptr := add(ptr, 1)

            // Write the 20-byte witness address immediately after the prefix.
            // We can write in one shot by shifting it into the high 20 bytes of a word.
            mstore(ptr, shl(96, witness160))
            ptr := add(ptr, 20)

            // -----------------------------------------------------------------
            // field 10: version (varint, key = (10 << 3) | 0 = 0x50)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x50)
            ptr := add(ptr, 1)
            // Current mainnet value is 32, which fits in a single varint byte.
            mstore8(ptr, _TRON_BLOCK_VERSION)
            ptr := add(ptr, 1)

            // Set the actual length of the bytes buffer to the number of bytes written.
            let used := sub(ptr, base)
            mstore(buf, used)
        }
    }

    /* solhint-enable function-max-lines */

    /// @notice Checks if a block is already stored as an anchor
    /// @param blockNumber The block number to check
    /// @param startingBlock The expected block ID for this block number
    /// @return True if the block is stored and matches the expected block ID, false otherwise
    function _isStoredAnchor(uint256 blockNumber, bytes32 startingBlock) internal view returns (bool) {
        bytes32 startingSlot = _blockIds[blockNumber];
        if (startingSlot != bytes32(0)) {
            if (startingSlot != startingBlock) revert InvalidChain();
            return true;
        }
        return false;
    }

    /// @notice Checks if a block intersects with an existing block in storage
    /// @param ctx The proof context to update if intersection is found
    /// @param blockNumber The block number to check
    /// @param blockId The block ID to verify against
    function _checkIntersection(ProveBlocksCtx memory ctx, uint256 blockNumber, bytes32 blockId) internal view {
        bytes32 existing = _blockIds[blockNumber];
        if (existing != bytes32(0)) {
            if (existing != blockId) revert InvalidChain();
            ctx.intersectedExisting = true;
        }
    }

    /// @notice Main loop for proving blocks in a range
    /// @param ctx The proof context containing state during verification
    /// @param compressedTronBlockMetadata Compressed block metadata for all blocks
    /// @param compressedSignatures Compressed signatures for all blocks
    /// @param numBlocks Number of blocks to prove
    function _proveBlocksLoop(
        ProveBlocksCtx memory ctx,
        bytes calldata compressedTronBlockMetadata,
        bytes calldata compressedSignatures,
        uint256 numBlocks
    ) internal view {
        // Scratch buffer reused across all blocks in the loop to avoid per-block `new bytes(128)` allocations
        // and the associated repeated memory expansion.
        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            TronBlockMetadata memory tronBlock = _decodeTronBlockAt(compressedTronBlockMetadata, i);

            if (tronBlock.parentHash != ctx.blockId) {
                revert InvalidParentBlockId(tronBlock.parentHash, ctx.blockId);
            }

            unchecked {
                ++ctx.blockNumber;
            }

            bytes32 blockHash = _hashBlockScratch(tronBlock, ctx.blockNumber, scratch);
            ctx.blockId = _makeBlockId(ctx.blockNumber, blockHash);

            bytes20 signer = _recoverSigner(blockHash, compressedSignatures, i, scratch);
            uint8 witnessIndex = tronBlock.witnessAddressIndex;
            if (signer != witnessDelegatees[witnessIndex]) revert InvalidWitnessSigner();

            uint8 group = uint8((_DELEGATEE_GROUPS_PACKED >> (uint256(witnessIndex) * 5)) & 0x1f);
            _enforceRecentUniqueGroup(ctx, group, signer);
            if (!ctx.intersectedExisting) {
                _checkIntersection(ctx, ctx.blockNumber, ctx.blockId);
            }

            ctx.lastTxTrieRoot = tronBlock.txTrieRoot;
            ctx.lastTimestamp = tronBlock.timestamp;
        }
    }

    /// @notice Recovers the signer address from a block hash and signature
    /// @param blockHash The hash of the block that was signed
    /// @param sigs Array of signatures (concatenated r, s, v values)
    /// @param i Index of the signature to recover
    /// @param scratch Scratch memory region to use for ecrecover input (must have >= 128 bytes capacity).
    /// @return signer The address of the signer
    function _recoverSigner(bytes32 blockHash, bytes calldata sigs, uint256 i, bytes memory scratch)
        internal
        view
        returns (bytes20 signer)
    {
        bytes32 r;
        bytes32 s;
        uint8 v;

        // signature layout: [r(32) | s(32) | v(1)]
        // load r, s, and the first byte at offset+64 as v
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let off := add(sigs.offset, mul(i, 65))
            r := calldataload(off)
            s := calldataload(add(off, 32))
            v := byte(0, calldataload(add(off, 64)))
        }
        unchecked {
            if (v < 27) v += 27;
        }

        // Basic signature validity checks
        if (v != 27 && v != 28) revert InvalidWitnessSigner();
        if (uint256(r) == 0) revert InvalidWitnessSigner();

        uint256 sNum = uint256(s);
        if (sNum == 0 || sNum > _SECP256K1N_HALF) revert InvalidWitnessSigner();

        bool ok;
        address recovered;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            let ptr := add(scratch, 32)

            mstore(ptr, blockHash)
            mstore(add(ptr, 32), v)
            mstore(add(ptr, 64), r)
            mstore(add(ptr, 96), s)

            ok := staticcall(10000, 0x01, ptr, 128, ptr, 32)
            recovered := and(mload(ptr), 0xffffffffffffffffffffffffffffffffffffffff)
        }

        if (!ok || recovered == address(0)) revert InvalidWitnessSigner();

        signer = bytes20(recovered);
    }

    // Conversion & helper (internal pure) functions

    /// @notice Creates a block ID by combining block number and block hash
    /// @param blockNumber The block number
    /// @param blockHash The block hash
    /// @return A bytes32 value containing the block number and hash
    function _makeBlockId(uint256 blockNumber, bytes32 blockHash) internal pure returns (bytes32) {
        uint256 tail = uint256(blockHash) & ((uint256(1) << 192) - 1);
        return bytes32((blockNumber << 192) | tail);
    }

    /// @notice Enforces that a delegatee-group hasn't produced a block recently and tracks it.
    /// @dev This is robust even if multiple witness indices share the same delegatee key.
    /// @param ctx The proof context containing recent creator tracking state.
    /// @param group Delegatee-group id (0..26).
    /// @param signer The recovered signer (only used for revert data).
    function _enforceRecentUniqueGroup(ProveBlocksCtx memory ctx, uint8 group, bytes20 signer) internal pure {
        uint32 bit = uint32(1) << group;
        if ((ctx.recentMask & bit) != 0) revert WitnessProducedRecently(signer);

        if (ctx.recentCount < _RECENT_BLOCK_CREATOR_WINDOW) {
            ctx.recentGroups[ctx.recentCount] = group;
            ctx.recentMask |= bit;
            unchecked {
                ++ctx.recentCount;
            }
        } else {
            uint8 old = ctx.recentGroups[ctx.recentPos];
            ctx.recentMask &= ~(uint32(1) << old);

            ctx.recentGroups[ctx.recentPos] = group;
            ctx.recentMask |= bit;

            unchecked {
                ++ctx.recentPos;
                if (ctx.recentPos == _RECENT_BLOCK_CREATOR_WINDOW) ctx.recentPos = 0;
            }
        }
    }

    /// @notice Extracts the Tron block height encoded in a `blockId`.
    /// @param blockId Tron `blockId` (`uint64(blockNumber) || sha256(header)[8:]`).
    /// @return blockNumber Tron block height.
    function _blockIdToNumber(bytes32 blockId) internal pure returns (uint256 blockNumber) {
        // In Tron, blockId is uint64(blockNumber) || sha256(BlockHeader_raw)[8:]
        return uint256(blockId) >> 192;
    }

    /// @dev Decode the Tron block metadata at a given index from a tightly packed calldata blob.
    /// Each block occupies TRON_BLOCK_METADATA_SIZE bytes, with layout:
    /// [0..31]  parentHash (bytes32)
    /// [32..63] txTrieRoot (bytes32)
    /// [64..67] timestamp (uint32, big-endian)
    /// [68]     witnessAddressIndex (uint8)
    /// @notice Decodes a single 69-byte metadata chunk from `data` at `index`.
    /// @param data Tightly packed calldata blob of metadata.
    /// @param index 0-based index of the block within `data`.
    /// @return tronBlock Decoded metadata struct for the requested block.
    function _decodeTronBlockAt(bytes calldata data, uint256 index)
        internal
        pure
        returns (TronBlockMetadata memory tronBlock)
    {
        uint256 offset = index * _TRON_BLOCK_METADATA_SIZE;

        bytes32 parentHash;
        bytes32 txTrieRoot;
        uint32 ts;
        uint8 witnessIndex;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            let base := add(data.offset, offset)

            parentHash := calldataload(base)
            txTrieRoot := calldataload(add(base, 32))

            // Load the 32-byte word starting at byte 64; we only care about:
            // - the first 4 bytes (big-endian timestamp)
            // - the 5th byte (witness index)
            let word := calldataload(add(base, 64))
            // High 4 bytes of `word` -> timestamp (seconds).
            ts := shr(224, word) // 28 * 8 = 224, keep top 4 bytes
            // 5th byte (index 4 from the MSB side) -> witnessIndex.
            witnessIndex := byte(4, word)
        }

        tronBlock.parentHash = parentHash;
        tronBlock.txTrieRoot = txTrieRoot;
        tronBlock.timestamp = ts;
        tronBlock.witnessAddressIndex = witnessIndex;
    }
}
