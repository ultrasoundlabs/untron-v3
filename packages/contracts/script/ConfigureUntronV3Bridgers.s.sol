// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {UntronV3} from "../src/evm/UntronV3.sol";
import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";
import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

/// @notice Configures UntronV3 bridgers from `config/bridgers.json`.
/// @dev Expects the caller to be the UntronV3 owner.
/// Env:
/// - PRIVATE_KEY (required)
/// - UNTRON (required)
/// - CCTPV2_BRIDGER (required)
/// - USDT0_BRIDGER (required)
/// - BRIDGER_CONFIG_PATH (optional; defaults to `<projectRoot>/script/bridgers.json`)
contract ConfigureUntronV3BridgersScript is UntronDeployer {
    using stdJson for string;

    function run() external {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");

        address untronAddr = vm.envAddress("UNTRON");
        address cctpV2BridgerAddr = vm.envAddress("CCTPV2_BRIDGER");
        address usdt0BridgerAddr = vm.envAddress("USDT0_BRIDGER");

        UntronV3 untron = UntronV3(payable(untronAddr));
        CCTPV2Bridger cctpV2Bridger = CCTPV2Bridger(cctpV2BridgerAddr);
        USDT0Bridger usdt0Bridger = USDT0Bridger(payable(usdt0BridgerAddr));

        require(cctpV2Bridger.UNTRON() == untronAddr, "CCTPV2 bridger UNTRON mismatch");
        require(usdt0Bridger.UNTRON() == untronAddr, "USDT0 bridger UNTRON mismatch");

        address usdc = address(cctpV2Bridger.USDC());
        address usdt0 = address(usdt0Bridger.USDT0());

        string memory json = vm.readFile(_bridgerConfigPath());
        uint256[] memory cctpChainIds = json.readUintArray(".cctpV2.supportedChainIds");
        uint256[] memory usdt0ChainIds = json.readUintArray(".usdt0.supportedChainIds");

        console2.log("Config path:", _bridgerConfigPath());
        console2.log("UntronV3:", untronAddr);
        console2.log("CCTPV2Bridger:", cctpV2BridgerAddr);
        console2.log("USDT0Bridger:", usdt0BridgerAddr);
        console2.log("USDC token:", usdc);
        console2.log("USDT0 token:", usdt0);

        vm.startBroadcast(deployerPk);
        _setBridgerRoutes(untron, usdc, cctpChainIds, cctpV2BridgerAddr);
        _setBridgerRoutes(untron, usdt0, usdt0ChainIds, usdt0BridgerAddr);

        vm.stopBroadcast();

        console2.log("Configured CCTP V2 routes:", cctpChainIds.length);
        console2.log("Configured USDT0 routes:", usdt0ChainIds.length);
    }
}
