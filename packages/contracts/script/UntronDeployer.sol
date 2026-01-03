// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronScriptBase} from "./UntronScriptBase.sol";

import {TronLightClient} from "../src/evm/TronLightClient.sol";
import {TronTxReader} from "../src/evm/TronTxReader.sol";
import {UntronV3} from "../src/evm/UntronV3.sol";
import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";
import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";
import {IBlockRangeProver} from "../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

/// @notice Shared deployment/configuration helpers for Untron Foundry scripts.
/// @dev This focuses on the *actions* (deploy + wire), so scripts can differ only in how they source params.
abstract contract UntronDeployer is UntronScriptBase {
    bytes1 internal constant _TRON_CREATE2_PREFIX = bytes1(0x41);

    function _deployUntronV3WithCreate2Prefix(address controllerAddress, bytes1 create2Prefix, address tronReceiverImpl)
        internal
        returns (UntronV3 untron)
    {
        untron = new UntronV3(controllerAddress, create2Prefix, tronReceiverImpl);
    }

    function _deployTronLightClient(
        address proverAddr,
        bytes32 initialBlockHash,
        bytes32 initialTxTrieRoot,
        uint32 initialTimestamp,
        bytes20[27] memory srs,
        bytes20[27] memory witnessDelegatees,
        bytes32 srDataHash
    ) internal returns (TronLightClient lc) {
        lc = new TronLightClient(
            IBlockRangeProver(proverAddr),
            initialBlockHash,
            initialTxTrieRoot,
            initialTimestamp,
            srs,
            witnessDelegatees,
            srDataHash
        );
    }

    function _deployTronTxReader(address tronLightClient) internal returns (TronTxReader reader) {
        reader = new TronTxReader(tronLightClient);
    }

    function _deployUntronV3(address controllerAddress, address tronReceiverImpl) internal returns (UntronV3 untron) {
        untron = _deployUntronV3WithCreate2Prefix(controllerAddress, _TRON_CREATE2_PREFIX, tronReceiverImpl);
    }

    function _setUntronTronReader(UntronV3 untron, address tronReader) internal {
        untron.setTronReader(tronReader);
    }

    function _setUntronUsdt(UntronV3 untron, address usdt) internal {
        untron.setUsdt(usdt);
    }

    function _deployCctpV2Bridger(
        address untron,
        address tokenMessengerV2,
        address usdc,
        uint256[] memory supportedChainIds,
        uint32[] memory circleDomains
    ) internal returns (CCTPV2Bridger bridger) {
        bridger = new CCTPV2Bridger(untron, tokenMessengerV2, usdc, supportedChainIds, circleDomains);
    }

    function _deployUsdt0Bridger(
        address untron,
        address usdt0,
        address oft,
        uint256[] memory supportedChainIds,
        uint32[] memory eids
    ) internal returns (USDT0Bridger bridger) {
        bridger = new USDT0Bridger(untron, usdt0, oft, supportedChainIds, eids);
    }

    function _setBridgerRoutes(UntronV3 untron, address token, uint256[] memory targetChainIds, address bridger)
        internal
    {
        for (uint256 i = 0; i < targetChainIds.length; i++) {
            untron.setBridger(token, targetChainIds[i], bridger);
        }
    }

    function _transferOwnershipsIfNeeded(
        address deployer,
        address finalOwner,
        UntronV3 untron,
        CCTPV2Bridger cctpV2Bridger,
        USDT0Bridger usdt0Bridger
    ) internal {
        if (finalOwner == address(0) || finalOwner == deployer) return;
        untron.transferOwnership(finalOwner);
        cctpV2Bridger.transferOwnership(finalOwner);
        usdt0Bridger.transferOwnership(finalOwner);
    }
}
