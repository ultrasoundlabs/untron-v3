// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";

contract DeployCCTPV2BridgerScript is Script {
    using stdJson for string;

    function _configPath() internal view returns (string memory) {
        try vm.envString("BRIDGER_CONFIG_PATH") returns (string memory path) {
            return path;
        } catch {
            return string.concat(vm.projectRoot(), "/script/bridgers.json");
        }
    }

    function _toUint32Array(uint256[] memory a) internal pure returns (uint32[] memory out) {
        out = new uint32[](a.length);
        for (uint256 i = 0; i < a.length; i++) {
            require(a[i] <= type(uint32).max, "uint32 overflow");
            out[i] = uint32(a[i]);
        }
    }

    function run() external returns (CCTPV2Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address tokenMessengerV2 = vm.envAddress("TOKEN_MESSENGER_V2");
        address usdc = vm.envAddress("USDC");

        string memory json = vm.readFile(_configPath());
        uint256[] memory supportedChainIds = json.readUintArray(".cctpV2.supportedChainIds");
        uint256[] memory domainsU = json.readUintArray(".cctpV2.circleDomains");
        uint32[] memory domains = _toUint32Array(domainsU);

        vm.startBroadcast(deployerPk);
        bridger = new CCTPV2Bridger(untron, tokenMessengerV2, usdc, supportedChainIds, domains);
        vm.stopBroadcast();

        console2.log("CCTPV2Bridger deployed at:", address(bridger));
    }
}
