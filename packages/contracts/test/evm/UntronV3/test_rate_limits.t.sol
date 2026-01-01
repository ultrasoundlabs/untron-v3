// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3} from "../../../src/evm/UntronV3.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3RateLimitTest is UntronV3TestBase {
    function testRealtorLeaseRateLimitConfigValidity() public {
        vm.expectRevert(UntronV3.LeaseRateLimitConfigInvalid.selector);
        _untron.setRealtorLeaseRateLimit(address(this), 0, 1);

        vm.expectRevert(UntronV3.LeaseRateLimitConfigInvalid.selector);
        _untron.setRealtorLeaseRateLimit(address(this), 1, 0);

        vm.expectRevert(UntronV3.LeaseRateLimitConfigInvalid.selector);
        _untron.setRealtorLeaseRateLimit(address(this), uint256(type(uint32).max) + 1, 1);

        vm.expectRevert(UntronV3.LeaseRateLimitConfigInvalid.selector);
        _untron.setRealtorLeaseRateLimit(address(this), 1, uint256(type(uint32).max) + 1);
    }

    function testLeaseRateLimitEnforcesRealtorLimit() public {
        _untron.setRealtorLeaseRateLimit(address(this), 2, 1 hours);

        _untron.createLease(
            keccak256("salt1"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
        _untron.createLease(
            keccak256("salt2"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        vm.expectRevert(UntronV3.LeaseRateLimitExceeded.selector);
        _untron.createLease(
            keccak256("salt3"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        vm.warp(block.timestamp + 1 hours);
        _untron.createLease(
            keccak256("salt4"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
    }

    function testLeaseRateLimitDisabledModeSkipsRateLimiting() public {
        _untron.setRealtorLeaseRateLimit(address(this), 0, 0);

        for (uint256 i = 0; i < 5; ++i) {
            _untron.createLease(
                keccak256(abi.encodePacked("salt", i)),
                address(0xBEEF),
                uint64(block.timestamp + 1 days),
                0,
                0,
                block.chainid,
                address(_usdt),
                address(0xB0B)
            );
        }
    }
}
