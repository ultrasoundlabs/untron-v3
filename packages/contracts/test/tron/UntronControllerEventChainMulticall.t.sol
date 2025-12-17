// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {EventChainGenesis} from "../../src/utils/EventChainGenesis.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract UntronControllerEventChainMulticallTest is Test {
    address internal constant _LP = address(0xB0B);

    function test_isEventChainTip_works() public {
        UntronController controller = new UntronController(0xff);
        bytes32 tip = controller.eventChainTip();

        assertTrue(controller.isEventChainTip(tip));

        vm.expectRevert();
        controller.isEventChainTip(bytes32(uint256(tip) + 1));
    }

    function test_tip_updatesOnlyWhenEventsEmit() public {
        UntronController controller = new UntronController(0xff);
        MockERC20 usdt = new MockERC20("USDT", "USDT", 18);
        controller.setUsdt(address(usdt));
        controller.setLp(_LP);

        bytes32 tipAfterSetup = controller.eventChainTip();

        // No-op pullFromReceivers (all sweep=0) should not emit or update the tip.
        bytes32 salt = keccak256("zero");
        controller.pullFromReceivers(address(usdt), _asArray(salt), _asArray(uint256(0)), 0);
        assertEq(controller.eventChainTip(), tipAfterSetup, "tip should not change on no-op pull");

        // No-op LP withdraw (amount=0) should not emit or update the tip.
        vm.prank(_LP);
        controller.lpWithdrawTokens(address(usdt), 0);
        assertEq(controller.eventChainTip(), tipAfterSetup, "tip should not change on no-op withdraw");

        // Event-emitting call should update the tip.
        controller.setExecutor(address(0xE0));
        assertTrue(controller.eventChainTip() != tipAfterSetup, "tip should change on event");
    }

    function test_exactTipHash_singleEvent() public {
        vm.roll(1_000);
        vm.warp(1_700_000_000);

        UntronController controller = new UntronController(0xff);

        bytes32 expected = sha256(
            abi.encodePacked(
                EventChainGenesis.UntronControllerIndex,
                block.number,
                block.timestamp,
                keccak256("OwnerChanged(address)"),
                abi.encode(address(this))
            )
        );
        assertEq(controller.eventChainTip(), expected, "constructor tip mismatch");

        vm.roll(1_001);
        vm.warp(1_700_000_123);

        controller.setExecutor(address(0xE0));

        bytes32 expected2 = sha256(
            abi.encodePacked(
                expected,
                block.number,
                block.timestamp,
                keccak256("ExecutorChanged(address)"),
                abi.encode(address(0xE0))
            )
        );
        assertEq(controller.eventChainTip(), expected2, "setExecutor tip mismatch");
    }

    function test_multicall_revertsIfSenderNotAuthorizedForAllSubcalls() public {
        UntronController controller = new UntronController(0xff);
        controller.setLp(_LP);

        bytes[] memory calls = new bytes[](2);
        calls[0] = abi.encodeCall(controller.setExecutor, (address(0xE0)));
        calls[1] = abi.encodeCall(controller.setLpExchangeRate, (address(0x1234), 1e18));

        vm.expectRevert(UntronController.OnlyLp.selector);
        controller.multicall(calls);

        assertEq(controller.executor(), address(0), "state should revert on multicall failure");

        bytes[] memory calls2 = new bytes[](2);
        calls2[0] = abi.encodeCall(controller.setLpExchangeRate, (address(0x1234), 1e18));
        calls2[1] = abi.encodeCall(controller.setExecutor, (address(0xE1)));

        vm.prank(_LP);
        vm.expectRevert(UntronController.OnlyOwner.selector);
        controller.multicall(calls2);

        assertEq(controller.lpExchangeRateFor(address(0x1234)), 0, "state should revert on multicall failure");
    }

    function test_multicall_tipUpdatesSequentially() public {
        vm.roll(10);
        vm.warp(123);

        UntronController controller = new UntronController(0xff);

        bytes32 tip0 = sha256(
            abi.encodePacked(
                EventChainGenesis.UntronControllerIndex,
                block.number,
                block.timestamp,
                keccak256("OwnerChanged(address)"),
                abi.encode(address(this))
            )
        );
        assertEq(controller.eventChainTip(), tip0, "unexpected initial tip");

        vm.roll(11);
        vm.warp(456);

        MockERC20 usdt = new MockERC20("USDT", "USDT", 18);

        bytes[] memory calls = new bytes[](2);
        calls[0] = abi.encodeCall(controller.setExecutor, (address(0xE0)));
        calls[1] = abi.encodeCall(controller.setUsdt, (address(usdt)));

        controller.multicall(calls);

        bytes32 tip1 = sha256(
            abi.encodePacked(
                tip0, block.number, block.timestamp, keccak256("ExecutorChanged(address)"), abi.encode(address(0xE0))
            )
        );
        bytes32 tip2 = sha256(
            abi.encodePacked(
                tip1, block.number, block.timestamp, keccak256("UsdtSet(address)"), abi.encode(address(usdt))
            )
        );

        assertEq(controller.eventChainTip(), tip2, "multicall tip mismatch");
    }

    function _asArray(bytes32 salt) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = salt;
    }

    function _asArray(uint256 amount) internal pure returns (uint256[] memory arr) {
        arr = new uint256[](1);
        arr[0] = amount;
    }
}
