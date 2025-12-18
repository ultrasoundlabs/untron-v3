// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {UntronController} from "../../src/tron/UntronController.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract UntronControllerPullFromReceiversUsdtTest is Test {
    bytes32 private constant _SIG_RECEIVER_DEPLOYED = keccak256("ReceiverDeployed(address,bytes32)");
    bytes32 private constant _SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");

    UntronController internal _controller;
    MockERC20 internal _usdt;

    function setUp() public {
        _controller = new UntronController(0xff);
        _usdt = new MockERC20("USDT", "USDT", 18);
        _controller.setUsdt(address(_usdt));
    }

    function test_pullUsdt_fromUndeployedReceiver_deploysAndAccounts() public {
        bytes32 salt = keccak256("receiver-1");
        address predictedReceiver = _controller.predictReceiverAddress(salt);
        assertEq(predictedReceiver.code.length, 0, "receiver should not be deployed");

        uint256 initial = 1_000;
        _usdt.mint(predictedReceiver, initial);
        uint256 expectedSweep = initial - 1;

        vm.recordLogs();
        _controller.pullFromReceivers(address(_usdt), _asArray(salt), _asArray(expectedSweep), 123);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertGt(predictedReceiver.code.length, 0, "receiver should be deployed");
        assertEq(_usdt.balanceOf(address(_controller)), expectedSweep, "controller should receive swept USDT");
        assertEq(_usdt.balanceOf(predictedReceiver), 1, "receiver should keep 1 unit dust");
        assertEq(_controller.pulledUsdt(), expectedSweep, "pulledUsdt should increase by swept amount");

        Vm.Log[] memory controllerLogs = _controllerLogs(logs);
        assertEq(controllerLogs.length, 2, "expected exactly two controller logs");

        _assertReceiverDeployedLog(controllerLogs[0], predictedReceiver, salt);
        _assertPulledFromReceiverLog(controllerLogs[1], salt, address(_usdt), expectedSweep, 1e18, expectedSweep);
    }

    function test_pullUsdt_zeroBalance_noDeploy_noEvents_noAccounting() public {
        bytes32 salt = keccak256("receiver-0");
        address predictedReceiver = _controller.predictReceiverAddress(salt);
        assertEq(predictedReceiver.code.length, 0, "receiver should not be deployed");

        bytes32 tipBefore = _controller.eventChainTip();

        vm.recordLogs();
        _controller.pullFromReceivers(address(_usdt), _asArray(salt), _asArray(uint256(0)), 0);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(predictedReceiver.code.length, 0, "receiver should remain undeployed");
        assertEq(_usdt.balanceOf(address(_controller)), 0, "controller should not receive USDT");
        assertEq(_controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");
        assertEq(_controller.eventChainTip(), tipBefore, "tip should not change on no-op");

        Vm.Log[] memory controllerLogs = _controllerLogs(logs);
        assertEq(controllerLogs.length, 0, "no controller events expected");
    }

    function test_pullUsdt_incorrectSweepAmount_revertsAndDoesNotDeploy() public {
        bytes32 salt = keccak256("receiver-mismatch");
        address predictedReceiver = _controller.predictReceiverAddress(salt);

        _usdt.mint(predictedReceiver, 100);

        vm.expectRevert(UntronController.IncorrectSweepAmount.selector);
        _controller.pullFromReceivers(address(_usdt), _asArray(salt), _asArray(uint256(98)), 0);

        assertEq(predictedReceiver.code.length, 0, "receiver should remain undeployed");
        assertEq(_usdt.balanceOf(address(_controller)), 0, "controller should not receive USDT");
        assertEq(_controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");
        assertEq(_usdt.balanceOf(predictedReceiver), 100, "receiver balance should remain");
    }

    function test_pullUsdt_lengthMismatch_reverts() public {
        bytes32[] memory salts = new bytes32[](1);
        salts[0] = keccak256("a");
        uint256[] memory amounts = new uint256[](0);

        vm.expectRevert(UntronController.LengthMismatch.selector);
        _controller.pullFromReceivers(address(_usdt), salts, amounts, 0);
    }

    function test_pullUsdt_multipleReceivers_aggregatesAndLeavesDust() public {
        bytes32 s1 = keccak256("r1");
        bytes32 s2 = keccak256("r2");
        bytes32 s3 = keccak256("r3");

        address r1 = _controller.predictReceiverAddress(s1);
        address r2 = _controller.predictReceiverAddress(s2);
        address r3 = _controller.predictReceiverAddress(s3);

        _usdt.mint(r1, 11); // sweep 10
        _usdt.mint(r3, 21); // sweep 20

        bytes32[] memory salts = new bytes32[](3);
        salts[0] = s1;
        salts[1] = s2;
        salts[2] = s3;

        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 10;
        amounts[1] = 0;
        amounts[2] = 20;

        vm.recordLogs();
        _controller.pullFromReceivers(address(_usdt), salts, amounts, 0);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(_usdt.balanceOf(address(_controller)), 30, "controller USDT balance should equal total sweep");
        assertEq(_controller.pulledUsdt(), 30, "pulledUsdt should equal total sweep");

        assertGt(r1.code.length, 0, "r1 should be deployed");
        assertEq(_usdt.balanceOf(r1), 1, "r1 dust");

        assertEq(r2.code.length, 0, "r2 should remain undeployed");
        assertEq(_usdt.balanceOf(r2), 0, "r2 balance should remain 0");

        assertGt(r3.code.length, 0, "r3 should be deployed");
        assertEq(_usdt.balanceOf(r3), 1, "r3 dust");

        Vm.Log[] memory controllerLogs = _controllerLogs(logs);
        assertEq(controllerLogs.length, 4, "expected 2 events per non-zero receiver");

        _assertReceiverDeployedLog(controllerLogs[0], r1, s1);
        _assertPulledFromReceiverLog(controllerLogs[1], s1, address(_usdt), 10, 1e18, 10);
        _assertReceiverDeployedLog(controllerLogs[2], r3, s3);
        _assertPulledFromReceiverLog(controllerLogs[3], s3, address(_usdt), 20, 1e18, 20);
    }

    function test_pullUsdt_batchMismatch_revertsAll_orNothing() public {
        bytes32 s1 = keccak256("r1");
        bytes32 s2 = keccak256("r2");
        bytes32 s3 = keccak256("r3");

        address r1 = _controller.predictReceiverAddress(s1);
        address r2 = _controller.predictReceiverAddress(s2);
        address r3 = _controller.predictReceiverAddress(s3);

        _usdt.mint(r1, 11); // sweep 10
        _usdt.mint(r3, 21); // sweep 20

        bytes32[] memory salts = new bytes32[](3);
        salts[0] = s1;
        salts[1] = s2;
        salts[2] = s3;

        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 10;
        amounts[1] = 0;
        amounts[2] = 19; // mismatch: should be 20

        vm.expectRevert(UntronController.IncorrectSweepAmount.selector);
        _controller.pullFromReceivers(address(_usdt), salts, amounts, 0);

        assertEq(_usdt.balanceOf(address(_controller)), 0, "controller should not receive USDT");
        assertEq(_controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");

        assertEq(r1.code.length, 0, "r1 should not be deployed");
        assertEq(r2.code.length, 0, "r2 should not be deployed");
        assertEq(r3.code.length, 0, "r3 should not be deployed");
    }

    function _controllerLogs(Vm.Log[] memory logs) internal view returns (Vm.Log[] memory filtered) {
        uint256 count = 0;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter == address(_controller)) count++;
        }

        filtered = new Vm.Log[](count);
        uint256 j = 0;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter == address(_controller)) {
                filtered[j] = logs[i];
                j++;
            }
        }
    }

    function _assertReceiverDeployedLog(Vm.Log memory log, address receiver, bytes32 salt) internal pure {
        assertEq(log.topics.length, 2, "ReceiverDeployed topics length");
        assertEq(log.topics[0], _SIG_RECEIVER_DEPLOYED, "ReceiverDeployed sig");
        assertEq(address(uint160(uint256(log.topics[1]))), receiver, "ReceiverDeployed receiver");
        assertEq(abi.decode(log.data, (bytes32)), salt, "ReceiverDeployed salt");
    }

    function _assertPulledFromReceiverLog(
        Vm.Log memory log,
        bytes32 receiverSalt,
        address token,
        uint256 tokenAmount,
        uint256 exchangeRate,
        uint256 usdtAmount
    ) internal pure {
        assertEq(log.topics.length, 3, "PulledFromReceiver topics length");
        assertEq(log.topics[0], _SIG_PULLED_FROM_RECEIVER, "PulledFromReceiver sig");
        assertEq(log.topics[1], receiverSalt, "PulledFromReceiver receiverSalt");
        assertEq(address(uint160(uint256(log.topics[2]))), token, "PulledFromReceiver token");

        (uint256 gotTokenAmount, uint256 gotRate, uint256 gotUsdtAmount) =
            abi.decode(log.data, (uint256, uint256, uint256));
        assertEq(gotTokenAmount, tokenAmount, "PulledFromReceiver tokenAmount");
        assertEq(gotRate, exchangeRate, "PulledFromReceiver exchangeRate");
        assertEq(gotUsdtAmount, usdtAmount, "PulledFromReceiver usdtAmount");
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
