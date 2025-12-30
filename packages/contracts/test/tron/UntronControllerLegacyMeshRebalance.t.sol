// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {LegacyMeshRebalancer, ILegacyMeshOFT} from "../../src/tron/rebalancers/LegacyMeshRebalancer.sol";

import {MockLegacyMeshOFT} from "./mocks/MockLegacyMeshOFT.sol";
import {TronUsdtLikeERC20} from "./mocks/TronUsdtLikeERC20.sol";
import {IERC20Errors} from "openzeppelin-contracts/contracts/interfaces/draft-IERC6093.sol";

contract UntronControllerLegacyMeshRebalanceTest is Test {
    bytes32 private constant _SIG_USDT_REBALANCED = keccak256("UsdtRebalanced(uint256,uint256,address)");

    UntronController internal _controller;
    TronUsdtLikeERC20 internal _usdt;
    LegacyMeshRebalancer internal _rebalancer;
    MockLegacyMeshOFT internal _oft;

    uint32 internal constant _DST_EID = 101;
    bytes32 internal constant _TO = bytes32(uint256(uint160(address(0xB0B))));

    function setUp() public {
        _controller = new UntronController(0xff);
        _usdt = new TronUsdtLikeERC20("USDT", "USDT", 18);
        _rebalancer = new LegacyMeshRebalancer();
        _oft = new MockLegacyMeshOFT(_usdt);

        _controller.setUsdt(address(_usdt));

        _oft.setFeeBps(100); // 1%
        _oft.setQuoteSendFee(0.3 ether, 0);

        _controller.setPayload(address(_rebalancer), abi.encode(ILegacyMeshOFT(address(_oft)), _DST_EID, _TO));
    }

    function test_legacyMeshRebalance_endToEnd_updatesAccounting_spendsUsdtAndNativeFee() public {
        _doUsdtPull(1000);

        uint256 inAmount = 400;
        uint256 expectedFee = inAmount * 100 / 10_000;
        uint256 expectedOut = inAmount - expectedFee;

        _controller.approveUsdt(address(_oft), 1000);
        assertEq(_usdt.allowance(address(_controller), address(_oft)), 1000, "allowance should be set by owner");

        vm.deal(address(this), 1 ether);

        uint256 pulledBefore = _controller.pulledUsdt();
        uint256 controllerUsdtBefore = _usdt.balanceOf(address(_controller));
        uint256 oftUsdtBefore = _usdt.balanceOf(address(_oft));
        uint256 controllerEthBefore = address(_controller).balance;
        uint256 oftEthBefore = address(_oft).balance;
        uint256 allowanceBefore = _usdt.allowance(address(_controller), address(_oft));

        vm.recordLogs();
        _controller.rebalanceUsdt{value: 1 ether}(address(_rebalancer), inAmount);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        Vm.Log memory log = _findSingleControllerLog(logs, _SIG_USDT_REBALANCED);
        (uint256 eventIn, uint256 eventOut) = abi.decode(log.data, (uint256, uint256));
        assertEq(eventIn, inAmount, "event inAmount mismatch");
        assertEq(eventOut, expectedOut, "event outAmount mismatch");
        assertEq(log.topics[1], bytes32(uint256(uint160(address(_rebalancer)))), "event rebalancer mismatch");

        assertEq(_controller.pulledUsdt(), pulledBefore - inAmount, "pulledUsdt should decrement by inAmount");

        assertEq(
            _usdt.balanceOf(address(_controller)),
            controllerUsdtBefore - inAmount,
            "controller USDT should be debited by OFT"
        );
        assertEq(_usdt.balanceOf(address(_oft)), oftUsdtBefore + inAmount, "OFT should receive USDT");

        assertEq(_usdt.allowance(address(_controller), address(_oft)), allowanceBefore - inAmount, "allowance mismatch");

        assertEq(address(_controller).balance, controllerEthBefore + 0.7 ether, "controller should keep unspent ETH");
        assertEq(address(_oft).balance, oftEthBefore + 0.3 ether, "OFT should receive native fee");

        assertEq(_oft.lastDstEid(), _DST_EID, "dstEid mismatch");
        assertEq(_oft.lastTo(), _TO, "to mismatch");
        assertEq(_oft.lastAmountLD(), inAmount, "amountLD mismatch");
        assertEq(_oft.lastMinAmountLD(), expectedOut, "minAmountLD mismatch");
        assertEq(_oft.lastRefundAddress(), address(_controller), "refundAddress mismatch");
        assertEq(_oft.lastMsgValue(), 0.3 ether, "msg.value mismatch");

        assertEq(controllerUsdtBefore, pulledBefore, "controller USDT should match pulledUsdt before");
    }

    function test_legacyMeshRebalance_withoutApproval_revertsAndDoesNotDecrementPulledUsdt() public {
        _doUsdtPull(1000);

        uint256 inAmount = 400;
        uint256 pulledBefore = _controller.pulledUsdt();

        vm.deal(address(this), 1 ether);

        vm.expectRevert(
            abi.encodeWithSelector(IERC20Errors.ERC20InsufficientAllowance.selector, address(_oft), 0, inAmount)
        );
        _controller.rebalanceUsdt{value: 1 ether}(address(_rebalancer), inAmount);

        assertEq(_controller.pulledUsdt(), pulledBefore, "pulledUsdt should not change on revert");
    }

    function test_approveUsdt_retriesWhenAllowanceMustBeZeroFirst() public {
        UntronController controller = new UntronController(0xff);
        TronUsdtLikeERC20 usdt = new TronUsdtLikeERC20("USDT", "USDT", 6);
        controller.setUsdt(address(usdt));

        address spender = address(0xCAFE);
        controller.approveUsdt(spender, 123);
        controller.approveUsdt(spender, 456);

        assertEq(usdt.allowance(address(controller), spender), 456, "allowance should update via retry flow");
    }

    function _doUsdtPull(uint256 sweepAmount) internal {
        bytes32 salt = keccak256(abi.encodePacked("usdt-pull", sweepAmount));
        address receiver = _controller.predictReceiverAddress(salt);
        _usdt.mint(receiver, sweepAmount + 1);
        _controller.pullFromReceivers(address(_usdt), _asArray(salt));
    }

    function _asArray(bytes32 salt) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = salt;
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
}
