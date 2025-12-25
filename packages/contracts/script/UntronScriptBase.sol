// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

/// @notice Small helper base for Untron Foundry scripts (config paths, env fallbacks, basic parsing).
abstract contract UntronScriptBase is Script {
    using stdJson for string;

    function _bridgerConfigPath() internal view returns (string memory) {
        try vm.envString("BRIDGER_CONFIG_PATH") returns (string memory path) {
            return path;
        } catch {
            return string.concat(vm.projectRoot(), "/script/bridgers.json");
        }
    }

    function _tlcConfigPath() internal view returns (string memory) {
        try vm.envString("TLC_CONFIG_PATH") returns (string memory path) {
            return path;
        } catch {
            return string.concat(vm.projectRoot(), "/script/tlc.json");
        }
    }

    function _outputPath() internal view returns (string memory) {
        try vm.envString("OUTPUT_PATH") returns (string memory path) {
            return path;
        } catch {
            return "";
        }
    }

    function _usdtOrUsdt0() internal view returns (address) {
        try vm.envAddress("USDT") returns (address usdt) {
            return usdt;
        } catch {
            return vm.envAddress("USDT0");
        }
    }

    function _finalOwnerOrDeployer(address deployer) internal view returns (address) {
        try vm.envAddress("OWNER") returns (address owner) {
            return owner;
        } catch {
            return deployer;
        }
    }

    function _toUint32Array(uint256[] memory a) internal pure returns (uint32[] memory out) {
        out = new uint32[](a.length);
        for (uint256 i = 0; i < a.length; i++) {
            require(a[i] <= type(uint32).max, "uint32 overflow");
            out[i] = uint32(a[i]);
        }
    }

    function _overrideAddress(string memory key, address fromJson) internal view returns (address) {
        try vm.envAddress(key) returns (address v) {
            return v;
        } catch {
            return fromJson;
        }
    }

    function _overrideBytes32(string memory key, bytes32 fromJson) internal view returns (bytes32) {
        try vm.envBytes32(key) returns (bytes32 v) {
            return v;
        } catch {
            return fromJson;
        }
    }
}

