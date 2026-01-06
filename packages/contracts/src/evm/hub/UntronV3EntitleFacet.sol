// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {ITronTxReader} from "../interfaces/ITronTxReader.sol";

/// @title UntronV3 entitle facet
/// @notice Tron deposit proof and (subjective) pre-entitlement logic.
/// @author Ultrasound Labs
contract UntronV3EntitleFacet is UntronV3Base {
    /* solhint-disable function-max-lines */

    /// @notice Prove and pre-entitle a recognizable TRC-20 deposit on Tron to a lease.
    /// @dev This wires together:
    /// - `ITronTxReader` (verifies inclusion + decodes a `TriggerSmartContract` tx),
    /// - `TronCalldataUtils` (parses recognizable TRC-20 transfer calldata),
    /// - and the lease timeline (`_findActiveLeaseId`) to attribute the deposit to the correct lease.
    ///
    /// What "pre-entitle" means:
    /// - This function does NOT move funds from Tron to this chain.
    /// - It recognizes that a Tron-side transfer into a deterministic receiver has happened, and
    ///   creates a USDT-denominated claim on this chain for later settlement.
    ///
    /// Replay protection:
    /// - Each recognized Tron tx is processed once via `depositProcessed[txId]`.
    ///
    /// Requirements:
    /// - The proved tx must be a successful `TriggerSmartContract` call to the TRC-20 contract `tronUsdt`.
    /// - The decoded TRC-20 transfer destination must match the predicted receiver for `receiverSalt`.
    /// - There must be an active lease for `receiverSalt` at the Tron tx timestamp.
    /// - The Tron tx timestamp must be strictly greater than the latest processed receiver pull timestamp for
    ///   `(receiverSalt, tronUsdt)` (see `lastReceiverPullTimestampByToken`).
    ///
    /// Accounting:
    /// - Increments `lease.recognizedRaw` and `lease.unbackedRaw` by the raw TRC-20 amount.
    /// - Computes net payout (`netOut`) using the lease fee schedule and books protocol fees into `protocolPnl`.
    /// - Enqueues a claim for `netOut` (if `netOut > 0`) under the lease's current `payout.targetToken`.
    ///
    /// @param receiverSalt CREATE2 salt used to derive the receiver address on Tron.
    /// @param blocks 20 Protobuf-encoded Tron block headers (first contains the tx).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @return queueIndex Index in `claimsByTargetToken[payout.targetToken]` where the claim was appended (0 if none).
    /// @return leaseId Lease id that the deposit was attributed to.
    /// @return netOut USDT-denominated net payout after fees (0 if fees exceed the amount).
    function preEntitle(
        bytes32 receiverSalt,
        bytes[20] calldata blocks,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external whenNotPaused returns (uint256 queueIndex, uint256 leaseId, uint256 netOut) {
        // Verify inclusion + success and decode into a generic TriggerSmartContract view.
        ITronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(blocks, encodedTx, proof, index);
        bytes32 txId = callData.txId;

        // Prevent double-processing of the same recognizable tx.
        if (depositProcessed[txId]) revert DepositAlreadyProcessed();
        depositProcessed[txId] = true;

        // Enforce that the TRC-20 contract called is exactly Tron USDT and decode the transfer amount.
        address tronUsdt_ = tronUsdt;
        uint256 amountQ = _decodeRecognizableTronUsdtDepositAmount(
            receiverSalt, callData.toTron, callData.senderTron, callData.data, tronUsdt_
        );

        // Enforce proof ordering against receiver pulls.
        _enforceDepositNotAfterLastReceiverPull(receiverSalt, tronUsdt_, callData.tronBlockTimestamp);

        // Token is no longer part of lease uniqueness; use receiver salt only.
        // Attribute to the lease that was active at the Tron tx timestamp.
        leaseId = _findActiveLeaseId(receiverSalt, callData.tronBlockTimestamp);
        if (leaseId == 0) revert NoActiveLease();
        Lease storage lease = _leaseStorage(leaseId);

        // Recognize raw volume and mark it as unbacked until the controller reports receiver pulls.
        lease.recognizedRaw += amountQ;
        lease.unbackedRaw += amountQ;

        // Compute net payout after lease fee schedule.
        netOut = _computeNetOut(lease, amountQ);
        // Book protocol fee revenue immediately as PnL (raw - netOut).
        _bookFee(amountQ, netOut);

        // If an LP previously paid out this exact tx subjectively (matching lease + raw amount),
        // reimburse their principal and skip creating a beneficiary claim.
        SubjectivePreEntitlement storage subjective = subjectivePreEntitlementByTxId[txId];
        if (subjective.sponsor != address(0)) {
            address sponsor = subjective.sponsor;
            bool matches = (subjective.leaseId == leaseId && subjective.rawAmount == amountQ);
            uint256 subjectiveQueueIndex = subjective.queueIndex;

            // Clear record (txId is now proven; regardless of match, it should not be reusable).
            delete subjectivePreEntitlementByTxId[txId];

            if (matches) {
                if (netOut != 0) {
                    lpPrincipal[sponsor] += netOut;
                }
            }

            if (matches) {
                // Return without creating a claim: the subjective pre-entitlement already created the claim.
                return (subjectiveQueueIndex, leaseId, netOut);
            }
        }

        // Enqueue a claim for later settlement if there is a positive net payout.
        if (netOut > 0) {
            PayoutConfig storage p = lease.payout;
            uint256 claimId;
            (queueIndex, claimId) =
                _enqueueClaimForTargetToken(p.targetToken, netOut, leaseId, p.targetChainId, p.beneficiary);
            _emitPreEntitleClaimCreated(
                leaseId, claimId, p, queueIndex, netOut, txId, callData.tronBlockTimestamp, amountQ
            );
        }
    }

    /*//////////////////////////////////////////////////////////////
                      SUBJECTIVE PRE-ENTITLEMENT (LP SPONSORED)
    //////////////////////////////////////////////////////////////*/

    /// @notice Pay a lease beneficiary early for an anticipated Tron deposit, funded by the caller's LP principal.
    /// @dev This does NOT prove any Tron tx. The sponsor is reimbursed only if a later `preEntitle` proves the exact
    ///      same `(txId, leaseId, rawAmount)`. Otherwise, the sponsor's payment is effectively a gift.
    ///
    /// Important invariants:
    /// - This debits `lpPrincipal[msg.sender]` immediately by `netOut`, so the sponsor cannot withdraw those funds
    ///   before reimbursement.
    /// - This enqueues a normal claim for the lease's current payout config (same as `preEntitle`), and relies on
    ///   the normal `fill()` path to settle it.
    ///
    /// @param txId Anticipated Tron transaction id (`sha256(raw_data)`).
    /// @param leaseId Lease expected to be active at the Tron tx timestamp.
    /// @param rawAmount Expected raw TRC-20 amount (before fees).
    /// @return queueIndex Index in `claimsByTargetToken[payout.targetToken]` where the claim was appended.
    /// @return netOut USDT-denominated claim amount after fees.
    function subjectivePreEntitle(bytes32 txId, uint256 leaseId, uint256 rawAmount)
        external
        whenNotPaused
        returns (uint256 queueIndex, uint256 netOut)
    {
        if (depositProcessed[txId]) revert DepositAlreadyProcessed();

        if (subjectivePreEntitlementByTxId[txId].sponsor != address(0)) revert SubjectivePreEntitlementAlreadyExists();

        if (rawAmount == 0) revert ZeroAmount();

        // Validate lease exists and compute net payout under that lease's fixed fee schedule.
        Lease storage lease = _leaseStorage(leaseId);
        netOut = _computeNetOut(lease, rawAmount);
        if (netOut == 0) revert SubjectiveNetOutZero();

        // Snapshot payout config (as of sponsor action) for claim creation.
        PayoutConfig storage p = lease.payout;
        // Validate that this payout route is currently supported/configured.
        _enforcePayoutConfigRoutable(p.targetChainId, p.targetToken);

        // Enforce sponsor principal availability.
        uint256 principal = lpPrincipal[msg.sender];
        if (principal < netOut) revert InsufficientLpPrincipal();

        // Debit principal before creating a claim.
        lpPrincipal[msg.sender] = principal - netOut;

        uint256 claimId;
        (queueIndex, claimId) =
            _enqueueClaimForTargetToken(p.targetToken, netOut, leaseId, p.targetChainId, p.beneficiary);

        subjectivePreEntitlementByTxId[txId] = SubjectivePreEntitlement({
            sponsor: msg.sender, leaseId: leaseId, rawAmount: rawAmount, queueIndex: queueIndex, claimId: claimId
        });

        _emitSubjectivePreEntitleClaimCreated(leaseId, claimId, p, queueIndex, netOut, txId, msg.sender, rawAmount);
    }

    /* solhint-enable function-max-lines */
}
