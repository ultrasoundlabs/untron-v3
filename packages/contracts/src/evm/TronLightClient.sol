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
    // ------------------------------------------------------------------------
    // Types
    // ------------------------------------------------------------------------

    struct ProveBlocksCtx {
        bytes32 blockId;
        uint256 blockNumber;

        bytes32 lastTxTrieRoot;

        // Ring buffer (last 18 creators) tracked by delegatee-group id
        // (indices with the same delegatee key share the same group id).
        uint8[18] recentGroups;
        uint32 recentMask;

        bool intersectedExisting;
        uint32 lastTimestamp;
        uint8 recentCount;
        uint8 recentPos;
    }

    // ------------------------------------------------------------------------
    // Constants
    // ------------------------------------------------------------------------

    uint256 internal constant _TRON_BLOCK_METADATA_SIZE = 69;
    uint256 internal constant _SIGNATURE_SIZE = 65; // bytes per secp256k1 signature (r,s,v)

    uint256 internal constant _TRON_BLOCK_VERSION = 32; // current observed Tron BlockHeader_raw.version
    uint256 internal constant _RECENT_BLOCK_CREATOR_WINDOW = 18; // sliding window size for block creator uniqueness

    // ------------------------------------------------------------------------
    // State
    // ------------------------------------------------------------------------

    /// @notice Verifier used to validate succinct proofs for block ranges.
    IBlockRangeProver public immutable BLOCK_RANGE_PROVER;

    // ------------------------------------------------------------------------
    // SR owners + witness delegatees as per-index immutables (fastest runtime reads)
    // ------------------------------------------------------------------------

    /// @notice SR owner accounts for the epoch (used for `witnessAddress` in header encoding).
    /// @dev These are the owner accounts that appear (with 0x41 prefix) in `BlockHeader_raw.witnessAddress`.
    bytes20 internal immutable _SR_0;
    bytes20 internal immutable _SR_1;
    bytes20 internal immutable _SR_2;
    bytes20 internal immutable _SR_3;
    bytes20 internal immutable _SR_4;
    bytes20 internal immutable _SR_5;
    bytes20 internal immutable _SR_6;
    bytes20 internal immutable _SR_7;
    bytes20 internal immutable _SR_8;
    bytes20 internal immutable _SR_9;
    bytes20 internal immutable _SR_10;
    bytes20 internal immutable _SR_11;
    bytes20 internal immutable _SR_12;
    bytes20 internal immutable _SR_13;
    bytes20 internal immutable _SR_14;
    bytes20 internal immutable _SR_15;
    bytes20 internal immutable _SR_16;
    bytes20 internal immutable _SR_17;
    bytes20 internal immutable _SR_18;
    bytes20 internal immutable _SR_19;
    bytes20 internal immutable _SR_20;
    bytes20 internal immutable _SR_21;
    bytes20 internal immutable _SR_22;
    bytes20 internal immutable _SR_23;
    bytes20 internal immutable _SR_24;
    bytes20 internal immutable _SR_25;
    bytes20 internal immutable _SR_26;

    /// @notice SR signing keys for the epoch (used for signature recovery checks).
    /// @dev A given SR may delegate its witness permission to a separate key; those delegatees live here.
    bytes20 internal immutable _WITNESS_DELEGATEE_0;
    bytes20 internal immutable _WITNESS_DELEGATEE_1;
    bytes20 internal immutable _WITNESS_DELEGATEE_2;
    bytes20 internal immutable _WITNESS_DELEGATEE_3;
    bytes20 internal immutable _WITNESS_DELEGATEE_4;
    bytes20 internal immutable _WITNESS_DELEGATEE_5;
    bytes20 internal immutable _WITNESS_DELEGATEE_6;
    bytes20 internal immutable _WITNESS_DELEGATEE_7;
    bytes20 internal immutable _WITNESS_DELEGATEE_8;
    bytes20 internal immutable _WITNESS_DELEGATEE_9;
    bytes20 internal immutable _WITNESS_DELEGATEE_10;
    bytes20 internal immutable _WITNESS_DELEGATEE_11;
    bytes20 internal immutable _WITNESS_DELEGATEE_12;
    bytes20 internal immutable _WITNESS_DELEGATEE_13;
    bytes20 internal immutable _WITNESS_DELEGATEE_14;
    bytes20 internal immutable _WITNESS_DELEGATEE_15;
    bytes20 internal immutable _WITNESS_DELEGATEE_16;
    bytes20 internal immutable _WITNESS_DELEGATEE_17;
    bytes20 internal immutable _WITNESS_DELEGATEE_18;
    bytes20 internal immutable _WITNESS_DELEGATEE_19;
    bytes20 internal immutable _WITNESS_DELEGATEE_20;
    bytes20 internal immutable _WITNESS_DELEGATEE_21;
    bytes20 internal immutable _WITNESS_DELEGATEE_22;
    bytes20 internal immutable _WITNESS_DELEGATEE_23;
    bytes20 internal immutable _WITNESS_DELEGATEE_24;
    bytes20 internal immutable _WITNESS_DELEGATEE_25;
    bytes20 internal immutable _WITNESS_DELEGATEE_26;

    /// @notice Packed mapping witnessIndex -> delegateeGroup (5 bits per index, 27 indices total).
    /// @dev Indices that share the same delegatee key share the same group id.
    ///      This lets us do "recently produced" checks by group without assuming delegatees are unique.
    uint256 internal immutable _DELEGATEE_GROUPS_PACKED;

    /// @notice ZK-friendly hash of public keys (not addresses!) of SRs and their delegatees.
    /// @dev This hash is used as a public input to the block range prover, to save proving cycles compared to
    ///      passing two arrays of Keccak-hashed arrays (as above).
    bytes32 public immutable SR_DATA_HASH;

    /// @notice Highest stored/proven Tron `blockId` by embedded block number.
    /// @dev This is monotonic w.r.t. the embedded height (`uint64(blockNumber)` in the top 8 bytes).
    bytes32 public latestProvenBlock;

    mapping(uint256 blockNumber => bytes32 blockId) internal _blockIds;
    mapping(uint256 blockNumber => bytes32 txTrieRoot) internal _txTrieRoots;
    mapping(uint256 blockNumber => uint32 timestamp) internal _blockTimestamps;

    // ------------------------------------------------------------------------
    // Errors
    // ------------------------------------------------------------------------

    error InvalidParentBlockId(bytes32 yours, bytes32 real);
    error BlockTooOld();
    error InvalidChain();
    error BlockNotRelayed();
    error InvalidCompressedTronBlockMetadataLength();
    error InvalidCompressedSignaturesLength();
    error InvalidIntersectionOffset(uint256 intersectionOffset, uint256 numBlocks);
    error InvalidIntersectionClaim(uint256 blockNumber, bytes32 blockId);
    error InvalidWitnessSigner();
    error WitnessProducedRecently(bytes20 signer);
    error UnanchoredBlockRange();
    error InvalidSrIndex(uint256 index);
    error InvalidWitnessDelegateeIndex(uint256 index);
    error Sha256PrecompileFailed();

    // ------------------------------------------------------------------------
    // Constructor
    // ------------------------------------------------------------------------

    /* solhint-disable function-max-lines */
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

        _SR_0 = _srs[0];
        _SR_1 = _srs[1];
        _SR_2 = _srs[2];
        _SR_3 = _srs[3];
        _SR_4 = _srs[4];
        _SR_5 = _srs[5];
        _SR_6 = _srs[6];
        _SR_7 = _srs[7];
        _SR_8 = _srs[8];
        _SR_9 = _srs[9];
        _SR_10 = _srs[10];
        _SR_11 = _srs[11];
        _SR_12 = _srs[12];
        _SR_13 = _srs[13];
        _SR_14 = _srs[14];
        _SR_15 = _srs[15];
        _SR_16 = _srs[16];
        _SR_17 = _srs[17];
        _SR_18 = _srs[18];
        _SR_19 = _srs[19];
        _SR_20 = _srs[20];
        _SR_21 = _srs[21];
        _SR_22 = _srs[22];
        _SR_23 = _srs[23];
        _SR_24 = _srs[24];
        _SR_25 = _srs[25];
        _SR_26 = _srs[26];

        _WITNESS_DELEGATEE_0 = _witnessDelegatees[0];
        _WITNESS_DELEGATEE_1 = _witnessDelegatees[1];
        _WITNESS_DELEGATEE_2 = _witnessDelegatees[2];
        _WITNESS_DELEGATEE_3 = _witnessDelegatees[3];
        _WITNESS_DELEGATEE_4 = _witnessDelegatees[4];
        _WITNESS_DELEGATEE_5 = _witnessDelegatees[5];
        _WITNESS_DELEGATEE_6 = _witnessDelegatees[6];
        _WITNESS_DELEGATEE_7 = _witnessDelegatees[7];
        _WITNESS_DELEGATEE_8 = _witnessDelegatees[8];
        _WITNESS_DELEGATEE_9 = _witnessDelegatees[9];
        _WITNESS_DELEGATEE_10 = _witnessDelegatees[10];
        _WITNESS_DELEGATEE_11 = _witnessDelegatees[11];
        _WITNESS_DELEGATEE_12 = _witnessDelegatees[12];
        _WITNESS_DELEGATEE_13 = _witnessDelegatees[13];
        _WITNESS_DELEGATEE_14 = _witnessDelegatees[14];
        _WITNESS_DELEGATEE_15 = _witnessDelegatees[15];
        _WITNESS_DELEGATEE_16 = _witnessDelegatees[16];
        _WITNESS_DELEGATEE_17 = _witnessDelegatees[17];
        _WITNESS_DELEGATEE_18 = _witnessDelegatees[18];
        _WITNESS_DELEGATEE_19 = _witnessDelegatees[19];
        _WITNESS_DELEGATEE_20 = _witnessDelegatees[20];
        _WITNESS_DELEGATEE_21 = _witnessDelegatees[21];
        _WITNESS_DELEGATEE_22 = _witnessDelegatees[22];
        _WITNESS_DELEGATEE_23 = _witnessDelegatees[23];
        _WITNESS_DELEGATEE_24 = _witnessDelegatees[24];
        _WITNESS_DELEGATEE_25 = _witnessDelegatees[25];
        _WITNESS_DELEGATEE_26 = _witnessDelegatees[26];

        SR_DATA_HASH = srDataHash;
    }

    /* solhint-enable function-max-lines */

    // ------------------------------------------------------------------------
    // External functions
    // ------------------------------------------------------------------------

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
    /// @param intersectionOffset 0-based index `i` (into the provided `N` blocks) at which to check for intersection with existing storage.
    ///        Note: `i == 0` corresponds to `startingBlockNumber + 1`, so `intersectionBlockNumber = startingBlockNumber + 1 + intersectionOffset`.
    ///        Sentinel: if `startingBlock` is already stored (anchored), callers MUST pass `type(uint256).max`.
    ///        Otherwise, callers MUST pass an offset within the range that corresponds to an already-stored block with the exact same `blockId`,
    ///        or the call will revert.
    function proveBlocks(
        bytes32 startingBlock,
        bytes calldata compressedTronBlockMetadata,
        bytes calldata compressedSignatures,
        uint256 intersectionOffset
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

        // Enforce a verifiable sentinel for anchored starts:
        // - If `startingBlock` is anchored, intersection is unnecessary, and callers MUST pass `type(uint256).max`.
        // - If `startingBlock` is unanchored, callers MUST provide an in-range offset where we will check exactly one storage slot.
        if (ctx.intersectedExisting) {
            if (intersectionOffset != type(uint256).max) {
                revert InvalidIntersectionOffset(intersectionOffset, numBlocks);
            }
        } else {
            // solhint-disable-next-line gas-strict-inequalities
            if (intersectionOffset >= numBlocks) revert InvalidIntersectionOffset(intersectionOffset, numBlocks);
        }

        _proveBlocksLoop(ctx, compressedTronBlockMetadata, compressedSignatures, numBlocks, intersectionOffset);

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

    /// @notice Returns the SR owner account for `index`.
    /// @dev This matches the prior auto-generated getter ABI of `bytes20[27] public srs`.
    /// @param index SR index (0..26).
    /// @return sr SR owner account (EVM address bytes).
    function srs(uint256 index) external view returns (bytes20 sr) {
        return _srAt(index);
    }

    /// @notice Returns the witness delegatee (signing key) for `index`.
    /// @dev This matches the prior auto-generated getter ABI of `bytes20[27] public witnessDelegatees`.
    /// @param index SR index (0..26).
    /// @return delegatee Delegatee signing key (EVM address bytes).
    function witnessDelegatees(uint256 index) external view returns (bytes20 delegatee) {
        return _witnessDelegateeAt(index);
    }

    /// @notice Returns the stored transaction trie root for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return txTrieRoot Stored transaction trie root at that height.
    function getTxTrieRoot(uint256 blockNumber) external view returns (bytes32 txTrieRoot) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return _txTrieRoots[blockNumber];
    }

    /// @notice Returns the stored block timestamp (seconds) for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return timestamp Stored block timestamp in seconds.
    function getBlockTimestamp(uint256 blockNumber) external view returns (uint32 timestamp) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return _blockTimestamps[blockNumber];
    }

    // ------------------------------------------------------------------------
    // Public functions
    // ------------------------------------------------------------------------

    /// @notice Returns the stored Tron `blockId` for `blockNumber`.
    /// @param blockNumber Tron block height to query.
    /// @return blockId Stored Tron `blockId` at that height.
    function getBlockId(uint256 blockNumber) public view returns (bytes32 blockId) {
        bytes32 stored = _blockIds[blockNumber];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        return stored;
    }

    // ------------------------------------------------------------------------
    // Internal functions
    // ------------------------------------------------------------------------

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

    /// @notice Returns the SR owner account at `index`.
    /// @dev Implemented as an if-ladder so the optimizer can flatten it; avoids `SLOAD`.
    /// @param index SR index (0..26).
    /// @return sr SR owner account (EVM address bytes).
    function _srAt(uint256 index) internal view returns (bytes20 sr) {
        if (index == 0) return _SR_0;
        if (index == 1) return _SR_1;
        if (index == 2) return _SR_2;
        if (index == 3) return _SR_3;
        if (index == 4) return _SR_4;
        if (index == 5) return _SR_5;
        if (index == 6) return _SR_6;
        if (index == 7) return _SR_7;
        if (index == 8) return _SR_8;
        if (index == 9) return _SR_9;
        if (index == 10) return _SR_10;
        if (index == 11) return _SR_11;
        if (index == 12) return _SR_12;
        if (index == 13) return _SR_13;
        if (index == 14) return _SR_14;
        if (index == 15) return _SR_15;
        if (index == 16) return _SR_16;
        if (index == 17) return _SR_17;
        if (index == 18) return _SR_18;
        if (index == 19) return _SR_19;
        if (index == 20) return _SR_20;
        if (index == 21) return _SR_21;
        if (index == 22) return _SR_22;
        if (index == 23) return _SR_23;
        if (index == 24) return _SR_24;
        if (index == 25) return _SR_25;
        if (index == 26) return _SR_26;
        revert InvalidSrIndex(index);
    }

    /// @notice Returns the witness delegatee (signing key) at `index`.
    /// @dev Implemented as an if-ladder so the optimizer can flatten it; avoids `SLOAD`.
    /// @param index SR index (0..26).
    /// @return delegatee Delegatee signing key (EVM address bytes).
    function _witnessDelegateeAt(uint256 index) internal view returns (bytes20 delegatee) {
        if (index == 0) return _WITNESS_DELEGATEE_0;
        if (index == 1) return _WITNESS_DELEGATEE_1;
        if (index == 2) return _WITNESS_DELEGATEE_2;
        if (index == 3) return _WITNESS_DELEGATEE_3;
        if (index == 4) return _WITNESS_DELEGATEE_4;
        if (index == 5) return _WITNESS_DELEGATEE_5;
        if (index == 6) return _WITNESS_DELEGATEE_6;
        if (index == 7) return _WITNESS_DELEGATEE_7;
        if (index == 8) return _WITNESS_DELEGATEE_8;
        if (index == 9) return _WITNESS_DELEGATEE_9;
        if (index == 10) return _WITNESS_DELEGATEE_10;
        if (index == 11) return _WITNESS_DELEGATEE_11;
        if (index == 12) return _WITNESS_DELEGATEE_12;
        if (index == 13) return _WITNESS_DELEGATEE_13;
        if (index == 14) return _WITNESS_DELEGATEE_14;
        if (index == 15) return _WITNESS_DELEGATEE_15;
        if (index == 16) return _WITNESS_DELEGATEE_16;
        if (index == 17) return _WITNESS_DELEGATEE_17;
        if (index == 18) return _WITNESS_DELEGATEE_18;
        if (index == 19) return _WITNESS_DELEGATEE_19;
        if (index == 20) return _WITNESS_DELEGATEE_20;
        if (index == 21) return _WITNESS_DELEGATEE_21;
        if (index == 22) return _WITNESS_DELEGATEE_22;
        if (index == 23) return _WITNESS_DELEGATEE_23;
        if (index == 24) return _WITNESS_DELEGATEE_24;
        if (index == 25) return _WITNESS_DELEGATEE_25;
        if (index == 26) return _WITNESS_DELEGATEE_26;
        revert InvalidWitnessDelegateeIndex(index);
    }

    /// @notice Checks if a block is already stored as an anchor.
    /// @param blockNumber The block number to check.
    /// @param startingBlock The expected block ID for this block number.
    /// @return isAnchor True if the block is stored and matches the expected block ID.
    function _isStoredAnchor(uint256 blockNumber, bytes32 startingBlock) internal view returns (bool isAnchor) {
        bytes32 startingSlot = _blockIds[blockNumber];
        if (startingSlot != bytes32(0)) {
            if (startingSlot != startingBlock) revert InvalidChain();
            return true;
        }
        return false;
    }

    /// @notice Conditionally checks for intersection at the caller-specified offset.
    /// @param ctx The proof context containing state during verification.
    /// @param i The index of the block being checked.
    /// @param intersectionOffset The offset of the intersection block.
    function _maybeCheckIntersection(ProveBlocksCtx memory ctx, uint256 i, uint256 intersectionOffset) internal view {
        // If the starting block wasn't anchored, the caller must point us at the ONE block in this range
        // that should already exist in storage (same blockNumber + exact blockId). We do exactly one read.
        if (ctx.intersectedExisting || i != intersectionOffset) return;

        bytes32 existing = _blockIds[ctx.blockNumber];

        // Since the caller claims this is the intersection slot, fail immediately if it's not actually anchored.
        if (existing == bytes32(0)) revert InvalidIntersectionClaim(ctx.blockNumber, ctx.blockId);

        // If it's anchored, it must match exactly (otherwise the caller is trying to splice chains).
        if (existing != ctx.blockId) revert InvalidChain();

        ctx.intersectedExisting = true;
    }

    /// @notice Main loop for proving blocks in a range.
    /// @param ctx The proof context containing state during verification.
    /// @param compressedTronBlockMetadata Compressed block metadata for all blocks.
    /// @param compressedSignatures Compressed signatures for all blocks.
    /// @param numBlocks Number of blocks to prove.
    /// @param intersectionOffset The offset of the intersection block.
    function _proveBlocksLoop(
        ProveBlocksCtx memory ctx,
        bytes calldata compressedTronBlockMetadata,
        bytes calldata compressedSignatures,
        uint256 numBlocks,
        uint256 intersectionOffset
    ) internal view {
        // Scratch buffer reused across all blocks in the loop to avoid per-block `new bytes(128)` allocations
        // and the associated repeated memory expansion.
        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 blockHash, uint8 witnessIndex) = _advanceAndHash(ctx, compressedTronBlockMetadata, i, scratch);

            bytes20 signer = _recoverSigner(blockHash, compressedSignatures, i, scratch);
            if (signer != _witnessDelegateeAt(witnessIndex)) revert InvalidWitnessSigner();

            uint8 group = uint8((_DELEGATEE_GROUPS_PACKED >> (uint256(witnessIndex) * 5)) & 0x1f);
            _enforceRecentUniqueGroup(ctx, group, signer);

            _maybeCheckIntersection(ctx, i, intersectionOffset);
        }
    }

    /// @notice Decodes, validates, advances `ctx.blockNumber`, hashes the block, and updates `ctx`.
    /// @param ctx The context to update.
    /// @param compressedTronBlockMetadata The compressed Tron block metadata.
    /// @param i The index of the block to decode.
    /// @param scratch The scratch memory region to use for encoding.
    /// @return blockHash The hash of the block.
    /// @return witnessIndex The witness index of the block.
    function _advanceAndHash(
        ProveBlocksCtx memory ctx,
        bytes calldata compressedTronBlockMetadata,
        uint256 i,
        bytes memory scratch
    ) internal view returns (bytes32 blockHash, uint8 witnessIndex) {
        (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts, uint8 wi) =
            _decodeTronBlockAtStack(compressedTronBlockMetadata, i);
        witnessIndex = wi;

        if (parentHash != ctx.blockId) revert InvalidParentBlockId(parentHash, ctx.blockId);

        unchecked {
            ++ctx.blockNumber;
        }

        blockHash = _hashBlockScratch(parentHash, txTrieRoot, ts, wi, ctx.blockNumber, scratch);
        ctx.blockId = _makeBlockId(ctx.blockNumber, blockHash);

        ctx.lastTxTrieRoot = txTrieRoot;
        ctx.lastTimestamp = ts;
    }

    /// @notice Computes the Tron `blockHash` (SHA256 of encoded `BlockHeader_raw`) from raw stack values.
    /// @param parentHash The parent hash of the block.
    /// @param txTrieRoot The transaction trie root of the block.
    /// @param timestampSec The timestamp of the block.
    /// @param witnessIndex The witness address index of the block.
    /// @param blockNumber The block number of the block.
    /// @param scratch The scratch memory region to use for encoding.
    /// @return blockHash The hash of the block.
    function _hashBlockScratch(
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestampSec,
        uint8 witnessIndex,
        uint256 blockNumber,
        bytes memory scratch
    ) internal view returns (bytes32 blockHash) {
        uint256 used = _encodeTronBlockHeaderInto(
            scratch, parentHash, txTrieRoot, timestampSec, witnessIndex, blockNumber
        );

        // Call SHA256 precompile (0x02) directly on the scratch region.
        // Output is written to memory slot 0x00 to avoid moving the free memory pointer.
        bool ok;

        // solhint-disable-next-line no-inline-assembly
        assembly {
            ok := staticcall(gas(), 0x02, add(scratch, 32), used, 0x00, 32)
            blockHash := mload(0x00)
        }

        if (!ok) revert Sha256PrecompileFailed();
    }

    /* solhint-disable function-max-lines */

    /// @notice Encodes a minimal Tron `BlockHeader_raw` protobuf message into an existing bytes buffer.
    /// @param buf The buffer to write to.
    /// @param parentHash The parent hash of the block.
    /// @param txTrieRoot The transaction trie root of the block.
    /// @param timestampSec The timestamp of the block in seconds.
    /// @param witnessIndex The index of the witness address in the static SR set.
    /// @param blockNumber The block number.
    /// @return used The number of bytes written to the buffer.
    function _encodeTronBlockHeaderInto(
        bytes memory buf,
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestampSec,
        uint8 witnessIndex,
        uint256 blockNumber
    ) internal view returns (uint256 used) {
        // Tron raw header uses timestamp in milliseconds.
        uint256 tsMillis = uint256(timestampSec) * 1000;

        // Resolve the Tron witness account address (owner) from the static SR set and convert it into
        // a Tron-style witness address (0x41 prefix + 20-byte EVM address).
        // Note: `_srAt` resolves the owner accounts that appear in `BlockHeader_raw.witnessAddress`,
        // while `_witnessDelegateeAt` holds the actual signing keys (which may be delegated to that account).
        bytes20 witness = _srAt(witnessIndex);
        // Treat the witness address as a 160-bit integer so we can explicitly
        // write its 20 bytes in big-endian order without relying on Solidity's
        // internal memory layout for bytes20.
        uint160 witness160 = uint160(witness);

        // Without assembly, this protobuf encoding would be much more complex and expensive.
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

            // Return the number of bytes written (do not mutate `buf.length` here).
            used := sub(ptr, base)
        }
    }

    /* solhint-enable function-max-lines */

    /// @notice Recovers the signer address from a block hash and signature.
    /// @param blockHash The hash of the block that was signed.
    /// @param sigs Array of signatures (concatenated r, s, v values).
    /// @param i Index of the signature to recover.
    /// @param scratch Scratch memory region to use for ecrecover input (must have >= 128 bytes capacity).
    /// @return signer The recovered signer (20-byte EVM address).
    function _recoverSigner(bytes32 blockHash, bytes calldata sigs, uint256 i, bytes memory scratch)
        internal
        view
        returns (bytes20 signer)
    {
        bytes32 r;
        bytes32 s;
        uint8 v;

        // Signature layout: [r(32) | s(32) | v(1)]
        // Load r, s, and the first byte at offset+64 as v.
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let off := add(sigs.offset, mul(i, 65))
            r := calldataload(off)
            s := calldataload(add(off, 32))
            v := byte(0, calldataload(add(off, 64)))
        }

        // Normalize v: allow 0/1 or 27/28.
        unchecked {
            if (v < 27) v += 27;
        }

        // Keep a strict v range check to avoid wasting the precompile call on garbage values.
        if (v != 27 && v != 28) revert InvalidWitnessSigner();

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

    /// @notice Creates a Tron `blockId` by combining block number and block hash.
    /// @param blockNumber Tron block height.
    /// @param blockHash Tron `blockHash` (`sha256(BlockHeader_raw)`).
    /// @return blockId Tron `blockId` (`uint64(blockNumber) || sha256(header)[8:]`).
    function _makeBlockId(uint256 blockNumber, bytes32 blockHash) internal pure returns (bytes32 blockId) {
        uint256 tail = uint256(blockHash) & ((uint256(1) << 192) - 1);
        return bytes32((blockNumber << 192) | tail);
    }

    /// @notice Extracts the Tron block height encoded in a `blockId`.
    /// @param blockId Tron `blockId` (`uint64(blockNumber) || sha256(header)[8:]`).
    /// @return blockNumber Tron block height.
    function _blockIdToNumber(bytes32 blockId) internal pure returns (uint256 blockNumber) {
        // In Tron, blockId is uint64(blockNumber) || sha256(BlockHeader_raw)[8:]
        return uint256(blockId) >> 192;
    }

    /// @notice Decodes a single 69-byte metadata chunk from `data` at `index` into stack values.
    /// @dev Decode layout per block:
    /// [0..31]  parentHash (bytes32)
    /// [32..63] txTrieRoot (bytes32)
    /// [64..67] timestamp (uint32, big-endian)
    /// [68]     witnessIndex (uint8)
    /// @param data Tightly packed calldata blob of metadata.
    /// @param index 0-based index of the block within `data`.
    /// @return parentHash The parent hash of the block.
    /// @return txTrieRoot The transaction trie root of the block.
    /// @return ts The timestamp of the block.
    /// @return witnessIndex The witness address index of the block.
    function _decodeTronBlockAtStack(bytes calldata data, uint256 index)
        internal
        pure
        returns (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts, uint8 witnessIndex)
    {
        uint256 offset = index * _TRON_BLOCK_METADATA_SIZE;

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
    }
}
