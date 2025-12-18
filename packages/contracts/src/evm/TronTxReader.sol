// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {TronLightClient} from "./TronLightClient.sol";
import {TronSha256MerkleVerifier} from "../utils/TronSha256MerkleVerifier.sol";
import {ProtoVarint, ProtoTruncated, ProtoInvalidWireType} from "../utils/ProtoVarint.sol";

/// @title TronTxReader
/// @notice Stateless helper bound to a Tron light client that verifies inclusion and exposes
///         a generic TriggerSmartContract call view (sender, to, calldata).
/// @dev
/// This contract does two distinct jobs:
/// 1) Inclusion verification: prove that a protobuf-encoded Tron `Transaction` is part of a
///    specific Tron block by verifying a SHA-256 Merkle proof against the transaction trie root
///    obtained from `TronLightClient`.
/// 2) Protobuf extraction: parse the same `Transaction` bytes and extract the embedded
///    `TriggerSmartContract` parameters (owner address, contract address, call data), plus
///    compute the Tron transaction id (`sha256(raw_data)`).
///
/// Design notes:
/// - Parsing is performed directly on calldata to minimize copying; only the final `data` slice
///   is materialized into memory.
/// - The parser is intentionally narrow: it expects exactly one `Contract` in `raw_data`, and
///   requires that contract to be of type `TriggerSmartContract` (type id 31).
/// - Tron addresses are returned as `bytes21` in the canonical "0x41 || 20-byte-address" form.
/// @author Ultrasound Labs
contract TronTxReader {
    // Types
    /// @notice Parsed subset of a Tron `TriggerSmartContract` transaction.
    /// @dev `txId` is the Tron transaction identifier shown by explorers and equals `sha256(raw_data)`.
    struct TriggerSmartContract {
        bytes32 txId; // not tx leaf!!! this is the actual tx ID you can put in e.g. Tronscan
        uint256 tronBlockNumber;
        uint32 tronBlockTimestamp;
        bytes21 senderTron;
        bytes21 toTron;
        bytes data;
    }

    // Protobuf wire types
    uint8 internal constant _WIRE_VARINT = 0;
    uint8 internal constant _WIRE_FIXED64 = 1;
    uint8 internal constant _WIRE_LENGTH_DELIMITED = 2;
    uint8 internal constant _WIRE_FIXED32 = 5;

    // Tron contract types
    uint64 internal constant _CONTRACT_TRIGGER_SMART = 31;

    // State
    /// @notice The light client used to obtain block metadata and the transaction Merkle root.
    TronLightClient public immutable TRON_LIGHT_CLIENT;

    // Errors
    error NoTronLightClient();
    error InvalidTxMerkleProof();
    error NotTriggerSmartContract();
    error TronTxNotSuccessful();
    error TronInvalidOwnerLength();
    error TronInvalidOwnerPrefix();
    error TronInvalidContractLength();
    error TronInvalidContractPrefix();

    /// @notice Sets the Tron light client used for transaction inclusion verification and timestamps.
    /// @param tronLightClient_ The deployed `TronLightClient` address (must be non-zero).
    constructor(address tronLightClient_) {
        if (tronLightClient_ == address(0)) revert NoTronLightClient();
        TRON_LIGHT_CLIENT = TronLightClient(tronLightClient_);
    }

    /// @notice Verifies inclusion of `encodedTx` in `tronBlockNumber` and returns parsed TriggerSmartContract call data.
    /// @dev Reverts if:
    /// - The Merkle proof is invalid (`InvalidTxMerkleProof`)
    /// - The transaction is not a single `TriggerSmartContract` (`NotTriggerSmartContract`)
    /// - The transaction result indicates failure (`TronTxNotSuccessful`)
    /// - The owner/contract addresses are not in canonical Tron `bytes21` format
    /// @param tronBlockNumber The Tron block number whose tx root is used for verification.
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param proof The SHA-256 Merkle proof for the transaction leaf within the block's transaction tree.
    /// @param index The 0-based leaf index in the Merkle tree used by the verifier.
    /// @return callData The extracted TriggerSmartContract view (txId, sender, to, calldata, block metadata).
    function readTriggerSmartContract(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (TriggerSmartContract memory callData) {
        verifyTxInclusion(tronBlockNumber, encodedTx, proof, index);
        callData = _parseTriggerSmartContract(encodedTx);
        callData.tronBlockNumber = tronBlockNumber;
        callData.tronBlockTimestamp = TRON_LIGHT_CLIENT.getBlockTimestamp(tronBlockNumber);
    }

    // ---------------- Merkle helpers ----------------
    /// @notice Verifies that `encodedTx` is included in `tronBlockNumber` using the provided Merkle proof.
    /// @dev
    /// - The leaf hash is `sha256(encodedTx)` (hash of the full Transaction bytes).
    /// - The expected root is `TronLightClient.getTxTrieRoot(tronBlockNumber)`.
    /// @param tronBlockNumber The Tron block number whose tx root is used for verification.
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param proof The sibling hashes forming a Merkle path from leaf to root.
    /// @param index The 0-based leaf index used by the Merkle verifier.
    function verifyTxInclusion(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) public view {
        bytes32 root = TRON_LIGHT_CLIENT.getTxTrieRoot(tronBlockNumber);
        bytes32 txLeaf = sha256(encodedTx);
        if (!TronSha256MerkleVerifier.verify(root, txLeaf, proof, index)) revert InvalidTxMerkleProof();
    }

    // ---------------- Generic TriggerSmartContract reader ----------------
    /// @notice Pure helper to parse a TriggerSmartContract from raw encoded transaction data.
    /// @dev
    /// Does not perform Merkle verification or light client checks.
    ///
    /// Parsing outline:
    /// 1) Extract `raw_data` (field #1) and compute `txId = sha256(raw_data)`.
    /// 2) Require exactly one `Contract` entry in `raw_data` (field #11).
    /// 3) Require contract type == 31 (`TriggerSmartContract`).
    /// 4) Extract the embedded `TriggerSmartContract` protobuf message from the contract parameter (Any.value).
    /// 5) Parse owner_address (field #1), contract_address (field #2), and data (field #4).
    /// 6) Parse transaction result status; require success.
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @return _partial A partially-filled TriggerSmartContract view (block metadata left as zero).
    function _parseTriggerSmartContract(bytes calldata encodedTx)
        internal
        pure
        returns (TriggerSmartContract memory _partial)
    {
        bytes32 txId;
        uint256 rawDataEnd;
        bytes21 ownerTron;
        bytes21 contractTron;
        uint256 dataStart;
        uint256 dataEnd;

        {
            uint256 rawDataStart;
            (rawDataStart, rawDataEnd, txId) = _parseRawData(encodedTx);
            // solhint-disable-next-line gas-strict-inequalities
            assert(rawDataStart <= rawDataEnd && rawDataEnd <= encodedTx.length);

            (ownerTron, contractTron, dataStart, dataEnd) =
                _parseTriggerFromRawData(encodedTx, rawDataStart, rawDataEnd);
        }

        if (dataStart == 0 && dataEnd == 0) revert NotTriggerSmartContract();

        if (!_parseTxSuccess(encodedTx, rawDataEnd, encodedTx.length)) revert TronTxNotSuccessful();

        _partial.txId = txId;
        _partial.senderTron = ownerTron;
        _partial.toTron = contractTron;
        _partial.data = _slice(encodedTx, dataStart, dataEnd);
    }

    /// @notice Parses trigger information from the raw data section of a transaction
    /// @param encodedTx The complete encoded transaction
    /// @param rawDataStart Starting position of the raw data in the encoded transaction
    /// @param rawDataEnd Ending position of the raw data in the encoded transaction
    /// @return ownerTron The Tron address of the transaction owner
    /// @return contractTron The Tron address of the contract being triggered
    /// @return dataStart Starting position of the call data
    /// @return dataEnd Ending position of the call data
    function _parseTriggerFromRawData(bytes calldata encodedTx, uint256 rawDataStart, uint256 rawDataEnd)
        private
        pure
        returns (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd)
    {
        // 1. Parse the single Contract in raw_data.
        uint256 cStart;
        uint256 cEnd;
        uint64 cType;
        (cStart, cEnd, cType) = _readSingleContract(encodedTx, rawDataStart, rawDataEnd);
        // solhint-disable-next-line gas-strict-inequalities
        assert(cStart < cEnd && cEnd <= rawDataEnd);

        // 2. Enforce that it is a TriggerSmartContract.
        if (cType != _CONTRACT_TRIGGER_SMART) revert NotTriggerSmartContract();

        // 3. Extract the TriggerSmartContract message from inside the Contract.
        uint256 trigStart;
        uint256 trigEnd;
        (trigStart, trigEnd) = _extractTriggerSmartContract(encodedTx, cStart, cEnd);
        if (trigStart == 0 && trigEnd == 0) revert NotTriggerSmartContract();

        // 4. Parse headers: owner, contract, and call data slice.
        (ownerTron, contractTron, dataStart, dataEnd) = _parseTriggerHeaders(encodedTx, trigStart, trigEnd);
    }

    // ---------------- Protobuf parsing ----------------
    /// @notice Locates the `raw_data` field in a Tron `Transaction` and computes the Tron transaction id.
    /// @dev
    /// Tron `Transaction` encoding starts with field #1 (`raw_data`) which is length-delimited.
    /// This function expects the first byte to be 0x0A (field 1, wire type 2).
    /// @param tx_ The raw protobuf-encoded Tron `Transaction` bytes.
    /// @return rawDataStart The start offset of the `raw_data` message within `tx_` (inclusive).
    /// @return rawDataEnd The end offset of the `raw_data` message within `tx_` (exclusive).
    /// @return txId The Tron transaction id, defined as `sha256(raw_data_bytes)`.
    function _parseRawData(bytes calldata tx_)
        internal
        pure
        returns (uint256 rawDataStart, uint256 rawDataEnd, bytes32 txId)
    {
        uint256 totalLen = tx_.length;
        if (totalLen == 0 || uint8(tx_[0]) != 0x0A) revert NotTriggerSmartContract();

        uint256 offset = 1;
        uint64 rawDataLen;
        (rawDataLen, offset) = ProtoVarint.read(tx_, offset, totalLen);

        rawDataStart = offset;
        rawDataEnd = _advance(offset, uint256(rawDataLen), totalLen);

        // txId = sha256(raw_data bytes)
        txId = sha256(tx_[rawDataStart:rawDataEnd]);
    }

    /// @notice Reads and validates the single `Contract` entry in `raw_data` and returns its byte range and type.
    /// @dev
    /// - `raw_data` contains a repeated `contract` field (field #11), each of which is a `Contract` message.
    /// - This parser enforces that there is exactly one such entry, and then reads its internal
    ///   `type` (field #1, varint).
    /// @param tx_ The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param rawDataStart Start offset of the `raw_data` message (inclusive).
    /// @param rawDataEnd End offset of the `raw_data` message (exclusive).
    /// @return cStart Start offset of the `Contract` message bytes (inclusive).
    /// @return cEnd End offset of the `Contract` message bytes (exclusive).
    /// @return contractType The parsed `Contract.type` enum value.
    function _readSingleContract(bytes calldata tx_, uint256 rawDataStart, uint256 rawDataEnd)
        internal
        pure
        returns (uint256 cStart, uint256 cEnd, uint64 contractType)
    {
        uint256 cursor = rawDataStart;
        bool seenContract;

        // Monotonicity assertion: cursor must increase.
        uint256 prevCursor = cursor;
        while (cursor < rawDataEnd) {
            prevCursor = cursor;

            uint64 fieldNum;
            uint64 wireType;
            (fieldNum, wireType, cursor) = _readKey(tx_, cursor, rawDataEnd);
            assert(cursor > prevCursor); // Ensure forward progress

            if (fieldNum == 11 && wireType == _WIRE_LENGTH_DELIMITED) {
                // Enforce "exactly one" contract at the protobuf level.
                if (seenContract) {
                    // Optional: define a dedicated error if you want.
                    // revert TronUnexpectedExtraContracts();
                    revert NotTriggerSmartContract();
                }
                seenContract = true;

                (cStart, cEnd, cursor) = _readLength(tx_, cursor, rawDataEnd);

                bool foundType;
                (contractType, foundType) = _readContractType(tx_, cStart, cEnd);
                if (!foundType) revert NotTriggerSmartContract();

                // We’ve found the single contract and its type; we don’t care about later fields in raw_data.
                break;
            } else {
                cursor = _skipField(tx_, cursor, rawDataEnd, wireType);
            }
        }

        if (!seenContract) revert NotTriggerSmartContract();
    }

    /// @notice Reads `Contract.type` (field #1) from a Tron `Contract` message.
    /// @param tx_ The transaction data.
    /// @param contractStart The start position of the contract data.
    /// @param contractEnd The end position of the contract data.
    /// @return contractType The type of the contract.
    /// @return foundType Whether the contract type was found.
    function _readContractType(bytes calldata tx_, uint256 contractStart, uint256 contractEnd)
        internal
        pure
        returns (uint64 contractType, bool foundType)
    {
        uint256 p = contractStart;

        // Monotonicity assertion: p must increase.
        uint256 prevP = p;
        while (p < contractEnd) {
            prevP = p;

            uint64 cFieldNum;
            uint64 cWireType;
            (cFieldNum, cWireType, p) = _readKey(tx_, p, contractEnd);
            assert(p > prevP); // Ensure forward progress
            if (cFieldNum == 1 && cWireType == _WIRE_VARINT) {
                (contractType, p) = ProtoVarint.read(tx_, p, contractEnd);
                return (contractType, true);
            }
            p = _skipField(tx_, p, contractEnd, cWireType);
        }
    }

    /// @notice Extracts the embedded `TriggerSmartContract` message from a `Contract`'s parameter field.
    /// @dev
    /// Tron stores contract parameters as a protobuf `Any` inside `Contract.parameter` (field #2).
    /// We parse `Any.value` (field #2 within Any) and return the byte range of the contained message.
    /// @param tx_ The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param contractStart Start offset of the `Contract` message bytes (inclusive).
    /// @param contractEnd End offset of the `Contract` message bytes (exclusive).
    /// @return trigStart Start offset of the embedded `TriggerSmartContract` message bytes (inclusive).
    /// @return trigEnd End offset of the embedded `TriggerSmartContract` message bytes (exclusive).
    function _extractTriggerSmartContract(bytes calldata tx_, uint256 contractStart, uint256 contractEnd)
        internal
        pure
        returns (uint256 trigStart, uint256 trigEnd)
    {
        uint256 p = contractStart;
        uint256 paramStart = 0;
        uint256 paramEnd = 0;

        // Monotonicity assertion: p must increase.
        uint256 prevP = p;
        while (p < contractEnd) {
            prevP = p;

            uint64 cFieldNum;
            uint64 cWireType;
            (cFieldNum, cWireType, p) = _readKey(tx_, p, contractEnd);
            assert(p > prevP); // Ensure forward progress
            if (cFieldNum == 2 && cWireType == _WIRE_LENGTH_DELIMITED) {
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

    /// @notice Parses a protobuf `Any` message and returns the byte range of its `value` field.
    /// @dev
    /// `Any` fields used here:
    /// - field #1: `type_url` (ignored)
    /// - field #2: `value` (length-delimited bytes containing the embedded message)
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param paramStart Start offset of the `Any` message bytes (inclusive).
    /// @param paramEnd End offset of the `Any` message bytes (exclusive).
    /// @return valueStart Start offset of the embedded message bytes (inclusive).
    /// @return valueEnd End offset of the embedded message bytes (exclusive).
    function _parseAnyValueField(bytes calldata encodedTx, uint256 paramStart, uint256 paramEnd)
        internal
        pure
        returns (uint256 valueStart, uint256 valueEnd)
    {
        uint256 q = paramStart;
        valueStart = 0;
        valueEnd = 0;

        // Monotonicity assertion: q must increase.
        uint256 prevQ = q;
        while (q < paramEnd) {
            prevQ = q;

            uint64 anyFieldNum;
            uint64 anyWireType;
            (anyFieldNum, anyWireType, q) = _readKey(encodedTx, q, paramEnd);
            assert(q > prevQ); // Ensure forward progress
            if (anyFieldNum == 1 && anyWireType == _WIRE_LENGTH_DELIMITED) {
                (, q,) = _readLength(encodedTx, q, paramEnd);
            } else if (anyFieldNum == 2 && anyWireType == _WIRE_LENGTH_DELIMITED) {
                (valueStart, q,) = _readLength(encodedTx, q, paramEnd);
                valueEnd = q;
            } else {
                q = _skipField(encodedTx, q, paramEnd, anyWireType);
            }
        }
    }

    /// @notice Loads a canonical Tron address (`bytes21`) from calldata.
    /// @dev Reads 21 bytes starting at `start`. Reverts if out-of-bounds.
    /// @param data The calldata byte array being parsed.
    /// @param start The start offset within `data` (inclusive).
    /// @return out The 21-byte value loaded from calldata.
    function _readBytes21(bytes calldata data, uint256 start) internal pure returns (bytes21 out) {
        if (start + 21 > data.length) revert ProtoTruncated();
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            out := calldataload(add(data.offset, start))
        }
    }

    /// @notice Parses the `TriggerSmartContract` message fields we care about (owner, contract, calldata).
    /// @dev Expected fields within `TriggerSmartContract`:
    /// - field #1: owner_address (length-delimited, must be 21 bytes, prefix 0x41)
    /// - field #2: contract_address (length-delimited, must be 21 bytes, prefix 0x41)
    /// - field #4: data (length-delimited, arbitrary bytes)
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param trigStart Start offset of the embedded `TriggerSmartContract` message bytes (inclusive).
    /// @param trigEnd End offset of the embedded `TriggerSmartContract` message bytes (exclusive).
    /// @return ownerTron The sender/owner Tron address in canonical bytes21 form.
    /// @return contractTron The destination contract Tron address in canonical bytes21 form.
    /// @return dataStart Start offset of the call data bytes within `encodedTx` (inclusive).
    /// @return dataEnd End offset of the call data bytes within `encodedTx` (exclusive).
    function _parseTriggerHeaders(bytes calldata encodedTx, uint256 trigStart, uint256 trigEnd)
        internal
        pure
        returns (bytes21 ownerTron, bytes21 contractTron, uint256 dataStart, uint256 dataEnd)
    {
        uint256 trigCursor = trigStart;

        // Monotonicity assertion: trigCursor must increase.
        uint256 prevTrigCursor = trigCursor;
        while (trigCursor < trigEnd) {
            prevTrigCursor = trigCursor;

            uint64 tFieldNum;
            uint64 tWireType;
            (tFieldNum, tWireType, trigCursor) = _readKey(encodedTx, trigCursor, trigEnd);
            assert(trigCursor > prevTrigCursor); // Ensure forward progress
            if (tFieldNum == 1 && tWireType == _WIRE_LENGTH_DELIMITED) {
                uint256 oStart;
                uint256 oEnd;
                (oStart, oEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (oEnd - oStart != 21) revert TronInvalidOwnerLength();
                ownerTron = _readBytes21(encodedTx, oStart);
                if (uint8(ownerTron[0]) != 0x41) revert TronInvalidOwnerPrefix();
            } else if (tFieldNum == 2 && tWireType == _WIRE_LENGTH_DELIMITED) {
                uint256 cStart;
                uint256 cEnd;
                (cStart, cEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
                if (cEnd - cStart != 21) revert TronInvalidContractLength();
                contractTron = _readBytes21(encodedTx, cStart);
                if (uint8(contractTron[0]) != 0x41) revert TronInvalidContractPrefix();
            } else if (tFieldNum == 4 && tWireType == _WIRE_LENGTH_DELIMITED) {
                (dataStart, dataEnd, trigCursor) = _readLength(encodedTx, trigCursor, trigEnd);
            } else {
                trigCursor = _skipField(encodedTx, trigCursor, trigEnd, tWireType);
            }
        }
    }

    /* solhint-disable function-max-lines */
    // TODO: maybe optimize it

    /// @notice Parses the transaction result status and returns whether it indicates success.
    /// @dev
    /// Tron `Transaction` includes optional `signature` entries and a `ret`/result section.
    /// This function:
    /// - Skips any signature fields (field #2, tag 0x12) starting at `offset`.
    /// - Requires at least one result entry (field #5, tag 0x2A) to be present.
    /// - For each result entry:
    ///   - If a status/code field (field #2, varint) is present, it must be 0 (SUCESS).
    ///   - The contract execution result field (field #3, varint) must be 1 (SUCCESS).
    /// @param encodedTx The raw protobuf-encoded Tron `Transaction` bytes.
    /// @param offset The starting offset within `encodedTx` where `raw_data` ended.
    /// @param totalLen The total length of `encodedTx` (upper bound for reads).
    /// @return success True if the parsed result indicates success; false otherwise.
    function _parseTxSuccess(bytes calldata encodedTx, uint256 offset, uint256 totalLen) internal pure returns (bool) {
        // Skip signatures (field 2, tag 0x12)
        uint256 prevOffset = offset;
        while (offset < totalLen && uint8(encodedTx[offset]) == 0x12) {
            prevOffset = offset;

            ++offset;
            uint64 sigLen;
            (sigLen, offset) = ProtoVarint.read(encodedTx, offset, totalLen);
            assert(offset > prevOffset); // Ensure forward progress
            offset = _advance(offset, uint256(sigLen), totalLen);
        }
        // Parse one or more ret entries (field 5, tag 0x2A).
        bool sawRet;
        // solhint-disable-next-line gas-strict-inequalities
        while (offset < totalLen && uint8(encodedTx[offset]) == 0x2A) {
            sawRet = true;
            ++offset;

            uint256 resStart;
            uint256 resEnd;
            (resStart, resEnd, offset) = _readLength(encodedTx, offset, totalLen);
            if (!_parseTxRetEntry(encodedTx, resStart, resEnd)) return false;
        }

        return sawRet;
    }

    /// @notice Parses a transaction return entry to check if the transaction was successful
    /// @param encodedTx The complete encoded transaction
    /// @param resStart Starting position of the return entry in the encoded transaction
    /// @param resEnd Ending position of the return entry in the encoded transaction
    /// @return True if the transaction was successful, false otherwise
    function _parseTxRetEntry(bytes calldata encodedTx, uint256 resStart, uint256 resEnd) private pure returns (bool) {
        // Defaults per proto3: code==0 (SUCESS) unless explicitly set.
        uint64 contractRet;
        bool sawContractRet;

        uint256 cursor = resStart;
        uint256 prevResOffset = cursor;
        while (cursor < resEnd) {
            prevResOffset = cursor;

            uint64 fieldNum;
            uint64 wireType;
            (fieldNum, wireType, cursor) = _readKey(encodedTx, cursor, resEnd);
            assert(cursor > prevResOffset); // Ensure forward progress

            if (wireType == _WIRE_VARINT) {
                uint64 v;
                (v, cursor) = ProtoVarint.read(encodedTx, cursor, resEnd);
                if (fieldNum == 2) {
                    if (v != 0) return false;
                } else if (fieldNum == 3) {
                    sawContractRet = true;
                    contractRet = v;
                }
            } else {
                cursor = _skipField(encodedTx, cursor, resEnd, wireType);
            }
        }

        // For TriggerSmartContract, require a contract execution result to be present and SUCCESS (1).
        return sawContractRet && contractRet == 1;
    }

    /* solhint-enable function-max-lines */

    // ---------------- Utilities ----------------
    /// @notice Reads a length-delimited protobuf field payload and returns its bounds.
    /// @dev Interprets the next bytes at `cursor` as a varint length `len`, then returns
    /// `[start, end)` where `start` is the first byte of the payload and `end = start + len`.
    /// Reverts if `len` exceeds `limit - start`.
    /// @param data The calldata byte array being parsed.
    /// @param cursor The cursor positioned at the start of the length varint.
    /// @param limit The maximum offset within `data` that may be read (exclusive).
    /// @return start The payload start offset (inclusive).
    /// @return end The payload end offset (exclusive).
    /// @return newCursor The cursor position after the payload (equals `end`).
    function _readLength(bytes calldata data, uint256 cursor, uint256 limit)
        internal
        pure
        returns (uint256 start, uint256 end, uint256 newCursor)
    {
        uint64 len;
        (len, cursor) = ProtoVarint.read(data, cursor, limit);
        start = cursor;
        uint256 ulen = uint256(len);
        if (ulen > limit - cursor) revert ProtoTruncated();
        end = cursor + ulen;
        return (start, end, end);
    }

    /// @notice Reads a protobuf key (field number + wire type) at `pos`.
    /// @dev A protobuf key is a varint where: `fieldNum = key >> 3` and `wireType = key & 0x7`.
    /// @param data The calldata byte array being parsed.
    /// @param pos The start offset within `data` (inclusive).
    /// @param limit The maximum offset within `data` that may be read (exclusive).
    /// @return fieldNum The decoded protobuf field number.
    /// @return wireType The decoded protobuf wire type (0, 1, 2, or 5 are expected here).
    /// @return newPos The first position in `data` after the key varint.
    function _readKey(bytes calldata data, uint256 pos, uint256 limit)
        internal
        pure
        returns (uint64 fieldNum, uint64 wireType, uint256 newPos)
    {
        (uint64 key, uint256 p) = ProtoVarint.read(data, pos, limit);
        return (key >> 3, key & 0x7, p);
    }

    /// @notice Skips over a protobuf field payload based on its wire type.
    /// @dev Supports VARINT, FIXED32, FIXED64, and LENGTH_DELIMITED. Reverts on unknown wire types.
    /// @param data The calldata byte array being parsed.
    /// @param cursor The cursor positioned at the start of the field payload (immediately after the key).
    /// @param limit The maximum offset within `data` that may be read (exclusive).
    /// @param wireType The protobuf wire type for the field payload.
    /// @return newCursor The cursor position immediately after the skipped payload.
    function _skipField(bytes calldata data, uint256 cursor, uint256 limit, uint64 wireType)
        internal
        pure
        returns (uint256 newCursor)
    {
        if (wireType == _WIRE_VARINT) return ProtoVarint.skip(data, cursor, limit);
        if (wireType == _WIRE_LENGTH_DELIMITED) {
            (, uint256 end,) = _readLength(data, cursor, limit);
            return end;
        }
        if (wireType == _WIRE_FIXED32) return _advance(cursor, 4, limit);
        if (wireType == _WIRE_FIXED64) return _advance(cursor, 8, limit);
        revert ProtoInvalidWireType();
    }

    /// @notice Advances `cursor` by `delta` bytes, bounded by `limit`.
    /// @dev Reverts with `ProtoTruncated()` if advancing would exceed `limit`.
    /// @param cursor The current cursor position.
    /// @param delta The number of bytes to advance.
    /// @param limit The maximum offset allowed (exclusive).
    /// @return The advanced cursor position (`cursor + delta`).
    function _advance(uint256 cursor, uint256 delta, uint256 limit) internal pure returns (uint256) {
        // Explicit non-overflow + bounds condition.
        if (delta > limit - cursor) revert ProtoTruncated();
        return cursor + delta;
    }

    /// @notice Copies a `[start, end)` slice from calldata into a new `bytes` in memory.
    /// @dev Reverts if `start/end` are out of bounds or inverted.
    /// @param data The calldata byte array to copy from.
    /// @param start The start offset within `data` (inclusive).
    /// @param end The end offset within `data` (exclusive).
    /// @return out The copied bytes slice in memory.
    function _slice(bytes calldata data, uint256 start, uint256 end) internal pure returns (bytes memory out) {
        if (end < start || end > data.length) revert ProtoTruncated();
        uint256 len = end - start;
        out = new bytes(len);
        for (uint256 i = 0; i < len; ++i) {
            out[i] = data[start + i];
        }
    }
}
