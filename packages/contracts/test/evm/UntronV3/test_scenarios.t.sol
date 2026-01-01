// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Call} from "../../../src/evm/SwapExecutor.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3ScenarioTest is UntronV3TestBase {
    function testScenarioLeaseDepositClaimFillLocal() public {
        bytes32 salt = keccak256("salt_scenario_1");
        address beneficiary = address(0xB0B);

        uint256 leaseId = _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 10_000, 0, block.chainid, address(_usdt), beneficiary
        );

        // Provide USDT liquidity for fill.
        _usdt.mint(address(_untron), 99);

        // Pre-entitle a 100 USDT deposit; netOut = 99.
        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);
        _reader.setNextCallData(
            keccak256("tx_s1"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );

        (uint256 claimIndex, uint256 gotLeaseId, uint256 netOut) =
            _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId, leaseId);
        assertEq(netOut, 99);
        assertEq(_untron.protocolPnl(), 1);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 1, noCalls);

        assertEq(_usdt.balanceOf(beneficiary), 99);
        assertEq(_untron.nextIndexByTargetToken(address(_usdt)), claimIndex + 1);
        (, uint256 amountUsdt,,,) = _untron.claimsByTargetToken(address(_usdt), claimIndex);
        assertEq(amountUsdt, 0);

        (,,,,,,, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3.PayoutConfig memory p) =
            _untron.leases(leaseId);
        assertEq(recognizedRaw, 100);
        assertEq(backedRaw, 0);
        assertEq(unbackedRaw, 100);
        assertEq(recognizedRaw, backedRaw + unbackedRaw);
        assertEq(p.targetChainId, block.chainid);
        assertEq(p.targetToken, address(_usdt));
        assertEq(p.beneficiary, beneficiary);
    }

    function testScenarioLeaseChangeoverDepositsGoToCorrectBeneficiaries() public {
        bytes32 salt = keccak256("salt_scenario_changeover");
        address ben1 = address(0xAAA1);
        address ben2 = address(0xAAA2);

        uint64 t1 = uint64(block.timestamp);
        uint256 lease1 = _untron.createLease(salt, address(0x1111), t1, 0, 0, block.chainid, address(_usdt), ben1);

        vm.warp(t1 + 100);
        uint64 t2 = uint64(block.timestamp);
        uint256 lease2 = _untron.createLease(salt, address(0x2222), t2, 0, 0, block.chainid, address(_usdt), ben2);

        _usdt.mint(address(_untron), 200);

        address receiver = _predictedReceiver(salt);
        bytes memory trc20Data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_before"),
            1,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t1),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        (, uint256 gotLeaseId1,) = _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId1, lease1);

        _reader.setNextCallData(
            keccak256("tx_after"),
            2,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t2),
            TronCalldataUtils.evmToTronAddress(address(0x2222)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        (, uint256 gotLeaseId2,) = _untron.preEntitle(salt, 2, hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId2, lease2);

        Call[] memory noCalls = new Call[](0);
        _untron.fill(address(_usdt), 2, noCalls);

        assertEq(_usdt.balanceOf(ben1), 100);
        assertEq(_usdt.balanceOf(ben2), 100);
    }

    function testScenarioControllerPullOrderingProtectionAndBacking() public {
        bytes32 salt = keccak256("salt_scenario_pull");
        uint64 t1 = uint64(block.timestamp);
        uint256 leaseId = _untron.createLease(
            salt, address(0xBEEF), t1 + 1 days, 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory trc20Data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_deposit"),
            1,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t1),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        _untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);

        uint64 t2 = t1 + 10;
        bytes32 pulledSig = keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
        bytes memory pulledData = abi.encode(salt, address(0), uint256(0), uint256(0), uint256(100));
        _untron.pushControllerEvent(pulledSig, pulledData, 2, t2);
        _untron.processControllerEvents(1);

        assertEq(_untron.lastReceiverPullTimestampByToken(salt, address(0)), t2);
        (,,,,,,, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3.PayoutConfig memory p) =
            _untron.leases(leaseId);
        assertEq(recognizedRaw, 100);
        assertEq(backedRaw, 100);
        assertEq(unbackedRaw, 0);
        assertEq(recognizedRaw, backedRaw + unbackedRaw);
        assertEq(p.targetChainId, block.chainid);
        assertEq(p.targetToken, address(_usdt));

        _reader.setNextCallData(
            keccak256("tx_too_old"),
            3,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t2),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            trc20Data
        );
        vm.expectRevert(UntronV3.DepositNotAfterLastReceiverPull.selector);
        _untron.preEntitle(salt, 3, hex"", new bytes32[](0), 0);
    }
}
