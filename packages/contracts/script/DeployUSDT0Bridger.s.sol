// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";

contract DeployUSDT0BridgerScript is Script {
    function run() external returns (USDT0Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address usdt0 = vm.envAddress("USDT0");
        address oft = vm.envAddress("OFT");

        vm.startBroadcast(deployerPk);
        bridger = new USDT0Bridger(untron, usdt0, oft);
        vm.stopBroadcast();

        console2.log("USDT0Bridger deployed at:", address(bridger));
    }
}
