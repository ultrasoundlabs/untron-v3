// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3PreEntitleTest is UntronV3TestBase {
    function testPreEntitleHappyPathRecognizesDepositAndCreatesClaim() public {
        // Set tronUsdt via controller event.
        address tronUsdt = address(0x1234);
        _untron.pushControllerEvent(keccak256("UsdtSet(address)"), abi.encode(tronUsdt), 1, uint64(block.timestamp));
        _untron.processControllerEvents(1);
        assertEq(_untron.tronUsdt(), tronUsdt);

        bytes32 salt = keccak256("salt_pre_entitle");
        (uint256 leaseId,) = _createLease(
            salt,
            address(this),
            uint64(block.timestamp + 1 days),
            10_000, // 1%
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        bytes32 txId = keccak256("tx_pre_entitle");
        _reader.setNextCallData(
            txId,
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(tronUsdt),
            data
        );

        (uint256 claimIndex, uint256 gotLeaseId, uint256 netOut) =
            _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        assertTrue(_untron.depositProcessed(txId));
        assertEq(gotLeaseId, leaseId);
        assertEq(netOut, 99);
        assertEq(_untron.protocolPnl(), 1);

        // Claim created for netOut.
        assertEq(_untron.claimQueueLength(address(_usdt)), 1);
        (uint256 claimId, uint256 amountUsdt, uint256 claimLeaseId, uint256 targetChainId, address beneficiary) =
            _untron.claimsByTargetToken(address(_usdt), claimIndex);
        assertEq(claimId, 0);
        assertEq(amountUsdt, 99);
        assertEq(claimLeaseId, leaseId);
        assertEq(targetChainId, block.chainid);
        assertEq(beneficiary, address(0xB0B));

        _assertLeaseAccounting(leaseId, 100, 0, 100, block.chainid, address(_usdt), address(0xB0B));
    }

    function testPreEntitleReplayProtection() public {
        bytes32 salt = keccak256("salt_pre_entitle_replay");
        _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        bytes32 txId = keccak256("tx_replay");
        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            txId,
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);

        _reader.setNextCallData(
            txId,
            2,
            uint32(block.timestamp + 1),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
        vm.expectRevert(UntronV3.DepositAlreadyProcessed.selector);
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
    }

    function testPreEntitleRevertsIfNotTronUsdt() public {
        address tronUsdt = address(0x1234);
        _untron.pushControllerEvent(keccak256("UsdtSet(address)"), abi.encode(tronUsdt), 1, uint64(block.timestamp));
        _untron.processControllerEvents(1);

        bytes32 salt = keccak256("salt_wrong_token");
        _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_wrong_token"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(address(0x9999)), // not tronUsdt
            data
        );

        vm.expectRevert(UntronV3.NotTronUsdt.selector);
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
    }

    function testPreEntitleRevertsIfReceiverDoesNotMatchSalt() public {
        bytes32 salt = keccak256("salt_wrong_receiver");
        _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        bytes memory data = _trc20TransferCalldata(address(0xDEAD), 100);

        _reader.setNextCallData(
            keccak256("tx_wrong_receiver"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );

        vm.expectRevert(UntronV3.InvalidReceiverForSalt.selector);
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
    }

    function testPreEntitleRevertsWhenNoActiveLeaseAtTronTimestamp() public {
        bytes32 salt = keccak256("salt_no_active_lease");
        uint64 t1 = uint64(block.timestamp);
        _createLease(salt, address(this), uint64(t1 + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B));

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_no_active"),
            1,
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(t1 - 1),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );

        vm.expectRevert(UntronV3.NoActiveLease.selector);
        _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
    }

    function testPreEntitleNetOutZeroDoesNotCreateClaimButBooksFee() public {
        bytes32 salt = keccak256("salt_net_out_zero");
        (uint256 leaseId,) = _createLease(
            salt,
            address(this),
            uint64(block.timestamp + 1 days),
            1_000_000, // 100% fee => netOut 0
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_net_out_zero"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );

        (uint256 claimIndex,, uint256 netOut) = _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
        assertEq(netOut, 0);
        assertEq(claimIndex, 0);
        assertEq(_untron.claimQueueLength(address(_usdt)), 0);

        // Full amount is booked as protocol PnL.
        assertEq(_untron.protocolPnl(), 100);

        _assertLeaseAccounting(leaseId, 100, 0, 100, block.chainid, address(_usdt), address(0xB0B));
    }

    function testPreEntitleFlatFeeCanZeroNetOut() public {
        bytes32 salt = keccak256("salt_flat_fee_zero");
        (uint256 leaseId,) = _createLease(
            salt,
            address(this),
            uint64(block.timestamp + 1 days),
            0,
            100, // flat fee equals amount => netOut 0
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        address receiver = _predictedReceiver(salt);
        bytes memory data = _trc20TransferCalldata(receiver, 100);

        _reader.setNextCallData(
            keccak256("tx_flat_fee_zero"),
            1,
            uint32(block.timestamp),
            TronCalldataUtils.evmToTronAddress(address(0x1111)),
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );

        (uint256 claimIndex,, uint256 netOut) = _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
        assertEq(netOut, 0);
        assertEq(claimIndex, 0);
        assertEq(_untron.claimQueueLength(address(_usdt)), 0);
        assertEq(_untron.protocolPnl(), 100);

        _assertLeaseAccounting(leaseId, 100, 0, 100, block.chainid, address(_usdt), address(0xB0B));
    }

    function _assertLeaseAccounting(
        uint256 leaseId,
        uint256 expectedRecognizedRaw,
        uint256 expectedBackedRaw,
        uint256 expectedUnbackedRaw,
        uint256 expectedTargetChainId,
        address expectedTargetToken,
        address expectedBeneficiary
    ) internal view {
        (,,,,,,, uint256 recognizedRaw, uint256 backedRaw, uint256 unbackedRaw, UntronV3.PayoutConfig memory p) =
            _untron.leases(leaseId);
        assertEq(recognizedRaw, expectedRecognizedRaw);
        assertEq(backedRaw, expectedBackedRaw);
        assertEq(unbackedRaw, expectedUnbackedRaw);
        assertEq(recognizedRaw, backedRaw + unbackedRaw);
        assertEq(p.targetChainId, expectedTargetChainId);
        assertEq(p.targetToken, expectedTargetToken);
        assertEq(p.beneficiary, expectedBeneficiary);
    }
}
