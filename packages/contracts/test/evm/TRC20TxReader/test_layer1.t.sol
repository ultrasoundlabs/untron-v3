// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {TronTxReader} from "../../../src/evm/TronTxReader.sol";
import {MockTronLightClient} from "./mocks/MockTronLightClient.sol";

/// @dev This test deploys TronTxReader and uses its public helpers.
contract TronTxReaderTest is Test {
    MockTronLightClient internal lightClient;
    TronTxReader internal reader;

    // TRC-20 selectors used in fixture.
    bytes4 internal constant SELECTOR_TRANSFER = bytes4(0xa9059cbb);
    bytes4 internal constant SELECTOR_TRANSFER_FROM = bytes4(0x23b872dd);

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
        bytes32 txLeaf;
        string amount;
        string tronBlockNumber;
        string tronBlockTimestamp;
        bytes txId;
        uint256 index;
    }

    /// @notice Deploy a mock light client and the TronTxReader bound to it.
    function setUp() public {
        lightClient = new MockTronLightClient();
        reader = new TronTxReader(address(lightClient));
    }

    /// @notice Test decoding of real TRC-20 transactions using a fixture from Tron mainnet.
    function testReadTrc20TransfersFromFixture() public {
        // Load the JSON fixture containing Tron block and TRC-20 transactions.
        // Safe in test context: fixture file is readonly and controlled
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/TRC20TxReader/fixtures/trc20_block_78115149.json");

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
        lightClient.setBlockTimestamp(blockNumber, blockTimestamp);

        // Loop through each TRC-20 transaction from the fixture.
        for (uint256 i = 0; i < trc20Txs.length; ++i) {
            Trc20TxJson memory txJson = trc20Txs[i];

            // For proof verification, use the txLeaf as the root and an empty proof.
            // (This simulates a block with a single transaction for simplicity.)
            lightClient.setTxTrieRoot(blockNumber, txJson.txLeaf);
            bytes32[] memory proof = new bytes32[](0);
            uint256 index = 0;

            // Call the TronTxReader function to verify inclusion and extract generic call data.
            TronTxReader.TriggerSmartContract memory callData =
                reader.readTriggerSmartContract(blockNumber, txJson.encodedTx, proof, index);

            // **Validate metadata against expected fixture data.**
            assertEq(callData.tronBlockNumber, blockNumber, "Block number mismatch");
            assertEq(callData.tronBlockTimestamp, blockTimestamp, "Block timestamp mismatch");
            assertEq(callData.txLeaf, txJson.txLeaf, "Tx leaf mismatch");

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
            bool isTransferFrom = _first4(callData.data) == SELECTOR_TRANSFER_FROM;
            assertEq(isTransferFrom, txJson.isTransferFrom, "Transfer type mismatch");

            // Selector sanity check.
            assertEq(_first4(callData.data), bytes4(txJson.selector), "Selector mismatch");

            // No nullifier logic in stateless reader; calling again should succeed and match.
            TronTxReader.TriggerSmartContract memory callData2 =
                reader.readTriggerSmartContract(blockNumber, txJson.encodedTx, proof, index);
            assertEq(callData2.txLeaf, callData.txLeaf, "Repeated read txLeaf mismatch");
        }
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

    function _decodeTrc20FromCalldata(bytes memory data, bytes21 senderTron)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        if (data.length < 4) revert("Trc20CalldataTooShort");
        bytes4 sig = _first4(data);
        if (sig == SELECTOR_TRANSFER) {
            (toTron, amount) = _decodeTrc20TransferArgs(data);
            fromTron = senderTron;
        } else if (sig == SELECTOR_TRANSFER_FROM) {
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
