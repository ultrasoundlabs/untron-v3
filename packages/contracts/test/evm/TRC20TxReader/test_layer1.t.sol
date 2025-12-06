// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {TRC20TxReader} from "../../../src/evm/TRC20TxReader.sol";
import {MockTronLightClient} from "./mocks/MockTronLightClient.sol";

/// @dev This test deploys TRC20TxReader and uses its public helpers.
contract TRC20TxReaderTest is Test {
    MockTronLightClient internal lightClient;
    TRC20TxReader internal reader;

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

    /// @notice Deploy a mock light client and the TRC20TxReader bound to it.
    function setUp() public {
        lightClient = new MockTronLightClient();
        reader = new TRC20TxReader(address(lightClient));
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

            // Call the TRC20TxReader function to verify and decode the transaction.
            TRC20TxReader.Trc20Transfer memory transfer =
                reader.readTrc20Transfer(blockNumber, txJson.encodedTx, proof, index);

            // **Validate the decoded transfer against expected fixture data.**
            // Check token contract address.
            assertEq(transfer.tronTokenEvm, txJson.tronTokenEvm, "Token contract address mismatch");
            // Check sender and recipient Tron addresses (21-byte hex strings).
            assertEq(transfer.fromTron, bytes21(txJson.fromTron), "From address mismatch");
            assertEq(transfer.toTron, bytes21(txJson.toTron), "To address mismatch");
            // Check transfer amount.
            uint256 expectedAmount = vm.parseUint(txJson.amount);
            assertEq(transfer.amount, expectedAmount, "Transfer amount mismatch");
            // Check transfer type (transfer vs transferFrom).
            assertEq(transfer.isTransferFrom, txJson.isTransferFrom, "Transfer type mismatch");
            // Check block context.
            assertEq(transfer.tronBlockNumber, blockNumber, "Block number mismatch");
            assertEq(transfer.tronBlockTimestamp, blockTimestamp, "Block timestamp mismatch");

            // No nullifier logic in stateless reader; calling again should succeed and match.
            TRC20TxReader.Trc20Transfer memory transfer2 =
                reader.readTrc20Transfer(blockNumber, txJson.encodedTx, proof, index);
            assertEq(transfer2.txLeaf, transfer.txLeaf, "Repeated read txLeaf mismatch");
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
}
