// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronTxReader} from "../../../src/evm/TronTxReader.sol";
import {MockTronLightClient} from "./mocks/MockTronLightClient.sol";

/// @dev This test deploys TronTxReader and uses its public helpers.
contract TronTxReaderTest is Test {
    MockTronLightClient internal _lightClient;
    TronTxReader internal _reader;

    // TRC-20 selectors used in fixture.
    bytes4 internal constant _SELECTOR_TRANSFER = bytes4(0xa9059cbb);
    bytes4 internal constant _SELECTOR_TRANSFER_FROM = bytes4(0x23b872dd);

    // IMPORTANT: Field order must match the JSON object's key order for abi.decode to succeed.
    // JSON key order (per fixture):
    // index, txId, txLeaf, encodedTx, tronBlockNumber, tronBlockTimestamp,
    // tronTokenEvm, fromTron, toTron, amount, isTransferFrom, success, selector
    struct Trc20TxJson {
        address tronTokenEvm;
        bool isTransferFrom;
        bool success;
        bytes encodedTx;
        bytes fromTron;
        bytes selector;
        bytes toTron;
        bytes32 txId;
        bytes32 txLeaf;
        string amount;
        string tronBlockNumber;
        string tronBlockTimestamp;
        uint256 index;
    }

    /// @notice Deploy a mock light client and the TronTxReader bound to it.
    function setUp() public {
        _lightClient = new MockTronLightClient();
        _reader = new TronTxReader(address(_lightClient));
    }

    /// @notice Test decoding of real TRC-20 transactions using a fixture from Tron mainnet.
    function testReadTrc20TransfersFromFixture() public {
        // Load the JSON fixture containing Tron block and TRC-20 transactions.
        // Safe in test context: fixture file is readonly and controlled
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/TronTxReader/fixtures/trc20_block_78115149.json");

        // Parse block-level data from the JSON.
        uint256 blockNumber = abi.decode(vm.parseJson(json, ".blockNumber"), (uint256));
        uint32 blockTimestamp = uint32(abi.decode(vm.parseJson(json, ".blockTimestamp"), (uint256)));

        // Determine number of TRC-20 transactions without decoding entire array at once (to avoid OOM).
        uint256 count = 0;
        while (vm.keyExistsJson(json, string.concat(".trc20Txs[", _uToString(count), "]"))) {
            unchecked {
                ++count;
            }
        }
        Trc20TxJson[] memory trc20Txs = new Trc20TxJson[](count);
        for (uint256 i = 0; i < count; ++i) {
            string memory base = string.concat(".trc20Txs[", _uToString(i), "]");
            trc20Txs[i].index = abi.decode(vm.parseJson(json, string.concat(base, ".index")), (uint256));
            trc20Txs[i].txId = abi.decode(vm.parseJson(json, string.concat(base, ".txId")), (bytes32));
            trc20Txs[i].txLeaf = abi.decode(vm.parseJson(json, string.concat(base, ".txLeaf")), (bytes32));
            trc20Txs[i].encodedTx = abi.decode(vm.parseJson(json, string.concat(base, ".encodedTx")), (bytes));
            trc20Txs[i].tronBlockNumber =
                abi.decode(vm.parseJson(json, string.concat(base, ".tronBlockNumber")), (string));
            trc20Txs[i].tronBlockTimestamp =
                abi.decode(vm.parseJson(json, string.concat(base, ".tronBlockTimestamp")), (string));
            trc20Txs[i].tronTokenEvm = abi.decode(vm.parseJson(json, string.concat(base, ".tronTokenEvm")), (address));
            trc20Txs[i].fromTron = abi.decode(vm.parseJson(json, string.concat(base, ".fromTron")), (bytes));
            trc20Txs[i].toTron = abi.decode(vm.parseJson(json, string.concat(base, ".toTron")), (bytes));
            trc20Txs[i].amount = abi.decode(vm.parseJson(json, string.concat(base, ".amount")), (string));
            trc20Txs[i].isTransferFrom = abi.decode(vm.parseJson(json, string.concat(base, ".isTransferFrom")), (bool));
            trc20Txs[i].success = abi.decode(vm.parseJson(json, string.concat(base, ".success")), (bool));
            trc20Txs[i].selector = abi.decode(vm.parseJson(json, string.concat(base, ".selector")), (bytes));
        }
        assertTrue(trc20Txs.length > 0, "No TRC20 transactions in fixture");

        // Set the block timestamp in our mock light client.
        _lightClient.setBlockTimestamp(blockNumber, blockTimestamp);

        // Loop through each TRC-20 transaction from the fixture.
        for (uint256 i = 0; i < trc20Txs.length; ++i) {
            _assertTrc20Tx(blockNumber, blockTimestamp, trc20Txs[i]);
        }
    }

    function _assertTrc20Tx(uint256 blockNumber, uint32 blockTimestamp, Trc20TxJson memory txJson) internal {
        // For proof verification, use the txLeaf as the root and an empty proof.
        // (This simulates a block with a single transaction for simplicity.)
        _lightClient.setTxTrieRoot(blockNumber, txJson.txLeaf);
        bytes32[] memory proof = new bytes32[](0);
        uint256 index = 0;

        // Call the TronTxReader function to verify inclusion and extract generic call data.
        TronTxReader.TriggerSmartContract memory callData =
            _reader.readTriggerSmartContract(blockNumber, txJson.encodedTx, proof, index);

        // **Validate metadata against expected fixture data.**
        assertEq(callData.tronBlockNumber, blockNumber, "Block number mismatch");
        assertEq(callData.tronBlockTimestamp, blockTimestamp, "Block timestamp mismatch");
        assertEq(callData.txId, txJson.txId, "TxId mismatch");

        // Token contract address (Tron -> EVM).
        address tokenFromCall = _tronToEvmAddress(callData.toTron);
        assertEq(tokenFromCall, txJson.tronTokenEvm, "Token contract address mismatch");

        // Parse TRC-20 calldata and validate logical fields.
        (bytes21 fromTron, bytes21 toTron, uint256 amount) =
            _decodeTrc20FromCalldata(callData.data, callData.senderTron);

        assertEq(fromTron, bytes21(txJson.fromTron), "From address mismatch");
        assertEq(toTron, bytes21(txJson.toTron), "To address mismatch");
        uint256 expectedAmount = vm.parseUint(txJson.amount);
        assertEq(amount, expectedAmount, "Transfer amount mismatch");
        bool isTransferFrom = _first4(callData.data) == _SELECTOR_TRANSFER_FROM;
        assertEq(isTransferFrom, txJson.isTransferFrom, "Transfer type mismatch");

        // Selector sanity check.
        assertEq(_first4(callData.data), bytes4(txJson.selector), "Selector mismatch");

        // No nullifier logic in stateless reader; calling again should succeed and match.
        TronTxReader.TriggerSmartContract memory callData2 =
            _reader.readTriggerSmartContract(blockNumber, txJson.encodedTx, proof, index);
        assertEq(callData2.txId, callData.txId, "Repeated read txId mismatch");
    }

    function testRevertWhenTxNotSuccessful() public {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/TronTxReader/fixtures/trc20_block_78115149.json");

        uint256 blockNumber = abi.decode(vm.parseJson(json, ".blockNumber"), (uint256));
        uint32 blockTimestamp = uint32(abi.decode(vm.parseJson(json, ".blockTimestamp"), (uint256)));
        _lightClient.setBlockTimestamp(blockNumber, blockTimestamp);

        Trc20TxJson memory txJson;
        string memory base = ".trc20Txs[0]";
        txJson.txLeaf = abi.decode(vm.parseJson(json, string.concat(base, ".txLeaf")), (bytes32));
        txJson.encodedTx = abi.decode(vm.parseJson(json, string.concat(base, ".encodedTx")), (bytes));

        // Mutate contractRet in the Transaction.Result payload from SUCCESS (1) -> REVERT (2).
        bytes memory badTx = txJson.encodedTx;
        uint256 contractRetPos = _findFirstRetContractRetValuePos(badTx);
        badTx[contractRetPos] = bytes1(uint8(2));

        // Recompute the leaf to satisfy inclusion with the mutated bytes.
        _lightClient.setTxTrieRoot(blockNumber, sha256(badTx));
        bytes32[] memory proof = new bytes32[](0);
        uint256 index = 0;

        vm.expectRevert(TronTxReader.TronTxNotSuccessful.selector);
        _reader.readTriggerSmartContract(blockNumber, badTx, proof, index);
    }

    function _uToString(uint256 value) internal pure returns (string memory) {
        if (value == 0) return "0";
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            // casting to 'uint8' is safe because value % 10 is in [0,9], so 48 + (...) is in [48,57]
            // forge-lint: disable-next-line(unsafe-typecast)
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }

    function _readVarintMem(bytes memory data, uint256 pos) internal pure returns (uint64 value, uint256 newPos) {
        uint64 v;
        uint64 shift;
        for (uint256 i = 0; i < 10; ++i) {
            if (pos >= data.length) revert("ProtoTruncated");
            uint8 b = uint8(data[pos++]);
            v |= uint64(b & 0x7F) << shift;
            if ((b & 0x80) == 0) {
                return (v, pos);
            }
            shift += 7;
        }
        revert("ProtoTruncated");
    }

    function _skipFieldMem(bytes memory data, uint256 pos, uint256 limit, uint64 wireType)
        internal
        pure
        returns (uint256 newPos)
    {
        if (wireType == 0) {
            (, pos) = _readVarintMem(data, pos);
            return pos;
        }
        if (wireType == 1) {
            if (pos + 8 > limit) revert("ProtoTruncated");
            return pos + 8;
        }
        if (wireType == 2) {
            (uint64 len, uint256 p) = _readVarintMem(data, pos);
            pos = p;
            if (pos + uint256(len) > limit) revert("ProtoTruncated");
            return pos + uint256(len);
        }
        if (wireType == 5) {
            if (pos + 4 > limit) revert("ProtoTruncated");
            return pos + 4;
        }
        revert("ProtoInvalidWireType");
    }

    function _findFirstRetContractRetValuePos(bytes memory encodedTx) internal pure returns (uint256) {
        // Transaction.raw_data is field 1 (key 0x0A, length-delimited).
        require(encodedTx.length > 0 && uint8(encodedTx[0]) == 0x0A, "No raw_data");
        uint256 pos = 1;
        (uint64 rawLen, uint256 p) = _readVarintMem(encodedTx, pos);
        pos = p + uint256(rawLen);
        require(pos <= encodedTx.length, "ProtoTruncated");

        // Skip signatures (field 2, key 0x12).
        while (pos < encodedTx.length && uint8(encodedTx[pos]) == 0x12) {
            ++pos;
            (uint64 sigLen, uint256 p2) = _readVarintMem(encodedTx, pos);
            pos = p2 + uint256(sigLen);
            require(pos <= encodedTx.length, "ProtoTruncated");
        }

        // ret (field 5, key 0x2A)
        require(pos < encodedTx.length && uint8(encodedTx[pos]) == 0x2A, "No ret");
        ++pos;
        (uint64 retLen, uint256 p3) = _readVarintMem(encodedTx, pos);
        pos = p3;
        uint256 end = pos + uint256(retLen);
        require(end <= encodedTx.length, "ProtoTruncated");

        // Parse Transaction.Result until contractRet (field 3, varint) value.
        while (pos < end) {
            (uint64 key, uint256 p4) = _readVarintMem(encodedTx, pos);
            pos = p4;
            uint64 fieldNum = key >> 3;
            uint64 wireType = key & 0x7;
            if (fieldNum == 3 && wireType == 0) {
                // Next byte(s) are the varint value; current fixture uses single-byte 0x01.
                require(pos < end, "ProtoTruncated");
                return pos;
            }
            pos = _skipFieldMem(encodedTx, pos, end, wireType);
        }
        revert("contractRet not found");
    }

    function _decodeTrc20FromCalldata(bytes memory data, bytes21 senderTron)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        if (data.length < 4) revert("Trc20CalldataTooShort");
        bytes4 sig = _first4(data);
        if (sig == _SELECTOR_TRANSFER) {
            (toTron, amount) = _decodeTrc20TransferArgs(data);
            fromTron = senderTron;
        } else if (sig == _SELECTOR_TRANSFER_FROM) {
            (fromTron, toTron, amount) = _decodeTrc20TransferFromArgs(data);
        } else {
            revert("NotATrc20Transfer");
        }
    }

    function _decodeTrc20TransferArgs(bytes memory data) internal pure returns (bytes21 toTron, uint256 amount) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 2) revert("InvalidTrc20DataLength");
        bytes32 word1;
        bytes32 word2;
        assembly ("memory-safe") {
            word1 := mload(add(data, 0x24)) // 0x20 (data) + 4 (selector)
            word2 := mload(add(data, 0x44)) // 0x20 (data) + 36
        }
        address toAddr = address(uint160(uint256(word1)));
        toTron = _evmToTronAddress(toAddr);
        amount = uint256(word2);
    }

    function _decodeTrc20TransferFromArgs(bytes memory data)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 3) revert("InvalidTrc20DataLength");
        bytes32 w1;
        bytes32 w2;
        bytes32 w3;
        assembly ("memory-safe") {
            w1 := mload(add(data, 0x24)) // from
            w2 := mload(add(data, 0x44)) // to
            w3 := mload(add(data, 0x64)) // amount
        }
        address fromAddr = address(uint160(uint256(w1)));
        address toAddr2 = address(uint160(uint256(w2)));
        fromTron = _evmToTronAddress(fromAddr);
        toTron = _evmToTronAddress(toAddr2);
        amount = uint256(w3);
    }

    function _first4(bytes memory data) internal pure returns (bytes4 sel) {
        uint32 w;
        assembly ("memory-safe") {
            w := shr(224, mload(add(data, 0x20)))
        }
        sel = bytes4(w);
    }

    function _tronToEvmAddress(bytes21 tron) internal pure returns (address) {
        return address(uint160(uint168(tron)));
    }

    function _evmToTronAddress(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }
}
