// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

import {Call} from "../../../src/evm/SwapExecutor.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {UntronV3Index} from "../../../src/evm/UntronV3Index.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";
import {MockERC20} from "./UntronV3TestUtils.sol";

contract UntronV3AdminAndPauseTest is UntronV3TestBase {
    function testSetUsdtUpdatesAccountingTokenAndTip() public {
        address newUsdt = address(new MockERC20("USDT2", "USDT2", 6));

        bytes32 tipBefore = _untron.eventChainTip();
        vm.roll(100);
        vm.warp(1_700_000_000);

        vm.expectEmit(true, false, false, false, address(_untron));
        emit UntronV3Index.UsdtSet(newUsdt);
        _untron.setUsdt(newUsdt);

        assertEq(_untron.usdt(), newUsdt);

        bytes32 expectedTip = sha256(
            abi.encodePacked(
                tipBefore, uint256(100), uint256(1_700_000_000), UntronV3Index.UsdtSet.selector, abi.encode(newUsdt)
            )
        );
        assertEq(_untron.eventChainTip(), expectedTip);
    }

    function testPauseGatesEntrypointsAndUnpauseRestores() public {
        _untron.pause();

        bytes32 salt = keccak256("salt_pause");
        Call[] memory noCalls = new Call[](0);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.createLease(
            salt, address(0xBEEF), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.fill(address(_usdt), 0, noCalls);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.deposit(1);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.withdraw(1);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.relayControllerEventChain(1, hex"", new bytes32[](0), 0, new UntronV3.ControllerEvent[](0));

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.processControllerEvents(1);

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.setPayoutConfig(1, block.chainid, address(_usdt), address(0x1234));

        vm.expectRevert(Pausable.EnforcedPause.selector);
        _untron.setPayoutConfigWithSig(
            1,
            UntronV3.PayoutConfig({
                targetChainId: block.chainid, targetToken: address(_usdt), beneficiary: address(0x1234)
            }),
            block.timestamp + 1 days,
            hex""
        );

        _untron.unpause();

        uint256 leaseId = _untron.createLease(
            salt, address(0xBEEF), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );
        assertEq(leaseId, 1);

        _usdt.mint(address(this), 10);
        _usdt.approve(address(_untron), 10);
        _untron.deposit(10);
        assertEq(_untron.lpPrincipal(address(this)), 10);
    }

    function testRescueTokensCannotRescueUsdt() public {
        MockERC20 other = new MockERC20("OTHER", "OTHER", 18);
        other.mint(address(_untron), 5);
        _usdt.mint(address(_untron), 7);

        _untron.rescueTokens(address(other), 5);
        assertEq(other.balanceOf(address(this)), 5);

        vm.expectRevert(UntronV3.CannotRescueUSDT.selector);
        _untron.rescueTokens(address(_usdt), 1);
    }

    function testWithdrawProtocolProfitBounds() public {
        // Provide actual USDT liquidity for the withdrawal.
        _usdt.mint(address(_untron), 100);

        bytes32 salt = keccak256("salt_profit");
        _createLease(
            salt,
            address(this),
            uint64(block.timestamp + 1 days),
            10_000, // 1% fee
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);
        _reader.setNextCallData(
            keccak256("tx_profit"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
        _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);
        assertEq(_untron.protocolPnl(), 1);

        vm.expectRevert(UntronV3.ZeroAmount.selector);
        _untron.withdrawProtocolProfit(0);

        vm.expectRevert(UntronV3.InsufficientProtocolProfit.selector);
        _untron.withdrawProtocolProfit(2);

        uint256 ownerBalBefore = _usdt.balanceOf(address(this));
        _untron.withdrawProtocolProfit(1);
        assertEq(_untron.protocolPnl(), 0);
        assertEq(_usdt.balanceOf(address(this)), ownerBalBefore + 1);
    }

    function testChainDeprecatedBlocksCreateLeaseAndPayoutConfigUpdate() public {
        bytes32 salt = keccak256("salt_chain_deprecated");
        uint256 leaseId = _createLease(
            salt, address(0xBEEF), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        _untron.setChainDeprecated(block.chainid, true);

        vm.expectRevert(UntronV3.ChainDeprecated.selector);
        _untron.createLease(
            keccak256("salt_chain_deprecated_2"),
            address(0xBEEF),
            uint64(block.timestamp + 2 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronV3.ChainDeprecated.selector);
        _untron.setPayoutConfig(leaseId, block.chainid, address(_usdt), address(0xCA11));
    }
}
