// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

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
}
