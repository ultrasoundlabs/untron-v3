// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {ITronTxReader} from "../interfaces/ITronTxReader.sol";
import {TronCalldataUtils} from "../../utils/TronCalldataUtils.sol";

/// @title UntronV3 controller facet
/// @notice Tron controller event-chain relay and processing.
/// @author Ultrasound Labs
contract UntronV3ControllerFacet is UntronV3Base {
    /// @notice Verify a Tron tx containing isEventChainTip (plain or inside multicall) and enqueue controller events.
    /// @dev This function is the bridge between the Tron-side controller's event hash-chain and this EVM contract.
    ///
    /// The Tron-side controller maintains an event hash-chain (a "tip"). To accept new events here, we require:
    /// 1) A proved Tron transaction that calls `isEventChainTip(bytes32)` on the controller (directly or via multicall),
    ///    establishing that `tipNew` is a valid controller tip at that Tron block.
    /// 2) A relayer-provided list of `events` such that iteratively hashing them from `lastControllerEventTip` yields
    ///    exactly `tipNew`.
    ///
    /// Important: This does NOT verify that each provided event corresponds to an actual Tron log. Instead, the
    /// security comes from the controller attesting that `tipNew` is valid; the hash-link check ensures the relayer
    /// cannot provide a different event sequence for the same `tipNew`.
    ///
    /// Side effects:
    /// - Emits a `ControllerEventChainTipUpdated` event for each provided event (for offchain indexing).
    /// - Appends each event into `_controllerEvents` for later processing via `processControllerEvents`.
    /// - Updates `lastControllerEventTip`.
    ///
    /// @param blocks 20 Protobuf-encoded Tron block headers (first contains the tx).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @param events Controller events that should extend the local tip to `tipNew`.
    /// @return tipNew The new controller event-chain tip proven by the Tron transaction.
    function relayControllerEventChain(
        bytes[20] calldata blocks,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index,
        ControllerEvent[] calldata events
    ) external whenNotPaused returns (bytes32 tipNew) {
        // Verify inclusion + success and decode into a generic TriggerSmartContract view.
        ITronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(blocks, encodedTx, proof, index);

        // Validate that the call is targeting the expected UntronController contract on Tron.
        bytes21 controllerTron = TronCalldataUtils.evmToTronAddress(CONTROLLER_ADDRESS);
        if (callData.toTron != controllerTron) revert NotEventChainTip();

        // Decode the new tip from either a direct `isEventChainTip` call or a multicall wrapper.
        tipNew = _decodeEventChainTip(callData.data);

        // Ensure progress (avoid accepting a no-op relay).
        bytes32 tip = lastControllerEventTip;
        if (tipNew == tip) revert EventRelayNoProgress();

        uint256 seq = lastControllerEventSeq;
        (bytes32 computedTip, uint256 seqNew) = _hashLinkControllerEventsAndEmit(tip, seq, events);
        if (computedTip != tipNew) revert EventTipMismatch();

        // Commit the new tip.
        lastControllerEventTip = tipNew;
        lastControllerEventSeq = seqNew;

        // Enqueue raw events for later processing (separated to allow partial processing/batching).
        _enqueueControllerEvents(events);
    }

    /// @notice Process up to `maxEvents` queued controller events.
    /// @dev Applies only events UntronV3 cares about; unknown events are skipped but still advance the cursor.
    /// This "cursor always advances" design ensures relayers cannot permanently DoS processing by inserting
    /// unknown events: the contract will simply ignore them and continue.
    /// @param maxEvents Maximum number of queued events to process in this call.
    function processControllerEvents(uint256 maxEvents) external whenNotPaused {
        // Snapshot cursor and bounds.
        uint256 idx = nextControllerEventIndex;
        uint256 end = _controllerEvents.length;
        uint256 processed;

        // Iterate sequentially until we hit either the end or the per-call processing limit.
        while (idx < end && processed < maxEvents) {
            ControllerEvent storage ev = _controllerEvents[idx];
            bytes32 sig = ev.sig;

            if (sig == _EVENT_SIG_PULLED_FROM_RECEIVER) {
                // Controller indicates funds were pulled out of a receiver and converted into USDT amount.
                (bytes32 receiverSalt, address token,/*tokenAmount*/,/* exchangeRate */, uint256 usdtAmount) =
                    abi.decode(ev.data, (bytes32, address, uint256, uint256, uint256));
                _processReceiverPulled(receiverSalt, token, usdtAmount, ev.blockTimestamp);
            } else if (sig == _EVENT_SIG_USDT_SET) {
                // Controller indicates the Tron USDT contract address has changed.
                address newTronUsdt = abi.decode(ev.data, (address));
                tronUsdt = newTronUsdt;
                _emitTronUsdtSet(newTronUsdt);
            } else if (sig == _EVENT_SIG_USDT_REBALANCED) {
                // Controller indicates a rebalance and reports the in/out amounts.
                (
                    uint256 inAmount,
                    uint256 outAmount, /*rebalancer*/
                ) = abi.decode(ev.data, (uint256, uint256, address));
                // solhint-disable-next-line gas-strict-inequalities
                int256 delta = outAmount >= inAmount ? _toInt(outAmount - inAmount) : -_toInt(inAmount - outAmount);
                _applyPnlDelta(delta, PnlReason.REBALANCE);
            } else if (sig == _EVENT_SIG_CONTROLLER_USDT_TRANSFER) {
                // Controller indicates USDT was transferred out (executor spend); treat as negative protocol PnL.
                (/*recipient*/, uint256 amount) = abi.decode(ev.data, (address, uint256));
                _applyPnlDelta(-_toInt(amount), PnlReason.CONTROLLER_USDT_TRANSFER);
            }

            // Emit an index event for every consumed controller event (including ignored/unknown signatures).
            _emitControllerEventProcessed(idx, ev.blockNumber, ev.blockTimestamp, sig, ev.data);

            unchecked {
                // Cursor and processed counters only increase; unchecked saves gas.
                ++idx;
                ++processed;
            }
        }

        // Commit updated cursor.
        nextControllerEventIndex = idx;
    }
}
