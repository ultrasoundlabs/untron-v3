// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {StatefulTronTxReader} from "../../../src/evm/StatefulTronTxReader.sol";
import {TronSha256MerkleVerifier} from "../../../src/utils/TronSha256MerkleVerifier.sol";

contract StatefulTronTxReaderGasHarness is StatefulTronTxReader {
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

    function parseTriggerSmartContract(bytes calldata encodedTx)
        external
        pure
        returns (TriggerSmartContract memory callData)
    {
        return _parseTriggerSmartContract(encodedTx);
    }

    function verifyInclusion(bytes32 root, bytes calldata encodedTx, bytes32[] calldata proof, uint256 index)
        external
        pure
        returns (bool ok)
    {
        ok = TronSha256MerkleVerifier.verify(root, sha256(encodedTx), proof, index);
    }
}

contract StatefulTronTxReaderGasBreakdownTest is Test {
    uint256 internal constant _PK = 0xBEEF;

    string internal _tronJson;
    string internal _txJson;

    bytes20[27] internal _srs;
    bytes20[27] internal _witnessDelegateesFixture;

    function setUp() public {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        _tronJson = vm.readFile("test/evm/fixtures/tron_78000000_78000099.json");
        // forge-lint: disable-next-line(unsafe-cheatcode)
        _txJson = vm.readFile("test/evm/fixtures/trc20_block_78115149.json");

        address[] memory srsAddrs = vm.parseJsonAddressArray(_tronJson, ".srs");
        require(srsAddrs.length == 27, "fixture srs must be length 27");

        address[] memory delegateeAddrs = vm.parseJsonAddressArray(_tronJson, ".witnessDelegatees");
        require(delegateeAddrs.length == 27, "fixture witnessDelegatees must be length 27");

        for (uint256 i = 0; i < 27; ++i) {
            _srs[i] = bytes20(srsAddrs[i]);
            _witnessDelegateesFixture[i] = bytes20(delegateeAddrs[i]);
        }
    }

    function _loadTrc20Tx0() internal view returns (bytes32 txId, bytes32 txLeaf, bytes memory encodedTx) {
        // IMPORTANT: Field order must match the JSON object's key order for abi.decode to succeed.
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = _txJson;
        string memory base = ".trc20Txs[0]";
        txId = abi.decode(vm.parseJson(json, string.concat(base, ".txId")), (bytes32));
        txLeaf = abi.decode(vm.parseJson(json, string.concat(base, ".txLeaf")), (bytes32));
        encodedTx = abi.decode(vm.parseJson(json, string.concat(base, ".encodedTx")), (bytes));
    }

    function _loadFixtureBlocks(uint256 startIndex) internal view returns (bytes[20] memory blocks) {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = _tronJson;
        for (uint256 i = 0; i < 20; ++i) {
            uint256 idx = startIndex + i;
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory raw = vm.parseJsonBytes(json, string.concat(".blockHeaderRawBytes[", _uToString(idx), "]"));
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory sig = vm.parseJsonBytes(json, string.concat(".witnessSignatures[", _uToString(idx), "]"));
            blocks[i] = abi.encodePacked(hex"0a69", raw, hex"1241", sig);
        }
    }

    function _buildBlocksWithTxTrieRoot(bytes32 txTrieRoot, uint256 pk)
        internal
        view
        returns (bytes[20] memory blocks)
    {
        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = _tronJson;
        bytes32 prevBlockId;

        for (uint256 i = 0; i < 20; ++i) {
            // forge-lint: disable-next-line(unsafe-cheatcode)
            bytes memory raw = vm.parseJsonBytes(json, string.concat(".blockHeaderRawBytes[", _uToString(i), "]"));

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

    function _gasUsedVerifyFinality(StatefulTronTxReaderGasHarness reader, bytes[20] memory blocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        reader.verifyFirstBlockFinality(blocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedParse(StatefulTronTxReaderGasHarness reader, bytes memory encodedTx)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        reader.parseTriggerSmartContract(encodedTx);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedVerifyInclusion(
        StatefulTronTxReaderGasHarness reader,
        bytes32 root,
        bytes memory encodedTx,
        bytes32[] memory proof,
        uint256 index
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        bool ok = reader.verifyInclusion(root, encodedTx, proof, index);
        require(ok, "inclusion failed");
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedFullRead(
        StatefulTronTxReaderGasHarness reader,
        bytes[20] memory blocks,
        bytes memory encodedTx,
        bytes32[] memory proof,
        uint256 index
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        reader.readTriggerSmartContract(blocks, encodedTx, proof, index);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _makeReaderAllSameDelegatee() internal returns (StatefulTronTxReaderGasHarness reader) {
        bytes20[27] memory witnessDelegateesAllSame;
        bytes20 signer = bytes20(vm.addr(_PK));
        for (uint256 i = 0; i < 27; ++i) {
            witnessDelegateesAllSame[i] = signer;
        }
        reader = new StatefulTronTxReaderGasHarness(_srs, witnessDelegateesAllSame);
    }

    function test_gasBreakdown_verifyFirstBlockFinality_fixtureBlocks() public {
        StatefulTronTxReaderGasHarness readerFixture =
            new StatefulTronTxReaderGasHarness(_srs, _witnessDelegateesFixture);
        bytes[20] memory fixtureBlocks = _loadFixtureBlocks(0);

        vm.pauseGasMetering();
        uint256 gasFinality = _gasUsedVerifyFinality(readerFixture, fixtureBlocks);
        emit log_named_uint("gas/verifyFirstBlockFinality(20).total", gasFinality);
        emit log_named_uint("gas/verifyFirstBlockFinality(1).approx", gasFinality / 20);
    }

    function test_gasBreakdown_parseTriggerSmartContract_fixtureTx() public {
        (,, bytes memory encodedTx) = _loadTrc20Tx0();
        StatefulTronTxReaderGasHarness reader = _makeReaderAllSameDelegatee();

        vm.pauseGasMetering();
        uint256 gasParse = _gasUsedParse(reader, encodedTx);
        emit log_named_uint("gas/parseTriggerSmartContract.total", gasParse);
    }

    function test_gasBreakdown_verifyInclusion_proofLen0_vs_1() public {
        (, bytes32 txLeaf, bytes memory encodedTx) = _loadTrc20Tx0();
        StatefulTronTxReaderGasHarness reader = _makeReaderAllSameDelegatee();

        bytes32[] memory proof0 = new bytes32[](0);

        bytes32 sibling = bytes32(uint256(1));
        bytes32 root1 = sha256(abi.encodePacked(sha256(encodedTx), sibling));
        bytes32[] memory proof1 = new bytes32[](1);
        proof1[0] = sibling;

        vm.pauseGasMetering();

        uint256 gas0 = _gasUsedVerifyInclusion(reader, txLeaf, encodedTx, proof0, 0);
        uint256 gas1 = _gasUsedVerifyInclusion(reader, root1, encodedTx, proof1, 0);

        emit log_named_uint("gas/verifyInclusion.proofLen0.total", gas0);
        emit log_named_uint("gas/verifyInclusion.proofLen1.total", gas1);
        emit log_named_uint("gas/verifyInclusion.proofLen1.incrementalApprox", gas1 > gas0 ? gas1 - gas0 : 0);
    }

    function test_gasBreakdown_fullRead_proofLen0_vs_1() public {
        (, bytes32 txLeaf, bytes memory encodedTx) = _loadTrc20Tx0();
        StatefulTronTxReaderGasHarness reader = _makeReaderAllSameDelegatee();

        bytes32[] memory proof0 = new bytes32[](0);
        bytes[20] memory blocksLeafRoot = _buildBlocksWithTxTrieRoot(txLeaf, _PK);

        bytes32 sibling = bytes32(uint256(1));
        bytes32 root1 = sha256(abi.encodePacked(sha256(encodedTx), sibling));
        bytes32[] memory proof1 = new bytes32[](1);
        proof1[0] = sibling;
        bytes[20] memory blocksRoot1 = _buildBlocksWithTxTrieRoot(root1, _PK);

        vm.pauseGasMetering();

        uint256 gasFull0 = _gasUsedFullRead(reader, blocksLeafRoot, encodedTx, proof0, 0);
        uint256 gasFull1 = _gasUsedFullRead(reader, blocksRoot1, encodedTx, proof1, 0);

        emit log_named_uint("gas/readTriggerSmartContract.proofLen0.total", gasFull0);
        emit log_named_uint("gas/readTriggerSmartContract.proofLen1.total", gasFull1);
        emit log_named_uint(
            "gas/readTriggerSmartContract.proofLen1.incrementalApprox", gasFull1 > gasFull0 ? gasFull1 - gasFull0 : 0
        );
    }

    function _parseBlockNumber(bytes memory raw) internal pure returns (uint256 number) {
        uint256 pos = 76;
        uint256 shift = 0;
        for (uint256 i = 0; i < 4; ++i) {
            uint8 b = uint8(raw[pos + i]);
            number |= uint256(b & 0x7F) << shift;
            shift += 7;
        }
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
