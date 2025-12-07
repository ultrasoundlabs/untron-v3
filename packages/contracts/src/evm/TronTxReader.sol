// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {TronLightClient} from "./TronLightClient.sol";
import {TronSha256MerkleVerifier} from "../utils/TronSha256MerkleVerifier.sol";

// Generic protobuf parsing errors (file-scoped so the library can use them)
error TronProtoTruncated();
error TronProtoInvalidWireType();

library ProtoVarint {
    function read(bytes calldata data, uint256 pos, uint256 limit)
        internal
        pure
        returns (uint64 value, uint256 newPos)
    {
        uint64 v;
        uint64 shift;

        // Max 10 bytes for a 64-bit varint.
        for (uint256 i = 0; i < 10; ++i) {
            if (pos >= limit) revert TronProtoTruncated();
            uint8 b = uint8(data[pos++]);
            v |= uint64(b & 0x7F) << shift;
            if ((b & 0x80) == 0) {
                return (v, pos);
            }
            shift += 7;
        }

        // If we exit without returning, it’s malformed.
        revert TronProtoTruncated();
    }

    function skip(bytes calldata data, uint256 pos, uint256 limit) internal pure returns (uint256 newPos) {
        // Max 10 bytes
        for (uint256 i = 0; i < 10; ++i) {
            if (pos >= limit) revert TronProtoTruncated();
            uint8 b = uint8(data[pos++]);
            if ((b & 0x80) == 0) {
                return pos;
            }
        }
        revert TronProtoTruncated();
    }
}

/// @title TronTxReader
/// @notice Stateless helper bound to a Tron light client that verifies inclusion and exposes
///         a generic TriggerSmartContract call view (sender, to, calldata).
contract TronTxReader {
    // Types
    struct TriggerSmartContract {
        bytes32 txLeaf;
        uint256 tronBlockNumber;
        uint32 tronBlockTimestamp;
        bytes21 senderTron;
        bytes21 toTron;
        bytes data;
    }

    // Protobuf wire types
    uint8 internal constant WIRE_VARINT = 0;
    uint8 internal constant WIRE_FIXED64 = 1;
    uint8 internal constant WIRE_LENGTH_DELIMITED = 2;
    uint8 internal constant WIRE_FIXED32 = 5;

    // Tron contract types
    uint64 internal constant CONTRACT_TRIGGER_SMART = 31;

    // State
    TronLightClient public immutable TRON_LIGHT_CLIENT;

    // Errors
    error InvalidTxMerkleProof();
    error NotTriggerSmartContract();
    error TronTxNotSuccessful();
    error TronInvalidOwnerLength();
    error TronInvalidOwnerPrefix();
    error TronInvalidContractLength();
    error TronInvalidContractPrefix();

    constructor(address tronLightClient_) {
        require(tronLightClient_ != address(0), "LightClientZero");
        TRON_LIGHT_CLIENT = TronLightClient(tronLightClient_);
    }

    // ---------------- Generic TriggerSmartContract reader ----------------
    function readTriggerSmartContract(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (TriggerSmartContract memory callData) {
        (bytes32 txLeaf, uint32 tronBlockTimestamp, uint256 rawDataStart, uint256 rawDataEnd) =
            _baseMetadata(tronBlockNumber, encodedTx, proof, index);

        // 1. Parse the single Contract in raw_data.
        (uint256 cStart, uint256 cEnd, uint64 cType) = _readSingleContract(encodedTx, rawDataStart, rawDataEnd);

        // 2. Enforce that it is a TriggerSmartContract.
        if (cType != CONTRACT_TRIGGER_SMART) revert NotTriggerSmartContract();

        // 3. Extract the TriggerSmartContract message from inside the Contract.
        (uint256 trigStart, uint256 trigEnd) = _extractTriggerSmartContract(encodedTx, cStart, cEnd);
        if (trigStart == 0 && trigEnd == 0) revert NotTriggerSmartContract();

        // 4. Parse headers: owner, contract, and call data slice.
        (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd) =
            _parseTriggerHeaders(encodedTx, trigStart, trigEnd);

        if (dataStart == 0 && dataEnd == 0) revert NotTriggerSmartContract();

        // 5. Enforce that the transaction result is successful.
        bool success = _parseTxSuccess(encodedTx, rawDataEnd, encodedTx.length);
        if (!success) revert TronTxNotSuccessful();

        // 6. Materialize calldata bytes.
        bytes memory data = _slice(encodedTx, dataStart, dataEnd);

        callData = TriggerSmartContract({
            txLeaf: txLeaf,
            tronBlockNumber: tronBlockNumber,
            tronBlockTimestamp: tronBlockTimestamp,
            senderTron: ownerTron,
            toTron: contractTron,
            data: data
        });
    }

    // ---------------- Merkle helpers ----------------
    function verifyTxInclusion(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) public view returns (bytes32 txLeaf) {
        bytes32 root = TRON_LIGHT_CLIENT.getTxTrieRoot(tronBlockNumber);
        txLeaf = sha256(encodedTx);
        if (!TronSha256MerkleVerifier.verify(root, txLeaf, proof, index)) revert InvalidTxMerkleProof();
    }

    function _baseMetadata(uint256 tronBlockNumber, bytes calldata encodedTx, bytes32[] calldata proof, uint256 index)
        internal
        view
        returns (bytes32 txLeaf, uint32 tronBlockTimestamp, uint256 rawDataStart, uint256 rawDataEnd)
    {
        txLeaf = verifyTxInclusion(tronBlockNumber, encodedTx, proof, index);
        (rawDataStart, rawDataEnd) = _parseRawData(encodedTx);
        tronBlockTimestamp = TRON_LIGHT_CLIENT.getBlockTimestamp(tronBlockNumber);
    }

    // ---------------- Protobuf parsing ----------------
    function _parseRawData(bytes calldata tx_) internal pure returns (uint256 rawDataStart, uint256 rawDataEnd) {
        uint256 totalLen = tx_.length;
        if (totalLen == 0 || uint8(tx_[0]) != 0x0A) revert NotTriggerSmartContract();
        uint256 offset = 1;
        uint64 rawDataLen;
        (rawDataLen, offset) = ProtoVarint.read(tx_, offset, totalLen);
        rawDataStart = offset;
        rawDataEnd = _advance(offset, uint256(rawDataLen), totalLen);
    }

    function _readSingleContract(bytes calldata tx_, uint256 rawDataStart, uint256 rawDataEnd)
        internal
        pure
        returns (uint256 cStart, uint256 cEnd, uint64 contractType)
    {
        uint256 cursor = rawDataStart;
        bool seenContract;

        while (cursor < rawDataEnd) {
            uint64 fieldNum;
            uint64 wireType;
            (fieldNum, wireType, cursor) = _readKey(tx_, cursor, rawDataEnd);

            if (fieldNum == 11 && wireType == WIRE_LENGTH_DELIMITED) {
                // Enforce "exactly one" contract at the protobuf level.
                if (seenContract) {
                    // Optional: define a dedicated error if you want.
                    // revert TronUnexpectedExtraContracts();
                    revert NotTriggerSmartContract();
                }
                seenContract = true;

                (cStart, cEnd, cursor) = _readLength(tx_, cursor, rawDataEnd);

                // Read contract type: field #1 (VARINT) inside the Contract message.
                uint256 p = cStart;
                bool foundType;
                while (p < cEnd) {
                    uint64 cFieldNum;
                    uint64 cWireType;
                    (cFieldNum, cWireType, p) = _readKey(tx_, p, cEnd);
                    if (cFieldNum == 1 && cWireType == WIRE_VARINT) {
                        (contractType, p) = ProtoVarint.read(tx_, p, cEnd);
                        foundType = true;
                        break;
                    }
                    p = _skipField(tx_, p, cEnd, cWireType);
                }

                if (!foundType) revert NotTriggerSmartContract();

                // We’ve found the single contract and its type; we don’t care about later fields in raw_data.
                break;
            } else {
                cursor = _skipField(tx_, cursor, rawDataEnd, wireType);
            }
        }

        if (!seenContract) revert NotTriggerSmartContract();
    }

    function _extractTriggerSmartContract(bytes calldata tx_, uint256 contractStart, uint256 contractEnd)
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
                (uint256 valueStart, uint256 valueEnd) = _parseAnyValueField(tx_, paramStart, paramEnd);
                if (valueStart != 0) {
                    trigStart = valueStart;
                    trigEnd = valueEnd;
                }
            } else {
                p = _skipField(tx_, p, contractEnd, cWireType);
            }
        }
    }

    function _parseAnyValueField(bytes calldata encodedTx, uint256 paramStart, uint256 paramEnd)
        internal
        pure
        returns (uint256 valueStart, uint256 valueEnd)
    {
        uint256 q = paramStart;
        valueStart = 0;
        valueEnd = 0;
        while (q < paramEnd) {
            uint64 anyFieldNum;
            uint64 anyWireType;
            (anyFieldNum, anyWireType, q) = _readKey(encodedTx, q, paramEnd);
            if (anyFieldNum == 1 && anyWireType == WIRE_LENGTH_DELIMITED) {
                (, q,) = _readLength(encodedTx, q, paramEnd);
            } else if (anyFieldNum == 2 && anyWireType == WIRE_LENGTH_DELIMITED) {
                (valueStart, q,) = _readLength(encodedTx, q, paramEnd);
                valueEnd = q;
            } else {
                q = _skipField(encodedTx, q, paramEnd, anyWireType);
            }
        }
    }

    function _parseTriggerHeaders(bytes calldata encodedTx, uint256 trigStart, uint256 trigEnd)
        internal
        pure
        returns (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd)
    {
        uint256 trigCursor = trigStart;
        while (trigCursor < trigEnd) {
            uint64 tFieldNum;
            uint64 tWireType;
            (tFieldNum, tWireType, trigCursor) = _readKey(encodedTx, trigCursor, trigEnd);
            if (tFieldNum == 1 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 oStart;
                uint256 oEnd;
                (oStart, oEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (oEnd - oStart != 21) revert TronInvalidOwnerLength();
                assembly ("memory-safe") {
                    ownerTron := calldataload(add(encodedTx.offset, oStart))
                }
                if (uint8(ownerTron[0]) != 0x41) revert TronInvalidOwnerPrefix();
            } else if (tFieldNum == 2 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 cStart;
                uint256 cEnd;
                (cStart, cEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (cEnd - cStart != 21) revert TronInvalidContractLength();
                assembly ("memory-safe") {
                    contractTron := calldataload(add(encodedTx.offset, cStart))
                }
                if (uint8(contractTron[0]) != 0x41) revert TronInvalidContractPrefix();
            } else if (tFieldNum == 4 && tWireType == WIRE_LENGTH_DELIMITED) {
                (dataStart, dataEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
            } else {
                trigCursor = _skipField(encodedTx, trigCursor, trigEnd, tWireType);
            }
        }
    }

    function _parseTxSuccess(bytes calldata encodedTx, uint256 offset, uint256 totalLen) internal pure returns (bool) {
        // Skip signatures (field 2, tag 0x12)
        while (offset < totalLen && uint8(encodedTx[offset]) == 0x12) {
            offset++;
            uint64 sigLen;
            (sigLen, offset) = ProtoVarint.read(encodedTx, offset, totalLen);
            offset += uint256(sigLen);
            if (offset > totalLen) revert TronProtoTruncated();
        }
        if (offset >= totalLen || uint8(encodedTx[offset]) != 0x2A) return true;
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

    // ---------------- Utilities ----------------
    function _readLength(bytes calldata data, uint256 cursor, uint256 limit)
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

    function _readKey(bytes calldata data, uint256 pos, uint256 limit)
        internal
        pure
        returns (uint64 fieldNum, uint64 wireType, uint256 newPos)
    {
        (uint64 key, uint256 p) = ProtoVarint.read(data, pos, limit);
        return (key >> 3, key & 0x7, p);
    }

    function _skipField(bytes calldata data, uint256 cursor, uint256 limit, uint64 wireType)
        internal
        pure
        returns (uint256 newCursor)
    {
        if (wireType == WIRE_VARINT) return ProtoVarint.skip(data, cursor, limit);
        if (wireType == WIRE_LENGTH_DELIMITED) {
            (, uint256 end,) = _readLength(data, cursor, limit);
            return end;
        }
        if (wireType == WIRE_FIXED32) return _advance(cursor, 4, limit);
        if (wireType == WIRE_FIXED64) return _advance(cursor, 8, limit);
        revert TronProtoInvalidWireType();
    }

    function _advance(uint256 cursor, uint256 delta, uint256 limit) internal pure returns (uint256) {
        unchecked {
            cursor += delta;
        }
        if (cursor > limit) revert TronProtoTruncated();
        return cursor;
    }

    function _slice(bytes calldata data, uint256 start, uint256 end) internal pure returns (bytes memory out) {
        if (end < start || end > data.length) revert TronProtoTruncated();
        uint256 len = end - start;
        out = new bytes(len);
        if (len == 0) return out;
        // Copy calldata segment [start, end) into freshly allocated bytes.
        assembly ("memory-safe") {
            calldatacopy(add(out, 0x20), add(data.offset, start), len)
        }
    }
}
