// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

contract DeployCCTPV2BridgerScript is UntronDeployer {
    using stdJson for string;

    function run() external returns (CCTPV2Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address tokenMessengerV2 = vm.envAddress("TOKEN_MESSENGER_V2");
        address usdc = vm.envAddress("USDC");

        string memory json = vm.readFile(_bridgerConfigPath());
        uint256[] memory supportedChainIds = json.readUintArray(".cctpV2.supportedChainIds");
        uint256[] memory domainsU = json.readUintArray(".cctpV2.circleDomains");
        uint32[] memory domains = _toUint32Array(domainsU);

        vm.startBroadcast(deployerPk);
        bridger = _deployCctpV2Bridger(untron, tokenMessengerV2, usdc, supportedChainIds, domains);
        vm.stopBroadcast();

        console2.log("CCTPV2Bridger deployed at:", address(bridger));
    }
}
