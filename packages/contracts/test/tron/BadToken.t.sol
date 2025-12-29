// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {TronTokenUtils} from "../../src/utils/TronTokenUtils.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {UntronReceiver} from "../../src/tron/UntronReceiver.sol";

import {RevertERC20} from "./mocks/RevertERC20.sol";

contract BadTokenTest is Test {
    function test_revertingToken_receiverPull_revertsAndDoesNotTransfer() public {
        UntronReceiver receiver = new UntronReceiver();
        RevertERC20 bad = new RevertERC20("Bad", "BAD", 18);

        bad.mint(address(receiver), 100);

        vm.expectRevert(TronTokenUtils.Trc20CallFailed.selector);
        receiver.pull(address(bad), 99);

        assertEq(bad.balanceOf(address(this)), 0, "controller should not receive tokens");
        assertEq(bad.balanceOf(address(receiver)), 100, "receiver balance should be unchanged");
    }

    function test_revertingToken_controllerPullFromReceivers_revertsAndDoesNotDeploy() public {
        UntronController controller = new UntronController(0xff);
        RevertERC20 bad = new RevertERC20("Bad", "BAD", 18);

        controller.setUsdt(address(bad));

        bytes32 salt = keccak256("bad-token");
        address predictedReceiver = controller.predictReceiverAddress(salt);
        bad.mint(predictedReceiver, 100); // expected sweep = 99

        vm.expectRevert(TronTokenUtils.Trc20CallFailed.selector);
        controller.pullFromReceivers(address(bad), _asArray(salt));

        assertEq(predictedReceiver.code.length, 0, "receiver deployment should revert");
        assertEq(bad.balanceOf(address(controller)), 0, "controller should not receive tokens");
        assertEq(bad.balanceOf(predictedReceiver), 100, "predicted receiver balance should be unchanged");
        assertEq(controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");
    }

    function _asArray(bytes32 value) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = value;
    }
}
