// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {UntronV3} from "../src/evm/UntronV3.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

contract DeployUntronV3Script is UntronDeployer {
    function run() external returns (UntronV3 bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address controllerAddress = vm.envAddress("CONTROLLER_ADDRESS");
        address tronReceiverImpl = vm.envAddress("TRON_RECEIVER_IMPL");
        address tronReader = vm.envAddress("TRON_READER");

        vm.startBroadcast(deployerPk);
        bridger = _deployUntronV3(controllerAddress, tronReceiverImpl);
        _setUntronTronReader(bridger, tronReader);
        vm.stopBroadcast();

        console2.log("UntronV3 deployed at:", address(bridger));
    }
}
