// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract UntronControllerAccountingTest is Test {
    UntronController internal _controller;
    MockERC20 internal _usdt;

    address internal constant _EXECUTOR = address(0xE0);
    address internal constant _LP = address(0xB0B);
    address internal constant _RECIPIENT = address(0xCAFE);

    function setUp() public {
        _controller = new UntronController(0xff);
        _usdt = new MockERC20("USDT", "USDT", 18);

        _controller.setUsdt(address(_usdt));
        _controller.setExecutor(_EXECUTOR);
        _controller.setLp(_LP);
    }

    function test_transferUsdtFromController_cannotSpendMoreThanPulledUsdt() public {
        _doUsdtPull(100);

        vm.prank(_EXECUTOR);
        vm.expectRevert(UntronController.InsufficientPulledAmount.selector);
        _controller.transferUsdtFromController(_RECIPIENT, 101);
    }

    function test_transferUsdtFromController_decrementsPulledUsdtExactly() public {
        _doUsdtPull(100);

        vm.prank(_EXECUTOR);
        _controller.transferUsdtFromController(_RECIPIENT, 40);

        assertEq(_controller.pulledUsdt(), 60, "pulledUsdt should decrease by spend amount");
        assertEq(_usdt.balanceOf(_RECIPIENT), 40, "recipient should receive USDT");
        assertEq(_usdt.balanceOf(address(_controller)), 60, "controller should have remaining USDT");
    }

    function test_maxWithdrawableUsdt_equalsBalanceMinusPulled_andDetectsBrokenInvariant() public {
        _doUsdtPull(100); // controller balance=100, pulledUsdt=100.

        // Add surplus USDT and ensure LP can withdraw exactly the surplus.
        _usdt.mint(_LP, 25);
        vm.prank(_LP);
        bool ok = _usdt.transfer(address(_controller), 25);
        require(ok, "USDT transfer failed");

        uint256 surplus = _usdt.balanceOf(address(_controller)) - _controller.pulledUsdt();
        assertEq(surplus, 25, "unexpected surplus");

        vm.prank(_LP);
        _controller.lpWithdrawTokens(address(_usdt), 25);
        assertEq(_usdt.balanceOf(_LP), 25, "LP should receive surplus");

        vm.prank(_LP);
        vm.expectRevert(UntronController.InsufficientPulledAmount.selector);
        _controller.lpWithdrawTokens(address(_usdt), 1);

        // Break invariant by burning USDT from the controller without updating accounting.
        _usdt.burn(address(_controller), 1);
        assertLt(_usdt.balanceOf(address(_controller)), _controller.pulledUsdt(), "invariant should be broken");

        vm.prank(_LP);
        vm.expectRevert(UntronController.InsufficientPulledAmount.selector);
        _controller.lpWithdrawTokens(address(_usdt), 1);
    }

    function _doUsdtPull(uint256 sweepAmount) internal {
        bytes32 salt = keccak256(abi.encodePacked("usdt-pull", sweepAmount));
        address receiver = _controller.predictReceiverAddress(salt);
        _usdt.mint(receiver, sweepAmount + 1);
        _controller.pullFromReceivers(address(_usdt), _asArray(salt), _asArray(sweepAmount), 0);
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
