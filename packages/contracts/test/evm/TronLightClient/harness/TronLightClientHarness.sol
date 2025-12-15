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

    function hashBlockPublic(TronBlockMetadata memory b, uint256 n) external view returns (bytes32) {
        return _hashBlock(b, n);
    }

    function encodeBlockHeaderPublic(TronBlockMetadata memory b, uint256 n) external view returns (bytes memory) {
        return _encodeTronBlockHeader(b, n);
    }

    function decodeAt(bytes calldata data, uint256 idx) external pure returns (TronBlockMetadata memory) {
        return _decodeTronBlockAt(data, idx);
    }
}
