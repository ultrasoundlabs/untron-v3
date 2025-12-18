// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";

contract DeployCCTPV2BridgerScript is Script {
    function run() external returns (CCTPV2Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address tokenMessengerV2 = vm.envAddress("TOKEN_MESSENGER_V2");
        address usdc = vm.envAddress("USDC");

        vm.startBroadcast(deployerPk);
        bridger = new CCTPV2Bridger(untron, tokenMessengerV2, usdc);
        vm.stopBroadcast();

        console2.log("CCTPV2Bridger deployed at:", address(bridger));
    }
}
