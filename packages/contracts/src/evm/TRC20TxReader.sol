// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {TronLightClient} from "./TronLightClient.sol";
import {TronSha256MerkleVerifier} from "../utils/TronSha256MerkleVerifier.sol";

// Generic protobuf parsing errors (file-scoped so the library can use them)
error TronProtoTruncated();
error TronProtoInvalidWireType();

library ProtoVarint {
    function read(bytes memory data, uint256 pos, uint256 limit) internal pure returns (uint64 value, uint256 newPos) {
        uint64 v;
        uint64 shift;
        while (true) {
            if (pos >= limit) revert TronProtoTruncated();
            uint8 b = uint8(data[pos++]);
            v |= uint64(b & 0x7F) << shift;
            if ((b & 0x80) == 0) break;
            shift += 7;
        }
        return (v, pos);
    }

    function skip(bytes memory data, uint256 pos, uint256 limit) internal pure returns (uint256 newPos) {
        while (true) {
            if (pos >= limit) revert TronProtoTruncated();
            uint8 b = uint8(data[pos++]);
            if ((b & 0x80) == 0) break;
        }
        return pos;
    }
}

/// @title TRC20TxReader
/// @notice Stateless helper contract bound to a Tron light client that verifies
///         transaction inclusion and decodes TRC-20 transfers.
/// @dev This contract is self‑contained and non‑upgradeable. It holds an immutable
///      reference to a Tron light client and exposes public/external helper
///      functions. It performs no nullifier or single‑use checks.
contract TRC20TxReader {
    // Protobuf wire types
    uint64 internal constant WIRE_VARINT = 0;
    uint64 internal constant WIRE_FIXED64 = 1;
    uint64 internal constant WIRE_LENGTH_DELIMITED = 2;
    uint64 internal constant WIRE_FIXED32 = 5;

    // Tron contract types
    uint64 internal constant CONTRACT_TRIGGER_SMART = 31;

    // Type declarations

    /// @notice Structured representation of a TRC-20 token transfer.
    /// @dev Tron addresses are 21 bytes (0x41 prefix + 20-byte EVM address).
    struct Trc20Transfer {
        bytes32 txLeaf; /// Merkle leaf hash = sha256(Transaction.encode(tx))
        uint256 tronBlockNumber; /// Tron block number containing this transaction
        uint32 tronBlockTimestamp; /// Tron block timestamp (seconds since epoch)
        address tronTokenEvm; /// Token contract address (20-byte EVM form)
        bytes21 fromTron; /// Sender address in Tron format (21 bytes)
        bytes21 toTron; /// Recipient address in Tron format (21 bytes)
        uint256 amount; /// Amount of tokens transferred
        bool isTransferFrom; /// True if transferFrom, false if simple transfer
    }

    struct TriggerParsed {
        bytes21 fromTron;
        bytes21 toTron;
        address tronTokenEvm;
        uint256 amount;
        bool isTransferFrom;
    }

    // State variables
    /// @notice Immutable Tron light client used for Merkle root and timestamp queries.
    TronLightClient public immutable TRON_LIGHT_CLIENT;
    /// @dev TRC-20 function selectors for transfer and transferFrom (Ethereum ABI format).
    bytes4 internal constant SELECTOR_TRANSFER = 0xa9059cbb; // transfer(address,uint256)
    bytes4 internal constant SELECTOR_TRANSFER_FROM = 0x23b872dd; // transferFrom(address,address,uint256)

    // Errors
    // -------------------------- Error Definitions --------------------------
    error InvalidTxMerkleProof();
    error NotATrc20Transfer();
    error Trc20TransferNotSuccessful();
    error TronInvalidOwnerLength();
    error TronInvalidOwnerPrefix();
    error TronInvalidContractLength();
    error TronInvalidContractPrefix();
    error TronInvalidTrc20DataLength();

    // Functions
    // constructor
    constructor(address tronLightClient_) {
        require(tronLightClient_ != address(0), "LightClientZero");
        TRON_LIGHT_CLIENT = TronLightClient(tronLightClient_);
    }

    // external
    /// @notice Proves inclusion and decodes a TRC‑20 transfer from a Tron transaction.
    /// @dev Reverts if proof is invalid, the tx is not a TRC‑20 transfer/transferFrom,
    ///      or the tx execution was unsuccessful on Tron.
    /// @param tronBlockNumber Tron block number in which the transaction was included.
    /// @param encodedTx Full protobuf‑encoded Tron `Transaction` bytes.
    /// @param proof Merkle proof sibling hashes from leaf to root.
    /// @param index Merkle path bitfield for the transaction’s position in the tree.
    /// @return transfer Decoded TRC‑20 transfer details.
    function readTrc20Transfer(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (Trc20Transfer memory transfer) {
        // 1) Verify inclusion in the specified block.
        bytes32 txLeaf = verifyTxInclusion(tronBlockNumber, encodedTx, proof, index);

        // 2) Decode TRC‑20 transfer details from transaction bytes.
        (bytes21 fromTron, bytes21 toTron, address tronTokenEvm, uint256 amount, bool isTransferFrom) =
            _decodeTrc20TransferFromTx(encodedTx);

        // 3) Fetch block timestamp for context and assemble result.
        uint32 blockTs = TRON_LIGHT_CLIENT.getBlockTimestamp(tronBlockNumber);
        transfer = Trc20Transfer({
            txLeaf: txLeaf,
            tronBlockNumber: tronBlockNumber,
            tronBlockTimestamp: blockTs,
            tronTokenEvm: tronTokenEvm,
            fromTron: fromTron,
            toTron: toTron,
            amount: amount,
            isTransferFrom: isTransferFrom
        });
    }

    /// @notice Computes the Tron transaction Merkle leaf as sha256(encodedTx)
    function computeTxLeaf(bytes memory encodedTx) external pure returns (bytes32) {
        return sha256(encodedTx);
    }

    // public
    /// @notice Verifies that a given encoded Tron transaction is included in a specified block (via Merkle proof).
    /// @param tronBlockNumber The Tron block number where the transaction should be included.
    /// @param encodedTx Full protobuf-encoded Tron `Transaction` bytes.
    /// @param proof Sibling hashes from the transaction leaf up to the Merkle root.
    /// @param index Bitfield representing the transaction’s position in the Merkle tree (0 = left, 1 = right at each level).
    /// @return txLeaf The SHA-256 hash of the encoded transaction (the Merkle leaf value).
    function verifyTxInclusion(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) public view returns (bytes32 txLeaf) {
        // Retrieve the expected Merkle root for the block from the light client.
        bytes32 root = TRON_LIGHT_CLIENT.getTxTrieRoot(tronBlockNumber);
        // Compute the leaf hash from the provided transaction bytes (Tron uses sha256 for transaction hashing).
        txLeaf = sha256(encodedTx);
        // Verify the Merkle proof using the computed leaf and provided siblings.
        if (!TronSha256MerkleVerifier.verify(root, txLeaf, proof, index)) {
            revert InvalidTxMerkleProof();
        }
    }

    // internal
    // ---------------- Internal helpers for splitting decoder ----------------
    function _parseRawData(bytes memory tx_) internal pure returns (uint256 rawDataStart, uint256 rawDataEnd) {
        uint256 totalLen = tx_.length;
        if (totalLen == 0 || uint8(tx_[0]) != 0x0A) revert NotATrc20Transfer();
        uint256 offset = 1;
        uint64 rawDataLen;
        (rawDataLen, offset) = ProtoVarint.read(tx_, offset, totalLen);
        rawDataStart = offset;
        rawDataEnd = _advance(offset, uint256(rawDataLen), totalLen);
    }

    function _nextContract(bytes memory tx_, uint256 cursor, uint256 rawDataEnd)
        internal
        pure
        returns (bool hasMore, uint256 nextCursor, uint256 contractStart, uint256 contractEnd, uint64 contractType)
    {
        while (cursor < rawDataEnd) {
            uint64 fieldNum;
            uint64 wireType;
            (fieldNum, wireType, cursor) = _readKey(tx_, cursor, rawDataEnd);

            if (fieldNum == 11 && wireType == WIRE_LENGTH_DELIMITED) {
                (contractStart, contractEnd, cursor) = _readLength(tx_, cursor, rawDataEnd);

                // Find contract type inside message (field #1 varint)
                contractType = 0;
                uint256 p = contractStart;
                while (p < contractEnd) {
                    uint64 cFieldNum;
                    uint64 cWireType;
                    (cFieldNum, cWireType, p) = _readKey(tx_, p, contractEnd);
                    if (cFieldNum == 1 && cWireType == WIRE_VARINT) {
                        (contractType, p) = ProtoVarint.read(tx_, p, contractEnd);
                    } else {
                        p = _skipField(tx_, p, contractEnd, cWireType);
                    }
                }

                hasMore = true;
                nextCursor = contractEnd;
                return (hasMore, nextCursor, contractStart, contractEnd, contractType);
            } else {
                cursor = _skipField(tx_, cursor, rawDataEnd, wireType);
            }
        }

        return (false, rawDataEnd, 0, 0, 0);
    }

    function _extractTriggerSmartContract(bytes memory tx_, uint256 contractStart, uint256 contractEnd)
        internal
        pure
        returns (uint256 trigStart, uint256 trigEnd)
    {
        uint256 p = contractStart;
        uint256 paramStart = 0;
        uint256 paramEnd = 0;
        while (p < contractEnd) {
            uint64 cFieldNum;
            uint64 cWireType;
            (cFieldNum, cWireType, p) = _readKey(tx_, p, contractEnd);
            if (cFieldNum == 2 && cWireType == WIRE_LENGTH_DELIMITED) {
                (paramStart, paramEnd, p) = _readLength(tx_, p, contractEnd);

                (uint256 valueStart, uint256 valueLen) = _parseAnyValueField(tx_, paramStart, paramEnd);
                if (valueStart != 0) {
                    trigStart = valueStart;
                    trigEnd = valueStart + valueLen;
                }
            } else {
                p = _skipField(tx_, p, contractEnd, cWireType);
            }
        }
    }

    /// @notice Decodes the details of a TRC-20 transfer from raw Tron transaction bytes.
    /// @dev Parses the protobuf Transaction to extract the first valid TRC-20 `transfer` or `transferFrom` call.
    ///      Reverts with `NotATrc20Transfer` if the transaction is not a TRC-20 transfer call.
    /// @param encodedTx Full protobuf-encoded Tron `Transaction` bytes.
    /// @return fromTron Sender’s Tron address (21-byte format).
    /// @return toTron Recipient’s Tron address (21-byte format).
    /// @return tronTokenEvm Token contract address in 20-byte EVM format.
    /// @return amount Token amount transferred.
    /// @return isTransferFrom True if it was a transferFrom call, false if a direct transfer.
    function _decodeTrc20TransferFromTx(bytes memory encodedTx)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, address tronTokenEvm, uint256 amount, bool isTransferFrom)
    {
        uint256 totalLen = encodedTx.length;

        // 1) Parse raw_data boundaries
        (uint256 rawDataStart, uint256 rawDataEnd) = _parseRawData(encodedTx);

        // 2) Iterate contracts and look for TriggerSmartContract carrying TRC-20 call
        bool found = false;
        uint256 cursor = rawDataStart;
        while (true) {
            (bool hasMore, uint256 nextCursor, uint256 contractStart, uint256 contractEnd, uint64 ctype) =
                _nextContract(encodedTx, cursor, rawDataEnd);
            if (!hasMore) break;

            (uint256 trigStart, uint256 trigEnd) = _extractTriggerSmartContract(encodedTx, contractStart, contractEnd);
            if (ctype == CONTRACT_TRIGGER_SMART && trigStart != 0) {
                (bool ok2, TriggerParsed memory tp) = _parseTriggerSmart(encodedTx, trigStart, trigEnd - trigStart);
                if (ok2) {
                    fromTron = tp.fromTron;
                    toTron = tp.toTron;
                    tronTokenEvm = tp.tronTokenEvm;
                    amount = tp.amount;
                    isTransferFrom = tp.isTransferFrom;
                    found = true;
                }
            }

            if (found) break;
            cursor = nextCursor;
        }

        if (!found) revert NotATrc20Transfer();

        // 3) Parse result section for success
        bool success = _parseTxSuccess(encodedTx, rawDataEnd, totalLen);
        if (!success) revert Trc20TransferNotSuccessful();
    }

    function _parseAnyValueField(bytes memory encodedTx, uint256 paramStart, uint256 paramEnd)
        internal
        pure
        returns (uint256 valueStart, uint256 valueLen)
    {
        uint256 q = paramStart;
        valueStart = 0;
        valueLen = 0;
        while (q < paramEnd) {
            uint64 anyFieldNum;
            uint64 anyWireType;
            (anyFieldNum, anyWireType, q) = _readKey(encodedTx, q, paramEnd);
            if (anyFieldNum == 1 && anyWireType == WIRE_LENGTH_DELIMITED) {
                (, q,) = _readLength(encodedTx, q, paramEnd);
            } else if (anyFieldNum == 2 && anyWireType == WIRE_LENGTH_DELIMITED) {
                (valueStart, q,) = _readLength(encodedTx, q, paramEnd);
                valueLen = q - valueStart;
            } else {
                q = _skipField(encodedTx, q, paramEnd, anyWireType);
            }
        }
    }

    function _parseTxSuccess(bytes memory encodedTx, uint256 offset, uint256 totalLen) internal pure returns (bool) {
        // Skip signatures (field 2, tag 0x12)
        while (offset < totalLen && uint8(encodedTx[offset]) == 0x12) {
            offset++;
            uint64 sigLen;
            (sigLen, offset) = ProtoVarint.read(encodedTx, offset, totalLen);
            offset += uint256(sigLen);
            if (offset > totalLen) revert TronProtoTruncated();
        }

        // No result section → treat as success
        if (offset >= totalLen || uint8(encodedTx[offset]) != 0x2A) {
            return true;
        }

        // Parse result message
        offset++;
        uint256 resStart;
        uint256 resEnd;
        (resStart, resEnd,) = _readLength(encodedTx, offset, totalLen);
        offset = resStart;
        while (offset < resEnd) {
            uint64 fieldNum;
            uint64 wireType;
            (fieldNum, wireType, offset) = _readKey(encodedTx, offset, resEnd);

            if (fieldNum == 2 && wireType == WIRE_VARINT) {
                uint64 statusCode;
                (statusCode, offset) = ProtoVarint.read(encodedTx, offset, resEnd);
                if (statusCode != 0) return false;
            } else {
                offset = _skipField(encodedTx, offset, resEnd, wireType);
            }
        }

        return true;
    }

    function _parseTriggerSmart(bytes memory encodedTx, uint256 valueStart, uint256 valueLen)
        internal
        pure
        returns (bool ok, TriggerParsed memory tp)
    {
        uint256 trigCursor = valueStart;
        uint256 trigEnd = valueStart + valueLen;
        while (trigCursor < trigEnd) {
            uint64 tFieldNum;
            uint64 tWireType;
            (tFieldNum, tWireType, trigCursor) = _readKey(encodedTx, trigCursor, trigEnd);

            if (tFieldNum == 1 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 addrStart;
                uint256 addrEnd;
                (addrStart, addrEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (addrEnd - addrStart != 21) revert TronInvalidOwnerLength();
                bytes21 tmp;
                assembly ("memory-safe") {
                    tmp := mload(add(add(encodedTx, 0x20), addrStart))
                }
                if (uint8(tmp[0]) != 0x41) revert TronInvalidOwnerPrefix();
                tp.fromTron = tmp;
            } else if (tFieldNum == 2 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 cStart;
                uint256 cEnd;
                (cStart, cEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (cEnd - cStart != 21) revert TronInvalidContractLength();
                bytes21 tmp2;
                assembly ("memory-safe") {
                    tmp2 := mload(add(add(encodedTx, 0x20), cStart))
                }
                if (uint8(tmp2[0]) != 0x41) revert TronInvalidContractPrefix();
                tp.tronTokenEvm = _tronToEvm(tmp2);
            } else if (tFieldNum == 4 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 dataStart;
                uint256 dataEnd;
                (dataStart, dataEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (dataEnd - dataStart < 4) revert NotATrc20Transfer();

                uint32 sel;
                {
                    uint256 pData = dataStart;
                    uint8 b0 = uint8(encodedTx[pData]);
                    uint8 b1 = uint8(encodedTx[pData + 1]);
                    uint8 b2 = uint8(encodedTx[pData + 2]);
                    uint8 b3 = uint8(encodedTx[pData + 3]);
                    sel = (uint32(b0) << 24) | (uint32(b1) << 16) | (uint32(b2) << 8) | uint32(b3);
                }
                bytes4 sig = bytes4(sel);
                if (sig == SELECTOR_TRANSFER) {
                    if (dataEnd - dataStart != 4 + 32 * 2) revert TronInvalidTrc20DataLength();
                    bytes32 word1;
                    bytes32 word2;
                    assembly ("memory-safe") {
                        word1 := mload(add(add(encodedTx, 0x20), add(dataStart, 4)))
                        word2 := mload(add(add(encodedTx, 0x20), add(dataStart, 36)))
                    }
                    address toAddr = address(uint160(uint256(word1)));
                    tp.toTron = _evmToTron(toAddr);
                    tp.amount = uint256(word2);
                    tp.isTransferFrom = false;
                    ok = true;
                } else if (sig == SELECTOR_TRANSFER_FROM) {
                    if (dataEnd - dataStart != 4 + 32 * 3) revert TronInvalidTrc20DataLength();
                    bytes32 w1;
                    bytes32 w2;
                    bytes32 w3;
                    assembly ("memory-safe") {
                        w1 := mload(add(add(encodedTx, 0x20), add(dataStart, 4)))
                        w2 := mload(add(add(encodedTx, 0x20), add(dataStart, 36)))
                        w3 := mload(add(add(encodedTx, 0x20), add(dataStart, 68)))
                    }
                    address fromAddr = address(uint160(uint256(w1)));
                    address toAddr2 = address(uint160(uint256(w2)));
                    tp.fromTron = _evmToTron(fromAddr);
                    tp.toTron = _evmToTron(toAddr2);
                    tp.amount = uint256(w3);
                    tp.isTransferFrom = true;
                    ok = true;
                }
                trigCursor = dataEnd;
            } else {
                trigCursor = _skipField(encodedTx, trigCursor, trigEnd, tWireType);
            }
        }
    }

    function _skipField(bytes memory data, uint256 cursor, uint256 limit, uint64 wireType)
        internal
        pure
        returns (uint256)
    {
        if (wireType == WIRE_VARINT) {
            return ProtoVarint.skip(data, cursor, limit);
        }
        if (wireType == WIRE_LENGTH_DELIMITED) {
            (, uint256 end,) = _readLength(data, cursor, limit);
            return end;
        }
        if (wireType == WIRE_FIXED32) {
            return _advance(cursor, 4, limit);
        }
        if (wireType == WIRE_FIXED64) {
            return _advance(cursor, 8, limit);
        }
        revert TronProtoInvalidWireType();
    }

    function _advance(uint256 cursor, uint256 delta, uint256 limit) internal pure returns (uint256) {
        unchecked {
            cursor += delta;
        }
        if (cursor > limit) revert TronProtoTruncated();
        return cursor;
    }

    function _readLength(bytes memory data, uint256 cursor, uint256 limit)
        internal
        pure
        returns (uint256 start, uint256 end, uint256 newCursor)
    {
        uint64 len;
        (len, cursor) = ProtoVarint.read(data, cursor, limit);
        start = cursor;
        end = cursor + uint256(len);
        if (end > limit) revert TronProtoTruncated();
        return (start, end, end);
    }

    function _readKey(bytes memory data, uint256 pos, uint256 limit)
        internal
        pure
        returns (uint64 fieldNum, uint64 wireType, uint256 newPos)
    {
        (uint64 key, uint256 p) = ProtoVarint.read(data, pos, limit);
        return (key >> 3, key & 0x7, p);
    }

    function _tronToEvm(bytes21 tron) internal pure returns (address) {
        return address(uint160(uint168(tron)));
    }

    function _evmToTron(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }
}
