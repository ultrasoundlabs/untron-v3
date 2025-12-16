// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {TronLightClientHarness} from "./harness/TronLightClientHarness.sol";
import {TronLightClient} from "../../../src/evm/TronLightClient.sol";
import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientLayer1Test is Test {
    TronLightClient internal _client;

    // Fixture data
    bytes32 internal _startingBlockId;
    bytes32 internal _startingBlockTxTrieRoot;
    uint32 internal _startingBlockTimestamp;
    bytes32 internal _endingBlockId;
    bytes32 internal _endingBlockTxTrieRoot;
    bytes internal _metadata;
    bytes internal _sigs;
    // SR owner accounts (Tron witnesses).
    bytes20[27] internal _srs;
    // Delegatee signing keys for each SR index.
    bytes20[27] internal _witnessDelegatees;
    uint256[] internal _blockNumbers;
    bytes32[] internal _blockIds;

    function _sliceBytes(bytes memory data, uint256 start, uint256 length) internal pure returns (bytes memory out) {
        require(start + length <= data.length, "slice out of bounds");
        out = new bytes(length);
        for (uint256 i = 0; i < length; i++) {
            out[i] = data[start + i];
        }
    }

    function _toBlockId(uint256 blockNumber, bytes32 blockHash) internal pure returns (bytes32) {
        uint256 tail = uint256(blockHash) & ((uint256(1) << 192) - 1);
        return bytes32((blockNumber << 192) | tail);
    }

    function _packMeta(bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(parentHash, txTrieRoot, timestamp, witnessIndex);
    }

    function _sign(uint256 pk, bytes32 digest) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, digest);
        return abi.encodePacked(r, s, v);
    }

    function setUp() public {
        // Read JSON fixture
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            // TODO: figure out how to iterate over all files in the fixtures folder
            "/test/evm/TronLightClient/fixtures/tron_78000000_78000099.json"
        );

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile(path);

        // Parse primitive fields
        _startingBlockId = vm.parseJsonBytes32(json, ".startingBlockId");
        _startingBlockTxTrieRoot = vm.parseJsonBytes32(json, ".startingBlockTxTrieRoot");
        uint256 startingTimestampSec = vm.parseJsonUint(json, ".startingBlockTimestamp");
        // casting to 'uint32' is safe because startingTimestampSec is masked to 32 bits
        // forge-lint: disable-next-line(unsafe-typecast)
        _startingBlockTimestamp = uint32(startingTimestampSec & 0xFFFFFFFF);
        _endingBlockId = vm.parseJsonBytes32(json, ".endingBlockId");
        _endingBlockTxTrieRoot = vm.parseJsonBytes32(json, ".endingBlockTxTrieRoot");

        _metadata = vm.parseJsonBytes(json, ".compressedTronBlockMetadata");
        _sigs = vm.parseJsonBytes(json, ".compressedSignatures");

        // Parse blockNumbers and blockIds for additional checks
        {
            uint256[] memory numsDyn = vm.parseJsonUintArray(json, ".blockNumbers");
            bytes32[] memory idsDyn = vm.parseJsonBytes32Array(json, ".blockIds");
            _blockNumbers = numsDyn;
            _blockIds = idsDyn;
        }

        // Parse SRS as addresses (SR owner accounts), then cast to bytes20[27]
        address[] memory srsAddrs = vm.parseJsonAddressArray(json, ".srs");
        require(srsAddrs.length == 27, "fixture srs must be length 27");

        address[] memory delegateeAddrs = vm.parseJsonAddressArray(json, ".witnessDelegatees");
        require(delegateeAddrs.length == 27, "fixture witnessDelegatees must be length 27");

        for (uint256 i = 0; i < 27; i++) {
            _srs[i] = bytes20(srsAddrs[i]);
            _witnessDelegatees[i] = bytes20(delegateeAddrs[i]);
        }

        _client = new TronLightClient(
            IBlockRangeProver(address(0)),
            _startingBlockId,
            _startingBlockTxTrieRoot,
            _startingBlockTimestamp,
            _srs,
            _witnessDelegatees,
            bytes32(0) // TODO: fix
        );
    }

    function test_proveBlocks_happyPath_fixture() public {
        // Call proveBlocks with the real Tron data
        _client.proveBlocks(_startingBlockId, _metadata, _sigs);

        // latestProvenBlock should be the endingBlockId from the fixture
        assertEq(_client.latestProvenBlock(), _endingBlockId, "latestProvenBlock mismatch");

        // The last block in the proven range should be present in storage.
        uint256 numLast = _blockNumbers[_blockNumbers.length - 1];

        assertEq(_client.getBlockId(numLast), _blockIds[_blockIds.length - 1], "getBlockId(last) mismatch");

        // We intentionally DO NOT store every intermediate block in storage; only
        // anchor blocks (e.g. the initial and ending ones) are persisted. A
        // mid-range block lookup should therefore fail with BlockNotRelayed.
        uint256 idxMid = _blockNumbers.length / 2;
        uint256 numMid = _blockNumbers[idxMid];
        vm.expectRevert(TronLightClient.BlockNotRelayed.selector);
        _client.getBlockId(numMid);

        // The starting anchor used in the constructor should also be present;
        // for this fixture it is the parent of blockNumbers[0].
        uint256 parentNum = _blockNumbers[0] - 1;
        assertEq(_client.getBlockId(parentNum), _startingBlockId, "getBlockId(parent) mismatch");

        // And the txTrieRoots for both the starting anchor and ending block should match the fixture.
        assertEq(_client.getTxTrieRoot(parentNum), _startingBlockTxTrieRoot, "getTxTrieRoot(parent) mismatch");
        assertEq(_client.getTxTrieRoot(numLast), _endingBlockTxTrieRoot, "getTxTrieRoot(last) mismatch");
    }

    function test_proveBlocks_allowsUnanchoredStartIfIntersectsLater() public {
        // First prove the full fixture range so we store the ending block as an anchor.
        _client.proveBlocks(_startingBlockId, _metadata, _sigs);

        // Now prove only the second half of the range, starting from a block that is not stored.
        // This succeeds because the proof intersects the already-stored ending anchor.
        uint256 startIdx = _blockNumbers.length / 2;
        require(startIdx > 0, "fixture must contain >1 block");

        uint256 numSlice = _blockNumbers.length - startIdx;
        bytes32 unanchoredStartingBlock = _blockIds[startIdx - 1];
        bytes memory metaSlice = _sliceBytes(_metadata, startIdx * 69, numSlice * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, startIdx * 65, numSlice * 65);

        _client.proveBlocks(unanchoredStartingBlock, metaSlice, sigsSlice);

        assertEq(_client.latestProvenBlock(), _endingBlockId, "latestProvenBlock mismatch after backfill");
    }

    function test_proveBlocks_revertsOnInvalidSignature() public {
        // Corrupt a single byte in the signatures to trigger InvalidWitnessSigner
        bytes memory badSigs = _sigs;
        badSigs[0] = bytes1(uint8(badSigs[0]) ^ 0x01);

        vm.expectRevert(TronLightClient.InvalidWitnessSigner.selector);
        _client.proveBlocks(_startingBlockId, _metadata, badSigs);
    }

    // -------------------------------------------------------------------------
    // Gas benchmarking
    // -------------------------------------------------------------------------

    function test_gasBenchmark_proveBlocks_fixture_fullRange() public {
        // Exclude all setup/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        uint256 gasUsed = _gasUsedProveBlocks(_client, _startingBlockId, _metadata, _sigs);
        emit log_named_uint("gas/proveBlocks.fixture.fullRange", gasUsed);

        assertEq(_client.latestProvenBlock(), _endingBlockId, "latestProvenBlock mismatch");
    }

    function test_gasBenchmark_proveBlocks_fixture_first10Blocks() public {
        uint256 n = 10;
        require(_blockNumbers.length >= n, "fixture too small");

        // Exclude slicing/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        bytes memory metaSlice = _sliceBytes(_metadata, 0, n * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, 0, n * 65);

        uint256 gasUsed = _gasUsedProveBlocks(_client, _startingBlockId, metaSlice, sigsSlice);
        emit log_named_uint("gas/proveBlocks.fixture.first10", gasUsed);

        assertEq(_client.latestProvenBlock(), _blockIds[n - 1], "latestProvenBlock mismatch");
    }

    function test_gasBenchmark_proveBlocks_backfill_secondHalf() public {
        // Exclude the initial seeding call + slicing/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        // Seed the ending anchor first (simulates a client that already has some history).
        _client.proveBlocks(_startingBlockId, _metadata, _sigs);

        // Now benchmark "backfilling" only the second half, starting from an unanchored block.
        uint256 startIdx = _blockNumbers.length / 2;
        require(startIdx > 0, "fixture must contain >1 block");

        uint256 numSlice = _blockNumbers.length - startIdx;
        bytes32 unanchoredStartingBlock = _blockIds[startIdx - 1];
        bytes memory metaSlice = _sliceBytes(_metadata, startIdx * 69, numSlice * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, startIdx * 65, numSlice * 65);

        uint256 gasUsed = _gasUsedProveBlocks(_client, unanchoredStartingBlock, metaSlice, sigsSlice);
        emit log_named_uint("gas/proveBlocks.backfill.secondHalf", gasUsed);

        assertEq(_client.latestProvenBlock(), _endingBlockId, "latestProvenBlock mismatch after backfill");
    }

    function test_gasBenchmark_proveBlocks_synthetic_twoBlocks() public {
        // Exclude deploy/proof-building/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        uint256 pk0 = 0xA11CE;
        uint256 pk1 = 0xB0B;

        (TronLightClientHarness cHarness, bytes32 startingBlockId) = _deployHarnessForTwoPks(pk0, pk1);
        (bytes memory meta, bytes memory sigs, bytes32 expectedEndingBlockId) =
            _buildTwoBlocksDifferentCreatorsProof(cHarness, startingBlockId, pk0, pk1);

        TronLightClient c = TronLightClient(address(cHarness));

        uint256 gasUsed = _gasUsedProveBlocks(c, startingBlockId, meta, sigs);
        emit log_named_uint("gas/proveBlocks.synthetic.twoBlocks", gasUsed);

        assertEq(c.latestProvenBlock(), expectedEndingBlockId, "latestProvenBlock mismatch");
    }

    function _gasUsedProveBlocks(TronLightClient c, bytes32 startingBlockId, bytes memory meta, bytes memory sigs)
        internal
        returns (uint256 gasUsed)
    {
        // Only the proveBlocks call (plus this tiny wrapper) should be included in the test's reported gas.
        vm.resumeGasMetering();

        vm.record();
        uint256 g0 = gasleft();
        c.proveBlocks(startingBlockId, meta, sigs);
        gasUsed = g0 - gasleft();
        (bytes32[] memory reads, bytes32[] memory writes) = vm.accesses(address(c));
        emit log_named_uint("gasUsed", gasUsed);
        emit log_named_uint("storageReads", reads.length);
        emit log_named_uint("storageWrites", writes.length);

        vm.pauseGasMetering();
    }

    function _deployHarnessForTwoPks(uint256 pk0, uint256 pk1)
        internal
        returns (TronLightClientHarness c, bytes32 startingBlockId)
    {
        bytes20[27] memory srs;
        bytes20[27] memory witnessDelegatees;

        srs[0] = bytes20(address(0x1111111111111111111111111111111111111111));
        srs[1] = bytes20(address(0x2222222222222222222222222222222222222222));

        witnessDelegatees[0] = bytes20(vm.addr(pk0));
        witnessDelegatees[1] = bytes20(vm.addr(pk1));

        uint256 parentNumber = 100;
        startingBlockId = bytes32((parentNumber << 192) | 1);

        c = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            startingBlockId,
            bytes32(0),
            uint32(0),
            srs,
            witnessDelegatees,
            bytes32(0) // TODO: fix
        );
    }

    function _buildTwoBlocksDifferentCreatorsProof(
        TronLightClientHarness c,
        bytes32 startingBlockId,
        uint256 pk0,
        uint256 pk1
    ) internal view returns (bytes memory meta, bytes memory sigs, bytes32 endingBlockId) {
        uint256 parentNumber = 100;

        TronLightClient.TronBlockMetadata memory b1 = TronLightClient.TronBlockMetadata({
            parentHash: startingBlockId, txTrieRoot: bytes32(uint256(1)), timestamp: uint32(1), witnessAddressIndex: 0
        });

        uint256 n1 = parentNumber + 1;
        bytes32 h1 = c.hashBlockPublic(b1, n1);
        bytes32 id1 = _toBlockId(n1, h1);

        TronLightClient.TronBlockMetadata memory b2 = TronLightClient.TronBlockMetadata({
            parentHash: id1, txTrieRoot: bytes32(uint256(2)), timestamp: uint32(2), witnessAddressIndex: 1
        });

        uint256 n2 = parentNumber + 2;
        bytes32 h2 = c.hashBlockPublic(b2, n2);
        endingBlockId = _toBlockId(n2, h2);

        (meta, sigs) = _twoBlockMetaAndSigsDifferent(pk0, pk1, h1, h2, b1, b2);
    }

    function _twoBlockMetaAndSigsDifferent(
        uint256 pk1,
        uint256 pk2,
        bytes32 h1,
        bytes32 h2,
        TronLightClient.TronBlockMetadata memory b1,
        TronLightClient.TronBlockMetadata memory b2
    ) internal pure returns (bytes memory meta, bytes memory sigs) {
        meta = new bytes(138);
        sigs = new bytes(130);

        bytes memory m1 = _packMeta(b1.parentHash, b1.txTrieRoot, b1.timestamp, b1.witnessAddressIndex);
        bytes memory m2 = _packMeta(b2.parentHash, b2.txTrieRoot, b2.timestamp, b2.witnessAddressIndex);
        bytes memory s1 = _sign(pk1, h1);
        bytes memory s2 = _sign(pk2, h2);

        for (uint256 i = 0; i < 69; ++i) {
            meta[i] = m1[i];
            meta[69 + i] = m2[i];
        }
        for (uint256 i = 0; i < 65; ++i) {
            sigs[i] = s1[i];
            sigs[65 + i] = s2[i];
        }
    }

    function test_proveBlocks_revertsIfCreatorProducedWithinPrevious18Blocks() public {
        _case_creatorProducedRecently();
    }

    function _case_creatorProducedRecently() internal {
        uint256 pk0 = 0xA11CE;

        (TronLightClientHarness c, bytes32 startingBlockId) = _deployHarnessForPk(pk0);
        (bytes memory meta, bytes memory sigs) = _buildTwoBlocksSameCreatorProof(c, startingBlockId, pk0);

        _expectWitnessProducedRecently(c, startingBlockId, meta, sigs, pk0);
    }

    function _deployHarnessForPk(uint256 pk0) internal returns (TronLightClientHarness c, bytes32 startingBlockId) {
        bytes20[27] memory srs;
        bytes20[27] memory witnessDelegatees;

        srs[0] = bytes20(address(0x1111111111111111111111111111111111111111));
        witnessDelegatees[0] = bytes20(vm.addr(pk0));

        uint256 parentNumber = 100;
        startingBlockId = bytes32((parentNumber << 192) | 1);

        c = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            startingBlockId,
            bytes32(0),
            uint32(0),
            srs,
            witnessDelegatees,
            bytes32(0) // TODO: fix
        );
    }

    function _buildTwoBlocksSameCreatorProof(TronLightClientHarness c, bytes32 startingBlockId, uint256 pk0)
        internal
        view
        returns (bytes memory meta, bytes memory sigs)
    {
        uint256 parentNumber = 100;

        TronLightClient.TronBlockMetadata memory b1 = TronLightClient.TronBlockMetadata({
            parentHash: startingBlockId, txTrieRoot: bytes32(uint256(1)), timestamp: uint32(1), witnessAddressIndex: 0
        });

        uint256 n1 = parentNumber + 1;
        bytes32 h1 = c.hashBlockPublic(b1, n1);
        bytes32 id1 = _toBlockId(n1, h1);

        TronLightClient.TronBlockMetadata memory b2 = TronLightClient.TronBlockMetadata({
            parentHash: id1, txTrieRoot: bytes32(uint256(2)), timestamp: uint32(2), witnessAddressIndex: 0
        });

        uint256 n2 = parentNumber + 2;
        bytes32 h2 = c.hashBlockPublic(b2, n2);

        (meta, sigs) = _twoBlockMetaAndSigs(pk0, h1, h2, b1, b2);
    }

    function _twoBlockMetaAndSigs(
        uint256 pk,
        bytes32 h1,
        bytes32 h2,
        TronLightClient.TronBlockMetadata memory b1,
        TronLightClient.TronBlockMetadata memory b2
    ) internal pure returns (bytes memory meta, bytes memory sigs) {
        meta = new bytes(138);
        sigs = new bytes(130);

        bytes memory m1 = _packMeta(b1.parentHash, b1.txTrieRoot, b1.timestamp, b1.witnessAddressIndex);
        bytes memory m2 = _packMeta(b2.parentHash, b2.txTrieRoot, b2.timestamp, b2.witnessAddressIndex);
        bytes memory s1 = _sign(pk, h1);
        bytes memory s2 = _sign(pk, h2);

        for (uint256 i = 0; i < 69; ++i) {
            meta[i] = m1[i];
            meta[69 + i] = m2[i];
        }
        for (uint256 i = 0; i < 65; ++i) {
            sigs[i] = s1[i];
            sigs[65 + i] = s2[i];
        }
    }

    function _expectWitnessProducedRecently(
        TronLightClientHarness c,
        bytes32 startingBlockId,
        bytes memory meta,
        bytes memory sigs,
        uint256 pk0
    ) internal {
        // keep this function *tiny* so we never hit stack-too-deep here
        bytes20 signer = bytes20(vm.addr(pk0));
        bytes memory err = abi.encodeWithSelector(TronLightClient.WitnessProducedRecently.selector, signer);

        vm.expectRevert(err);
        c.proveBlocks(startingBlockId, meta, sigs);
    }
}
