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

/// @title TronTxReader
/// @notice Stateless helper bound to a Tron light client that verifies inclusion and decodes
///         TRC‑20 transfers and UntronController.pullFromReceivers calls from Tron transactions.
contract TronTxReader {
    // Types
    struct Trc20Transfer {
        bytes32 txLeaf;
        uint256 tronBlockNumber;
        uint32 tronBlockTimestamp;
        address tronTokenEvm;
        bytes21 fromTron;
        bytes21 toTron;
        uint256 amount;
        bool isTransferFrom;
    }

    struct PullFromReceiversCall {
        bytes32 txLeaf;
        uint256 tronBlockNumber;
        uint32 tronBlockTimestamp;
        address token;
        bytes32[] receiverSalts;
        uint256[] amounts;
    }

    struct TriggerSmartContext {
        bytes21 ownerTron;
        bytes21 contractTron;
        uint256 dataStart;
        uint256 dataEnd;
        bool found;
    }

    // Protobuf wire types
    uint8 internal constant WIRE_VARINT = 0;
    uint8 internal constant WIRE_FIXED64 = 1;
    uint8 internal constant WIRE_LENGTH_DELIMITED = 2;
    uint8 internal constant WIRE_FIXED32 = 5;

    // Tron contract types
    uint64 internal constant CONTRACT_TRIGGER_SMART = 31;

    // TRC-20 function selectors
    bytes4 internal constant SELECTOR_TRANSFER = bytes4(keccak256("transfer(address,uint256)"));
    bytes4 internal constant SELECTOR_TRANSFER_FROM = bytes4(keccak256("transferFrom(address,address,uint256)"));
    bytes4 internal constant SELECTOR_PULL_FROM_RECEIVERS =
        bytes4(keccak256("pullFromReceivers(address,bytes32[],uint256[])"));

    // State
    TronLightClient public immutable TRON_LIGHT_CLIENT;

    // Errors
    error InvalidTxMerkleProof();
    error NotATrc20Transfer();
    error Trc20TransferNotSuccessful();
    error TronInvalidOwnerLength();
    error TronInvalidOwnerPrefix();
    error TronInvalidContractLength();
    error TronInvalidContractPrefix();
    error TronInvalidTrc20DataLength();
    error NotAPullFromReceivers();
    error TronInvalidCalldataLength();

    constructor(address tronLightClient_) {
        require(tronLightClient_ != address(0), "LightClientZero");
        TRON_LIGHT_CLIENT = TronLightClient(tronLightClient_);
    }

    // ---------------- TRC-20 Transfer path ----------------
    function readTrc20Transfer(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (Trc20Transfer memory transfer) {
        (bytes32 txLeaf, uint32 tronBlockTimestamp, uint256 rawDataStart, uint256 rawDataEnd) =
            _baseMetadata(tronBlockNumber, encodedTx, proof, index);
        (bytes21 fromTron, bytes21 toTron, address tronTokenEvm, uint256 amount, bool isTransferFrom) =
            _decodeTrc20TransferFromTx(encodedTx, rawDataStart, rawDataEnd);
        bool success = _parseTxSuccess(encodedTx, rawDataEnd, encodedTx.length);
        if (!success) revert Trc20TransferNotSuccessful();

        transfer = Trc20Transfer({
            txLeaf: txLeaf,
            tronBlockNumber: tronBlockNumber,
            tronBlockTimestamp: tronBlockTimestamp,
            tronTokenEvm: tronTokenEvm,
            fromTron: fromTron,
            toTron: toTron,
            amount: amount,
            isTransferFrom: isTransferFrom
        });
    }

    // ---------------- pullFromReceivers path ----------------
    function readPullFromReceivers(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index,
        address controllerEvm
    ) external view returns (PullFromReceiversCall memory callData) {
        (bytes32 txLeaf, uint32 tronBlockTimestamp, uint256 rawDataStart, uint256 rawDataEnd) =
            _baseMetadata(tronBlockNumber, encodedTx, proof, index);
        (address token, bytes32[] memory salts, uint256[] memory amounts) =
            _decodePullFromReceiversFromTx(encodedTx, controllerEvm, rawDataStart, rawDataEnd);
        bool ok = _parseTxSuccess(encodedTx, rawDataEnd, encodedTx.length);
        if (!ok) revert Trc20TransferNotSuccessful();

        callData = PullFromReceiversCall({
            txLeaf: txLeaf,
            tronBlockNumber: tronBlockNumber,
            tronBlockTimestamp: tronBlockTimestamp,
            token: token,
            receiverSalts: salts,
            amounts: amounts
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

    function _parseTriggerHeaders(bytes memory encodedTx, uint256 valueStart, uint256 valueLen)
        internal
        pure
        returns (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd)
    {
        uint256 trigCursor = valueStart;
        uint256 trigEnd = valueStart + valueLen;
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
                    ownerTron := mload(add(add(encodedTx, 0x20), oStart))
                }
                if (uint8(ownerTron[0]) != 0x41) revert TronInvalidOwnerPrefix();
            } else if (tFieldNum == 2 && tWireType == WIRE_LENGTH_DELIMITED) {
                uint256 cStart;
                uint256 cEnd;
                (cStart, cEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (cEnd - cStart != 21) revert TronInvalidContractLength();
                assembly ("memory-safe") {
                    contractTron := mload(add(add(encodedTx, 0x20), cStart))
                }
                if (uint8(contractTron[0]) != 0x41) revert TronInvalidContractPrefix();
            } else if (tFieldNum == 4 && tWireType == WIRE_LENGTH_DELIMITED) {
                (dataStart, dataEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
            } else {
                trigCursor = _skipField(encodedTx, trigCursor, trigEnd, tWireType);
            }
        }
    }

    function _findTriggerSmart(
        bytes memory encodedTx,
        uint256 rawDataStart,
        uint256 rawDataEnd,
        bytes21 expectedContractOrZero
    ) internal pure returns (TriggerSmartContext memory ctx) {
        uint256 cursor = rawDataStart;
        while (true) {
            (bool hasMore, uint256 nextCursor, uint256 contractStart, uint256 contractEnd, uint64 ctype) =
                _nextContract(encodedTx, cursor, rawDataEnd);
            if (!hasMore) break;

            if (ctype == CONTRACT_TRIGGER_SMART) {
                (uint256 trigStart, uint256 trigEnd) =
                    _extractTriggerSmartContract(encodedTx, contractStart, contractEnd);
                if (trigStart != 0) {
                    (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd) =
                        _parseTriggerHeaders(encodedTx, trigStart, trigEnd - trigStart);

                    if (expectedContractOrZero != bytes21(0) && contractTron != expectedContractOrZero) {
                        revert TronInvalidContractPrefix();
                    }

                    if (dataEnd != 0 || dataStart != 0) {
                        ctx = TriggerSmartContext({
                            ownerTron: ownerTron,
                            contractTron: contractTron,
                            dataStart: dataStart,
                            dataEnd: dataEnd,
                            found: true
                        });
                        return ctx;
                    }
                }
            }

            cursor = nextCursor;
        }
    }

    function _decodeTrc20TransferFromTx(bytes memory encodedTx, uint256 rawDataStart, uint256 rawDataEnd)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, address tronTokenEvm, uint256 amount, bool isTransferFrom)
    {
        bool found;
        uint256 cursor = rawDataStart;
        while (true) {
            (bool hasMore, uint256 nextCursor, uint256 contractStart, uint256 contractEnd, uint64 ctype) =
                _nextContract(encodedTx, cursor, rawDataEnd);
            if (!hasMore) break;
            if (ctype == CONTRACT_TRIGGER_SMART) {
                (uint256 trigStart, uint256 trigEnd) =
                    _extractTriggerSmartContract(encodedTx, contractStart, contractEnd);
                if (trigStart != 0) {
                    (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd) =
                        _parseTriggerHeaders(encodedTx, trigStart, trigEnd - trigStart);

                    // Generic TriggerSmartContract header parsing; TRC-20 specifics are below.
                    fromTron = ownerTron;
                    tronTokenEvm = _tronToEvm(contractTron);

                    // If there is no calldata field we keep scanning other contracts.
                    if (dataEnd == 0 && dataStart == 0) {
                        cursor = nextCursor;
                        continue;
                    }

                    if (dataEnd - dataStart < 4) revert NotATrc20Transfer();
                    bytes4 sig = _first4(encodedTx, dataStart);
                    if (sig == SELECTOR_TRANSFER) {
                        (toTron, amount) = _decodeTrc20TransferArgs(encodedTx, dataStart, dataEnd);
                        isTransferFrom = false;
                        found = true;
                    } else if (sig == SELECTOR_TRANSFER_FROM) {
                        (fromTron, toTron, amount) = _decodeTrc20TransferFromArgs(encodedTx, dataStart, dataEnd);
                        isTransferFrom = true;
                        found = true;
                    }
                }
            }
            if (found) break;
            cursor = nextCursor;
        }
        if (!found) revert NotATrc20Transfer();
    }

    function _decodeTrc20TransferArgs(bytes memory data, uint256 dataStart, uint256 dataEnd)
        internal
        pure
        returns (bytes21 toTron, uint256 amount)
    {
        if (dataEnd - dataStart != 4 + 32 * 2) revert TronInvalidTrc20DataLength();
        bytes32 word1;
        bytes32 word2;
        assembly ("memory-safe") {
            word1 := mload(add(add(data, 0x20), add(dataStart, 4)))
            word2 := mload(add(add(data, 0x20), add(dataStart, 36)))
        }
        address toAddr = address(uint160(uint256(word1)));
        toTron = _evmToTron(toAddr);
        amount = uint256(word2);
    }

    function _decodeTrc20TransferFromArgs(bytes memory data, uint256 dataStart, uint256 dataEnd)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        if (dataEnd - dataStart != 4 + 32 * 3) revert TronInvalidTrc20DataLength();
        bytes32 w1;
        bytes32 w2;
        bytes32 w3;
        assembly ("memory-safe") {
            w1 := mload(add(add(data, 0x20), add(dataStart, 4)))
            w2 := mload(add(add(data, 0x20), add(dataStart, 36)))
            w3 := mload(add(add(data, 0x20), add(dataStart, 68)))
        }
        address fromAddr = address(uint160(uint256(w1)));
        address toAddr2 = address(uint160(uint256(w2)));
        fromTron = _evmToTron(fromAddr);
        toTron = _evmToTron(toAddr2);
        amount = uint256(w3);
    }

    function _decodePullFromReceiversFromTx(
        bytes memory encodedTx,
        address controllerEvm,
        uint256 rawDataStart,
        uint256 rawDataEnd
    ) internal pure returns (address token, bytes32[] memory salts, uint256[] memory amounts) {
        bytes21 controllerTron = _evmToTron(controllerEvm);
        TriggerSmartContext memory ctx = _findTriggerSmart(encodedTx, rawDataStart, rawDataEnd, controllerTron);
        if (!ctx.found) revert NotAPullFromReceivers();
        if (ctx.dataEnd - ctx.dataStart < 4) revert TronInvalidCalldataLength();

        bytes4 sel = _first4(encodedTx, ctx.dataStart);
        if (sel != SELECTOR_PULL_FROM_RECEIVERS) revert NotAPullFromReceivers();

        (token, salts, amounts) = _decodePullFromReceiversArgs(encodedTx, ctx.dataStart, ctx.dataEnd);
    }

    function _decodePullFromReceiversArgs(bytes memory data, uint256 dataStart, uint256 dataEnd)
        internal
        pure
        returns (address token, bytes32[] memory salts, uint256[] memory amounts)
    {
        if (dataEnd - dataStart < 4 + 32 * 3) revert TronInvalidCalldataLength();
        uint256 p = dataStart + 4;
        bytes32 word1;
        bytes32 offSalts;
        bytes32 offAmounts;
        assembly ("memory-safe") {
            word1 := mload(add(add(data, 0x20), p))
            offSalts := mload(add(add(data, 0x20), add(p, 32)))
            offAmounts := mload(add(add(data, 0x20), add(p, 64)))
        }
        token = address(uint160(uint256(word1)));
        uint256 base = dataStart + 4;
        uint256 saltsOffset = base + uint256(offSalts);
        uint256 amountsOffset = base + uint256(offAmounts);

        (uint256 saltsStart, uint256 saltsEnd,) = _readDyn(data, saltsOffset, dataEnd);
        uint256 nSalts = _readU256(data, saltsStart);
        salts = new bytes32[](nSalts);
        uint256 q = saltsStart + 32;
        for (uint256 i = 0; i < nSalts; ++i) {
            bytes32 v;
            assembly ("memory-safe") {
                v := mload(add(add(data, 0x20), q))
            }
            salts[i] = v;
            q += 32;
        }
        if (q != saltsEnd) revert TronInvalidCalldataLength();

        (uint256 amtsStart, uint256 amtsEnd,) = _readDyn(data, amountsOffset, dataEnd);
        uint256 nAmts = _readU256(data, amtsStart);
        amounts = new uint256[](nAmts);
        q = amtsStart + 32;
        for (uint256 j = 0; j < nAmts; ++j) {
            uint256 v2 = _readU256(data, q);
            amounts[j] = v2;
            q += 32;
        }
        if (q != amtsEnd) revert TronInvalidCalldataLength();
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
    function _readDyn(bytes memory data, uint256 offset, uint256 limit)
        internal
        pure
        returns (uint256 start, uint256 end, uint256 newCursor)
    {
        start = offset;
        uint256 len = _readU256(data, start);
        end = start + 32 + len;
        if (end > limit) revert TronProtoTruncated();
        return (start, end, end);
    }

    function _readU256(bytes memory data, uint256 offset) internal pure returns (uint256 v) {
        assembly ("memory-safe") {
            v := mload(add(add(data, 0x20), offset))
        }
    }

    function _first4(bytes memory data, uint256 offset) internal pure returns (bytes4 sel) {
        uint32 w;
        assembly ("memory-safe") {
            w := shr(224, mload(add(add(data, 0x20), offset)))
        }
        sel = bytes4(w);
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

    function _skipField(bytes memory data, uint256 cursor, uint256 limit, uint64 wireType)
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

    function _tronToEvm(bytes21 tron) internal pure returns (address) {
        return address(uint160(uint168(tron)));
    }

    function _evmToTron(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }
}
