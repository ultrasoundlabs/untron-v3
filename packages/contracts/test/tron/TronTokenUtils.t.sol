// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {TronTokenUtils as TokenUtils} from "../../src/utils/TronTokenUtils.sol";

import {MockERC20} from "./mocks/MockERC20.sol";
import {ReturnFalseERC20} from "./mocks/ReturnFalseERC20.sol";
import {RevertERC20} from "./mocks/RevertERC20.sol";
import {ApproveRequiresZeroERC20} from "./mocks/ApproveRequiresZeroERC20.sol";

contract RejectTrxReceiver {
    receive() external payable {
        revert("NO_TRX");
    }
}

contract EmptyFallbackNoReturnData {
    fallback() external payable {}
}

contract TronTokenUtilsHarness {
    function getBalanceOf(address token, address account) external view returns (uint256) {
        return TokenUtils.getBalanceOf(token, account);
    }

    function balanceOf(address token, address account) external view returns (uint256) {
        return TokenUtils.balanceOf(token, account);
    }

    function transfer(address token, address payable recipient, uint256 amount) external {
        TokenUtils.transfer(token, recipient, amount);
    }

    function transferFrom(address token, address from, address payable recipient, uint256 amount) external {
        TokenUtils.transferFrom(token, from, recipient, amount);
    }

    function approve(address token, address spender, uint256 amount) external {
        TokenUtils.approve(token, spender, amount);
    }
}

contract TronTokenUtilsTest is Test {
    address internal constant _ALICE = address(0xA11CE);
    address internal constant _BOB = address(0xB0B);
    TronTokenUtilsHarness internal _h;

    function setUp() public {
        _h = new TronTokenUtilsHarness();
    }

    function test_getBalanceOf_trx_readsNativeBalance() public {
        vm.deal(_ALICE, 123);
        assertEq(_h.getBalanceOf(address(0), _ALICE), 123);
    }

    function test_getBalanceOf_trc20_readsTokenBalance() public {
        MockERC20 token = new MockERC20("Mock", "MOCK", 18);
        token.mint(_ALICE, 999);
        assertEq(_h.getBalanceOf(address(token), _ALICE), 999);
    }

    function test_balanceOf_missingFunction_reverts() public {
        EmptyFallbackNoReturnData token = new EmptyFallbackNoReturnData();
        vm.expectRevert(TokenUtils.Trc20BadReturnData.selector);
        _h.balanceOf(address(token), _ALICE);
    }

    function test_transfer_trc20_returnFalse_stillTransfers() public {
        ReturnFalseERC20 token = new ReturnFalseERC20("USDT", "USDT", 6);
        token.mint(address(_h), 50);

        _h.transfer(address(token), payable(_BOB), 12);

        assertEq(token.balanceOf(_BOB), 12);
        assertEq(token.balanceOf(address(_h)), 38);
    }

    function test_transfer_trc20_revert_isFailure() public {
        RevertERC20 token = new RevertERC20("Bad", "BAD", 18);
        token.mint(address(_h), 50);

        vm.expectRevert(TokenUtils.Trc20CallFailed.selector);
        _h.transfer(address(token), payable(_BOB), 12);

        assertEq(token.balanceOf(_BOB), 0);
        assertEq(token.balanceOf(address(_h)), 50);
    }

    function test_transfer_trx_succeeds() public {
        vm.deal(address(_h), 1 ether);
        _h.transfer(address(0), payable(_BOB), 0.2 ether);
        assertEq(_BOB.balance, 0.2 ether);
    }

    function test_transfer_trx_receiverRejects_reverts() public {
        vm.deal(address(this), 1 ether);
        RejectTrxReceiver recv = new RejectTrxReceiver();
        vm.expectRevert(TokenUtils.TrxTransferFailed.selector);
        _h.transfer(address(0), payable(address(recv)), 1);
    }

    function test_transferFrom_trx_isNoop() public {
        vm.deal(_ALICE, 1 ether);
        vm.deal(_BOB, 2 ether);

        _h.transferFrom(address(0), _ALICE, payable(_BOB), 123);

        assertEq(_ALICE.balance, 1 ether);
        assertEq(_BOB.balance, 2 ether);
    }

    function test_approve_trx_isNoop() public {
        _h.approve(address(0), _BOB, 123);
    }

    function test_approve_trc20_retriesZeroFirst() public {
        ApproveRequiresZeroERC20 token = new ApproveRequiresZeroERC20("USDT-like", "USDTL", 6);

        _h.approve(address(token), _BOB, 1);
        assertEq(token.allowance(address(_h), _BOB), 1);

        // Second approve to non-zero would revert without a 0-reset.
        _h.approve(address(token), _BOB, 2);
        assertEq(token.allowance(address(_h), _BOB), 2);
    }
}
