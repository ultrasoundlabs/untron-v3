// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";
import {MockBridger} from "./UntronV3TestUtils.sol";

contract UntronV3LeaseCreationTest is UntronV3TestBase {
    function testCreateLeaseRevertsForNonRealtor() public {
        vm.prank(address(0x1234));
        vm.expectRevert(UntronV3.NotRealtor.selector);
        _untron.createLease(
            keccak256("salt"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
    }

    function testCreateLeaseEnforcesFeeFloorsAndBounds() public {
        _untron.setProtocolFloorPpm(100);
        _untron.setRealtorMinFeePpm(address(this), 200);

        uint256 minFee = 200;

        vm.expectRevert(UntronV3.LeaseFeeTooLow.selector);
        _untron.createLease(
            keccak256("salt_fee_low"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(minFee - 1),
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        uint256 leaseId = _untron.createLease(
            keccak256("salt_fee_ok"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(minFee),
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
        assertEq(leaseId, 1);

        vm.expectRevert(UntronV3.LeaseFeeTooLow.selector);
        _untron.createLease(
            keccak256("salt_fee_too_high"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            uint32(1_000_001),
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
    }

    function testCreateLeaseEnforcesTimeframe() public {
        vm.expectRevert(UntronV3.InvalidLeaseTimeframe.selector);
        _untron.createLease(
            keccak256("salt_timeframe"),
            address(0xBEEF),
            uint64(block.timestamp - 1),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );
    }

    function testReusingReceiverSaltRespectsNukeableAfterAndTimelineAppends() public {
        bytes32 salt = keccak256("salt_reuse");

        uint64 t0 = uint64(block.timestamp);
        uint64 nukeableAfter1 = t0 + 1000;

        uint256 lease1 = _untron.createLease(
            salt, address(0xBEEF), nukeableAfter1, 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );
        assertEq(lease1, 1);

        vm.warp(t0 + 500);
        vm.expectRevert(UntronV3.LeaseNotNukeableYet.selector);
        _untron.createLease(
            salt, address(0xBEEF), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        vm.warp(nukeableAfter1);
        uint256 lease2 = _untron.createLease(
            salt, address(0xBEEF), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );
        assertEq(lease2, 2);

        uint256[] memory ids = _untron.leaseIdsByReceiver(salt);
        assertEq(ids.length, 2);
        assertEq(ids[0], lease1);
        assertEq(ids[1], lease2);
    }

    function testActiveLeaseSelectionByTimestampUsesTimeline() public {
        bytes32 salt = keccak256("salt_timeline");

        uint64 t1 = uint64(block.timestamp);
        uint256 lease1 = _untron.createLease(
            // Make the first lease immediately nukeable so we can create lease2 later for the same salt.
            salt,
            address(0x1111),
            t1,
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xAAA1)
        );

        vm.warp(t1 + 100);
        uint64 t2 = uint64(block.timestamp);
        uint256 lease2 = _untron.createLease(
            salt, address(0x2222), uint64(t2 + 1 days), 0, 0, block.chainid, address(_usdt), address(0xAAA2)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        // Timestamp between t1..t2-1 selects lease1.
        _reader.setNextCallData(
            keccak256("tx_between"),
            1,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t1 + 50),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
        (, uint256 gotLeaseId1,) = _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId1, lease1);

        // Timestamp >= t2 selects lease2.
        _reader.setNextCallData(
            keccak256("tx_after"),
            2,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t2),
            TronCalldataUtils.evmToTronAddress(address(0x2222)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
        (, uint256 gotLeaseId2,) = _untron.preEntitle(salt, 2, hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId2, lease2);

        // Claims should retain lease ids and beneficiaries.
        assertEq(_untron.claimQueueLength(address(_usdt)), 2);
        (uint256 amt0, uint256 id0, uint256 chain0, address ben0) = _untron.claimsByTargetToken(address(_usdt), 0);
        assertEq(amt0, 100);
        assertEq(id0, lease1);
        assertEq(chain0, block.chainid);
        assertEq(ben0, address(0xAAA1));

        (uint256 amt1, uint256 id1, uint256 chain1, address ben1) = _untron.claimsByTargetToken(address(_usdt), 1);
        assertEq(amt1, 100);
        assertEq(id1, lease2);
        assertEq(chain1, block.chainid);
        assertEq(ben1, address(0xAAA2));
    }

    function testCreateLeaseRoutabilityChecks() public {
        bytes32 salt0 = keccak256("salt_route_0");
        _untron.createLease(
            salt0,
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        // Token != USDT requires swap rate.
        vm.expectRevert(UntronV3.RateNotSet.selector);
        _untron.createLease(
            keccak256("salt_route_swap_missing"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_tokenX),
            address(0xB0B)
        );

        // Chain != local requires bridger (even for USDT).
        uint256 otherChainId = block.chainid + 1;
        vm.expectRevert(UntronV3.NoBridger.selector);
        _untron.createLease(
            keccak256("salt_route_bridge_missing"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            otherChainId,
            address(_usdt),
            address(0xB0B)
        );

        // With both configured, cross-chain + swap route is allowed.
        _untron.setSwapRate(address(_tokenX), 2_000_000); // 2.0 tokenX per USDT
        _untron.setBridger(address(_tokenX), otherChainId, address(new MockBridger()));

        uint256 leaseId = _untron.createLease(
            keccak256("salt_route_ok"),
            address(0xBEEF),
            uint64(block.timestamp + 1 days),
            0,
            0,
            otherChainId,
            address(_tokenX),
            address(0xB0B)
        );
        assertEq(leaseId, 2);
    }
}
