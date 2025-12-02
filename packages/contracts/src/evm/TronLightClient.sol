// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {IBlockRangeProver} from "./blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClient {
    // 1 day = 86400 seconds = 28800 blocks (3 sec each) = 4 Tron epochs (7200 each)
    uint256 internal constant LATEST_BLOCK_IDS_ARRAY_LENGTH = 28800;
    uint256 internal constant TRON_BLOCK_METADATA_SIZE = 69; // bytes per packed TronBlockMetadata
    uint256 internal constant SIGNATURE_SIZE = 65; // bytes per secp256k1 signature (r,s,v)
    uint256 internal constant TRON_BLOCK_VERSION = 32; // current observed Tron BlockHeader_raw.version

    IBlockRangeProver public immutable blockRangeProver;
    bytes20[27] public srs;
    bytes32 public latestProvenBlock;
    bytes32[LATEST_BLOCK_IDS_ARRAY_LENGTH] internal latestBlockIds;

    constructor(IBlockRangeProver _blockRangeProver, bytes32 initialBlockHash, bytes20[27] memory _srs) {
        blockRangeProver = _blockRangeProver;
        appendBlockId(initialBlockHash);
        srs = _srs;
    }

    struct TronBlockMetadata {
        bytes32 parentHash;
        bytes32 txTrieRoot;
        // In Tron protocol, it's expressed in milliseconds, but block time is 3s and actual time is never
        // more granular than .......000. So we can /1000 it and store efficiently in 32-bit ints (seconds).
        uint32 timestamp;
        uint8 witnessAddressIndex;
    }

    error NotEnoughBlocksOrSignatures();
    error InvalidParentBlockId(bytes32 yours, bytes32 real);
    error BlockTooOld();
    error InvalidChain();
    error BlockNotRelayed();
    error InvalidCompressedTronBlockMetadataLength();
    error InvalidCompressedSignaturesLength();
    error InvalidWitnessSigner();

    function proveBlocks(
        bytes32 startingBlock,
        bytes calldata compressedTronBlockMetadata,
        bytes calldata compressedSignatures
    ) external {
        uint256 tronBlocksLength = compressedTronBlockMetadata.length;
        if (tronBlocksLength == 0 || tronBlocksLength % TRON_BLOCK_METADATA_SIZE != 0) {
            revert InvalidCompressedTronBlockMetadataLength();
        }

        uint256 numBlocks = tronBlocksLength / TRON_BLOCK_METADATA_SIZE;

        if (compressedSignatures.length != numBlocks * SIGNATURE_SIZE) {
            revert InvalidCompressedSignaturesLength();
        }

        bytes32 blockId = startingBlock;
        bytes memory signature = new bytes(SIGNATURE_SIZE);

        // Recover the parent block number from the starting blockId and
        // increment it as we walk forward through the chain.
        uint256 blockNumber = blockIdToNumber(startingBlock);

        for (uint256 i = 0; i < numBlocks; i++) {
            TronBlockMetadata memory tronBlock = _decodeTronBlockAt(compressedTronBlockMetadata, i);

            if (tronBlock.parentHash != blockId) {
                revert InvalidParentBlockId(tronBlock.parentHash, blockId);
            }

            unchecked {
                // Child block number is parent + 1.
                blockNumber++;
            }

            bytes32 blockHash = hashBlock(tronBlock, blockNumber);
            blockId = bytes32(uint256(blockNumber) << 224 | uint224(uint256(blockHash)));

            // Copy the i-th packed signature (65 bytes) from calldata into memory for ECDSA.recover
            assembly {
                calldatacopy(
                    add(signature, 32),
                    add(compressedSignatures.offset, mul(i, SIGNATURE_SIZE)),
                    SIGNATURE_SIZE
                )
            }

            bytes20 signer = bytes20(ECDSA.recover(blockHash, signature));
            if (signer != srs[tronBlock.witnessAddressIndex]) revert InvalidWitnessSigner();
            bytes32 blockIdSlotAtOurIndex = latestBlockIds[blockNumber % LATEST_BLOCK_IDS_ARRAY_LENGTH];
            if (blockIdSlotAtOurIndex != bytes32(0)) {
                if (blockIdToNumber(blockIdSlotAtOurIndex) > blockNumber) revert BlockTooOld();
                if (blockIdSlotAtOurIndex != blockId) revert InvalidChain();
            }
        }

        appendBlockId(blockId);
        latestProvenBlock = blockId;
    }

    function proveBlockRange(bytes32 startingBlock, bytes32 endingBlock, bytes calldata zkProof) external {
        if (startingBlock != latestProvenBlock) revert BlockTooOld();
        blockRangeProver.proveBlockRange(srs, startingBlock, endingBlock, zkProof);
        appendBlockId(endingBlock);
        latestProvenBlock = endingBlock;
    }

    function getBlockId(uint256 blockNumber) public view returns (bytes32) {
        bytes32 stored = latestBlockIds[blockNumber % LATEST_BLOCK_IDS_ARRAY_LENGTH];
        if (stored == bytes32(0)) revert BlockNotRelayed();
        if (blockIdToNumber(stored) != blockNumber) revert BlockTooOld();
        return stored;
    }

    function appendBlockId(bytes32 blockId) internal {
        uint256 blockNumber = blockIdToNumber(blockId);
        latestBlockIds[blockNumber % LATEST_BLOCK_IDS_ARRAY_LENGTH] = blockId;
        if (blockIdToNumber(latestProvenBlock) < blockNumber) {
            latestProvenBlock = blockId;
        }
    }

    // Conversion & helper (internal pure) functions

    function blockIdToNumber(bytes32 blockId) internal pure returns (uint256 blockNumber) {
        return uint256(blockId) >> 224; // in Tron, blockId is uint32(blockNumber) || sha256(blockHeader)[4:]
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
    function hashBlock(TronBlockMetadata memory tronBlock, uint256 blockNumber)
        internal
        view
        returns (bytes32 blockHash)
    {
        bytes32 parentHash = tronBlock.parentHash;
        bytes32 txTrieRoot = tronBlock.txTrieRoot;

        // Tron raw header uses timestamp in milliseconds.
        uint256 tsMillis = uint256(tronBlock.timestamp) * 1000;

        // Resolve the witness address from the static SR set and convert it into
        // a Tron-style witness address (0x41 prefix + 20-byte EVM address).
        bytes20 witness = srs[tronBlock.witnessAddressIndex];

        // Allocate a small buffer that is large enough for the encoded header.
        bytes memory buf = new bytes(128);

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

            // Store the 20-byte witness address immediately after the prefix.
            // `witness` is a bytes20 value, which in memory occupies the low
            // 20 bytes of a 32-byte word. We shift it left by 96 bits so that
            // the high 20 bytes of the word (the first 20 bytes written) are
            // exactly the address bytes we want in the protobuf field.
            mstore(ptr, shl(96, witness))
            ptr := add(ptr, 21)

            // -----------------------------------------------------------------
            // field 10: version (varint, key = (10 << 3) | 0 = 0x50)
            // -----------------------------------------------------------------
            mstore8(ptr, 0x50)
            ptr := add(ptr, 1)
            // Current mainnet value is 32, which fits in a single varint byte.
            mstore8(ptr, TRON_BLOCK_VERSION)
            ptr := add(ptr, 1)

            // Set the actual length of the bytes buffer to the number of bytes written.
            let used := sub(ptr, base)
            mstore(buf, used)
        }

        return sha256(buf);
    }

    /// @dev Decode the Tron block metadata at a given index from a tightly packed calldata blob.
    /// Each block occupies TRON_BLOCK_METADATA_SIZE bytes, with layout:
    /// [0..31]  parentHash (bytes32)
    /// [32..63] txTrieRoot (bytes32)
    /// [64..67] timestamp (uint32, big-endian)
    /// [68]     witnessAddressIndex (uint8)
    function _decodeTronBlockAt(bytes calldata data, uint256 index)
        internal
        pure
        returns (TronBlockMetadata memory tronBlock)
    {
        uint256 offset = index * TRON_BLOCK_METADATA_SIZE;

        bytes32 parentHash;
        bytes32 txTrieRoot;
        uint32 ts;
        uint8 witnessIndex;

        assembly {
            let base := add(data.offset, offset)

            parentHash := calldataload(base)
            txTrieRoot := calldataload(add(base, 32))

            // Load the 32-byte word starting at byte 64; we only care about the first 5 bytes.
            let word := calldataload(add(base, 64))
            // Shift so that the first 5 bytes (timestamp[4] || index[1]) end up in the low 5 bytes.
            let shifted := shr(216, word) // 27 * 8 = 216
            // High 4 bytes -> timestamp (seconds), low 1 byte -> witnessIndex.
            ts := shr(8, shifted)
            witnessIndex := byte(0, shifted)
        }

        tronBlock.parentHash = parentHash;
        tronBlock.txTrieRoot = txTrieRoot;
        tronBlock.timestamp = ts;
        tronBlock.witnessAddressIndex = witnessIndex;
    }
}
