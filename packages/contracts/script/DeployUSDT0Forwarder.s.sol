// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/console2.sol";

import {IOFT} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {USDT0Forwarder} from "../src/evm/USDT0Forwarder.sol";

contract DeployUSDT0ForwarderScript is Script {
    function _dstEid() internal view returns (uint32) {
        uint256 dstEidU = vm.envUint("DST_EID");
        require(dstEidU <= type(uint32).max, "DST_EID overflow");
        // casting to 'uint32' is safe because dstEidU is checked to fit in uint32 above
        // forge-lint: disable-next-line(unsafe-typecast)
        return uint32(dstEidU);
    }

    function _token() internal view returns (address) {
        try vm.envAddress("TOKEN") returns (address token) {
            return token;
        } catch {
            return vm.envAddress("USDT0");
        }
    }

    function _beneficiary() internal view returns (bytes32) {
        try vm.envBytes32("BENEFICIARY") returns (bytes32 beneficiary) {
            return beneficiary;
        } catch {
            address beneficiaryAddress = vm.envAddress("BENEFICIARY_ADDRESS");
            return bytes32(uint256(uint160(beneficiaryAddress)));
        }
    }

    function run() external returns (USDT0Forwarder forwarder) {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address token = _token();
        address oft = vm.envAddress("OFT");
        uint32 dstEid = _dstEid();
        bytes32 beneficiary = _beneficiary();

        vm.startBroadcast(deployerPk);
        forwarder = new USDT0Forwarder(token, IOFT(oft), dstEid, beneficiary);
        vm.stopBroadcast();

        console2.log("USDT0Forwarder deployed at:", address(forwarder));
        console2.log("TOKEN:", token);
        console2.log("OFT:", oft);
        console2.log("DST_EID:", dstEid);
        console2.logBytes32(beneficiary);
    }
}
