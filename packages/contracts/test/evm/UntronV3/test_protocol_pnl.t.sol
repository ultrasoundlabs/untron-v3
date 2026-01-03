// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {MockTronTxReader, UntronV3Harness} from "./UntronV3TestUtils.sol";

contract UntronV3ProtocolPnlTest is Test {
    MockTronTxReader internal _reader;
    UntronV3Harness internal _untron;

    address internal constant _DUMMY_USDT = address(0x1000);
    address internal constant _CONTROLLER = address(0xCAFE);
    address internal constant _RECEIVER_IMPL_OVERRIDE = address(0xBEEF);

    function setUp() public {
        _reader = new MockTronTxReader();
        _untron = new UntronV3Harness(_CONTROLLER, 0xff, _RECEIVER_IMPL_OVERRIDE);
        _untron.setTronReader(address(_reader));
        _untron.setUsdt(_DUMMY_USDT);
        _untron.setRealtor(address(this), true);
    }

    function _emptyBlocks() internal pure returns (bytes[20] memory blocks) {}

    function testPreEntitleBooksFee() public {
        bytes32 salt = keccak256("salt1");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        (uint256 leaseId,) = _untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );

        address predictedReceiver = _untron.predictReceiverAddress(_CONTROLLER, salt);
        bytes memory trc20Data =
            abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), predictedReceiver, uint256(100));

        _reader.setNextCallData(
            keccak256("tx1"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(address(0)),
            trc20Data
        );

        (uint256 claimIndex, uint256 gotLeaseId, uint256 netOut) =
            _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        assertEq(gotLeaseId, leaseId);
        assertEq(netOut, 99);
        assertEq(_untron.protocolPnl(), 1);
        assertEq(_untron.claimQueueLength(_DUMMY_USDT), 1);
        (uint256 claimId, uint256 claimAmount, uint256 claimLeaseId,,) =
            _untron.claimsByTargetToken(_DUMMY_USDT, claimIndex);
        assertEq(claimId, 0);
        assertEq(claimAmount, 99);
        assertEq(claimLeaseId, leaseId);
    }

    function testPreEntitleNetOutZeroStillBooksFeeNoClaim() public {
        bytes32 salt = keccak256("salt2");
        uint32 leaseFeePpm = 0;
        uint64 flatFee = 200;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        _untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );

        address predictedReceiver = _untron.predictReceiverAddress(_CONTROLLER, salt);
        bytes memory trc20Data =
            abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), predictedReceiver, uint256(100));

        _reader.setNextCallData(
            keccak256("tx2"),
            2,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x2222)),
            TronCalldataUtils.evmToTronAddress(address(0)),
            trc20Data
        );

        (,, uint256 netOut) = _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        assertEq(netOut, 0);
        assertEq(_untron.protocolPnl(), 100);
        assertEq(_untron.claimQueueLength(_DUMMY_USDT), 0);
    }

    function testPreEntitleRevertsIfNotAfterLastReceiverPull() public {
        bytes32 salt = keccak256("salt_pull_guard");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        _untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );

        uint64 pullTs = uint64(block.timestamp);
        // Pulling a non-deposit token should not affect the preEntitle cutoff for `tronUsdt`.
        _untron.exposedProcessReceiverPulled(salt, address(0x1234), 1, pullTs);

        address predictedReceiver = _untron.predictReceiverAddress(_CONTROLLER, salt);
        bytes memory trc20Data =
            abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), predictedReceiver, uint256(100));

        // Deposit at the same timestamp should still be allowed since the pull was for a different token.
        _reader.setNextCallData(
            keccak256("tx_pull_guard_other_token_ok"),
            3,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(pullTs),
            TronCalldataUtils.evmToTronAddress(address(0x4444)),
            TronCalldataUtils.evmToTronAddress(address(0)),
            trc20Data
        );
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        // Now record a pull for the deposit token (`tronUsdt`, which is `address(0)` in this test harness).
        _untron.exposedProcessReceiverPulled(salt, address(0), 1, pullTs + 1);

        _reader.setNextCallData(
            keccak256("tx_pull_guard_eq"),
            4,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(pullTs + 1),
            TronCalldataUtils.evmToTronAddress(address(0x4444)),
            TronCalldataUtils.evmToTronAddress(address(0)),
            trc20Data
        );

        vm.expectRevert(UntronV3.DepositNotAfterLastReceiverPull.selector);
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        _reader.setNextCallData(
            keccak256("tx_pull_guard_ok"),
            5,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(pullTs + 2),
            TronCalldataUtils.evmToTronAddress(address(0x5555)),
            TronCalldataUtils.evmToTronAddress(address(0)),
            trc20Data
        );

        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
    }

    function testUsdtRebalancedBooksDrift() public {
        bytes32 sig = keccak256("UsdtRebalanced(uint256,uint256,address)");
        bytes memory data = abi.encode(uint256(1000), uint256(995), address(0x3333));
        _untron.pushControllerEvent(sig, data, 1, uint64(block.timestamp));

        _untron.processControllerEvents(1);

        assertEq(_untron.protocolPnl(), -5);
    }

    function testReceiverPulledRemainingBooksFeeAndClaim() public {
        bytes32 salt = keccak256("salt3");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        (uint256 leaseId,) = _untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );

        _untron.exposedProcessReceiverPulled(salt, address(0), 100, uint64(block.timestamp));

        assertEq(_untron.protocolPnl(), 1);
        assertEq(_untron.claimQueueLength(_DUMMY_USDT), 1);
        (uint256 claimId, uint256 claimAmount, uint256 claimLeaseId,,) = _untron.claimsByTargetToken(_DUMMY_USDT, 0);
        assertEq(claimId, 0);
        assertEq(claimAmount, 99);
        assertEq(claimLeaseId, leaseId);
    }

    function testReceiverPulledNoActiveLeaseBooksProtocolProfit() public {
        bytes32 salt = keccak256("salt_no_lease");

        _untron.exposedProcessReceiverPulled(salt, address(0), 100, uint64(block.timestamp));

        assertEq(_untron.protocolPnl(), 100);
        assertEq(_untron.claimQueueLength(_DUMMY_USDT), 0);
    }

    function testReceiverPulledDumpBeforeFirstLeaseStartBooksProtocolProfit() public {
        bytes32 salt = keccak256("salt_future_start");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        _untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );

        _untron.exposedProcessReceiverPulled(salt, address(0), 100, uint64(block.timestamp - 1));

        assertEq(_untron.protocolPnl(), 100);
        assertEq(_untron.claimQueueLength(_DUMMY_USDT), 0);
    }

    function _evmToTron(address a) internal pure returns (bytes21) {
        return TronCalldataUtils.evmToTronAddress(a);
    }
}
