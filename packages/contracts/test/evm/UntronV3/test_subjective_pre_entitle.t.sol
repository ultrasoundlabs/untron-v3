// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3SubjectivePreEntitleTest is UntronV3TestBase {
    function testSubjectivePreEntitleHappyPathCreatesClaimThenPreEntitleReimbursesAndDoesNotCreateSecondClaim() public {
        bytes32 salt = keccak256("salt_subjective_happy");
        address beneficiary = address(0xB0B);
        (uint256 leaseId,) = _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), beneficiary
        );

        address lp = address(0xA11CE);
        _untron.setLp(lp, true);
        _usdt.mint(lp, 10_000);
        vm.startPrank(lp);
        _usdt.approve(address(_untron), type(uint256).max);
        _untron.deposit(10_000);
        assertEq(_untron.lpPrincipal(lp), 10_000);

        bytes32 txId = keccak256("tx_subjective_happy");
        uint256 rawAmount = 100;

        (uint256 subjectiveClaimIndex, uint256 netOut) = _untron.subjectivePreEntitle(txId, leaseId, rawAmount);
        assertEq(netOut, rawAmount);
        assertEq(subjectiveClaimIndex, 0);
        assertEq(_untron.lpPrincipal(lp), 10_000 - rawAmount);
        vm.stopPrank();

        // Claim is created and queued.
        assertEq(_untron.claimQueueLength(address(_usdt)), 1);
        {
            (, uint256 amountUsdt, uint256 claimLeaseId, uint256 targetChainId, address gotBeneficiary) =
                _untron.claimsByTargetToken(address(_usdt), subjectiveClaimIndex);
            assertEq(amountUsdt, rawAmount);
            assertEq(claimLeaseId, leaseId);
            assertEq(targetChainId, block.chainid);
            assertEq(gotBeneficiary, beneficiary);
        }

        // Prove the actual Tron tx; should reimburse principal and skip creating a claim.
        {
            address receiver = _predictedReceiver(salt);
            bytes memory data = _trc20TransferCalldata(receiver, rawAmount);
            _reader.setNextCallData(
                txId,
                1,
                uint32(block.timestamp),
                TronCalldataUtils.evmToTronAddress(address(0x1111)),
                TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
                data
            );
        }

        (uint256 claimIndex2, uint256 gotLeaseId, uint256 netOut2) =
            _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId, leaseId);
        assertEq(netOut2, rawAmount);
        assertEq(claimIndex2, 0);
        // No second claim is enqueued for the same tx.
        assertEq(_untron.claimQueueLength(address(_usdt)), 1);
        assertTrue(_untron.depositProcessed(txId));

        // Principal reimbursed (accounting credit).
        assertEq(_untron.lpPrincipal(lp), 10_000);
    }

    function testSubjectivePreEntitleMismatchIsForfeitedAndPreEntitleCreatesClaim() public {
        bytes32 salt = keccak256("salt_subjective_mismatch");
        address beneficiary = address(0xB0B);
        (uint256 leaseId,) = _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), beneficiary
        );

        address lp = address(0xA11CE);
        _untron.setLp(lp, true);
        _usdt.mint(lp, 10_000);
        vm.startPrank(lp);
        _usdt.approve(address(_untron), type(uint256).max);
        _untron.deposit(10_000);

        bytes32 txId = keccak256("tx_subjective_mismatch");
        _untron.subjectivePreEntitle(txId, leaseId, 99);
        vm.stopPrank();

        // Prove a tx with a different raw amount.
        uint256 provenRaw = 100;
        {
            address receiver = _predictedReceiver(salt);
            bytes memory data = _trc20TransferCalldata(receiver, provenRaw);
            _reader.setNextCallData(
                txId,
                1,
                uint32(block.timestamp),
                TronCalldataUtils.evmToTronAddress(address(0x1111)),
                TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
                data
            );
        }

        (uint256 claimIndex, uint256 gotLeaseId, uint256 netOut) =
            _untron.preEntitle(salt, _emptyBlocks(), hex"", new bytes32[](0), 0);
        assertEq(gotLeaseId, leaseId);
        assertEq(netOut, provenRaw);
        assertEq(claimIndex, 1);

        // Claim created for the proven netOut (subjective sponsor is not reimbursed).
        assertEq(_untron.claimQueueLength(address(_usdt)), 2);
        {
            (, uint256 amountUsdt, uint256 claimLeaseId, uint256 targetChainId, address gotBeneficiary) =
                _untron.claimsByTargetToken(address(_usdt), 1);
            assertEq(amountUsdt, provenRaw);
            assertEq(claimLeaseId, leaseId);
            assertEq(targetChainId, block.chainid);
            assertEq(gotBeneficiary, beneficiary);
        }

        // Sponsor principal remains debited (gift).
        assertEq(_untron.lpPrincipal(lp), 10_000 - 99);
    }

    function testSubjectivePreEntitleRevertsIfAlreadyExists() public {
        bytes32 salt = keccak256("salt_subjective_exists");
        (uint256 leaseId,) = _createLease(
            salt, address(this), uint64(block.timestamp + 1 days), 0, 0, block.chainid, address(_usdt), address(0xB0B)
        );

        address lp = address(0xA11CE);
        _untron.setLp(lp, true);
        _usdt.mint(lp, 1_000);
        vm.startPrank(lp);
        _usdt.approve(address(_untron), type(uint256).max);
        _untron.deposit(1_000);

        bytes32 txId = keccak256("tx_subjective_exists");
        _untron.subjectivePreEntitle(txId, leaseId, 10);

        vm.expectRevert(UntronV3.SubjectivePreEntitlementAlreadyExists.selector);
        _untron.subjectivePreEntitle(txId, leaseId, 10);
        vm.stopPrank();
    }
}
