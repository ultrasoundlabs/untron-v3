// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Call} from "../../../src/evm/SwapExecutor.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";

import {MockBridger, ReentrantBridger} from "./UntronV3TestUtils.sol";
import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3FillTest is UntronV3TestBase {
    function testFillLocalUsdtClaimsTransfersAndDeletes() public {
        address beneficiary1 = address(0xA11CE);
        address beneficiary2 = address(0xB0B);

        _untron.enqueueClaim(address(_usdt), 10, 1, block.chainid, beneficiary1);
        _untron.enqueueClaim(address(_usdt), 20, 2, block.chainid, beneficiary2);

        _usdt.mint(address(_untron), 30);
        assertEq(_usdt.balanceOf(address(_untron.SWAP_EXECUTOR())), 0);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 100, noCalls);

        assertEq(_usdt.balanceOf(beneficiary1), 10);
        assertEq(_usdt.balanceOf(beneficiary2), 20);

        assertEq(_untron.nextIndexByTargetToken(address(_usdt)), 2);
        (uint256 a0,,,) = _untron.claimsByTargetToken(address(_usdt), 0);
        (uint256 a1,,,) = _untron.claimsByTargetToken(address(_usdt), 1);
        assertEq(a0, 0);
        assertEq(a1, 0);

        // No swap executor involvement for local USDT fills.
        assertEq(_usdt.balanceOf(address(_untron.SWAP_EXECUTOR())), 0);
    }

    function testFillMaxClaimsZeroIsNoop() public {
        _untron.enqueueClaim(address(_usdt), 10, 1, block.chainid, address(0xA11CE));
        _usdt.mint(address(_untron), 10);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 0, noCalls);

        assertEq(_untron.nextIndexByTargetToken(address(_usdt)), 0);
        (uint256 a0,,,) = _untron.claimsByTargetToken(address(_usdt), 0);
        assertEq(a0, 10);
    }

    function testFillStopsWhenLiquidityInsufficient() public {
        address beneficiary1 = address(0xA11CE);
        address beneficiary2 = address(0xB0B);

        _untron.enqueueClaim(address(_usdt), 10, 1, block.chainid, beneficiary1);
        _untron.enqueueClaim(address(_usdt), 20, 2, block.chainid, beneficiary2);

        _usdt.mint(address(_untron), 10);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 10, noCalls);

        assertEq(_usdt.balanceOf(beneficiary1), 10);
        assertEq(_usdt.balanceOf(beneficiary2), 0);
        assertEq(_untron.nextIndexByTargetToken(address(_usdt)), 1);

        (uint256 a0,,,) = _untron.claimsByTargetToken(address(_usdt), 0);
        (uint256 a1,,,) = _untron.claimsByTargetToken(address(_usdt), 1);
        assertEq(a0, 0);
        assertEq(a1, 20);
    }

    function testFillWithSwapNoSurplusPaysBeneficiaries() public {
        address beneficiary = address(0xB0B);
        _untron.setSwapRate(address(_tokenX), 2_000_000); // 2 tokenX per USDT

        _untron.enqueueClaim(address(_tokenX), 10, 1, block.chainid, beneficiary);
        _usdt.mint(address(_untron), 10);

        uint256 expectedOut = 20;
        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            to: address(_swapRouter),
            value: 0,
            data: abi.encodeWithSelector(_swapRouter.mintToCaller.selector, address(_tokenX), expectedOut)
        });

        _untron.fill(address(_tokenX), 10, calls);

        assertEq(_tokenX.balanceOf(beneficiary), expectedOut);
        assertEq(_tokenX.balanceOf(address(this)), 0);
    }

    function testSwapSurplusIsPaidToFiller() public {
        address beneficiary = address(0xB0B);
        _untron.setSwapRate(address(_tokenX), 2_000_000); // 2 tokenX per USDT

        _untron.enqueueClaim(address(_tokenX), 10, 1, block.chainid, beneficiary);
        _usdt.mint(address(_untron), 10);

        uint256 expectedOut = 20;
        uint256 actualOut = 25;
        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            to: address(_swapRouter),
            value: 0,
            data: abi.encodeWithSelector(_swapRouter.mintToCaller.selector, address(_tokenX), actualOut)
        });

        _untron.fill(address(_tokenX), 10, calls);

        assertEq(_tokenX.balanceOf(beneficiary), expectedOut);
        assertEq(_tokenX.balanceOf(address(this)), actualOut - expectedOut);
    }

    function testFillWithBridgingCallsBridgerPerClaim() public {
        uint256 otherChainId = block.chainid + 1;
        MockBridger bridger = new MockBridger();

        _untron.setSwapRate(address(_tokenX), 1_000_000); // 1 tokenX per USDT
        _untron.setBridger(address(_tokenX), otherChainId, address(bridger));

        address beneficiary = address(0xB0B);
        _untron.enqueueClaim(address(_tokenX), 10, 1, otherChainId, beneficiary);
        _usdt.mint(address(_untron), 10);

        uint256 expectedOut = 10;
        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            to: address(_swapRouter),
            value: 0,
            data: abi.encodeWithSelector(_swapRouter.mintToCaller.selector, address(_tokenX), expectedOut)
        });

        _untron.fill(address(_tokenX), 1, calls);

        assertEq(bridger.callCount(), 1);
        (address token, uint256 amount, uint256 chainId, address ben) = bridger.callAt(0);
        assertEq(token, address(_tokenX));
        assertEq(amount, expectedOut);
        assertEq(chainId, otherChainId);
        assertEq(ben, beneficiary);

        assertEq(_tokenX.balanceOf(address(bridger)), expectedOut);
        assertEq(_tokenX.balanceOf(beneficiary), 0);
    }

    function testFillRevertsOnMissingRateOrMissingBridger() public {
        Call[] memory noCalls = new Call[](0);

        _untron.enqueueClaim(address(_tokenX), 10, 1, block.chainid, address(0xB0B));
        _usdt.mint(address(_untron), 10);

        vm.expectRevert(UntronV3.RateNotSet.selector);
        _untron.fill(address(_tokenX), 1, noCalls);

        // Missing bridger is detected during planning (before swap transfer).
        uint256 otherChainId = block.chainid + 1;
        _untron.setSwapRate(address(_tokenX), 1_000_000);
        _untron.enqueueClaim(address(_tokenX), 10, 2, otherChainId, address(0xB0B));
        _usdt.mint(address(_untron), 10); // ensure batch planning reaches the bridged claim

        uint256 untronUsdtBefore = _usdt.balanceOf(address(_untron));
        assertEq(_usdt.balanceOf(address(_untron.SWAP_EXECUTOR())), 0);

        vm.expectRevert(UntronV3.NoBridger.selector);
        _untron.fill(address(_tokenX), 10, noCalls);

        assertEq(_usdt.balanceOf(address(_untron)), untronUsdtBefore);
        assertEq(_usdt.balanceOf(address(_untron.SWAP_EXECUTOR())), 0);
    }

    function testFillDeletesClaimBeforeExternalBridgeAndPreventsReentrancy() public {
        uint256 otherChainId = block.chainid + 1;
        _untron.setSwapRate(address(_tokenX), 1_000_000);

        _untron.enqueueClaim(address(_tokenX), 10, 1, otherChainId, address(0xB0B));
        _usdt.mint(address(_untron), 10);

        ReentrantBridger bridger = new ReentrantBridger(_untron, address(_tokenX), 0);
        _untron.setBridger(address(_tokenX), otherChainId, address(bridger));

        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            to: address(_swapRouter),
            value: 0,
            data: abi.encodeWithSelector(_swapRouter.mintToCaller.selector, address(_tokenX), uint256(10))
        });

        _untron.fill(address(_tokenX), 1, calls);

        assertTrue(bridger.didCheckDeletion());
        assertTrue(bridger.didReenter());
        (uint256 a0,,,) = _untron.claimsByTargetToken(address(_tokenX), 0);
        assertEq(a0, 0);
    }

    function testFillOutputAmountMathUsesFloorMulDiv() public {
        address beneficiary = address(0xB0B);
        _untron.setSwapRate(address(_tokenX), 500_000); // 0.5 tokenX per USDT

        _untron.enqueueClaim(address(_tokenX), 3, 1, block.chainid, beneficiary);
        _usdt.mint(address(_untron), 3);

        // floor(3 * 0.5) = 1
        Call[] memory calls = new Call[](1);
        calls[0] = Call({
            to: address(_swapRouter),
            value: 0,
            data: abi.encodeWithSelector(_swapRouter.mintToCaller.selector, address(_tokenX), uint256(1))
        });

        _untron.fill(address(_tokenX), 1, calls);
        assertEq(_tokenX.balanceOf(beneficiary), 1);
    }
}
