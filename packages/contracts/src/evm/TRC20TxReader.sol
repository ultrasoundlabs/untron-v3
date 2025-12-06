// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {TronLightClient} from "./TronLightClient.sol";
import {TronSha256MerkleVerifier} from "../utils/TronSha256MerkleVerifier.sol";

/// @title TRC20TxReader
/// @notice Verifies Tron transactions via Merkle proofs, enforces one-time nullifiers,
///         and decodes TRC-20 transfers into a structured form.
/// @dev Import this contract to read TRC-20 transactions within another contract.
contract TRC20TxReader {
    TronLightClient public tronLightClient;

    /// @notice Nullifier set to prevent double-use of the same Tron transaction.
    mapping(bytes32 => bool) public txNullifierSet;

    /// @dev TRC-20 function selectors for transfer and transferFrom (Ethereum ABI format).
    bytes4 internal constant SELECTOR_TRANSFER = 0xa9059cbb; // transfer(address,uint256)
    bytes4 internal constant SELECTOR_TRANSFER_FROM = 0x23b872dd; // transferFrom(address,address,uint256)

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

    // -------------------------- Error Definitions --------------------------

    error TronLightClientNotSet();
    error InvalidTxMerkleProof();
    error TxAlreadyNullified(bytes32 txLeaf);
    error NotATrc20Transfer();
    error Trc20TransferNotSuccessful();

    /// @notice Internal function to update the linked Tron light client contract.
    /// @dev Should be called in the inheriting contract’s initializer.
    function _updateTronLightClient(address newTronLightClient) internal {
        tronLightClient = TronLightClient(newTronLightClient);
    }

    /// @notice High-level function to prove, nullify, and decode a TRC-20 transfer from a Tron transaction.
    /// @dev Reverts if:
    ///      – The Merkle proof is invalid (transaction not in the claimed block).
    ///      – The transaction was already consumed (nullifier hit).
    ///      – The transaction is not a valid TRC-20 transfer or transferFrom.
    ///      – The transaction execution was not successful on Tron.
    /// @param tronBlockNumber The Tron block number in which the transaction was included.
    /// @param encodedTx The full protobuf-encoded Tron `Transaction` bytes.
    /// @param proof Merkle proof sibling hashes from leaf to root.
    /// @param index Merkle path bitfield for the transaction’s position in the tree.
    /// @return transfer The decoded TRC-20 transfer details (if proof and decoding succeed).
    function readAndNullifyTrc20Transfer(
        uint256 tronBlockNumber,
        bytes memory encodedTx,
        bytes32[] memory proof,
        uint256 index
    ) internal returns (Trc20Transfer memory transfer) {
        // **1. Verify inclusion in block and mark transaction as consumed (nullifier).**
        bytes32 txLeaf = _consumeTronTransaction(tronBlockNumber, encodedTx, proof, index);

        // **2. Decode TRC-20 transfer details from the transaction bytes.**
        (bytes21 fromTron, bytes21 toTron, address tronTokenEvm, uint256 amount, bool isTransferFrom, bool success) =
            _decodeTrc20TransferFromTx(encodedTx);

        if (!success) {
            // The Tron transaction executed with a failure status code.
            revert Trc20TransferNotSuccessful();
        }

        // **3. Fetch the block timestamp from the light client for context.**
        uint32 blockTs = tronLightClient.getBlockTimestamp(tronBlockNumber);

        // **4. Assemble the transfer struct to return.**
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

    /// @notice Verifies that a given encoded Tron transaction is included in a specified block (via Merkle proof).
    /// @param tronBlockNumber The Tron block number where the transaction should be included.
    /// @param encodedTx Full protobuf-encoded Tron `Transaction` bytes.
    /// @param proof Sibling hashes from the transaction leaf up to the Merkle root.
    /// @param index Bitfield representing the transaction’s position in the Merkle tree (0 = left, 1 = right at each level).
    /// @return txLeaf The SHA-256 hash of the encoded transaction (the Merkle leaf value).
    function _verifyTxInclusion(uint256 tronBlockNumber, bytes memory encodedTx, bytes32[] memory proof, uint256 index)
        internal
        view
        returns (bytes32 txLeaf)
    {
        if (address(tronLightClient) == address(0)) {
            // The Tron light client contract must be set.
            revert TronLightClientNotSet();
        }
        // Retrieve the expected Merkle root for the block from the light client.
        bytes32 root = tronLightClient.getTxTrieRoot(tronBlockNumber);
        // Compute the leaf hash from the provided transaction bytes (Tron uses sha256 for transaction hashing).
        txLeaf = sha256(encodedTx);
        // Verify the Merkle proof using the computed leaf and provided siblings.
        bool valid = TronSha256MerkleVerifier.verify(root, txLeaf, proof, index);
        if (!valid) {
            // Proof did not yield the expected root.
            revert InvalidTxMerkleProof();
        }
    }

    /// @notice Verifies inclusion and marks a Tron transaction as used (one-time consumption).
    /// @dev This will revert if the transaction was already used before.
    /// @return txLeaf The Merkle leaf hash of the transaction (for downstream use as nullifier key).
    function _consumeTronTransaction(
        uint256 tronBlockNumber,
        bytes memory encodedTx,
        bytes32[] memory proof,
        uint256 index
    ) private returns (bytes32 txLeaf) {
        // Prove the transaction is in the block.
        txLeaf = _verifyTxInclusion(tronBlockNumber, encodedTx, proof, index);
        // Enforce one-time usage of this transaction via nullifier set.
        if (txNullifierSet[txLeaf]) {
            revert TxAlreadyNullified(txLeaf);
        }
        txNullifierSet[txLeaf] = true;
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
    /// @return success True if the Tron transaction’s execution status was SUCCESS.
    function _decodeTrc20TransferFromTx(bytes memory encodedTx)
        internal
        pure
        returns (
            bytes21 fromTron,
            bytes21 toTron,
            address tronTokenEvm,
            uint256 amount,
            bool isTransferFrom,
            bool success
        )
    {
        uint256 totalLen = encodedTx.length;
        uint256 offset = 0;

        // **(a) Parse `Transaction.raw_data` (field 1) to find contract details.**
        if (totalLen == 0 || uint8(encodedTx[0]) != 0x0A) {
            // The first byte 0x0A indicates field #1 (raw_data) with wire type 2 (length-delimited).
            // If it's missing or incorrect, this is not a valid Tron Transaction encoding.
            revert NotATrc20Transfer();
        }
        offset += 1;

        // Read the length of the `raw_data` message (varint).
        uint64 rawDataLen = 0;
        uint64 shift = 0;
        while (true) {
            require(offset < totalLen, "Truncated raw_data length");
            uint8 b = uint8(encodedTx[offset++]);
            rawDataLen |= uint64(b & 0x7F) << shift;
            if ((b & 0x80) == 0) break;
            shift += 7;
        }
        uint256 rawDataStart = offset;
        uint256 rawDataEnd = offset + uint256(rawDataLen);
        require(rawDataEnd <= totalLen, "Truncated raw_data bytes");

        // **(b) Traverse `raw_data.contract` fields (field #11) to find a TriggerSmartContract.**
        bool found = false;
        bytes21 ownerAddr; // Tron sender address from TriggerSmartContract.owner_address
        bytes21 contractAddr; // Tron contract address (token contract) from TriggerSmartContract.contract_address
        address toEvmAddr;
        address fromEvmAddr;
        bytes4 selector = 0x0;

        uint256 cursor = rawDataStart;
        while (cursor < rawDataEnd) {
            // Read the next field key in raw_data (could be contract or other fields).
            uint64 fieldKey = 0;
            shift = 0;
            while (true) {
                require(cursor < rawDataEnd, "Truncated raw_data field");
                uint8 b = uint8(encodedTx[cursor++]);
                fieldKey |= uint64(b & 0x7F) << shift;
                if ((b & 0x80) == 0) break;
                shift += 7;
            }
            uint64 fieldNum = fieldKey >> 3;
            uint64 wireType = fieldKey & 0x7;

            if (fieldNum == 11 && wireType == 2) {
                // **Found a Contract message (field #11). Parse it.**
                // First, read the length of this Contract message.
                uint64 contractLen = 0;
                shift = 0;
                while (true) {
                    require(cursor < rawDataEnd, "Truncated contract length");
                    uint8 b = uint8(encodedTx[cursor++]);
                    contractLen |= uint64(b & 0x7F) << shift;
                    if ((b & 0x80) == 0) break;
                    shift += 7;
                }
                uint256 contractStart = cursor;
                uint256 contractEnd = contractStart + uint256(contractLen);
                require(contractEnd <= rawDataEnd, "Truncated contract bytes");

                // Within the Contract message, we expect:
                // field #1: type (enum value), field #2: parameter (Any), etc.
                uint64 contractType = 0;
                uint256 paramStart = 0;
                uint256 paramEnd = 0;

                uint256 p = contractStart;
                while (p < contractEnd) {
                    // Read field key inside Contract
                    uint64 cKey = 0;
                    shift = 0;
                    while (true) {
                        require(p < contractEnd, "Truncated contract field");
                        uint8 b = uint8(encodedTx[p++]);
                        cKey |= uint64(b & 0x7F) << shift;
                        if ((b & 0x80) == 0) break;
                        shift += 7;
                    }
                    uint64 cFieldNum = cKey >> 3;
                    uint64 cWireType = cKey & 0x7;
                    if (cFieldNum == 1 && cWireType == 0) {
                        // Contract type (varint).
                        uint64 cTypeVal = 0;
                        shift = 0;
                        while (true) {
                            require(p < contractEnd, "Truncated contract type");
                            uint8 b = uint8(encodedTx[p++]);
                            cTypeVal |= uint64(b & 0x7F) << shift;
                            if ((b & 0x80) == 0) break;
                            shift += 7;
                        }
                        contractType = cTypeVal;
                    } else if (cFieldNum == 2 && cWireType == 2) {
                        // Parameter field (google.protobuf.Any message).
                        // Read length of the Any message.
                        uint64 anyLen = 0;
                        shift = 0;
                        while (true) {
                            require(p < contractEnd, "Truncated Any length");
                            uint8 b = uint8(encodedTx[p++]);
                            anyLen |= uint64(b & 0x7F) << shift;
                            if ((b & 0x80) == 0) break;
                            shift += 7;
                        }
                        paramStart = p;
                        paramEnd = p + uint256(anyLen);
                        require(paramEnd <= contractEnd, "Truncated Any bytes");

                        // **Parse the Any message to extract the TriggerSmartContract bytes.**
                        // The Any message contains:
                        // field #1: type_url (string), field #2: value (bytes of actual message).
                        uint256 q = paramStart;
                        uint256 valueStart = 0;
                        uint256 valueLen = 0;
                        while (q < paramEnd) {
                            uint64 anyKey = 0;
                            shift = 0;
                            while (true) {
                                require(q < paramEnd, "Truncated Any field");
                                uint8 b = uint8(encodedTx[q++]);
                                anyKey |= uint64(b & 0x7F) << shift;
                                if ((b & 0x80) == 0) break;
                                shift += 7;
                            }
                            uint64 anyFieldNum = anyKey >> 3;
                            uint64 anyWireType = anyKey & 0x7;
                            if (anyFieldNum == 1 && anyWireType == 2) {
                                // type_url field: skip its contents.
                                uint64 urlLen = 0;
                                shift = 0;
                                while (true) {
                                    require(q < paramEnd, "Truncated type_url length");
                                    uint8 b = uint8(encodedTx[q++]);
                                    urlLen |= uint64(b & 0x7F) << shift;
                                    if ((b & 0x80) == 0) break;
                                    shift += 7;
                                }
                                q += uint256(urlLen); // skip the type_url bytes
                                require(q <= paramEnd, "Truncated type_url bytes");
                            } else if (anyFieldNum == 2 && anyWireType == 2) {
                                // value field: this contains the TriggerSmartContract message bytes.
                                uint64 valLen = 0;
                                shift = 0;
                                while (true) {
                                    require(q < paramEnd, "Truncated value length");
                                    uint8 b = uint8(encodedTx[q++]);
                                    valLen |= uint64(b & 0x7F) << shift;
                                    if ((b & 0x80) == 0) break;
                                    shift += 7;
                                }
                                valueStart = q;
                                valueLen = uint256(valLen);
                                q += valueLen; // skip past the TriggerSmartContract bytes
                                require(q <= paramEnd, "Truncated TriggerSmartContract bytes");
                            } else if (anyWireType == 0) {
                                // Skip unknown varint field in Any.
                                while (true) {
                                    require(q < paramEnd, "Truncated Any varint");
                                    uint8 b = uint8(encodedTx[q++]);
                                    if ((b & 0x80) == 0) break;
                                }
                            } else if (anyWireType == 2) {
                                // Skip unknown length-delimited field in Any.
                                uint64 skipLen = 0;
                                shift = 0;
                                while (true) {
                                    require(q < paramEnd, "Truncated Any skip length");
                                    uint8 b = uint8(encodedTx[q++]);
                                    skipLen |= uint64(b & 0x7F) << shift;
                                    if ((b & 0x80) == 0) break;
                                    shift += 7;
                                }
                                q += uint256(skipLen);
                                require(q <= paramEnd, "Truncated Any skip bytes");
                            } else if (anyWireType == 5) {
                                q += 4; // skip 32-bit
                            } else if (anyWireType == 1) {
                                q += 8; // skip 64-bit
                            } else {
                                revert NotATrc20Transfer();
                            }
                        } // end while parsing Any

                        // If this contract is TriggerSmartContract (type 31) and we have extracted its bytes:
                        if (contractType == 31 && valueStart != 0) {
                            // **Parse the TriggerSmartContract message** to get TRC-20 call details.
                            uint256 trigCursor = valueStart;
                            uint256 trigEnd = valueStart + valueLen;
                            // Reset local variables for each contract attempt
                            ownerAddr = bytes21(0);
                            contractAddr = bytes21(0);
                            tronTokenEvm = address(0);
                            fromTron = bytes21(0);
                            toTron = bytes21(0);
                            amount = 0;
                            isTransferFrom = false;
                            selector = 0x0;
                            toEvmAddr = address(0);
                            fromEvmAddr = address(0);

                            while (trigCursor < trigEnd) {
                                // Read field key in TriggerSmartContract
                                uint64 tKey = 0;
                                shift = 0;
                                while (true) {
                                    require(trigCursor < trigEnd, "Truncated Trigger field");
                                    uint8 b = uint8(encodedTx[trigCursor++]);
                                    tKey |= uint64(b & 0x7F) << shift;
                                    if ((b & 0x80) == 0) break;
                                    shift += 7;
                                }
                                uint64 tFieldNum = tKey >> 3;
                                uint64 tWireType = tKey & 0x7;

                                if (tFieldNum == 1 && tWireType == 2) {
                                    // owner_address (bytes21)
                                    uint64 ownerLen = 0;
                                    shift = 0;
                                    while (true) {
                                        require(trigCursor < trigEnd, "Truncated owner_address length");
                                        uint8 b = uint8(encodedTx[trigCursor++]);
                                        ownerLen |= uint64(b & 0x7F) << shift;
                                        if ((b & 0x80) == 0) break;
                                        shift += 7;
                                    }
                                    require(ownerLen == 21, "Invalid owner_address length");
                                    require(trigCursor + 21 <= trigEnd, "Truncated owner_address");
                                    // Load the 21-byte owner address from memory
                                    bytes21 tmp;
                                    assembly ("memory-safe") {
                                        tmp := mload(add(add(encodedTx, 0x20), trigCursor))
                                    }
                                    require(uint8(tmp[0]) == 0x41, "owner_address prefix invalid");
                                    ownerAddr = tmp;
                                    trigCursor += 21;
                                } else if (tFieldNum == 2 && tWireType == 2) {
                                    // contract_address (bytes21 of token contract)
                                    uint64 contractLen2 = 0;
                                    shift = 0;
                                    while (true) {
                                        require(trigCursor < trigEnd, "Truncated contract_address length");
                                        uint8 b = uint8(encodedTx[trigCursor++]);
                                        contractLen2 |= uint64(b & 0x7F) << shift;
                                        if ((b & 0x80) == 0) break;
                                        shift += 7;
                                    }
                                    require(contractLen2 == 21, "Invalid contract_address length");
                                    require(trigCursor + 21 <= trigEnd, "Truncated contract_address");
                                    bytes21 tmp;
                                    assembly ("memory-safe") {
                                        tmp := mload(add(add(encodedTx, 0x20), trigCursor))
                                    }
                                    require(uint8(tmp[0]) == 0x41, "contract_address prefix invalid");
                                    contractAddr = tmp;
                                    // Compute the 20-byte EVM token address by stripping the 0x41 prefix.
                                    // The lower 20 bytes of the 21-byte Tron address are the EVM address.
                                    tronTokenEvm = address(uint160(uint168(tmp)));
                                    trigCursor += 21;
                                } else if (tFieldNum == 4 && tWireType == 2) {
                                    // data (contract call data for TRC-20 function)
                                    uint64 dataLen = 0;
                                    shift = 0;
                                    while (true) {
                                        require(trigCursor < trigEnd, "Truncated data length");
                                        uint8 b = uint8(encodedTx[trigCursor++]);
                                        dataLen |= uint64(b & 0x7F) << shift;
                                        if ((b & 0x80) == 0) break;
                                        shift += 7;
                                    }
                                    uint256 dataStart = trigCursor;
                                    uint256 dataEnd = dataStart + uint256(dataLen);
                                    require(dataEnd <= trigEnd, "Truncated data bytes");
                                    // Need at least 4 bytes for function selector.
                                    if (dataLen < 4) {
                                        revert NotATrc20Transfer();
                                    }
                                    // Read the 4-byte selector from the data using byte-wise reads
                                    bytes4 sig;
                                    bytes32 first32;
                                    uint32 sel;
                                    {
                                        uint256 pData = dataStart;
                                        uint8 b0 = uint8(encodedTx[pData]);
                                        uint8 b1 = uint8(encodedTx[pData + 1]);
                                        uint8 b2 = uint8(encodedTx[pData + 2]);
                                        uint8 b3 = uint8(encodedTx[pData + 3]);
                                        sel = (uint32(b0) << 24) | (uint32(b1) << 16) | (uint32(b2) << 8) | uint32(b3);
                                        sig = bytes4(sel);
                                    }
                                    assembly ("memory-safe") {
                                        first32 := mload(add(add(encodedTx, 0x20), dataStart))
                                    }
                                    // no-op

                                    if (sig == SELECTOR_TRANSFER) {
                                        // **TRC-20 transfer(address to, uint256 amount) detected.**
                                        require(dataLen == 4 + 32 * 2, "transfer data length mismatch");
                                        // Load the 64 bytes of function arguments (skip the 4-byte selector).
                                        bytes32 word1;
                                        bytes32 word2;
                                        assembly ("memory-safe") {
                                            word1 := mload(add(add(encodedTx, 0x20), add(dataStart, 4)))
                                            word2 := mload(add(add(encodedTx, 0x20), add(dataStart, 36)))
                                        }
                                        // The `word1` contains the 32-byte ABI-encoded address (12 leading zero bytes + 20-byte address).
                                        toEvmAddr = address(uint160(uint256(word1)));
                                        // Construct Tron-format address for recipient: prefix 0x41 + 20-byte address.
                                        // casting to 'bytes21' is safe because we explicitly combine a 1-byte prefix with a 20-byte address
                                        // forge-lint: disable-next-line(unsafe-typecast)
                                        toTron = bytes21(uint168(uint160(uint256(word1))) | (uint168(0x41) << 160));
                                        // Sender (fromTron) is the ownerAddr from TriggerSmartContract.
                                        fromTron = ownerAddr;
                                        // Decode token amount.
                                        amount = uint256(word2);
                                        isTransferFrom = false;
                                        selector = sig;
                                    } else if (sig == SELECTOR_TRANSFER_FROM) {
                                        // **TRC-20 transferFrom(address from, address to, uint256 amount) detected.**
                                        require(dataLen == 4 + 32 * 3, "transferFrom data length mismatch");
                                        bytes32 word1;
                                        bytes32 word2;
                                        bytes32 word3;
                                        assembly ("memory-safe") {
                                            word1 := mload(add(add(encodedTx, 0x20), add(dataStart, 4)))
                                            word2 := mload(add(add(encodedTx, 0x20), add(dataStart, 36)))
                                            word3 := mload(add(add(encodedTx, 0x20), add(dataStart, 68)))
                                        }
                                        fromEvmAddr = address(uint160(uint256(word1)));
                                        toEvmAddr = address(uint160(uint256(word2)));
                                        // Construct Tron addresses (0x41 prefix + 20 bytes).
                                        // casting to 'bytes21' is safe because we explicitly combine a 1-byte prefix with a 20-byte address
                                        // forge-lint: disable-next-line(unsafe-typecast)
                                        fromTron = bytes21(uint168(uint160(uint256(word1))) | (uint168(0x41) << 160));
                                        // casting to 'bytes21' is safe because we explicitly combine a 1-byte prefix with a 20-byte address
                                        // forge-lint: disable-next-line(unsafe-typecast)
                                        toTron = bytes21(uint168(uint160(uint256(word2))) | (uint168(0x41) << 160));
                                        amount = uint256(word3);
                                        isTransferFrom = true;
                                        selector = sig;
                                    } else {
                                        // Not a TRC-20 transfer function - ignore this contract and continue.
                                        selector = 0x0;
                                    }
                                    trigCursor = dataEnd; // move cursor to end of data field
                                } else if (tWireType == 0) {
                                    // Skip any varint fields in TriggerSmartContract (e.g., call_value or token_id).
                                    while (trigCursor < trigEnd) {
                                        uint8 b = uint8(encodedTx[trigCursor++]);
                                        if ((b & 0x80) == 0) break;
                                    }
                                } else if (tWireType == 2) {
                                    // Skip unknown length-delimited fields (not expected for known Trigger fields).
                                    uint64 skipLength = 0;
                                    shift = 0;
                                    while (true) {
                                        require(trigCursor < trigEnd, "Truncated trigger skip field");
                                        uint8 b = uint8(encodedTx[trigCursor++]);
                                        skipLength |= uint64(b & 0x7F) << shift;
                                        if ((b & 0x80) == 0) break;
                                        shift += 7;
                                    }
                                    trigCursor += uint256(skipLength);
                                    require(trigCursor <= trigEnd, "Truncated trigger skip bytes");
                                } else if (tWireType == 5) {
                                    trigCursor += 4; // skip 32-bit field
                                } else if (tWireType == 1) {
                                    trigCursor += 8; // skip 64-bit field
                                } else {
                                    revert NotATrc20Transfer();
                                }
                            } // end inner while (TriggerSmartContract fields)

                            if (selector == SELECTOR_TRANSFER || selector == SELECTOR_TRANSFER_FROM) {
                                // We successfully decoded a TRC-20 transfer call.
                                found = true;
                                // Exit the parsing loops early since we have what we need.
                                p = contractEnd; // break out of Contract parsing loop
                                cursor = rawDataEnd; // break out of raw_data loop
                            }
                        } // end if contractType == 31
                        // Advance pointer p to end of Any message (skip any remaining if not already skipped).
                        p = paramEnd;
                    } else if (cWireType == 0) {
                        // Skip unknown varint field inside Contract (e.g., Permission_id).
                        while (p < contractEnd) {
                            uint8 b = uint8(encodedTx[p++]);
                            if ((b & 0x80) == 0) break;
                        }
                    } else if (cWireType == 2) {
                        // Skip unknown length-delimited field inside Contract.
                        uint64 skipLen2 = 0;
                        shift = 0;
                        while (true) {
                            require(p < contractEnd, "Truncated contract skip length");
                            uint8 b = uint8(encodedTx[p++]);
                            skipLen2 |= uint64(b & 0x7F) << shift;
                            if ((b & 0x80) == 0) break;
                            shift += 7;
                        }
                        p += uint256(skipLen2);
                        require(p <= contractEnd, "Truncated contract skip bytes");
                    } else if (cWireType == 5) {
                        p += 4; // skip 32-bit
                    } else if (cWireType == 1) {
                        p += 8; // skip 64-bit
                    } else {
                        revert NotATrc20Transfer();
                    }
                } // end while (Contract fields)

                // Move main cursor to the end of this Contract message.
                cursor = contractEnd;
            } else if (wireType == 0) {
                // Skip any other varint field in raw_data.
                while (cursor < rawDataEnd) {
                    uint8 b = uint8(encodedTx[cursor++]);
                    if ((b & 0x80) == 0) break;
                }
            } else if (wireType == 2) {
                // Skip any other length-delimited field in raw_data.
                uint64 skipLen = 0;
                shift = 0;
                while (true) {
                    require(cursor < rawDataEnd, "Truncated raw_data skip length");
                    uint8 b = uint8(encodedTx[cursor++]);
                    skipLen |= uint64(b & 0x7F) << shift;
                    if ((b & 0x80) == 0) break;
                    shift += 7;
                }
                cursor += uint256(skipLen);
                require(cursor <= rawDataEnd, "Truncated raw_data skip bytes");
            } else if (wireType == 5) {
                cursor += 4; // skip 32-bit field in raw_data (if any)
            } else if (wireType == 1) {
                cursor += 8; // skip 64-bit field in raw_data (if any)
            } else {
                // Unknown wire type in raw_data
                revert NotATrc20Transfer();
            }
        } // end while (raw_data fields)

        if (!found) {
            // No valid TRC-20 transfer call was found in any contract.
            revert NotATrc20Transfer();
        }

        // **(c) After decoding the transfer call, parse the Transaction result to determine `success`.**
        // Move offset to the end of raw_data (skip signatures and then get the first Result).
        offset = rawDataEnd;
        // Skip all signature fields (field #2) if present.
        while (offset < totalLen && uint8(encodedTx[offset]) == 0x12) {
            // 0x12 indicates field #2 (signature) with wire type 2.
            offset += 1;
            // Read the length of the signature (varint) and skip that many bytes.
            uint64 sigLen = 0;
            shift = 0;
            while (true) {
                require(offset < totalLen, "Truncated signature length");
                uint8 b = uint8(encodedTx[offset++]);
                sigLen |= uint64(b & 0x7F) << shift;
                if ((b & 0x80) == 0) break;
                shift += 7;
            }
            offset += uint256(sigLen);
            require(offset <= totalLen, "Truncated signature bytes");
        }
        // The next field should be field #5 (Result) if present.
        success = true; // assume success unless we find an explicit failure code
        if (offset < totalLen && uint8(encodedTx[offset]) == 0x2A) {
            // 0x2A indicates field #5 (Transaction.Result) with wire type 2.
            offset += 1;
            // Read the length of the Result message.
            uint64 resLen = 0;
            shift = 0;
            while (true) {
                require(offset < totalLen, "Truncated result length");
                uint8 b = uint8(encodedTx[offset++]);
                resLen |= uint64(b & 0x7F) << shift;
                if ((b & 0x80) == 0) break;
                shift += 7;
            }
            uint256 resStart = offset;
            uint256 resEnd = resStart + uint256(resLen);
            require(resEnd <= totalLen, "Truncated result bytes");

            // Parse fields in Transaction.Result (looking for the `ret` code field).
            uint256 r = resStart;
            while (r < resEnd) {
                uint64 rKey = 0;
                shift = 0;
                while (true) {
                    require(r < resEnd, "Truncated result field");
                    uint8 b = uint8(encodedTx[r++]);
                    rKey |= uint64(b & 0x7F) << shift;
                    if ((b & 0x80) == 0) break;
                    shift += 7;
                }
                uint64 rFieldNum = rKey >> 3;
                uint64 rWireType = rKey & 0x7;
                if (rFieldNum == 2 && rWireType == 0) {
                    // This is the `ret` field (Transaction.Result.code) as a varint.
                    uint64 statusCode = 0;
                    shift = 0;
                    while (true) {
                        require(r < resEnd, "Truncated ret code");
                        uint8 b = uint8(encodedTx[r++]);
                        statusCode |= uint64(b & 0x7F) << shift;
                        if ((b & 0x80) == 0) break;
                        shift += 7;
                    }
                    // Tron’s SUCCESS code is 0.
                    if (statusCode != 0) {
                        success = false;
                    }
                } else if (rWireType == 0) {
                    // Skip other varint fields in Result (fee, contractRet code, etc).
                    while (r < resEnd) {
                        uint8 b = uint8(encodedTx[r++]);
                        if ((b & 0x80) == 0) break;
                    }
                } else if (rWireType == 2) {
                    // Skip any length-delimited fields in Result (e.g., error message).
                    uint64 resSkipLen = 0;
                    shift = 0;
                    while (true) {
                        require(r < resEnd, "Truncated result skip length");
                        uint8 b = uint8(encodedTx[r++]);
                        resSkipLen |= uint64(b & 0x7F) << shift;
                        if ((b & 0x80) == 0) break;
                        shift += 7;
                    }
                    r += uint256(resSkipLen);
                    require(r <= resEnd, "Truncated result skip bytes");
                } else if (rWireType == 5) {
                    r += 4; // skip 32-bit field
                } else if (rWireType == 1) {
                    r += 8; // skip 64-bit field
                } else {
                    revert NotATrc20Transfer();
                }
            }
        }
        // By this point, fromTron, toTron, tronTokenEvm, amount, isTransferFrom, and success have been set appropriately.
    }

    /// @notice Computes the Tron transaction Merkle leaf as sha256(encodedTx)
    function _computeTxLeaf(bytes memory encodedTx) internal pure returns (bytes32) {
        return sha256(encodedTx);
    }
}
