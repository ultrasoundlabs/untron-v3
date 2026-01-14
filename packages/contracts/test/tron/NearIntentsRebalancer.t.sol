// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {IRebalancer} from "../../src/tron/rebalancers/interfaces/IRebalancer.sol";
import {NearIntentsRebalancer} from "../../src/tron/rebalancers/NearIntentsRebalancer.sol";
import {TronTokenUtils} from "../../src/utils/TronTokenUtils.sol";

import {TronUsdtLikeERC20} from "./mocks/TronUsdtLikeERC20.sol";
import {RevertERC20} from "./mocks/RevertERC20.sol";

contract NearIntentsRebalancerTest is Test {
    address payable internal constant _POOL = payable(address(0xBEEF));
    uint256 internal constant _IN_AMOUNT = 123;

    NearIntentsRebalancer internal _rebalancer;

    function setUp() public {
        _rebalancer = new NearIntentsRebalancer();
    }

    function test_rebalance_payloadMissing_reverts() public {
        TronUsdtLikeERC20 usdt = new TronUsdtLikeERC20("USDT", "USDT", 6);
        usdt.mint(address(this), _IN_AMOUNT);

        vm.expectRevert();
        _delegateRebalance(address(_rebalancer), address(usdt), _IN_AMOUNT, "");
    }

    function test_rebalance_poolZero_reverts() public {
        TronUsdtLikeERC20 usdt = new TronUsdtLikeERC20("USDT", "USDT", 6);
        usdt.mint(address(this), _IN_AMOUNT);

        vm.expectRevert(NearIntentsRebalancer.InvalidPool.selector);
        _delegateRebalance(address(_rebalancer), address(usdt), _IN_AMOUNT, abi.encode(payable(address(0))));
    }

    function test_rebalance_delegatecall_transfersToPool_andReturnsInAmount() public {
        TronUsdtLikeERC20 usdt = new TronUsdtLikeERC20("USDT", "USDT", 6);
        usdt.mint(address(this), _IN_AMOUNT);

        uint256 outAmount = _delegateRebalance(address(_rebalancer), address(usdt), _IN_AMOUNT, abi.encode(_POOL));

        assertEq(outAmount, _IN_AMOUNT, "outAmount mismatch");
        assertEq(usdt.balanceOf(_POOL), _IN_AMOUNT, "pool should receive tokens");
        assertEq(usdt.balanceOf(address(this)), 0, "caller should spend tokens");
    }

    function test_rebalance_delegatecall_acceptsValue_andDoesNotTransferNativeToPool() public {
        TronUsdtLikeERC20 usdt = new TronUsdtLikeERC20("USDT", "USDT", 6);
        usdt.mint(address(this), _IN_AMOUNT);

        vm.deal(address(this), 1 ether);
        uint256 poolEthBefore = _POOL.balance;
        uint256 thisEthBefore = address(this).balance;

        uint256 outAmount = this.delegateRebalanceWithValue{value: 0.2 ether}(
            address(_rebalancer), address(usdt), _IN_AMOUNT, abi.encode(_POOL)
        );

        assertEq(outAmount, _IN_AMOUNT, "outAmount mismatch");
        assertEq(_POOL.balance, poolEthBefore, "pool should not receive native token");
        assertEq(address(this).balance, thisEthBefore, "caller native balance should not change");
        assertEq(usdt.balanceOf(_POOL), _IN_AMOUNT, "pool should receive tokens");
    }

    function test_rebalance_delegatecall_transferFailure_reverts() public {
        RevertERC20 token = new RevertERC20("Bad", "BAD", 18);
        token.mint(address(this), _IN_AMOUNT);

        vm.expectRevert(TronTokenUtils.Trc20CallFailed.selector);
        _delegateRebalance(address(_rebalancer), address(token), _IN_AMOUNT, abi.encode(_POOL));
    }

    function delegateRebalanceWithValue(address impl, address token, uint256 inAmount, bytes calldata payload)
        external
        payable
        returns (uint256)
    {
        return _delegateRebalance(impl, token, inAmount, payload);
    }

    function _delegateRebalance(address impl, address token, uint256 inAmount, bytes memory payload)
        internal
        returns (uint256 outAmount)
    {
        bytes memory callData = abi.encodeCall(IRebalancer.rebalance, (token, inAmount, payload));
        (bool ok, bytes memory ret) = impl.delegatecall(callData);
        if (!ok) {
            // solhint-disable-next-line no-inline-assembly
            assembly {
                revert(add(ret, 0x20), mload(ret))
            }
        }
        outAmount = abi.decode(ret, (uint256));
    }
}
