// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronLightClientHarness} from "./harness/TronLightClientHarness.sol";
import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientProveBlocksGasBreakdownTest is Test {
    TronLightClientHarness internal _client;

    bytes32 internal _startingBlockId;
    bytes32 internal _startingBlockTxTrieRoot;
    uint32 internal _startingBlockTimestamp;
    bytes internal _metadata;
    bytes internal _sigs;
    bytes20[27] internal _srs;
    bytes20[27] internal _witnessDelegatees;

    function _sliceBytes(bytes memory data, uint256 start, uint256 length) internal pure returns (bytes memory out) {
        require(start + length <= data.length, "slice out of bounds");
        out = new bytes(length);
        for (uint256 i = 0; i < length; i++) {
            out[i] = data[start + i];
        }
    }

    function _storeOnly(uint256 offset) internal pure returns (uint256 storeOffsets16) {
        require(offset < type(uint16).max, "offset too large");
        storeOffsets16 = type(uint256).max;
        storeOffsets16 = (storeOffsets16 & ~uint256(type(uint16).max)) | offset;
    }

    function _storeNone() internal pure returns (uint256 storeOffsets16) {
        return type(uint256).max;
    }

    function _witnessIndexAt(bytes memory meta, uint256 index) internal pure returns (uint8 wi) {
        wi = uint8(meta[index * 69 + 68]);
    }

    function _popcount32(uint32 x) internal pure returns (uint8 c) {
        while (x != 0) {
            x &= (x - 1);
            unchecked {
                ++c;
            }
        }
    }

    function _latestFinalizedStoreOffset(bytes memory meta, uint256 numBlocks) internal pure returns (uint256 offset) {
        // Same criteria as TronLightClient: need >=19 distinct SRs producing blocks after the checkpoint.
        uint8 threshold = 19;
        uint32 suffixMask = 0;

        for (uint256 i = numBlocks; i > 0; --i) {
            uint256 idx = i - 1;
            uint32 bit = uint32(1) << _witnessIndexAt(meta, idx);
            if (_popcount32(suffixMask | bit) >= threshold) return idx;
            suffixMask |= bit;
        }

        revert("no finalized offset in range");
    }

    function setUp() public {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/test/evm/fixtures/tron_78000000_78000099.json");

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile(path);

        _startingBlockId = vm.parseJsonBytes32(json, ".startingBlockId");
        _startingBlockTxTrieRoot = vm.parseJsonBytes32(json, ".startingBlockTxTrieRoot");

        uint256 startingTimestampSec = vm.parseJsonUint(json, ".startingBlockTimestamp");
        // forge-lint: disable-next-line(unsafe-typecast)
        _startingBlockTimestamp = uint32(startingTimestampSec & 0xFFFFFFFF);

        _metadata = vm.parseJsonBytes(json, ".compressedTronBlockMetadata");
        _sigs = vm.parseJsonBytes(json, ".compressedSignatures");

        address[] memory srsAddrs = vm.parseJsonAddressArray(json, ".srs");
        require(srsAddrs.length == 27, "fixture srs must be length 27");
        address[] memory delegateeAddrs = vm.parseJsonAddressArray(json, ".witnessDelegatees");
        require(delegateeAddrs.length == 27, "fixture witnessDelegatees must be length 27");

        for (uint256 i = 0; i < 27; i++) {
            _srs[i] = bytes20(srsAddrs[i]);
            _witnessDelegatees[i] = bytes20(delegateeAddrs[i]);
        }

        _client = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            _startingBlockId,
            _startingBlockTxTrieRoot,
            _startingBlockTimestamp,
            _srs,
            _witnessDelegatees,
            bytes32(0)
        );
    }

    function _gasUsedBenchDecode(bytes memory meta, uint256 numBlocks) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchDecode(meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchDecodeMinimal(bytes memory meta, uint256 numBlocks) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchDecodeMinimal(meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchAdvanceAndHash(bytes32 startingBlockId, bytes memory meta, uint256 numBlocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchAdvanceAndHash(startingBlockId, meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchSrAtOnly(uint8 witnessIndex, uint256 iters) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchSrAtOnly(witnessIndex, iters);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchSrAt(bytes memory meta, uint256 numBlocks) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchSrAt(meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchEncodeHeaderFromMeta(bytes32 startingBlockId, bytes memory meta, uint256 numBlocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchEncodeHeaderFromMeta(startingBlockId, meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchEncodeHeaderFromMetaNoSrLookup(
        bytes32 startingBlockId,
        bytes memory meta,
        uint256 numBlocks,
        bytes20 witness
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchEncodeHeaderFromMetaNoSrLookup(startingBlockId, meta, numBlocks, witness);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchEncodeHeaderDirectNoSrLookup(
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestampSec,
        bytes20 witness,
        uint256 firstBlockNumber,
        uint256 iters
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchEncodeHeaderDirectNoSrLookup(
            parentHash, txTrieRoot, timestampSec, witness, firstBlockNumber, iters
        );
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchSha256AfterEncode(bytes32 startingBlockId, bytes memory meta, uint256 numBlocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchSha256AfterEncode(startingBlockId, meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchMakeBlockId(bytes32 startingBlockId, bytes32 blockHash, uint256 numBlocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchMakeBlockId(startingBlockId, blockHash, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchRecoverSigner(bytes32 digest, bytes memory sigs, uint256 numBlocks)
        internal
        returns (uint256 gasUsed)
    {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchRecoverSigner(digest, sigs, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchWitnessDelegateeAt(bytes memory meta, uint256 numBlocks) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchWitnessDelegateeAt(meta, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedBenchFullVerifyNoStore(
        bytes32 startingBlockId,
        bytes memory meta,
        bytes memory sigs,
        uint256 numBlocks
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        _client.benchFullVerifyNoStore(startingBlockId, meta, sigs, numBlocks);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function _gasUsedProveBlocks(
        TronLightClientHarness c,
        bytes32 startingBlockId,
        bytes memory meta,
        bytes memory sigs,
        uint256 storeOffsets16
    ) internal returns (uint256 gasUsed) {
        vm.resumeGasMetering();
        uint256 g0 = gasleft();
        c.proveBlocks(startingBlockId, meta, sigs, type(uint256).max, storeOffsets16);
        gasUsed = g0 - gasleft();
        vm.pauseGasMetering();
    }

    function test_gasBreakdown_proveBlocksLoop_first10_fixture() public {
        uint256 n = 10;
        require(_metadata.length / 69 >= n, "fixture too small");

        bytes memory metaSlice = _sliceBytes(_metadata, 0, n * 69);
        bytes memory sigsSlice = _sliceBytes(_sigs, 0, n * 65);

        vm.pauseGasMetering();

        {
            uint256 gasDecodeMin = _gasUsedBenchDecodeMinimal(metaSlice, n);
            emit log_named_uint("gas/benchDecodeMinimal.total", gasDecodeMin);
            emit log_named_uint("gas/benchDecodeMinimal.perBlock", gasDecodeMin / n);
        }

        {
            uint256 gasDecode = _gasUsedBenchDecode(metaSlice, n);
            emit log_named_uint("gas/benchDecode.total", gasDecode);
            emit log_named_uint("gas/benchDecode.perBlock", gasDecode / n);
        }

        {
            uint256 gasSrAtOnly = _gasUsedBenchSrAtOnly(_witnessIndexAt(metaSlice, 0), 1024);
            emit log_named_uint("gas/benchSrAtOnly(1024).total", gasSrAtOnly);
            emit log_named_uint("gas/benchSrAtOnly(1).approx", gasSrAtOnly / 1024);
        }

        {
            uint256 gasSrAt = _gasUsedBenchSrAt(metaSlice, n);
            emit log_named_uint("gas/benchSrAt(decode+lookup).total", gasSrAt);
            emit log_named_uint("gas/benchSrAt(decode+lookup).perBlock", gasSrAt / n);
        }

        {
            uint256 gasEncodeHeaderNoLookup =
                _gasUsedBenchEncodeHeaderFromMetaNoSrLookup(_startingBlockId, metaSlice, n, _srs[0]);
            uint256 gasEncodeHeader = _gasUsedBenchEncodeHeaderFromMeta(_startingBlockId, metaSlice, n);

            emit log_named_uint("gas/benchEncodeHeaderFromMetaNoSrLookup.total", gasEncodeHeaderNoLookup);
            emit log_named_uint("gas/benchEncodeHeaderFromMetaNoSrLookup.perBlock", gasEncodeHeaderNoLookup / n);
            emit log_named_uint("gas/benchEncodeHeaderFromMeta.total", gasEncodeHeader);
            emit log_named_uint("gas/benchEncodeHeaderFromMeta.perBlock", gasEncodeHeader / n);
            emit log_named_uint(
                "gas/encodeSrLookupIncremental(perBlock)",
                gasEncodeHeader > gasEncodeHeaderNoLookup ? (gasEncodeHeader - gasEncodeHeaderNoLookup) / n : 0
            );
        }

        {
            uint256 gasEncodeHeader = _gasUsedBenchEncodeHeaderFromMeta(_startingBlockId, metaSlice, n);
            uint256 gasSha256AfterEncode = _gasUsedBenchSha256AfterEncode(_startingBlockId, metaSlice, n);

            emit log_named_uint("gas/benchSha256AfterEncode.total", gasSha256AfterEncode);
            emit log_named_uint("gas/benchSha256AfterEncode.perBlock", gasSha256AfterEncode / n);
            emit log_named_uint(
                "gas/sha256Incremental(total)",
                gasSha256AfterEncode > gasEncodeHeader ? gasSha256AfterEncode - gasEncodeHeader : 0
            );
            emit log_named_uint(
                "gas/sha256Incremental(perBlock)",
                gasSha256AfterEncode > gasEncodeHeader ? (gasSha256AfterEncode - gasEncodeHeader) / n : 0
            );
        }

        {
            uint256 gasMakeBlockId = _gasUsedBenchMakeBlockId(_startingBlockId, bytes32(uint256(1)), n);
            emit log_named_uint("gas/benchMakeBlockId.total", gasMakeBlockId);
            emit log_named_uint("gas/benchMakeBlockId.perBlock", gasMakeBlockId / n);
        }

        {
            uint256 gasAdvanceAndHash = _gasUsedBenchAdvanceAndHash(_startingBlockId, metaSlice, n);
            emit log_named_uint("gas/benchAdvanceAndHash.total", gasAdvanceAndHash);
            emit log_named_uint("gas/benchAdvanceAndHash.perBlock", gasAdvanceAndHash / n);
        }

        {
            uint256 gasRecoverConstDigest = _gasUsedBenchRecoverSigner(bytes32(uint256(1)), sigsSlice, n);
            emit log_named_uint("gas/benchRecoverSigner.constDigest.total", gasRecoverConstDigest);
            emit log_named_uint("gas/benchRecoverSigner.constDigest.perBlock", gasRecoverConstDigest / n);
        }

        {
            uint256 gasWitnessDelegateeAt = _gasUsedBenchWitnessDelegateeAt(metaSlice, n);
            emit log_named_uint("gas/benchWitnessDelegateeAt(decode+lookup).total", gasWitnessDelegateeAt);
            emit log_named_uint("gas/benchWitnessDelegateeAt(decode+lookup).perBlock", gasWitnessDelegateeAt / n);
        }

        {
            uint256 gasFullVerifyNoStore = _gasUsedBenchFullVerifyNoStore(_startingBlockId, metaSlice, sigsSlice, n);
            emit log_named_uint("gas/benchFullVerifyNoStore.total", gasFullVerifyNoStore);
            emit log_named_uint("gas/benchFullVerifyNoStore.perBlock", gasFullVerifyNoStore / n);
        }

        {
            uint256 gasProveBlocksNoStore =
                _gasUsedProveBlocks(_client, _startingBlockId, metaSlice, sigsSlice, _storeNone());
            emit log_named_uint("gas/proveBlocks(noStore).total", gasProveBlocksNoStore);
            emit log_named_uint("gas/proveBlocks(noStore).perBlock", gasProveBlocksNoStore / n);
        }
    }

    function test_gasBreakdown_proveBlocks_storeOverhead_fullRange_fixture() public {
        uint256 numBlocks = _metadata.length / 69;
        uint256 storeOffset = _latestFinalizedStoreOffset(_metadata, numBlocks);

        vm.pauseGasMetering();

        TronLightClientHarness cNoStore = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            _startingBlockId,
            _startingBlockTxTrieRoot,
            _startingBlockTimestamp,
            _srs,
            _witnessDelegatees,
            bytes32(0)
        );

        TronLightClientHarness cStore = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            _startingBlockId,
            _startingBlockTxTrieRoot,
            _startingBlockTimestamp,
            _srs,
            _witnessDelegatees,
            bytes32(0)
        );

        uint256 gasNoStore = _gasUsedProveBlocks(cNoStore, _startingBlockId, _metadata, _sigs, _storeNone());
        emit log_named_uint("gas/proveBlocks.fullRange.noStore", gasNoStore);

        uint256 gasStoreOne = _gasUsedProveBlocks(cStore, _startingBlockId, _metadata, _sigs, _storeOnly(storeOffset));
        emit log_named_uint("gas/proveBlocks.fullRange.storeOne", gasStoreOne);

        emit log_named_uint("gas/proveBlocks.fullRange.storeOverhead", gasStoreOne - gasNoStore);
    }

    function test_gasBreakdown_encodeInner_singleBlockParams() public {
        // Use enough iterations to amortize CALL overhead, but stay within the fixture size.
        uint256 iters = _metadata.length / 69;
        require(iters > 0, "fixture empty");

        // Pull one sample block's params from the fixture (outside gas metering).
        (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts, uint8 wi) = _client.decodeAt(_metadata, 0);
        bytes20 witness = _srs[wi];
        uint256 firstBlockNumber = (uint256(_startingBlockId) >> 192) + 1;

        vm.pauseGasMetering();

        uint256 gasEncodeDirect =
            _gasUsedBenchEncodeHeaderDirectNoSrLookup(parentHash, txTrieRoot, ts, witness, firstBlockNumber, iters);
        emit log_named_uint("gas/benchEncodeHeaderDirectNoSrLookup(1024).total", gasEncodeDirect);
        emit log_named_uint("gas/benchEncodeHeaderDirectNoSrLookup(1).approx", gasEncodeDirect / iters);

        // Compare to "decode+encode" path for the same count.
        uint256 gasDecodeEncode =
            _gasUsedBenchEncodeHeaderFromMetaNoSrLookup(_startingBlockId, _metadata, iters, witness);
        emit log_named_uint("gas/benchEncodeHeaderFromMetaNoSrLookup(1024).total", gasDecodeEncode);
        emit log_named_uint("gas/benchEncodeHeaderFromMetaNoSrLookup(1).approx", gasDecodeEncode / iters);

        emit log_named_uint(
            "gas/decodeOverheadApprox(1)",
            gasDecodeEncode > gasEncodeDirect ? (gasDecodeEncode - gasEncodeDirect) / iters : 0
        );
    }
}
