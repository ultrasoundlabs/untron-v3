// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";

contract DeployUSDT0BridgerScript is Script {
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

    function run() external returns (USDT0Bridger bridger) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address untron = vm.envAddress("UNTRON");
        address usdt0 = vm.envAddress("USDT0");
        address oft = vm.envAddress("OFT");

        string memory json = vm.readFile(_configPath());
        uint256[] memory supportedChainIds = json.readUintArray(".usdt0.supportedChainIds");
        uint256[] memory eidsU = json.readUintArray(".usdt0.eids");
        uint32[] memory eids = _toUint32Array(eidsU);

        vm.startBroadcast(deployerPk);
        bridger = new USDT0Bridger(untron, usdt0, oft, supportedChainIds, eids);
        vm.stopBroadcast();

        console2.log("USDT0Bridger deployed at:", address(bridger));
    }
}
