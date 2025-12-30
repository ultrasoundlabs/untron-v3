// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {EventChainGenesis} from "../../src/utils/EventChainGenesis.sol";

contract UntronControllerAccessControlTest is Test {
    UntronController internal _controller;

    address internal constant _LP = address(0xB0B);
    address internal constant _EXECUTOR = address(0xE0);

    function setUp() public {
        _controller = new UntronController(0xff);
    }

    function test_ownerSetOnDeploy_andTipMovesFromGenesis() public view {
        assertEq(_controller.owner(), address(this), "owner should be deployer");
        assertTrue(_controller.eventChainTip() != EventChainGenesis.UntronControllerIndex, "tip should advance once");
    }

    function test_onlyOwner_setExecutor() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.setExecutor(_EXECUTOR);
    }

    function test_onlyOwner_setUsdt() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.setUsdt(address(0x1000));
    }

    function test_onlyOwner_setLp() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.setLp(_LP);
    }

    function test_onlyOwner_setPayload() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.setPayload(address(0xCAFE), hex"1234");
    }

    function test_onlyOwner_setOwner() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.setOwner(address(0xA11CE));
    }

    function test_onlyOwner_approveUsdt() public {
        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyOwner.selector);
        _controller.approveUsdt(address(0xCAFE), 1);
    }

    function test_setOwner_rejectsZero() public {
        vm.expectRevert(UntronController.ZeroOwnerAddress.selector);
        _controller.setOwner(address(0));
    }

    function test_onlyLp_setLpExchangeRate() public {
        _controller.setLp(_LP);

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyLp.selector);
        _controller.setLpExchangeRate(address(0x1234), 1e18);
    }

    function test_onlyLp_lpWithdrawTokens() public {
        _controller.setLp(_LP);

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyLp.selector);
        _controller.lpWithdrawTokens(address(0x1234), 1);
    }

    function test_onlyExecutor_transferUsdtFromController() public {
        _controller.setExecutor(_EXECUTOR);

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronController.OnlyExecutor.selector);
        _controller.transferUsdtFromController(address(0xCAFE), 1);
    }
}
