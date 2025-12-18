// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {UntronV3} from "../src/evm/UntronV3.sol";

contract DeployUntronV3Script is Script {
    function run() external returns (UntronV3 bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address controllerAddress = vm.envAddress("CONTROLLER_ADDRESS");
        address tronReader = vm.envAddress("TRON_READER");

        vm.startBroadcast(deployerPk);
        bridger = new UntronV3(controllerAddress, bytes1(0x41), tronReader);
        vm.stopBroadcast();

        console2.log("UntronV3 deployed at:", address(bridger));
    }
}
