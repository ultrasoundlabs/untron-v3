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
        bytes memory scratch = new bytes(128);
        return _hashBlockScratch(b, n, scratch);
    }

    function encodeBlockHeaderPublic(TronBlockMetadata memory b, uint256 n) external view returns (bytes memory) {
        bytes memory buf = new bytes(128);
        _encodeTronBlockHeaderInto(buf, b, n);
        return buf;
    }

    function decodeAt(bytes calldata data, uint256 idx) external pure returns (TronBlockMetadata memory) {
        return _decodeTronBlockAt(data, idx);
    }
}
