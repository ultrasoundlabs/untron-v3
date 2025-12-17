// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {UntronReceiver} from "../../src/tron/UntronReceiver.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract UntronReceiverTest is Test {
    function test_pull_onlyByController_directDeploy() public {
        UntronReceiver receiver = new UntronReceiver();
        MockERC20 token = new MockERC20("Mock", "MOCK", 18);

        token.mint(address(receiver), 100);

        receiver.pull(address(token), 99);
        assertEq(token.balanceOf(address(this)), 99, "controller should receive tokens");
        assertEq(token.balanceOf(address(receiver)), 1, "receiver should keep dust");

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronReceiver.NotController.selector);
        receiver.pull(address(token), 1);
    }

    function test_pull_onlyByController_create2Receiver() public {
        UntronController controller = new UntronController(0xff);
        MockERC20 token = new MockERC20("Mock", "MOCK", 18);

        bytes32 salt = keccak256("receiver");
        address payable receiver = controller.deployReceiver(salt);

        token.mint(receiver, 100);

        vm.prank(address(controller));
        UntronReceiver(receiver).pull(address(token), 99);

        assertEq(token.balanceOf(address(controller)), 99, "controller should receive tokens");
        assertEq(token.balanceOf(receiver), 1, "receiver should keep dust");

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronReceiver.NotController.selector);
        UntronReceiver(receiver).pull(address(token), 1);
    }

    function test_pull_amountZero_noop() public {
        UntronReceiver receiver = new UntronReceiver();
        MockERC20 token = new MockERC20("Mock", "MOCK", 18);

        token.mint(address(receiver), 100);

        uint256 receiverBefore = token.balanceOf(address(receiver));
        uint256 controllerBefore = token.balanceOf(address(this));

        receiver.pull(address(token), 0);

        assertEq(token.balanceOf(address(receiver)), receiverBefore, "receiver balance should be unchanged");
        assertEq(token.balanceOf(address(this)), controllerBefore, "controller balance should be unchanged");
    }

    function test_receiver_acceptsNativeValue() public {
        UntronReceiver receiver = new UntronReceiver();

        vm.deal(address(this), 1 ether);
        (bool ok,) = payable(address(receiver)).call{value: 1 ether}("");
        assertTrue(ok, "native transfer failed");
        assertEq(address(receiver).balance, 1 ether, "receiver native balance should increase");
    }
}
