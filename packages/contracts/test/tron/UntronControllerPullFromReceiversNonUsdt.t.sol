// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {UntronController} from "../../src/tron/UntronController.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract UntronControllerPullFromReceiversNonUsdtTest is Test {
    bytes32 private constant _SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");

    UntronController internal _controller;
    MockERC20 internal _usdt;
    MockERC20 internal _tokenX;

    address internal constant _LP = address(0xB0B);

    function setUp() public {
        _controller = new UntronController(0xff);
        _usdt = new MockERC20("USDT", "USDT", 18);
        _tokenX = new MockERC20("TOKEN_X", "TKX", 18);

        _controller.setUsdt(address(_usdt));
        _controller.setLp(_LP);
    }

    function test_exchangeRateMismatch_revertsWhenUnconfigured() public {
        bytes32 salt = keccak256("tokenX-unconfigured");
        address receiver = _controller.predictReceiverAddress(salt);

        _tokenX.mint(receiver, 100); // sweep 99

        vm.expectRevert(UntronController.ExchangeRateMismatch.selector);
        _controller.pullFromReceivers(address(_tokenX), _asArray(salt));

        assertEq(receiver.code.length, 0, "receiver should remain undeployed");
        assertEq(_tokenX.balanceOf(address(_controller)), 0, "controller should not receive token");
        assertEq(_controller.pulledUsdt(), 0, "pulledUsdt should remain unchanged");
    }

    function test_insufficientLpLiquidity_reverts() public {
        vm.prank(_LP);
        _controller.setLpExchangeRate(address(_tokenX), 1e18);

        // Create a non-zero pulledUsdt balance via a prior USDT sweep: pulledUsdt = 900.
        _doUsdtPull(900);
        // Deposit 100 additional USDT as LP liquidity, so free liquidity is 100.
        _lpDepositUsdt(100);

        assertEq(_usdt.balanceOf(address(_controller)), 1_000, "unexpected controller USDT balance");
        assertEq(_controller.pulledUsdt(), 900, "unexpected pulledUsdt");

        bytes32 salt = keccak256("tokenX-liquidity");
        address receiver = _controller.predictReceiverAddress(salt);

        // Make totalUsdt = 101 > lpFreeUsdt = 100.
        uint256 sweepAmount = 101;
        _tokenX.mint(receiver, sweepAmount + 1);

        vm.expectRevert(UntronController.InsufficientLpLiquidity.selector);
        _controller.pullFromReceivers(address(_tokenX), _asArray(salt));

        assertEq(receiver.code.length, 0, "receiver deployment should revert");
        assertEq(_tokenX.balanceOf(address(_controller)), 0, "controller should not receive token");
        assertEq(_controller.pulledUsdt(), 900, "pulledUsdt should remain unchanged");
    }

    function test_nonUsdtPull_increasesPulledUsdt_withoutMovingUsdt() public {
        uint256 rate = 2e18;
        vm.prank(_LP);
        _controller.setLpExchangeRate(address(_tokenX), rate);

        _lpDepositUsdt(1_000);

        bytes32 salt = keccak256("tokenX-success");
        address receiver = _controller.predictReceiverAddress(salt);

        _tokenX.mint(receiver, 11); // sweep 10

        uint256 usdtBefore = _usdt.balanceOf(address(_controller));
        uint256 pulledBefore = _controller.pulledUsdt();

        vm.recordLogs();
        _controller.pullFromReceivers(address(_tokenX), _asArray(salt));
        Vm.Log[] memory logs = vm.getRecordedLogs();

        assertEq(_tokenX.balanceOf(address(_controller)), 10, "controller should receive swept token");
        assertEq(_tokenX.balanceOf(receiver), 1, "receiver should keep dust");
        assertEq(_usdt.balanceOf(address(_controller)), usdtBefore, "USDT balance should not move");
        assertEq(_controller.pulledUsdt(), pulledBefore + 20, "pulledUsdt should increase by converted amount");

        // Sanity: PulledFromReceiver logs exchangeRate and computed usdtAmount.
        Vm.Log memory pulledLog = _findSingleControllerLog(logs, _SIG_PULLED_FROM_RECEIVER);
        (, uint256 exchangeRate, uint256 usdtAmount) = abi.decode(pulledLog.data, (uint256, uint256, uint256));
        assertEq(exchangeRate, rate, "event exchangeRate mismatch");
        assertEq(usdtAmount, 20, "event usdtAmount mismatch");
    }

    function test_lpWithdrawToken_doesNotAffectPulledUsdt() public {
        _setupSuccessfulNonUsdtPull();

        uint256 pulledBefore = _controller.pulledUsdt();

        vm.prank(_LP);
        _controller.lpWithdrawTokens(address(_tokenX), 10);

        assertEq(_tokenX.balanceOf(_LP), 10, "LP should receive purchased token");
        assertEq(_tokenX.balanceOf(address(_controller)), 0, "controller token balance should be withdrawn");
        assertEq(_controller.pulledUsdt(), pulledBefore, "pulledUsdt should be unchanged");
    }

    function test_lpWithdrawUsdt_onlyFromSurplus_andZeroIsNoop() public {
        _setupSuccessfulNonUsdtPull();

        uint256 surplus = _usdt.balanceOf(address(_controller)) - _controller.pulledUsdt();
        assertEq(surplus, 980, "unexpected USDT surplus");

        bytes32 tipBefore = _controller.eventChainTip();
        vm.prank(_LP);
        _controller.lpWithdrawTokens(address(_usdt), 0);
        assertEq(_controller.eventChainTip(), tipBefore, "withdrawing 0 should not emit or change tip");

        vm.prank(_LP);
        _controller.lpWithdrawTokens(address(_usdt), surplus);

        assertEq(_usdt.balanceOf(_LP), surplus, "LP should receive USDT surplus");
        assertEq(
            _usdt.balanceOf(address(_controller)), _controller.pulledUsdt(), "controller should keep accounted USDT"
        );

        vm.prank(_LP);
        vm.expectRevert(UntronController.InsufficientPulledAmount.selector);
        _controller.lpWithdrawTokens(address(_usdt), 1);
    }

    function test_usdtAndNonUsdtPull_interplay_respectsLpFreeUsdt() public {
        vm.prank(_LP);
        _controller.setLpExchangeRate(address(_tokenX), 1e18);

        _doUsdtPull(100); // pulledUsdt=100, USDT balance=100.
        _lpDepositUsdt(100); // USDT balance=200, free=100.

        // Pull tokenX worth exactly 100 USDT.
        bytes32 s1 = keccak256("tokenX-100");
        address r1 = _controller.predictReceiverAddress(s1);
        _tokenX.mint(r1, 101); // sweep 100

        _controller.pullFromReceivers(address(_tokenX), _asArray(s1));

        assertEq(_controller.pulledUsdt(), 200, "pulledUsdt should increase by 100");
        assertEq(_usdt.balanceOf(address(_controller)), 200, "USDT balance should remain");

        // Now free liquidity is zero; any further non-USDT pull should revert.
        bytes32 s2 = keccak256("tokenX-1");
        address r2 = _controller.predictReceiverAddress(s2);
        _tokenX.mint(r2, 2); // sweep 1

        vm.expectRevert(UntronController.InsufficientLpLiquidity.selector);
        _controller.pullFromReceivers(address(_tokenX), _asArray(s2));
    }

    function _setupSuccessfulNonUsdtPull() internal {
        uint256 rate = 2e18;
        vm.prank(_LP);
        _controller.setLpExchangeRate(address(_tokenX), rate);

        _lpDepositUsdt(1_000);

        bytes32 salt = keccak256("tokenX-success");
        address receiver = _controller.predictReceiverAddress(salt);
        _tokenX.mint(receiver, 11); // sweep 10

        _controller.pullFromReceivers(address(_tokenX), _asArray(salt));
    }

    function _doUsdtPull(uint256 sweepAmount) internal {
        bytes32 salt = keccak256(abi.encodePacked("usdt-pull", sweepAmount));
        address receiver = _controller.predictReceiverAddress(salt);
        _usdt.mint(receiver, sweepAmount + 1);
        _controller.pullFromReceivers(address(_usdt), _asArray(salt));
    }

    function _lpDepositUsdt(uint256 amount) internal {
        _usdt.mint(_LP, amount);
        vm.prank(_LP);
        bool ok = _usdt.transfer(address(_controller), amount);
        require(ok, "USDT transfer failed");
    }

    function _findSingleControllerLog(Vm.Log[] memory logs, bytes32 sig) internal returns (Vm.Log memory found) {
        bool hasFound = false;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter != address(_controller)) continue;
            if (logs[i].topics.length == 0 || logs[i].topics[0] != sig) continue;
            if (hasFound) fail("found multiple logs for signature");
            found = logs[i];
            hasFound = true;
        }
        if (!hasFound) fail("log not found");
    }

    function _asArray(bytes32 salt) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = salt;
    }
}
