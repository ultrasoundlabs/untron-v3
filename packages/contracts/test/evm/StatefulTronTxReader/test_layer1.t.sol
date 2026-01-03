// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {StatefulTronTxReader} from "../../../src/evm/StatefulTronTxReader.sol";
import {ITronTxReader} from "../../../src/evm/interfaces/ITronTxReader.sol";

contract StatefulTronTxReaderHarness is StatefulTronTxReader {
    constructor(bytes20[27] memory _srs, bytes20[27] memory _witnessDelegatees)
        StatefulTronTxReader(_srs, _witnessDelegatees)
    {}

    function verifyFirstBlockFinality(bytes[20] calldata blocks)
        external
        view
        returns (uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot)
    {
        return _verifyFirstBlockFinality(blocks);
    }

    function verifySingleBlock(bytes calldata block_)
        external
        view
        returns (bytes32 nextBlockId, uint32 nextSeen, uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot)
    {
        return _verifyBlock(block_, bytes32(0), uint32(0));
    }

    function parseTriggerSmartContract(bytes calldata encodedTx)
        external
        pure
        returns (TriggerSmartContract memory callData)
    {
        return _parseTriggerSmartContract(encodedTx);
    }
}

contract StatefulTronTxReaderLayer1Test is Test {
    // TRC-20 selectors used in fixture.
    bytes4 internal constant _SELECTOR_TRANSFER = bytes4(0xa9059cbb);
    bytes4 internal constant _SELECTOR_TRANSFER_FROM = bytes4(0x23b872dd);

    uint256 internal constant _PK = 0xBEEF;

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

    function test_verifyFirstBlockFinality_fromFixtureBlocks() public {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");

        (bytes20[27] memory srs, bytes20[27] memory witnessDelegatees) = _parseSrSet(json);

        StatefulTronTxReaderHarness reader = new StatefulTronTxReaderHarness(srs, witnessDelegatees);

        bytes[20] memory blocks = _loadFixtureBlocks(json, 0);

        (uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot) = reader.verifyFirstBlockFinality(blocks);

        uint256 expectedBlockNumber = vm.parseJsonUint(json, ".blockNumbers[0]");
        // Derive expected timestamp from the actual header bytes (fixture timestamps are not always aligned).
        // forge-lint: disable-next-line(unsafe-cheatcode)
        bytes memory raw0 = vm.parseJsonBytes(json, ".blockHeaderRawBytes[0]");
        uint32 expectedTimestamp = _parseTimestampSecFromRaw(raw0);
        bytes32 expectedTxTrieRoot = _readBytes32(raw0, 9);

        assertEq(blockNumber, expectedBlockNumber, "blockNumber mismatch");
        assertEq(blockTimestamp, expectedTimestamp, "blockTimestamp mismatch");
        assertEq(txTrieRoot, expectedTxTrieRoot, "txTrieRoot mismatch");
    }

    function test_fixtureBlockEncoding_hasExpectedLengths() public view {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");
        bytes[20] memory blocks = _loadFixtureBlocks(json, 0);
        for (uint256 i = 0; i < 20; ++i) {
            assertEq(blocks[i].length, 174, "encoded block length mismatch");
        }
    }

    function test_verifySingleBlock_succeedsOnFixtureBlock() public {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");

        (bytes20[27] memory srs, bytes20[27] memory witnessDelegatees) = _parseSrSet(json);
        StatefulTronTxReaderHarness reader = new StatefulTronTxReaderHarness(srs, witnessDelegatees);

        bytes[20] memory blocks = _loadFixtureBlocks(json, 0);
        (bytes32 nextBlockId,, uint256 blockNumber, uint32 blockTimestamp, bytes32 txTrieRoot) =
            reader.verifySingleBlock(blocks[0]);

        assertTrue(nextBlockId != bytes32(0), "expected nextBlockId");
        assertEq(blockNumber, vm.parseJsonUint(json, ".blockNumbers[0]"), "blockNumber mismatch");
        // forge-lint: disable-next-line(unsafe-cheatcode)
        bytes memory raw0 = vm.parseJsonBytes(json, ".blockHeaderRawBytes[0]");
        assertEq(blockTimestamp, _parseTimestampSecFromRaw(raw0), "blockTimestamp mismatch");
        assertEq(txTrieRoot, _readBytes32(raw0, 9), "txTrieRoot mismatch");
    }

    function test_fixtureWitnessSignature_recoversExpectedDelegatee() public view {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");

        // forge-lint: disable-next-line(unsafe-cheatcode)
        bytes memory raw = vm.parseJsonBytes(json, ".blockHeaderRawBytes[0]");
        // forge-lint: disable-next-line(unsafe-cheatcode)
        bytes memory sig = vm.parseJsonBytes(json, ".witnessSignatures[0]");
        address expected = vm.parseJsonAddress(json, ".witnessEvmAddresses[0]");

        bytes32 digest = sha256(raw);

        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }

        address recovered = ecrecover(digest, uint8(v + 27), r, s);
        assertEq(recovered, expected, "ecrecover mismatch");
    }

    function test_verifyFirstBlockFinality_revertsOnInvalidSignature() public {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");

        (bytes20[27] memory srs, bytes20[27] memory witnessDelegatees) = _parseSrSet(json);

        StatefulTronTxReaderHarness reader = new StatefulTronTxReaderHarness(srs, witnessDelegatees);

        bytes[20] memory blocks = _loadFixtureBlocks(json, 0);

        // Corrupt the signature of the first block (first byte of r at offset 109).
        blocks[0][109] = bytes1(uint8(blocks[0][109]) ^ uint8(0x01));

        vm.expectRevert(StatefulTronTxReader.InvalidWitnessSignature.selector);
        reader.verifyFirstBlockFinality(blocks);
    }

    function test_readTriggerSmartContract_endToEnd_withFixtureTx() public {
        // Fixtures:
        // - tx + txLeaf from real Tron mainnet TRC-20 block
        // - block header layout and witness rotation from real Tron mainnet block range

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory tronJson = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory txJsonStr = vm.readFile("test/evm/fixtures/trc20_block_78115149.json");

        bytes20[27] memory srs = _parseSrs(tronJson);
        bytes20[27] memory witnessDelegatees;
        bytes20 signer = bytes20(vm.addr(_PK));
        for (uint256 i = 0; i < 27; ++i) {
            witnessDelegatees[i] = signer;
        }

        StatefulTronTxReader reader = new StatefulTronTxReader(srs, witnessDelegatees);

        Trc20TxJson memory txJson = _loadTrc20Tx(txJsonStr, 0);
        assertTrue(txJson.success, "expected fixture tx success");

        // Build 20 sequential blocks whose first txTrieRoot matches the fixture tx leaf.
        bytes[20] memory blocks = _buildBlocksWithTxTrieRoot(tronJson, txJson.txLeaf, _PK);

        bytes32[] memory proof = new bytes32[](0);
        uint256 index = 0;

        ITronTxReader.TriggerSmartContract memory callData =
            reader.readTriggerSmartContract(blocks, txJson.encodedTx, proof, index);

        assertEq(callData.txId, txJson.txId, "txId mismatch");

        address tokenFromCall = _tronToEvmAddress(callData.toTron);
        assertEq(tokenFromCall, txJson.tronTokenEvm, "token contract mismatch");

        (bytes21 fromTron, bytes21 toTron, uint256 amount) =
            _decodeTrc20FromCalldata(callData.data, callData.senderTron);
        assertEq(fromTron, _toBytes21(txJson.fromTron), "fromTron mismatch");
        assertEq(toTron, _toBytes21(txJson.toTron), "toTron mismatch");
        assertEq(amount, vm.parseUint(txJson.amount), "amount mismatch");

        bool isTransferFrom = _first4(callData.data) == _SELECTOR_TRANSFER_FROM;
        assertEq(isTransferFrom, txJson.isTransferFrom, "transfer type mismatch");
        assertEq(_first4(callData.data), bytes4(txJson.selector), "selector mismatch");
    }

    // ---------------- Fixtures parsing ----------------

    function _parseSrs(string memory json) internal pure returns (bytes20[27] memory srs) {
        address[] memory srsAddrs = vm.parseJsonAddressArray(json, ".srs");
        require(srsAddrs.length == 27, "fixture srs must be length 27");
        for (uint256 i = 0; i < 27; ++i) {
            srs[i] = bytes20(srsAddrs[i]);
        }
    }

    function _parseSrSet(string memory json)
        internal
        pure
        returns (bytes20[27] memory srs, bytes20[27] memory witnessDelegatees)
    {
        srs = _parseSrs(json);
        address[] memory delAddrs = vm.parseJsonAddressArray(json, ".witnessDelegatees");
        require(delAddrs.length == 27, "fixture witnessDelegatees must be length 27");
        for (uint256 i = 0; i < 27; ++i) {
            witnessDelegatees[i] = bytes20(delAddrs[i]);
        }
    }

    function _loadFixtureBlocks(string memory json, uint256 startIndex)
        internal
        pure
        returns (bytes[20] memory blocks)
    {
        for (uint256 i = 0; i < 20; ++i) {
            uint256 idx = startIndex + i;
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory raw = vm.parseJsonBytes(json, string.concat(".blockHeaderRawBytes[", _uToString(idx), "]"));
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory sig = vm.parseJsonBytes(json, string.concat(".witnessSignatures[", _uToString(idx), "]"));

            blocks[i] = abi.encodePacked(hex"0a69", raw, hex"1241", sig);
        }
    }

    function _loadTrc20Tx(string memory json, uint256 i) internal pure returns (Trc20TxJson memory txJson) {
        string memory base = string.concat(".trc20Txs[", _uToString(i), "]");
        txJson.index = abi.decode(vm.parseJson(json, string.concat(base, ".index")), (uint256));
        txJson.txId = abi.decode(vm.parseJson(json, string.concat(base, ".txId")), (bytes32));
        txJson.txLeaf = abi.decode(vm.parseJson(json, string.concat(base, ".txLeaf")), (bytes32));
        txJson.encodedTx = abi.decode(vm.parseJson(json, string.concat(base, ".encodedTx")), (bytes));
        txJson.tronBlockNumber = abi.decode(vm.parseJson(json, string.concat(base, ".tronBlockNumber")), (string));
        txJson.tronBlockTimestamp = abi.decode(vm.parseJson(json, string.concat(base, ".tronBlockTimestamp")), (string));
        txJson.tronTokenEvm = abi.decode(vm.parseJson(json, string.concat(base, ".tronTokenEvm")), (address));
        txJson.fromTron = abi.decode(vm.parseJson(json, string.concat(base, ".fromTron")), (bytes));
        txJson.toTron = abi.decode(vm.parseJson(json, string.concat(base, ".toTron")), (bytes));
        txJson.amount = abi.decode(vm.parseJson(json, string.concat(base, ".amount")), (string));
        txJson.isTransferFrom = abi.decode(vm.parseJson(json, string.concat(base, ".isTransferFrom")), (bool));
        txJson.success = abi.decode(vm.parseJson(json, string.concat(base, ".success")), (bool));
        txJson.selector = abi.decode(vm.parseJson(json, string.concat(base, ".selector")), (bytes));
    }

    // ---------------- Block builder (fixed layout) ----------------

    function _buildBlocksWithTxTrieRoot(string memory tronJson, bytes32 txTrieRoot, uint256 pk)
        internal
        pure
        returns (bytes[20] memory blocks)
    {
        bytes32 prevBlockId;

        for (uint256 i = 0; i < 20; ++i) {
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory raw = vm.parseJsonBytes(tronJson, string.concat(".blockHeaderRawBytes[", _uToString(i), "]"));

            // Patch txTrieRoot for the first block only.
            if (i == 0) {
                _writeBytes32(raw, 9, txTrieRoot);
            } else {
                _writeBytes32(raw, 43, prevBlockId);
            }

            bytes32 blockHash = sha256(raw);
            prevBlockId = _makeBlockId(_parseBlockNumber(raw), blockHash);

            (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, blockHash);
            bytes memory sig = abi.encodePacked(r, s, v);

            blocks[i] = abi.encodePacked(hex"0a69", raw, hex"1241", sig);
        }
    }

    function _parseBlockNumber(bytes memory raw) internal pure returns (uint256 number) {
        // In this fixed layout, block number varint is at raw offset 76 and is always 4 bytes.
        // Decode manually (little-endian 7-bit groups).
        uint256 pos = 76;
        uint256 shift = 0;
        for (uint256 i = 0; i < 4; ++i) {
            uint8 b = uint8(raw[pos + i]);
            number |= uint256(b & 0x7F) << shift;
            shift += 7;
        }
    }

    function _parseTimestampSecFromRaw(bytes memory raw) internal pure returns (uint32 tsSec) {
        // raw begins with 0x08 (field 1, varint). Timestamp varint starts at raw offset 1.
        uint256 pos = 1;
        uint256 value;
        uint256 shift;
        for (uint256 i = 0; i < 10; ++i) {
            require(pos < raw.length, "timestamp truncated");
            uint8 b = uint8(raw[pos++]);
            value |= uint256(b & 0x7F) << shift;
            if ((b & 0x80) == 0) break;
            shift += 7;
        }
        // forge-lint: disable-next-line(unsafe-typecast)
        tsSec = uint32(value / 1000);
    }

    function _makeBlockId(uint256 blockNumber, bytes32 blockHash) internal pure returns (bytes32 blockId) {
        uint256 tail = uint256(blockHash) & ((uint256(1) << 192) - 1);
        return bytes32((blockNumber << 192) | tail);
    }

    function _writeBytes32(bytes memory data, uint256 start, bytes32 value) internal pure {
        require(start + 32 <= data.length, "write out of bounds");
        assembly {
            mstore(add(add(data, 32), start), value)
        }
    }

    function _readBytes32(bytes memory data, uint256 start) internal pure returns (bytes32 value) {
        require(start + 32 <= data.length, "read out of bounds");
        assembly {
            value := mload(add(add(data, 32), start))
        }
    }

    // ---------------- TRC-20 decoding helpers ----------------

    function _toBytes21(bytes memory b) internal pure returns (bytes21 out) {
        require(b.length == 21, "expected tron address bytes21");
        assembly {
            out := mload(add(b, 32))
        }
    }

    function _evmToTron(address a) internal pure returns (bytes21 tron) {
        tron = bytes21((uint168(0x41) << 160) | uint160(a));
    }

    function _tronToEvmAddress(bytes21 tron) internal pure returns (address a) {
        require(tron[0] == 0x41, "invalid tron prefix");
        a = address(uint160(uint168(tron)));
    }

    function _first4(bytes memory data) internal pure returns (bytes4 sel) {
        if (data.length < 4) revert("calldata too short");
        uint32 w;
        assembly ("memory-safe") {
            w := shr(224, mload(add(data, 0x20)))
        }
        sel = bytes4(w);
    }

    function _decodeTrc20FromCalldata(bytes memory data, bytes21 senderTron)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        bytes4 sel = _first4(data);

        if (sel == _SELECTOR_TRANSFER) {
            if (data.length != 4 + 32 * 2) revert("InvalidTrc20DataLength");
            bytes32 w1;
            bytes32 w2;
            assembly ("memory-safe") {
                w1 := mload(add(data, 0x24))
                w2 := mload(add(data, 0x44))
            }
            address toAddr = address(uint160(uint256(w1)));
            fromTron = senderTron;
            toTron = _evmToTron(toAddr);
            amount = uint256(w2);
            return (fromTron, toTron, amount);
        }

        if (sel == _SELECTOR_TRANSFER_FROM) {
            if (data.length != 4 + 32 * 3) revert("InvalidTrc20DataLength");
            bytes32 w1;
            bytes32 w2;
            bytes32 w3;
            assembly ("memory-safe") {
                w1 := mload(add(data, 0x24))
                w2 := mload(add(data, 0x44))
                w3 := mload(add(data, 0x64))
            }
            address fromAddr = address(uint160(uint256(w1)));
            address toAddr2 = address(uint160(uint256(w2)));
            fromTron = _evmToTron(fromAddr);
            toTron = _evmToTron(toAddr2);
            amount = uint256(w3);
            return (fromTron, toTron, amount);
        }

        revert("unexpected selector");
    }

    // ---------------- Misc ----------------

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
            // forge-lint: disable-next-line(unsafe-typecast)
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }
}
