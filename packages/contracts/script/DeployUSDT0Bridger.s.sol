// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

contract DeployUSDT0BridgerScript is UntronDeployer {
    using stdJson for string;

    function run() external returns (USDT0Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address usdt0 = vm.envAddress("USDT0");
        address oft = vm.envAddress("OFT");

        string memory json = vm.readFile(_bridgerConfigPath());
        uint256[] memory supportedChainIds = json.readUintArray(".usdt0.supportedChainIds");
        uint256[] memory eidsU = json.readUintArray(".usdt0.eids");
        uint32[] memory eids = _toUint32Array(eidsU);

        vm.startBroadcast(deployerPk);
        bridger = _deployUsdt0Bridger(untron, usdt0, oft, supportedChainIds, eids);
        vm.stopBroadcast();

        console2.log("USDT0Bridger deployed at:", address(bridger));
    }
}
