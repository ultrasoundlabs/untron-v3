// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronLightClientHarness} from "./harness/TronLightClientHarness.sol";
import {TronLightClient} from "../../../src/evm/TronLightClient.sol";
import {TronLightClientIndex} from "../../../src/evm/TronLightClientIndex.sol";
import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";
import {EventChainGenesis} from "../../../src/utils/EventChainGenesis.sol";

contract TronLightClientLayer1Test is Test {
    TronLightClient internal _client;

    uint8 internal constant _FINALITY_DISTINCT_SR_THRESHOLD = 19;

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

    function _storeOnly(uint256 offset) internal pure returns (uint256 storeOffsets16) {
        // TronLightClient interprets 0xFFFF as sentinel ("no more offsets"), so real offsets must be < 0xFFFF.
        require(offset < type(uint16).max, "offset too large");

        // Sentinel is 0xFFFF in each lane. Start with all lanes set to sentinel, then overwrite lane0.
        storeOffsets16 = type(uint256).max;
        storeOffsets16 = (storeOffsets16 & ~uint256(type(uint16).max)) | offset;
    }

    function _storeNone() internal pure returns (uint256 storeOffsets16) {
        return type(uint256).max;
    }

    function _blockIdToNumber(bytes32 blockId) internal pure returns (uint256) {
        return uint256(blockId) >> 192;
    }

    function _sortedSrsFixtureless() internal pure returns (bytes20[27] memory srs) {
        for (uint256 i = 0; i < 27; ++i) {
            // forge-lint: disable-next-line(unsafe-typecast)
            srs[i] = bytes20(uint160(i + 1));
        }
    }

    function _popcount32(uint32 x) internal pure returns (uint8 c) {
        while (x != 0) {
            x &= (x - 1);
            unchecked {
                ++c;
            }
        }
    }

    function _witnessIndexAt(bytes memory meta, uint256 index) internal pure returns (uint8 wi) {
        wi = uint8(meta[index * 69 + 68]);
    }

    function _txTrieRootAt(bytes memory meta, uint256 index) internal pure returns (bytes32 txTrieRoot) {
        uint256 off = index * 69 + 32;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            txTrieRoot := mload(add(add(meta, 32), off))
        }
    }

    function _latestFinalizedStoreOffset(bytes memory meta, uint256 numBlocks) internal pure returns (uint256 offset) {
        uint32 suffixMask = 0;
        for (uint256 i = numBlocks; i > 0; --i) {
            uint256 idx = i - 1;
            uint32 bit = uint32(1) << _witnessIndexAt(meta, idx);
            if (_popcount32(suffixMask | bit) >= _FINALITY_DISTINCT_SR_THRESHOLD) return idx;
            suffixMask |= bit;
        }
        revert("no finalized offset in range");
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

    function test_eventChainTip_constructor_matchesExpected() public view {
        uint256 deployBlockNumber = block.number;
        uint256 deployBlockTimestamp = block.timestamp;

        bytes20[27] memory srs = _srs;
        bytes20[27] memory witnessDelegatees = _witnessDelegatees;

        bytes32 tip1 = sha256(
            abi.encodePacked(
                EventChainGenesis.TronLightClientIndex,
                deployBlockNumber,
                deployBlockTimestamp,
                TronLightClientIndex.TronLightClientConfigured.selector,
                abi.encode(
                    address(0),
                    bytes32(0),
                    _startingBlockId,
                    _startingBlockTxTrieRoot,
                    _startingBlockTimestamp,
                    srs,
                    witnessDelegatees
                )
            )
        );

        uint256 startingBlockNumber = _blockIdToNumber(_startingBlockId);
        bytes32 tip2 = sha256(
            abi.encodePacked(
                tip1,
                deployBlockNumber,
                deployBlockTimestamp,
                TronLightClientIndex.TronBlockStored.selector,
                abi.encode(startingBlockNumber, _startingBlockId, _startingBlockTxTrieRoot, _startingBlockTimestamp)
            )
        );

        bytes32 expectedTip = tip2;
        if (startingBlockNumber > 0) {
            expectedTip = sha256(
                abi.encodePacked(
                    tip2,
                    deployBlockNumber,
                    deployBlockTimestamp,
                    TronLightClientIndex.LatestProvenBlockUpdated.selector,
                    abi.encode(bytes32(0), _startingBlockId, startingBlockNumber)
                )
            );
        }

        assertEq(_client.eventChainTip(), expectedTip, "constructor eventChainTip mismatch");
    }

    function test_constructor_revertsIfSrsNotSorted() public {
        bytes20[27] memory srs = _sortedSrsFixtureless();
        bytes20[27] memory witnessDelegatees;

        // Break strict ordering at index=1 (srs[0] >= srs[1]).
        (srs[0], srs[1]) = (srs[1], srs[0]);

        bytes memory err = abi.encodeWithSelector(TronLightClient.SrSetNotSorted.selector, 1, srs[0], srs[1]);
        vm.expectRevert(err);

        new TronLightClient(
            IBlockRangeProver(address(0)),
            _startingBlockId,
            _startingBlockTxTrieRoot,
            _startingBlockTimestamp,
            srs,
            witnessDelegatees,
            bytes32(0)
        );
    }

    function test_proveBlocks_happyPath_fixture() public {
        // Call proveBlocks with the real Tron data.
        uint256 numBlocks = _metadata.length / 69;
        uint256 storeOffset = _latestFinalizedStoreOffset(_metadata, numBlocks);
        uint256 storeOffsets16 = _storeOnly(storeOffset);
        _client.proveBlocks(_startingBlockId, _metadata, _sigs, type(uint256).max, storeOffsets16);

        // latestProvenBlock should be the stored checkpoint (finalized within the proven segment).
        assertEq(_client.latestProvenBlock(), _blockIds[storeOffset], "latestProvenBlock mismatch");

        // The stored checkpoint should be present in storage.
        uint256 numStored = _blockNumbers[storeOffset];
        assertEq(_client.getBlockId(numStored), _blockIds[storeOffset], "getBlockId(stored) mismatch");

        // The last block in the proven range is not necessarily stored (it cannot be finalized within this segment).
        uint256 numLast = _blockNumbers[_blockNumbers.length - 1];
        vm.expectRevert(TronLightClient.BlockNotRelayed.selector);
        _client.getBlockId(numLast);

        // We intentionally DO NOT store every intermediate block in storage; only
        // anchor blocks (e.g. the initial and ending ones) are persisted. A
        // mid-range block lookup should therefore fail with BlockNotRelayed.
        uint256 idxMid = storeOffset == 0 ? 1 : 0;
        uint256 numMid = _blockNumbers[idxMid];
        vm.expectRevert(TronLightClient.BlockNotRelayed.selector);
        _client.getBlockId(numMid);

        // The starting anchor used in the constructor should also be present;
        // for this fixture it is the parent of blockNumbers[0].
        uint256 parentNum = _blockNumbers[0] - 1;
        assertEq(_client.getBlockId(parentNum), _startingBlockId, "getBlockId(parent) mismatch");

        // And the txTrieRoots for both the starting anchor and stored checkpoint should match.
        assertEq(_client.getTxTrieRoot(parentNum), _startingBlockTxTrieRoot, "getTxTrieRoot(parent) mismatch");
        assertEq(
            _client.getTxTrieRoot(numStored), _txTrieRootAt(_metadata, storeOffset), "getTxTrieRoot(stored) mismatch"
        );
    }

    function test_proveBlocks_allowsUnanchoredStartIfIntersectsLater() public {
        // First prove a full fixture range and store a finalized checkpoint as an anchor.
        uint256 numBlocks = _metadata.length / 69;
        uint256 storeOffset = _latestFinalizedStoreOffset(_metadata, numBlocks);
        uint256 storeOffsets16 = _storeOnly(storeOffset);
        _client.proveBlocks(_startingBlockId, _metadata, _sigs, type(uint256).max, storeOffsets16);

        // Now prove only the second half of the range, starting from a block that is not stored.
        // This succeeds because the proof intersects the already-stored anchor checkpoint.
        uint256 startIdx = _blockNumbers.length / 2;
        require(startIdx > 0, "fixture must contain >1 block");
        if (storeOffset < startIdx) startIdx = storeOffset;
        if (startIdx == 0) startIdx = 1;

        uint256 numSlice = _blockNumbers.length - startIdx;
        bytes32 unanchoredStartingBlock = _blockIds[startIdx - 1];
        bytes memory metaSlice = _sliceBytes(_metadata, startIdx * 69, numSlice * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, startIdx * 65, numSlice * 65);

        // No need to store anything in the backfill call; it only needs to intersect an existing anchor.
        uint256 intersectionOffset = storeOffset - startIdx;
        _client.proveBlocks(unanchoredStartingBlock, metaSlice, sigsSlice, intersectionOffset, _storeNone());

        assertEq(_client.latestProvenBlock(), _blockIds[storeOffset], "latestProvenBlock mismatch after backfill");
    }

    function test_proveBlocks_revertsOnInvalidSignature() public {
        // Corrupt a single byte in the signatures to trigger InvalidWitnessSigner
        bytes memory badSigs = _sigs;
        badSigs[0] = bytes1(uint8(badSigs[0]) ^ 0x01);

        vm.expectRevert(TronLightClient.InvalidWitnessSigner.selector);
        _client.proveBlocks(_startingBlockId, _metadata, badSigs, type(uint256).max, type(uint256).max);
    }

    // -------------------------------------------------------------------------
    // Gas benchmarking
    // -------------------------------------------------------------------------

    function test_gasBenchmark_proveBlocks_fixture_fullRange() public {
        // Exclude all setup/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        uint256 numBlocks = _metadata.length / 69;
        uint256 storeOffset = _latestFinalizedStoreOffset(_metadata, numBlocks);
        uint256 storeOffsets16 = _storeOnly(storeOffset);
        uint256 gasUsed =
            _gasUsedProveBlocks(_client, _startingBlockId, _metadata, _sigs, type(uint256).max, storeOffsets16);
        emit log_named_uint("gas/proveBlocks.fixture.fullRange", gasUsed);

        assertEq(_client.latestProvenBlock(), _blockIds[storeOffset], "latestProvenBlock mismatch");
    }

    function test_gasBenchmark_proveBlocks_fixture_first10Blocks() public {
        uint256 n = 10;
        require(_blockNumbers.length >= n, "fixture too small");

        // Exclude slicing/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        bytes memory metaSlice = _sliceBytes(_metadata, 0, n * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, 0, n * 65);

        // Too few blocks to finalize any checkpoint; benchmark verification-only.
        uint256 storeOffsets16 = _storeNone();
        uint256 gasUsed =
            _gasUsedProveBlocks(_client, _startingBlockId, metaSlice, sigsSlice, type(uint256).max, storeOffsets16);
        emit log_named_uint("gas/proveBlocks.fixture.first10", gasUsed);

        assertEq(_client.latestProvenBlock(), _startingBlockId, "latestProvenBlock mismatch");
    }

    function test_gasBenchmark_proveBlocks_backfill_secondHalf() public {
        // Exclude the initial seeding call + slicing/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        // Seed an anchor first (simulates a client that already has some history).
        uint256 numBlocks = _metadata.length / 69;
        uint256 storeOffset = _latestFinalizedStoreOffset(_metadata, numBlocks);
        uint256 storeOffsets16Seed = _storeOnly(storeOffset);
        _client.proveBlocks(_startingBlockId, _metadata, _sigs, type(uint256).max, storeOffsets16Seed);

        // Now benchmark "backfilling" only the second half, starting from an unanchored block.
        uint256 startIdx = _blockNumbers.length / 2;
        require(startIdx > 0, "fixture must contain >1 block");
        if (storeOffset < startIdx) startIdx = storeOffset;
        if (startIdx == 0) startIdx = 1;

        uint256 numSlice = _blockNumbers.length - startIdx;
        bytes32 unanchoredStartingBlock = _blockIds[startIdx - 1];
        bytes memory metaSlice = _sliceBytes(_metadata, startIdx * 69, numSlice * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, startIdx * 65, numSlice * 65);

        uint256 intersectionOffset = storeOffset - startIdx;
        uint256 storeOffsets16Slice = _storeNone();
        uint256 gasUsed = _gasUsedProveBlocks(
            _client, unanchoredStartingBlock, metaSlice, sigsSlice, intersectionOffset, storeOffsets16Slice
        );
        emit log_named_uint("gas/proveBlocks.backfill.secondHalf", gasUsed);

        assertEq(_client.latestProvenBlock(), _blockIds[storeOffset], "latestProvenBlock mismatch after backfill");
    }

    function test_gasBenchmark_proveBlocks_synthetic_twoBlocks() public {
        // Exclude deploy/proof-building/assert/log overhead from the reported test gas.
        vm.pauseGasMetering();

        uint256 pk0 = 0xA11CE;
        uint256 pk1 = 0xB0B;

        (TronLightClientHarness cHarness, bytes32 startingBlockId) = _deployHarnessForTwoPks(pk0, pk1);
        (bytes memory meta, bytes memory sigs,) =
            _buildTwoBlocksDifferentCreatorsProof(cHarness, startingBlockId, pk0, pk1);

        TronLightClient c = TronLightClient(address(cHarness));

        // Too few blocks to finalize any checkpoint; benchmark verification-only.
        uint256 storeOffsets16 = _storeNone();
        uint256 gasUsed = _gasUsedProveBlocks(c, startingBlockId, meta, sigs, type(uint256).max, storeOffsets16);
        emit log_named_uint("gas/proveBlocks.synthetic.twoBlocks", gasUsed);

        assertEq(c.latestProvenBlock(), startingBlockId, "latestProvenBlock mismatch");
    }

    function _gasUsedProveBlocks(
        TronLightClient c,
        bytes32 startingBlockId,
        bytes memory meta,
        bytes memory sigs,
        uint256 intersectionOffset,
        uint256 storeOffsets16
    ) internal returns (uint256 gasUsed) {
        // Only the proveBlocks call (plus this tiny wrapper) should be included in the test's reported gas.
        vm.resumeGasMetering();

        vm.record();
        uint256 g0 = gasleft();
        c.proveBlocks(startingBlockId, meta, sigs, intersectionOffset, storeOffsets16);
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
        bytes20[27] memory srs = _sortedSrsFixtureless();
        bytes20[27] memory witnessDelegatees;

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
        uint256 n1 = 101;
        bytes32 h1 = c.hashBlockPublic(startingBlockId, bytes32(uint256(1)), uint32(1), 0, n1);
        bytes32 id1 = _toBlockId(n1, h1);

        uint256 n2 = 102;
        bytes32 h2 = c.hashBlockPublic(id1, bytes32(uint256(2)), uint32(2), 1, n2);
        endingBlockId = _toBlockId(n2, h2);

        bytes memory m1 = _packMeta(startingBlockId, bytes32(uint256(1)), uint32(1), 0);
        bytes memory m2 = _packMeta(id1, bytes32(uint256(2)), uint32(2), 1);

        (meta, sigs) = _twoBlockMetaAndSigsDifferent(pk0, pk1, h1, h2, m1, m2);
    }

    function _twoBlockMetaAndSigsDifferent(
        uint256 pk1,
        uint256 pk2,
        bytes32 h1,
        bytes32 h2,
        bytes memory m1,
        bytes memory m2
    ) internal pure returns (bytes memory meta, bytes memory sigs) {
        require(m1.length == 69 && m2.length == 69, "meta chunk must be 69 bytes");

        meta = new bytes(138);
        sigs = new bytes(130);

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

    function test_proveBlocks_allowsCreatorRepeatWithinPrevious18Blocks() public {
        uint256 pk0 = 0xA11CE;

        (TronLightClientHarness c, bytes32 startingBlockId) = _deployHarnessForPk(pk0);
        (bytes memory meta, bytes memory sigs) = _buildTwoBlocksSameCreatorProof(c, startingBlockId, pk0);

        c.proveBlocks(startingBlockId, meta, sigs, type(uint256).max, _storeNone());
    }

    function _deployHarnessForPk(uint256 pk0) internal returns (TronLightClientHarness c, bytes32 startingBlockId) {
        bytes20[27] memory srs = _sortedSrsFixtureless();
        bytes20[27] memory witnessDelegatees;

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
        uint256 n1 = 101;
        bytes32 h1 = c.hashBlockPublic(startingBlockId, bytes32(uint256(1)), uint32(1), 0, n1);
        bytes32 id1 = _toBlockId(n1, h1);

        uint256 n2 = 102;
        bytes32 h2 = c.hashBlockPublic(id1, bytes32(uint256(2)), uint32(2), 0, n2);

        bytes memory m1 = _packMeta(startingBlockId, bytes32(uint256(1)), uint32(1), 0);
        bytes memory m2 = _packMeta(id1, bytes32(uint256(2)), uint32(2), 0);

        (meta, sigs) = _twoBlockMetaAndSigs(pk0, h1, h2, m1, m2);
    }

    function _twoBlockMetaAndSigs(uint256 pk, bytes32 h1, bytes32 h2, bytes memory m1, bytes memory m2)
        internal
        pure
        returns (bytes memory meta, bytes memory sigs)
    {
        require(m1.length == 69 && m2.length == 69, "meta chunk must be 69 bytes");

        meta = new bytes(138);
        sigs = new bytes(130);

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

    function test_proveBlocks_revertsIfCheckpointNotFinalized() public {
        uint256 pk0 = 0xA11CE;
        uint256 pk1 = 0xB0B;

        (TronLightClientHarness cHarness, bytes32 startingBlockId) = _deployHarnessForTwoPks(pk0, pk1);
        (bytes memory meta, bytes memory sigs,) =
            _buildTwoBlocksDifferentCreatorsProof(cHarness, startingBlockId, pk0, pk1);

        TronLightClient c = TronLightClient(address(cHarness));

        bytes memory err = abi.encodeWithSelector(TronLightClient.CheckpointNotFinalized.selector, uint256(0), uint8(2));
        vm.expectRevert(err);
        c.proveBlocks(startingBlockId, meta, sigs, type(uint256).max, _storeOnly(0));
    }

    function test_proveBlocks_storesCheckpointOnceFinalized() public {
        uint256 pk0 = 0xA11CE;

        (TronLightClientHarness c, bytes32 startingBlockId) = _deployHarnessWithAllWitnessIndices(pk0);

        uint256 n = 20;
        (bytes memory meta, bytes memory sigs, bytes32 firstBlockId) =
            _buildNBlocksDistinctWitnessIndexProof(c, startingBlockId, pk0, n);

        c.proveBlocks(startingBlockId, meta, sigs, type(uint256).max, _storeOnly(0));

        uint256 firstBlockNumber = 101;
        assertEq(TronLightClient(address(c)).latestProvenBlock(), firstBlockId, "latestProvenBlock mismatch");
        assertEq(TronLightClient(address(c)).getBlockId(firstBlockNumber), firstBlockId, "stored blockId mismatch");
    }

    function _deployHarnessWithAllWitnessIndices(uint256 pk0)
        internal
        returns (TronLightClientHarness c, bytes32 startingBlockId)
    {
        bytes20[27] memory srs = _sortedSrsFixtureless();
        bytes20[27] memory witnessDelegatees;

        bytes20 signer = bytes20(vm.addr(pk0));
        for (uint256 i = 0; i < 27; ++i) {
            witnessDelegatees[i] = signer;
        }

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

    function _buildNBlocksDistinctWitnessIndexProof(
        TronLightClientHarness c,
        bytes32 startingBlockId,
        uint256 pk,
        uint256 n
    ) internal view returns (bytes memory meta, bytes memory sigs, bytes32 firstBlockId) {
        require(n <= 27, "n too large");
        require(n >= _FINALITY_DISTINCT_SR_THRESHOLD + 1, "n too small to finalize offset 0");

        meta = new bytes(n * 69);
        sigs = new bytes(n * 65);

        bytes32 parent = startingBlockId;

        for (uint256 i = 0; i < n; ++i) {
            uint256 blockNumber = 101 + i;
            bytes32 txTrieRoot = bytes32(i + 1);
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32 timestamp = uint32(i + 1);
            // forge-lint: disable-next-line(unsafe-typecast)
            uint8 witnessIndex = uint8(i);

            bytes32 blockHash = c.hashBlockPublic(parent, txTrieRoot, timestamp, witnessIndex, blockNumber);
            bytes32 blockId = _toBlockId(blockNumber, blockHash);
            if (i == 0) firstBlockId = blockId;

            bytes memory m = _packMeta(parent, txTrieRoot, timestamp, witnessIndex);
            bytes memory s = _sign(pk, blockHash);

            for (uint256 j = 0; j < 69; ++j) {
                meta[i * 69 + j] = m[j];
            }
            for (uint256 j = 0; j < 65; ++j) {
                sigs[i * 65 + j] = s[j];
            }

            parent = blockId;
        }
    }

    function test_proveBlocks_allowsSameDelegateeDifferentSrWithinPrevious18Blocks() public {
        uint256 pk0 = 0xA11CE;

        bytes20[27] memory srs = _sortedSrsFixtureless();
        bytes20[27] memory witnessDelegatees;

        witnessDelegatees[0] = bytes20(vm.addr(pk0));
        witnessDelegatees[1] = bytes20(vm.addr(pk0));

        uint256 parentNumber = 100;
        bytes32 startingBlockId = bytes32((parentNumber << 192) | 1);

        TronLightClientHarness c = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            startingBlockId,
            bytes32(0),
            uint32(0),
            srs,
            witnessDelegatees,
            bytes32(0) // TODO: fix
        );

        uint256 n1 = 101;
        bytes32 h1 = c.hashBlockPublic(startingBlockId, bytes32(uint256(1)), uint32(1), 0, n1);
        bytes32 id1 = _toBlockId(n1, h1);

        uint256 n2 = 102;
        bytes32 h2 = c.hashBlockPublic(id1, bytes32(uint256(2)), uint32(2), 1, n2);

        bytes memory m1 = _packMeta(startingBlockId, bytes32(uint256(1)), uint32(1), 0);
        bytes memory m2 = _packMeta(id1, bytes32(uint256(2)), uint32(2), 1);

        (bytes memory meta, bytes memory sigs) = _twoBlockMetaAndSigs(pk0, h1, h2, m1, m2);

        c.proveBlocks(startingBlockId, meta, sigs, type(uint256).max, type(uint256).max);
    }
}
