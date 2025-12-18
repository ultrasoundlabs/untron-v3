// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";

import {MockERC20} from "./mocks/MockERC20.sol";
import {MockRebalancer} from "./mocks/MockRebalancer.sol";

contract UntronControllerRebalanceTest is Test {
    UntronController internal _controller;
    MockERC20 internal _usdt;
    MockRebalancer internal _rebalancer;

    address internal constant _SINK = address(0xBEEF);

    function setUp() public {
        _controller = new UntronController(0xff);
        _usdt = new MockERC20("USDT", "USDT", 18);
        _rebalancer = new MockRebalancer();

        _controller.setUsdt(address(_usdt));
    }

    function test_rebalance_routeNotSet_reverts() public {
        vm.expectRevert(UntronController.RouteNotSet.selector);
        _controller.rebalanceUsdt(address(_rebalancer), 0, 0);
    }

    function test_rebalance_cannotSpendMoreThanPulledUsdt() public {
        _doUsdtPull(10);
        _setRebalancerPayload(
            _config({outAmount: 0, sink: address(0), spendTokenInAmount: false, ethToSink: 0, shouldRevert: false})
        );

        vm.expectRevert(UntronController.InsufficientPulledAmount.selector);
        _controller.rebalanceUsdt(address(_rebalancer), 11, 0);
    }

    function test_rebalance_outAmountMismatch_revertsWithoutChangingAccounting() public {
        _doUsdtPull(100);
        _setRebalancerPayload(
            _config({outAmount: 123, sink: address(0), spendTokenInAmount: false, ethToSink: 0, shouldRevert: false})
        );

        uint256 pulledBefore = _controller.pulledUsdt();
        uint256 usdtBefore = _usdt.balanceOf(address(_controller));

        vm.expectRevert(UntronController.OutAmountMismatch.selector);
        _controller.rebalanceUsdt(address(_rebalancer), 10, 124);

        assertEq(_controller.pulledUsdt(), pulledBefore, "pulledUsdt should not change on revert");
        assertEq(_usdt.balanceOf(address(_controller)), usdtBefore, "USDT balance should not change on revert");
    }

    function test_rebalance_rebalancerRevert_bubbles() public {
        _doUsdtPull(100);
        _setRebalancerPayload(
            _config({outAmount: 0, sink: address(0), spendTokenInAmount: false, ethToSink: 0, shouldRevert: true})
        );

        uint256 pulledBefore = _controller.pulledUsdt();

        vm.expectRevert(MockRebalancer.RebalanceReverted.selector);
        _controller.rebalanceUsdt(address(_rebalancer), 10, 0);

        assertEq(_controller.pulledUsdt(), pulledBefore, "pulledUsdt should not change on revert");
    }

    function test_rebalance_delegatecall_canSpendControllerUsdtAndValue() public {
        _doUsdtPull(100);

        MockRebalancer.Config memory config =
            _config({outAmount: 777, sink: _SINK, spendTokenInAmount: true, ethToSink: 0.4 ether, shouldRevert: false});
        _setRebalancerPayload(config);

        vm.deal(address(this), 1 ether);

        uint256 controllerEthBefore = address(_controller).balance;
        uint256 sinkEthBefore = _SINK.balance;

        _controller.rebalanceUsdt{value: 1 ether}(address(_rebalancer), 50, 777);

        assertEq(_controller.pulledUsdt(), 50, "pulledUsdt should decrement by inAmount");
        assertEq(_usdt.balanceOf(_SINK), 50, "sink should receive USDT");
        assertEq(_usdt.balanceOf(address(_controller)), 50, "controller should have remaining USDT");

        assertEq(address(_controller).balance, controllerEthBefore + 0.6 ether, "controller should keep unspent ETH");
        assertEq(_SINK.balance, sinkEthBefore + 0.4 ether, "sink should receive ETH spent by rebalancer");
    }

    function test_rebalance_accountingUpdatesEvenIfRebalancerDoesNothing() public {
        _doUsdtPull(100);

        _setRebalancerPayload(
            _config({outAmount: 999, sink: address(0), spendTokenInAmount: false, ethToSink: 0, shouldRevert: false})
        );

        uint256 usdtBefore = _usdt.balanceOf(address(_controller));

        _controller.rebalanceUsdt(address(_rebalancer), 40, 999);

        assertEq(_controller.pulledUsdt(), 60, "pulledUsdt should decrement even if rebalancer does nothing");
        assertEq(_usdt.balanceOf(address(_controller)), usdtBefore, "USDT balance should not change");
    }

    function _doUsdtPull(uint256 sweepAmount) internal {
        bytes32 salt = keccak256(abi.encodePacked("usdt-pull", sweepAmount));
        address receiver = _controller.predictReceiverAddress(salt);
        _usdt.mint(receiver, sweepAmount + 1);
        _controller.pullFromReceivers(address(_usdt), _asArray(salt), _asArray(sweepAmount), 0);
    }

    function _setRebalancerPayload(MockRebalancer.Config memory config) internal {
        _controller.setPayload(address(_rebalancer), abi.encode(config));
    }

    function _config(uint256 outAmount, address sink, bool spendTokenInAmount, uint256 ethToSink, bool shouldRevert)
        internal
        pure
        returns (MockRebalancer.Config memory config)
    {
        config = MockRebalancer.Config({
            outAmount: outAmount,
            sink: sink,
            spendTokenInAmount: spendTokenInAmount,
            ethToSink: ethToSink,
            shouldRevert: shouldRevert
        });
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
