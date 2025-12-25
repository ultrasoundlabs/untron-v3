// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {TronTxReader} from "../src/evm/TronTxReader.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

contract DeployTronTxReaderScript is UntronDeployer {
    function run() external returns (TronTxReader reader) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address tronLightClient = vm.envAddress("TRON_LIGHT_CLIENT");
        require(tronLightClient != address(0), "tronLightClient is zero");

        vm.startBroadcast(deployerPk);
        reader = _deployTronTxReader(tronLightClient);
        vm.stopBroadcast();

        console2.log("TronTxReader deployed at:", address(reader));
        console2.log("Bound TronLightClient:", address(tronLightClient));
    }
}
