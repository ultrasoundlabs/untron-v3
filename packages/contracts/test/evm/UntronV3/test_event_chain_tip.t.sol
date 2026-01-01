// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Call} from "../../../src/evm/SwapExecutor.sol";
import {UntronV3Index} from "../../../src/evm/UntronV3Index.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3EventChainTipTest is UntronV3TestBase {
    function testEventChainTipForLeaseCreatedAndPayoutConfigUpdated() public {
        bytes32 tipBefore = _untron.eventChainTip();
        uint256 seqBefore = _untron.eventSeq();

        vm.roll(123);
        vm.warp(1_700_000_000);

        bytes32 salt = keccak256("salt_tip_lease");
        address lessee = address(0xBEEF);
        uint64 startTime = uint64(block.timestamp);
        uint64 nukeableAfter = startTime + 1 days;

        (uint256 leaseId, uint256 leaseNumber) =
            _untron.createLease(salt, lessee, nukeableAfter, 10_000, 7, block.chainid, address(_usdt), address(0xB0B));

        bytes32 tip1 = sha256(
            abi.encodePacked(
                tipBefore,
                seqBefore + 1,
                uint256(123),
                uint256(1_700_000_000),
                UntronV3Index.LeaseCreated.selector,
                abi.encode(leaseId, salt, leaseNumber, address(this), lessee, startTime, nukeableAfter, 10_000, 7)
            )
        );
        bytes32 expectedTip = sha256(
            abi.encodePacked(
                tip1,
                seqBefore + 2,
                uint256(123),
                uint256(1_700_000_000),
                UntronV3Index.PayoutConfigUpdated.selector,
                abi.encode(leaseId, block.chainid, address(_usdt), address(0xB0B))
            )
        );

        assertEq(_untron.eventChainTip(), expectedTip);
    }

    function testEventChainTipForClaimFilled() public {
        _untron.enqueueClaim(address(_usdt), 10, 1, block.chainid, address(0xA11CE));
        _usdt.mint(address(_untron), 10);

        bytes32 tipBefore = _untron.eventChainTip();
        uint256 seqBefore = _untron.eventSeq();
        vm.roll(200);
        vm.warp(1_700_000_100);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 1, noCalls);

        bytes32 expectedTip = sha256(
            abi.encodePacked(
                tipBefore,
                seqBefore + 1,
                uint256(200),
                uint256(1_700_000_100),
                UntronV3Index.ClaimFilled.selector,
                abi.encode(
                    uint256(1), uint256(0), address(_usdt), uint256(0), uint256(10), block.chainid, address(0xA11CE)
                )
            )
        );
        assertEq(_untron.eventChainTip(), expectedTip);
    }

    function testEventChainTipForOwnershipTransferred() public {
        address oldOwner = _untron.owner();
        address newOwner = address(0xBEEF);

        bytes32 tipBefore = _untron.eventChainTip();
        uint256 seqBefore = _untron.eventSeq();
        vm.roll(300);
        vm.warp(1_700_000_200);

        _untron.transferOwnership(newOwner);

        bytes32 expectedTip = sha256(
            abi.encodePacked(
                tipBefore,
                seqBefore + 1,
                uint256(300),
                uint256(1_700_000_200),
                UntronV3Index.OwnershipTransferred.selector,
                abi.encode(oldOwner, newOwner)
            )
        );
        assertEq(_untron.eventChainTip(), expectedTip);
        assertEq(_untron.owner(), newOwner);
    }
}
