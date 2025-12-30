// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {TronLightClient} from "../../../../src/evm/TronLightClient.sol";
import {IBlockRangeProver} from "../../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientHarness is TronLightClient {
    constructor(
        IBlockRangeProver p,
        bytes32 initial,
        bytes32 initialTxTrieRoot,
        uint32 initialTimestamp,
        bytes20[27] memory srs_,
        bytes20[27] memory witnessDelegatees_,
        bytes32 srDataHash_
    ) TronLightClient(p, initial, initialTxTrieRoot, initialTimestamp, srs_, witnessDelegatees_, srDataHash_) {}

    function benchNoop(uint256 iters) external pure returns (uint256 acc) {
        for (uint256 i = 0; i < iters; ++i) {
            unchecked {
                acc += i;
            }
        }
    }

    function hashBlockPublic(bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex, uint256 n)
        external
        view
        returns (bytes32)
    {
        bytes memory scratch = new bytes(128);
        return _hashBlockScratch(parentHash, txTrieRoot, timestamp, witnessIndex, n, scratch);
    }

    function encodeBlockHeaderPublic(
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestamp,
        uint8 witnessIndex,
        uint256 n
    ) external view returns (bytes memory) {
        bytes memory buf = new bytes(128);

        uint256 used = _encodeTronBlockHeaderInto(buf, parentHash, txTrieRoot, timestamp, witnessIndex, n);

        // Preserve the prior behavior: shrink `buf` to the number of bytes written.
        // solhint-disable-next-line no-inline-assembly
        assembly {
            mstore(buf, used)
        }

        return buf;
    }

    function decodeAt(bytes calldata data, uint256 idx)
        external
        pure
        returns (bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex)
    {
        (parentHash, txTrieRoot, timestamp, witnessIndex) = _decodeTronBlockAtStack(data, idx);
    }

    function benchDecode(bytes calldata data, uint256 numBlocks) external pure returns (bytes32 acc) {
        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex) =
                _decodeTronBlockAtStack(data, i);
            acc ^= parentHash;
            acc ^= txTrieRoot;
            acc ^= bytes32(uint256(timestamp));
            acc ^= bytes32(uint256(witnessIndex));
        }
    }

    function benchDecodeMinimal(bytes calldata data, uint256 numBlocks) external pure returns (bytes32 acc) {
        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 parentHash,, uint32 timestamp, uint8 witnessIndex) = _decodeTronBlockAtStack(data, i);
            acc ^= parentHash;
            acc ^= bytes32(uint256(timestamp));
            acc ^= bytes32(uint256(witnessIndex));
        }
    }

    function benchSrAtOnly(uint8 witnessIndex, uint256 iters) external view returns (bytes20 acc) {
        uint160 x;
        for (uint256 i = 0; i < iters; ++i) {
            x ^= uint160(_srAt(witnessIndex));
        }
        acc = bytes20(x);
    }

    function benchSrAt(bytes calldata meta, uint256 numBlocks) external view returns (bytes20 acc) {
        uint160 x;
        for (uint256 i = 0; i < numBlocks; ++i) {
            (,,, uint8 witnessIndex) = _decodeTronBlockAtStack(meta, i);
            bytes20 sr = _srAt(witnessIndex);
            x ^= uint160(sr);
        }
        acc = bytes20(x);
    }

    function _encodeTronBlockHeaderIntoWithWitness(
        bytes memory buf,
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestampSec,
        bytes20 witness,
        uint256 blockNumber
    ) internal pure returns (uint256 used) {
        uint256 tsMillis = uint256(timestampSec) * 1000;
        uint160 witness160 = uint160(witness);

        // solhint-disable-next-line no-inline-assembly
        assembly {
            let base := add(buf, 32)
            let ptr := base

            // field 1: timestamp (varint, key = 0x08)
            mstore8(ptr, 0x08)
            ptr := add(ptr, 1)
            {
                let v := tsMillis
                for {} gt(v, 0x7f) {} {
                    mstore8(ptr, or(and(v, 0x7f), 0x80))
                    ptr := add(ptr, 1)
                    v := shr(7, v)
                }
                mstore8(ptr, and(v, 0x7f))
                ptr := add(ptr, 1)
            }

            // field 2: txTrieRoot (bytes, key = 0x12), len=32
            mstore8(ptr, 0x12)
            ptr := add(ptr, 1)
            mstore8(ptr, 32)
            ptr := add(ptr, 1)
            mstore(ptr, txTrieRoot)
            ptr := add(ptr, 32)

            // field 3: parentHash (bytes, key = 0x1a), len=32
            mstore8(ptr, 0x1a)
            ptr := add(ptr, 1)
            mstore8(ptr, 32)
            ptr := add(ptr, 1)
            mstore(ptr, parentHash)
            ptr := add(ptr, 32)

            // field 7: number (varint, key = 0x38)
            mstore8(ptr, 0x38)
            ptr := add(ptr, 1)
            {
                let vNum := blockNumber
                for {} gt(vNum, 0x7f) {} {
                    mstore8(ptr, or(and(vNum, 0x7f), 0x80))
                    ptr := add(ptr, 1)
                    vNum := shr(7, vNum)
                }
                mstore8(ptr, and(vNum, 0x7f))
                ptr := add(ptr, 1)
            }

            // field 9: witnessAddress (bytes, key = 0x4a), len=21 (0x41 + 20 bytes)
            mstore8(ptr, 0x4a)
            ptr := add(ptr, 1)
            mstore8(ptr, 21)
            ptr := add(ptr, 1)
            mstore8(ptr, 0x41)
            ptr := add(ptr, 1)
            mstore(ptr, shl(96, witness160))
            ptr := add(ptr, 20)

            // field 10: version (varint, key = 0x50), value = 32
            mstore8(ptr, 0x50)
            ptr := add(ptr, 1)
            mstore8(ptr, _TRON_BLOCK_VERSION)
            ptr := add(ptr, 1)

            used := sub(ptr, base)
        }
    }

    function benchEncodeHeaderDirectNoSrLookup(
        bytes32 parentHash,
        bytes32 txTrieRoot,
        uint32 timestampSec,
        bytes20 witness,
        uint256 firstBlockNumber,
        uint256 iters
    ) external pure returns (uint256 usedTotal, bytes32 acc) {
        bytes memory scratch = new bytes(128);
        for (uint256 i = 0; i < iters; ++i) {
            uint256 blockNumber = firstBlockNumber + i;
            uint256 used = _encodeTronBlockHeaderIntoWithWitness(
                scratch, parentHash, txTrieRoot, timestampSec, witness, blockNumber
            );
            usedTotal += used;
            acc ^= bytes32(used);
        }
    }

    function benchEncodeHeaderFromMeta(bytes32 startingBlockId, bytes calldata meta, uint256 numBlocks)
        external
        view
        returns (uint256 usedTotal, bytes32 acc)
    {
        uint256 startingBlockNumber = _blockIdToNumber(startingBlockId);
        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts, uint8 wi) = _decodeTronBlockAtStack(meta, i);
            uint256 blockNumber = startingBlockNumber + i + 1;

            uint256 used = _encodeTronBlockHeaderInto(scratch, parentHash, txTrieRoot, ts, wi, blockNumber);
            usedTotal += used;
            acc ^= bytes32(used);
        }
    }

    function benchEncodeHeaderFromMetaNoSrLookup(
        bytes32 startingBlockId,
        bytes calldata meta,
        uint256 numBlocks,
        bytes20 witness
    ) external pure returns (uint256 usedTotal, bytes32 acc) {
        uint256 startingBlockNumber = _blockIdToNumber(startingBlockId);
        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts,) = _decodeTronBlockAtStack(meta, i);
            uint256 blockNumber = startingBlockNumber + i + 1;

            uint256 used =
                _encodeTronBlockHeaderIntoWithWitness(scratch, parentHash, txTrieRoot, ts, witness, blockNumber);
            usedTotal += used;
            acc ^= bytes32(used);
        }
    }

    function benchSha256AfterEncode(bytes32 startingBlockId, bytes calldata meta, uint256 numBlocks)
        external
        view
        returns (bytes32 acc)
    {
        uint256 startingBlockNumber = _blockIdToNumber(startingBlockId);
        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 ts, uint8 wi) = _decodeTronBlockAtStack(meta, i);
            uint256 blockNumber = startingBlockNumber + i + 1;
            uint256 used = _encodeTronBlockHeaderInto(scratch, parentHash, txTrieRoot, ts, wi, blockNumber);

            bytes32 h;
            // solhint-disable-next-line no-inline-assembly
            assembly {
                if iszero(staticcall(gas(), 0x02, add(scratch, 32), used, 0x00, 32)) { revert(0, 0) }
                h := mload(0x00)
            }

            acc ^= h;
        }
    }

    function benchAdvanceAndHash(bytes32 startingBlockId, bytes calldata meta, uint256 numBlocks)
        external
        view
        returns (bytes32 acc)
    {
        ProveBlocksCtx memory ctx;
        ctx.blockId = startingBlockId;
        ctx.blockNumber = _blockIdToNumber(startingBlockId);

        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 blockHash, uint8 witnessIndex) = _advanceAndHash(ctx, meta, i, scratch);
            acc ^= blockHash;
            acc ^= bytes32(uint256(witnessIndex));
        }
    }

    function benchRecoverSigner(bytes32 digest, bytes calldata sigs, uint256 numBlocks)
        external
        view
        returns (bytes20 acc)
    {
        bytes memory scratch = new bytes(128);
        uint160 x;

        for (uint256 i = 0; i < numBlocks; ++i) {
            bytes20 signer = _recoverSigner(digest, sigs, i, scratch);
            x ^= uint160(signer);
        }

        acc = bytes20(x);
    }

    function benchWitnessDelegateeAt(bytes calldata meta, uint256 numBlocks) external view returns (bytes20 acc) {
        uint160 x;

        for (uint256 i = 0; i < numBlocks; ++i) {
            (,,, uint8 witnessIndex) = _decodeTronBlockAtStack(meta, i);
            bytes20 delegatee = _witnessDelegateeAt(witnessIndex);
            x ^= uint160(delegatee);
        }

        acc = bytes20(x);
    }

    function benchMakeBlockId(bytes32 startingBlockId, bytes32 blockHash, uint256 numBlocks)
        external
        pure
        returns (bytes32 acc)
    {
        uint256 startingBlockNumber = _blockIdToNumber(startingBlockId);
        for (uint256 i = 0; i < numBlocks; ++i) {
            uint256 blockNumber = startingBlockNumber + i + 1;
            acc ^= _makeBlockId(blockNumber, blockHash);
        }
    }

    function benchFullVerifyNoStore(
        bytes32 startingBlockId,
        bytes calldata meta,
        bytes calldata sigs,
        uint256 numBlocks
    ) external view returns (bytes32 acc) {
        ProveBlocksCtx memory ctx;
        ctx.blockId = startingBlockId;
        ctx.blockNumber = _blockIdToNumber(startingBlockId);

        bytes memory scratch = new bytes(128);

        for (uint256 i = 0; i < numBlocks; ++i) {
            (bytes32 blockHash, uint8 witnessIndex) = _advanceAndHash(ctx, meta, i, scratch);

            bytes20 signer = _recoverSigner(blockHash, sigs, i, scratch);
            bytes20 expected = _witnessDelegateeAt(witnessIndex);
            if (signer != expected) revert InvalidWitnessSigner();

            acc ^= bytes32(uint256(uint160(signer)));
        }
    }
}
