// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3ControllerEventsTest is UntronV3TestBase {
    function testRelayControllerEventChainValidatesControllerTarget() public {
        bytes32 tipOld = _untron.lastControllerEventTip();
        bytes memory callData = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), tipOld);

        _reader.setNextCallData(
            keccak256("tx_wrong_to"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(address(0xBEEF)), // not controller
            callData
        );

        vm.expectRevert(UntronV3.NotEventChainTip.selector);
        _untron.relayControllerEventChain(1, hex"", new bytes32[](0), 0, new UntronV3.ControllerEvent[](0));
    }

    function testRelayControllerEventChainDecodesSelectors() public {
        bytes32 tipOld = _untron.lastControllerEventTip();
        bytes32 tipNew = keccak256("new_tip");

        UntronV3.ControllerEvent[] memory events = new UntronV3.ControllerEvent[](0);
        bytes21 controllerTron = _untron.evmToTron(_untron.CONTROLLER_ADDRESS());

        // Direct selector.
        bytes memory isTip = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), tipOld);
        assertEq(_untron.exposedDecodeEventChainTip(isTip), tipOld);
        _reader.setNextCallData(
            keccak256("tx_direct_sel"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            isTip
        );
        vm.expectRevert(UntronV3.EventRelayNoProgress.selector);
        _untron.relayControllerEventChain(1, hex"", new bytes32[](0), 0, events);

        // Multicall wrapper selector (also EventRelayNoProgress because tip==tipOld).
        bytes[] memory calls = new bytes[](1);
        calls[0] = isTip;
        bytes memory multicall = abi.encodeWithSelector(bytes4(keccak256("multicall(bytes[])")), calls);
        _reader.setNextCallData(
            keccak256("tx_multicall_sel"),
            2,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            multicall
        );
        vm.expectRevert(UntronV3.EventRelayNoProgress.selector);
        _untron.relayControllerEventChain(2, hex"", new bytes32[](0), 0, events);

        // Unknown selector.
        bytes memory wrongSel = abi.encodeWithSelector(bytes4(keccak256("notATip(bytes32)")), tipNew);
        _reader.setNextCallData(
            keccak256("tx_wrong_sel"),
            3,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            wrongSel
        );
        vm.expectRevert(UntronV3.NotEventChainTip.selector);
        _untron.relayControllerEventChain(3, hex"", new bytes32[](0), 0, events);

        // Too-short calldata (<4 bytes).
        _reader.setNextCallData(
            keccak256("tx_short_sel"),
            4,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            hex"01"
        );
        vm.expectRevert(UntronV3.TronInvalidCalldataLength.selector);
        _untron.relayControllerEventChain(4, hex"", new bytes32[](0), 0, events);
    }

    function testRelayControllerEventChainProgressAndHashLinking() public {
        bytes32 tipOld = _untron.lastControllerEventTip();
        bytes21 controllerTron = _untron.evmToTron(_untron.CONTROLLER_ADDRESS());

        // tipNew != tipOld but events don't hash-link => mismatch.
        bytes32 arbitraryTip = keccak256("arbitrary_tip");
        bytes memory callData = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), arbitraryTip);
        _reader.setNextCallData(
            keccak256("tx_mismatch"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            callData
        );
        vm.expectRevert(UntronV3.EventTipMismatch.selector);
        _untron.relayControllerEventChain(1, hex"", new bytes32[](0), 0, new UntronV3.ControllerEvent[](0));

        // Correct events hash-link to tipNew.
        UntronV3.ControllerEvent[] memory events = new UntronV3.ControllerEvent[](2);
        events[0] = UntronV3.ControllerEvent({
            sig: keccak256("E0(uint256)"),
            data: abi.encode(uint256(1)),
            blockNumber: uint64(10),
            blockTimestamp: uint64(100)
        });
        events[1] = UntronV3.ControllerEvent({
            sig: keccak256("E1(address)"),
            data: abi.encode(address(0xBEEF)),
            blockNumber: uint64(11),
            blockTimestamp: uint64(101)
        });

        bytes32 tip = tipOld;
        tip = sha256(
            abi.encodePacked(
                tip, uint256(events[0].blockNumber), uint256(events[0].blockTimestamp), events[0].sig, events[0].data
            )
        );
        tip = sha256(
            abi.encodePacked(
                tip, uint256(events[1].blockNumber), uint256(events[1].blockTimestamp), events[1].sig, events[1].data
            )
        );

        bytes memory callData2 = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), tip);
        _reader.setNextCallData(
            keccak256("tx_ok"),
            2,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            controllerTron,
            callData2
        );

        bytes32 gotTip = _untron.relayControllerEventChain(2, hex"", new bytes32[](0), 0, events);
        assertEq(gotTip, tip);
        assertEq(_untron.lastControllerEventTip(), tip);

        assertEq(_untron.controllerEventsLength(), 2);
        (bytes32 sig0, bytes memory data0, uint64 bn0, uint64 ts0) = _untron.controllerEventAt(0);
        assertEq(sig0, events[0].sig);
        assertEq(bn0, events[0].blockNumber);
        assertEq(ts0, events[0].blockTimestamp);
        assertEq(data0, events[0].data);
    }

    function testProcessControllerEventsUnknownSigIsSkippedButCursorAdvances() public {
        bytes32 sig = keccak256("Unknown(uint256)");
        bytes memory data = abi.encode(uint256(123));

        _untron.pushControllerEvent(sig, data, 1, uint64(block.timestamp));
        assertEq(_untron.nextControllerEventIndex(), 0);

        _untron.processControllerEvents(1);
        assertEq(_untron.nextControllerEventIndex(), 1);
        assertEq(_untron.protocolPnl(), 0);
        assertEq(_untron.tronUsdt(), address(0));
    }

    function testPulledFromReceiverBindsUnbackedOldestFirst() public {
        bytes32 salt = keccak256("salt_backing");
        uint64 t1 = uint64(block.timestamp);

        uint256 lease1 =
            _untron.createLease(salt, address(0x1111), t1, 0, 0, block.chainid, address(_usdt), address(0xAAA1));

        vm.warp(t1 + 100);
        uint64 t2 = uint64(block.timestamp);
        uint256 lease2 =
            _untron.createLease(salt, address(0x2222), t2, 0, 0, block.chainid, address(_usdt), address(0xAAA2));

        address receiver = _predictedReceiver(salt);
        bytes memory trc20Data = _trc20TransferCalldata(receiver, 100);

        // Pre-entitle once for lease1.
        _reader.setNextCallData(
            keccak256("tx_l1"),
            1,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t1),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);

        // Pre-entitle once for lease2.
        _reader.setNextCallData(
            keccak256("tx_l2"),
            2,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t2),
            TronCalldataUtils.evmToTronAddress(address(0x2222)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        _untron.preEntitle(salt, 2, hex"", new bytes32[](0), 0);

        // Pull backs 150 USDT at t2: covers lease1 fully (100) then lease2 (50).
        bytes32 pulledSig = keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
        bytes memory pulledData = abi.encode(salt, address(0), uint256(0), uint256(0), uint256(150));
        _untron.pushControllerEvent(pulledSig, pulledData, 3, t2);
        _untron.processControllerEvents(1);

        assertEq(_untron.lastReceiverPullTimestamp(salt), t2);

        (,,,,,,, uint256 r1, uint256 b1, uint256 u1, UntronV3.PayoutConfig memory p1) = _untron.leases(lease1);
        (,,,,,,, uint256 r2, uint256 b2, uint256 u2, UntronV3.PayoutConfig memory p2) = _untron.leases(lease2);

        assertEq(r1, 100);
        assertEq(b1, 100);
        assertEq(u1, 0);
        assertEq(r1, b1 + u1);
        assertEq(p1.targetChainId, block.chainid);
        assertEq(p1.targetToken, address(_usdt));
        assertEq(p1.beneficiary, address(0xAAA1));

        assertEq(r2, 100);
        assertEq(b2, 50);
        assertEq(u2, 50);
        assertEq(r2, b2 + u2);
        assertEq(p2.targetChainId, block.chainid);
        assertEq(p2.targetToken, address(_usdt));
        assertEq(p2.beneficiary, address(0xAAA2));
    }

    function testPulledFromReceiverRemainderBecomesProfitVolumeAndCreatesClaim() public {
        bytes32 salt = keccak256("salt_profit_volume");
        uint64 t0 = uint64(block.timestamp);

        uint256 leaseId = _untron.createLease(
            salt, address(0xBEEF), t0 + 1 days, 10_000, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        bytes32 pulledSig = keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
        bytes memory pulledData = abi.encode(salt, address(0), uint256(0), uint256(0), uint256(100));
        _untron.pushControllerEvent(pulledSig, pulledData, 1, t0);
        _untron.processControllerEvents(1);

        assertEq(_untron.protocolPnl(), 1);
        assertEq(_untron.claimQueueLength(address(_usdt)), 1);
        (uint256 amountUsdt, uint256 gotLeaseId,,) = _untron.claimsByTargetToken(address(_usdt), 0);
        assertEq(amountUsdt, 99);
        assertEq(gotLeaseId, leaseId);

        (,,,,,,, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3.PayoutConfig memory p) =
            _untron.leases(leaseId);
        assertEq(recognizedRaw, 100);
        assertEq(backedRaw, 100);
        assertEq(unbackedRaw, 0);
        assertEq(recognizedRaw, backedRaw + unbackedRaw);
        assertEq(p.targetChainId, block.chainid);
        assertEq(p.targetToken, address(_usdt));
        assertEq(p.beneficiary, address(0xB0B));
    }

    function testUsdtSetUpdatesTronUsdt() public {
        address tronUsdt = address(0x1234);
        bytes32 sig = keccak256("UsdtSet(address)");
        _untron.pushControllerEvent(sig, abi.encode(tronUsdt), 1, uint64(block.timestamp));
        _untron.processControllerEvents(1);
        assertEq(_untron.tronUsdt(), tronUsdt);
    }

    function testUsdtRebalancedAndControllerUsdtTransferUpdateProtocolPnl() public {
        bytes32 rebalanceSig = keccak256("UsdtRebalanced(uint256,uint256,address)");
        _untron.pushControllerEvent(
            rebalanceSig, abi.encode(uint256(100), uint256(105), address(0x1)), 1, uint64(block.timestamp)
        );

        bytes32 transferSig = keccak256("ControllerUsdtTransfer(address,uint256)");
        _untron.pushControllerEvent(transferSig, abi.encode(address(0xBEEF), uint256(3)), 2, uint64(block.timestamp));

        _untron.processControllerEvents(2);
        assertEq(_untron.protocolPnl(), 2); // +5 -3
    }

    function testProcessControllerEventsIsBatchBounded() public {
        for (uint256 i = 0; i < 3; ++i) {
            _untron.pushControllerEvent(
                keccak256(abi.encodePacked("E", i)),
                abi.encode(i),
                // forge-lint: disable-next-line(unsafe-typecast)
                uint64(i + 1),
                uint64(block.timestamp)
            );
        }
        assertEq(_untron.controllerEventsLength(), 3);

        _untron.processControllerEvents(1);
        assertEq(_untron.nextControllerEventIndex(), 1);

        _untron.processControllerEvents(2);
        assertEq(_untron.nextControllerEventIndex(), 3);
    }
}
