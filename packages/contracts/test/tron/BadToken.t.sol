// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {UntronReceiver} from "../../src/tron/UntronReceiver.sol";

import {ReturnFalseERC20} from "./mocks/ReturnFalseERC20.sol";

contract BadTokenTest is Test {
    function test_returnFalseToken_receiverPull_revertsAndDoesNotTransfer() public {
        UntronReceiver receiver = new UntronReceiver();
        ReturnFalseERC20 bad = new ReturnFalseERC20("Bad", "BAD", 18);

        bad.mint(address(receiver), 100);

        vm.expectRevert(SafeTransferLib.TransferFailed.selector);
        receiver.pull(address(bad), 99);

        assertEq(bad.balanceOf(address(this)), 0, "controller should not receive tokens");
        assertEq(bad.balanceOf(address(receiver)), 100, "receiver balance should be unchanged");
    }

    function test_returnFalseToken_controllerPullFromReceivers_revertsAndDoesNotDeploy() public {
        UntronController controller = new UntronController(0xff);
        ReturnFalseERC20 bad = new ReturnFalseERC20("Bad", "BAD", 18);

        controller.setUsdt(address(bad));

        bytes32 salt = keccak256("bad-token");
        address predictedReceiver = controller.predictReceiverAddress(salt);
        bad.mint(predictedReceiver, 100); // expected sweep = 99

        vm.expectRevert(SafeTransferLib.TransferFailed.selector);
        controller.pullFromReceivers(address(bad), _asArray(salt), _asArray(uint256(99)), 0);

        assertEq(predictedReceiver.code.length, 0, "receiver deployment should revert");
        assertEq(bad.balanceOf(address(controller)), 0, "controller should not receive tokens");
        assertEq(bad.balanceOf(predictedReceiver), 100, "predicted receiver balance should be unchanged");
        assertEq(controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");
    }

    function _asArray(bytes32 value) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = value;
    }

    function _asArray(uint256 value) internal pure returns (uint256[] memory arr) {
        arr = new uint256[](1);
        arr[0] = value;
    }
}

