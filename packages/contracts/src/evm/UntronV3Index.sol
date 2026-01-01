// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EventChainGenesis} from "../utils/EventChainGenesis.sol";

/// @title  UntronV3Index
/// @notice Hash-chain-based event index for Untron V3 hub, friendly to offchain indexers.
/// @dev    UntronV3 must not emit events itself. All events must be defined and emitted through UntronV3Index.
/// @author Ultrasound Labs
contract UntronV3Index {
    // Protocol PnL update reason codes.
    enum PnlReason {
        FEE, // positive
        REBALANCE, // positive if rebalanced at <0 bps fee, negative otherwise
        WITHDRAWAL, // negative
        RECEIVER_PULL_NO_LEASE, // positive
        CONTROLLER_USDT_TRANSFER, // negative
        DEPOSIT // positive
    }

    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = EventChainGenesis.UntronV3Index;

    /// @notice Monotonically increasing sequence number for appended events.
    /// @dev    Increments exactly once per `_appendEventChain` call.
    uint256 public eventSeq;

    // TODO: make per-event sig or per-object event chains

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the USDT token address is set.
    /// @param usdt The USDT token address.
    event UsdtSet(address indexed usdt);

    /// @notice Emitted when a realtor is added/removed from the allowlist.
    /// @param realtor The realtor address.
    /// @param allowed Whether the realtor is allowed.
    event RealtorSet(address indexed realtor, bool allowed);

    /// @notice Emitted when a target chain is marked (or unmarked) as deprecated.
    /// @param targetChainId The target chain id.
    /// @param deprecated Whether the chain is deprecated.
    event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);

    /// @notice Emitted when the protocol floor (in PPM) is set.
    /// @param floorPpm The floor in parts-per-million.
    event ProtocolFloorSet(uint256 floorPpm);

    /// @notice Emitted when the protocol flat fee floor is set.
    /// @param floorFlatFee The floor flat fee in USDT units.
    event ProtocolFlatFeeFloorSet(uint64 floorFlatFee);

    /// @notice Emitted when the protocol max lease duration is set.
    /// @param maxLeaseDurationSeconds The max lease duration in seconds (0 disables).
    event ProtocolMaxLeaseDurationSet(uint32 maxLeaseDurationSeconds);

    /// @notice Emitted when a realtor minimum fee (in PPM) is set.
    /// @param realtor The realtor address.
    /// @param minFeePpm The minimum fee in parts-per-million.
    event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);

    /// @notice Emitted when a realtor minimum flat fee (in USDT units) is set.
    /// @param realtor The realtor address.
    /// @param minFlatFee The minimum flat fee in USDT units.
    event RealtorMinFlatFeeSet(address indexed realtor, uint64 minFlatFee);

    /// @notice Emitted when a realtor max lease duration is set.
    /// @param realtor The realtor address.
    /// @param maxLeaseDurationSeconds The max lease duration in seconds (0 disables).
    event RealtorMaxLeaseDurationSet(address indexed realtor, uint32 maxLeaseDurationSeconds);

    /// @notice Emitted when a realtor-specific lease rate limit is set.
    /// @param realtor The realtor address.
    /// @param maxLeases The maximum number of leases allowed in the window.
    /// @param windowSeconds The size of the rolling window in seconds.
    event RealtorLeaseRateLimitSet(address indexed realtor, uint256 maxLeases, uint256 windowSeconds);

    /// @notice Emitted when the lessee payout config update rate limit is set.
    /// @param maxUpdates The maximum number of updates allowed in the window.
    /// @param windowSeconds The size of the rolling window in seconds.
    event LesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds);

    /// @notice Emitted when the Tron USDT token address is set.
    /// @param tronUsdt The Tron USDT token address (as an EVM address representation).
    event TronUsdtSet(address indexed tronUsdt);

    /// @notice Emitted when a swap rate (in PPM) is set for a target token.
    /// @param targetToken The target token address.
    /// @param ratePpm The swap rate in parts-per-million.
    event SwapRateSet(address indexed targetToken, uint256 ratePpm);

    /// @notice Emitted when a bridger is set for a target token and chain.
    /// @param targetToken The target token address.
    /// @param targetChainId The target chain id.
    /// @param bridger The bridger address.
    event BridgerSet(address indexed targetToken, uint256 indexed targetChainId, address bridger);

    /// @notice Emitted when a lease is created.
    /// @param leaseId The lease id.
    /// @param receiverSalt The receiver salt used to derive a receiver address.
    /// @param realtor The realtor address.
    /// @param lessee The lessee address.
    /// @param startTime The start timestamp for the lease.
    /// @param nukeableAfter The timestamp after which the lease is nukeable.
    /// @param leaseFeePpm The lease fee in parts-per-million.
    /// @param flatFee The flat fee amount.
    event LeaseCreated(
        uint256 indexed leaseId,
        bytes32 indexed receiverSalt,
        address realtor,
        address lessee,
        uint64 startTime,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee
    );

    /// @notice Emitted when a lease payout configuration is updated.
    /// @param leaseId The lease id.
    /// @param targetChainId The target chain id.
    /// @param targetToken The target token address.
    /// @param beneficiary The beneficiary address.
    event PayoutConfigUpdated(uint256 indexed leaseId, uint256 targetChainId, address targetToken, address beneficiary);

    // forge-lint: disable-next-line(mixed-case-variable)
    /// @notice Emitted when a claim is created.
    /// @param leaseId The lease id.
    /// @param claimId The per-lease claim identifier (0-indexed).
    /// @param targetToken The target token used for settlement of the claim queue.
    /// @param queueIndex The claim index within the per-`targetToken` queue.
    /// @param amountUsdt The claim amount in USDT units.
    /// @param targetChainId Destination chain for this claim's payout.
    /// @param beneficiary Recipient of the payout (either local transfer or bridged recipient).
    event ClaimCreated(
        uint256 indexed leaseId,
        uint256 indexed claimId,
        address targetToken,
        uint256 queueIndex,
        uint256 amountUsdt,
        uint256 targetChainId,
        address beneficiary
    );
    // forge-lint: disable-next-line(mixed-case-variable)
    /// @notice Emitted when a claim is filled.
    /// @param leaseId The lease id.
    /// @param claimId The per-lease claim identifier (0-indexed).
    /// @param targetToken The target token used for settlement of the claim queue.
    /// @param queueIndex The claim index within the per-`targetToken` queue.
    /// @param amountUsdt The filled amount in USDT units.
    /// @param targetChainId Destination chain for this claim's payout.
    /// @param beneficiary Recipient of the payout (either local transfer or bridged recipient).
    event ClaimFilled(
        uint256 indexed leaseId,
        uint256 indexed claimId,
        address targetToken,
        uint256 queueIndex,
        uint256 amountUsdt,
        uint256 targetChainId,
        address beneficiary
    );

    /// @notice Emitted when a deposit is pre-entitled to a lease.
    /// @param txId The deposit transaction id.
    /// @param leaseId The lease id.
    /// @param rawAmount The raw deposit amount (before fees).
    /// @param netOut The net amount out (after fees).
    event DepositPreEntitled(bytes32 indexed txId, uint256 indexed leaseId, uint256 rawAmount, uint256 netOut);

    /// @notice Emitted when an LP deposits funds.
    /// @param lp The LP address.
    /// @param amount The amount deposited.
    event LpDeposited(address indexed lp, uint256 amount);

    /// @notice Emitted when an LP withdraws funds.
    /// @param lp The LP address.
    /// @param amount The amount withdrawn.
    event LpWithdrawn(address indexed lp, uint256 amount);

    /// @notice Emitted when the Tron reader address is set.
    /// @param reader The Tron reader address.
    event TronReaderSet(address indexed reader);

    /// @notice Emitted when a controller event chain tip update is recorded.
    /// @param previousTip The controller chain tip prior to the update.
    /// @param blockNumber The block number where the controller update occurred.
    /// @param blockTimestamp The block timestamp where the controller update occurred.
    /// @param eventSignature The controller event signature hash (topic0).
    /// @param abiEncodedEventData The ABI-encoded controller event data.
    event ControllerEventChainTipUpdated(
        bytes32 previousTip,
        uint256 indexed blockNumber,
        uint256 blockTimestamp,
        bytes32 indexed eventSignature,
        bytes abiEncodedEventData
    );

    /// @notice Emitted when a queued controller-side event is processed on the EVM side.
    /// @dev Emitted by `processControllerEvents` for each consumed queue item (including ignored/unknown signatures).
    /// @param eventIndex The index in the controller-event processing queue that was consumed.
    /// @param blockNumber The block number where the controller event occurred (as provided by the relayed event).
    /// @param blockTimestamp The block timestamp where the controller event occurred (as provided by the relayed event).
    /// @param eventSignature The controller event signature hash (topic0).
    /// @param abiEncodedEventData The ABI-encoded controller event data.
    event ControllerEventProcessed(
        uint256 indexed eventIndex,
        uint256 indexed blockNumber,
        uint256 blockTimestamp,
        bytes32 indexed eventSignature,
        bytes abiEncodedEventData
    );

    /// @notice Emitted when protocol PnL is updated.
    /// @param pnl The current total protocol PnL.
    /// @param delta The change applied to the PnL.
    /// @param reason The reason code for the update.
    event ProtocolPnlUpdated(int256 pnl, int256 delta, PnlReason reason);

    /// @notice Emitted when a lease nonce is updated.
    /// @param leaseId The lease id.
    /// @param nonce The new nonce.
    event LeaseNonceUpdated(uint256 indexed leaseId, uint256 nonce);

    /// @notice Emitted when tokens are rescued from the protocol.
    /// @param token The token address.
    /// @param amount The amount rescued.
    event TokensRescued(address token, uint256 amount);

    /// @dev The ownership is transferred from `oldOwner` to `newOwner`.
    /// This event is intentionally kept the same as OpenZeppelin's Ownable to be
    /// compatible with indexers and [EIP-173](https://eips.ethereum.org/EIPS/eip-173),
    /// despite it not being as lightweight as a single argument event.
    /// @notice Emitted when contract ownership is transferred.
    /// @param oldOwner The previous owner.
    /// @param newOwner The new owner.
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature of the event.
    /// @param abiEncodedEventData The ABI-encoded data of the event.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        unchecked {
            ++eventSeq;
        }
        eventChainTip = sha256(
            abi.encodePacked(
                eventChainTip, eventSeq, block.number, block.timestamp, eventSignature, abiEncodedEventData
            )
        );
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emits {UsdtSet} and appends it to the event chain.
    /// @param usdt_ The USDT token address.
    function _emitUsdtSet(address usdt_) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(usdt_));
        emit UsdtSet(usdt_);
    }

    /// @notice Emits {RealtorSet} and appends it to the event chain.
    /// @param realtor The realtor address.
    /// @param allowed Whether the realtor is allowed.
    function _emitRealtorSet(address realtor, bool allowed) internal {
        _appendEventChain(RealtorSet.selector, abi.encode(realtor, allowed));
        emit RealtorSet(realtor, allowed);
    }

    /// @notice Emits {ChainDeprecatedSet} and appends it to the event chain.
    /// @param targetChainId The target chain id.
    /// @param deprecated Whether the chain is deprecated.
    function _emitChainDeprecatedSet(uint256 targetChainId, bool deprecated) internal {
        _appendEventChain(ChainDeprecatedSet.selector, abi.encode(targetChainId, deprecated));
        emit ChainDeprecatedSet(targetChainId, deprecated);
    }

    /// @notice Emits {ProtocolFloorSet} and appends it to the event chain.
    /// @param floorPpm The floor in parts-per-million.
    function _emitProtocolFloorSet(uint256 floorPpm) internal {
        _appendEventChain(ProtocolFloorSet.selector, abi.encode(floorPpm));
        emit ProtocolFloorSet(floorPpm);
    }

    /// @notice Emits {ProtocolFlatFeeFloorSet} and appends it to the event chain.
    /// @param floorFlatFee The floor flat fee in USDT units.
    function _emitProtocolFlatFeeFloorSet(uint64 floorFlatFee) internal {
        _appendEventChain(ProtocolFlatFeeFloorSet.selector, abi.encode(floorFlatFee));
        emit ProtocolFlatFeeFloorSet(floorFlatFee);
    }

    /// @notice Emits {ProtocolMaxLeaseDurationSet} and appends it to the event chain.
    /// @param maxLeaseDurationSeconds The max lease duration in seconds (0 disables).
    function _emitProtocolMaxLeaseDurationSet(uint32 maxLeaseDurationSeconds) internal {
        _appendEventChain(ProtocolMaxLeaseDurationSet.selector, abi.encode(maxLeaseDurationSeconds));
        emit ProtocolMaxLeaseDurationSet(maxLeaseDurationSeconds);
    }

    /// @notice Emits {RealtorMinFeeSet} and appends it to the event chain.
    /// @param realtor The realtor address.
    /// @param minFeePpm The minimum fee in parts-per-million.
    function _emitRealtorMinFeeSet(address realtor, uint256 minFeePpm) internal {
        _appendEventChain(RealtorMinFeeSet.selector, abi.encode(realtor, minFeePpm));
        emit RealtorMinFeeSet(realtor, minFeePpm);
    }

    /// @notice Emits {RealtorMinFlatFeeSet} and appends it to the event chain.
    /// @param realtor The realtor address.
    /// @param minFlatFee The minimum flat fee in USDT units.
    function _emitRealtorMinFlatFeeSet(address realtor, uint64 minFlatFee) internal {
        _appendEventChain(RealtorMinFlatFeeSet.selector, abi.encode(realtor, minFlatFee));
        emit RealtorMinFlatFeeSet(realtor, minFlatFee);
    }

    /// @notice Emits {RealtorMaxLeaseDurationSet} and appends it to the event chain.
    /// @param realtor The realtor address.
    /// @param maxLeaseDurationSeconds The max lease duration in seconds (0 disables).
    function _emitRealtorMaxLeaseDurationSet(address realtor, uint32 maxLeaseDurationSeconds) internal {
        _appendEventChain(RealtorMaxLeaseDurationSet.selector, abi.encode(realtor, maxLeaseDurationSeconds));
        emit RealtorMaxLeaseDurationSet(realtor, maxLeaseDurationSeconds);
    }

    /// @notice Emits {LesseePayoutConfigRateLimitSet} and appends it to the event chain.
    /// @param maxUpdates The maximum number of updates allowed in the window.
    /// @param windowSeconds The size of the rolling window in seconds.
    function _emitLesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds) internal {
        _appendEventChain(LesseePayoutConfigRateLimitSet.selector, abi.encode(maxUpdates, windowSeconds));
        emit LesseePayoutConfigRateLimitSet(maxUpdates, windowSeconds);
    }

    /// @notice Emits {RealtorLeaseRateLimitSet} and appends it to the event chain.
    /// @param realtor The realtor address.
    /// @param maxLeases The maximum number of leases allowed in the window.
    /// @param windowSeconds The size of the rolling window in seconds.
    function _emitRealtorLeaseRateLimitSet(address realtor, uint256 maxLeases, uint256 windowSeconds) internal {
        _appendEventChain(RealtorLeaseRateLimitSet.selector, abi.encode(realtor, maxLeases, windowSeconds));
        emit RealtorLeaseRateLimitSet(realtor, maxLeases, windowSeconds);
    }

    /// @notice Emits {TronReaderSet} and appends it to the event chain.
    /// @param reader The Tron reader address.
    function _emitTronReaderSet(address reader) internal {
        _appendEventChain(TronReaderSet.selector, abi.encode(reader));
        emit TronReaderSet(reader);
    }

    /// @notice Emits {TronUsdtSet} and appends it to the event chain.
    /// @param tronUsdt The Tron USDT token address (as an EVM address representation).
    function _emitTronUsdtSet(address tronUsdt) internal {
        _appendEventChain(TronUsdtSet.selector, abi.encode(tronUsdt));
        emit TronUsdtSet(tronUsdt);
    }

    /// @notice Emits {SwapRateSet} and appends it to the event chain.
    /// @param targetToken The target token address.
    /// @param ratePpm The swap rate in parts-per-million.
    function _emitSwapRateSet(address targetToken, uint256 ratePpm) internal {
        _appendEventChain(SwapRateSet.selector, abi.encode(targetToken, ratePpm));
        emit SwapRateSet(targetToken, ratePpm);
    }

    /// @notice Emits {BridgerSet} and appends it to the event chain.
    /// @param targetToken The target token address.
    /// @param targetChainId The target chain id.
    /// @param bridger The bridger address.
    function _emitBridgerSet(address targetToken, uint256 targetChainId, address bridger) internal {
        _appendEventChain(BridgerSet.selector, abi.encode(targetToken, targetChainId, bridger));
        emit BridgerSet(targetToken, targetChainId, bridger);
    }

    /// @notice Emits {LeaseCreated} and appends it to the event chain.
    /// @param leaseId The lease id.
    /// @param receiverSalt The receiver salt used to derive a receiver address.
    /// @param realtor The realtor address.
    /// @param lessee The lessee address.
    /// @param startTime The start timestamp for the lease.
    /// @param nukeableAfter The timestamp after which the lease is nukeable.
    /// @param leaseFeePpm The lease fee in parts-per-million.
    /// @param flatFee The flat fee amount.
    function _emitLeaseCreated(
        uint256 leaseId,
        bytes32 receiverSalt,
        address realtor,
        address lessee,
        uint64 startTime,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee
    ) internal {
        _appendEventChain(
            LeaseCreated.selector,
            abi.encode(leaseId, receiverSalt, realtor, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee)
        );
        emit LeaseCreated(leaseId, receiverSalt, realtor, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee);
    }

    /// @notice Emits {PayoutConfigUpdated} and appends it to the event chain.
    /// @param leaseId The lease id.
    /// @param targetChainId The target chain id.
    /// @param targetToken The target token address.
    /// @param beneficiary The beneficiary address.
    function _emitPayoutConfigUpdated(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        internal
    {
        _appendEventChain(PayoutConfigUpdated.selector, abi.encode(leaseId, targetChainId, targetToken, beneficiary));
        emit PayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    /// @notice Emits {ControllerEventChainTipUpdated} and appends it to the event chain.
    /// @param previousTip The controller chain tip prior to the update.
    /// @param blockNumber The block number where the controller update occurred.
    /// @param blockTimestamp The block timestamp where the controller update occurred.
    /// @param eventSignature The controller event signature hash (topic0).
    /// @param abiEncodedEventData The ABI-encoded controller event data.
    function _emitControllerEventChainTipUpdated(
        bytes32 previousTip,
        uint256 blockNumber,
        uint256 blockTimestamp,
        bytes32 eventSignature,
        bytes memory abiEncodedEventData
    ) internal {
        _appendEventChain(
            ControllerEventChainTipUpdated.selector,
            abi.encode(previousTip, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData)
        );
        emit ControllerEventChainTipUpdated(
            previousTip, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData
        );
    }

    /// @notice Emits {ControllerEventProcessed} and appends it to the event chain.
    /// @param eventIndex The index in the controller-event processing queue that was consumed.
    /// @param blockNumber The block number where the controller event occurred.
    /// @param blockTimestamp The block timestamp where the controller event occurred.
    /// @param eventSignature The controller event signature hash (topic0).
    /// @param abiEncodedEventData The ABI-encoded controller event data.
    function _emitControllerEventProcessed(
        uint256 eventIndex,
        uint256 blockNumber,
        uint256 blockTimestamp,
        bytes32 eventSignature,
        bytes memory abiEncodedEventData
    ) internal {
        _appendEventChain(
            ControllerEventProcessed.selector,
            abi.encode(eventIndex, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData)
        );
        emit ControllerEventProcessed(eventIndex, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    /// @notice Emits {ClaimCreated} and appends it to the event chain.
    /// @param leaseId The lease id.
    /// @param claimId The per-lease claim identifier (0-indexed).
    /// @param targetToken The target token used for settlement of the claim queue.
    /// @param queueIndex The claim index within the per-`targetToken` queue.
    /// @param amountUsdt The claim amount in USDT units.
    /// @param targetChainId Destination chain for this claim's payout.
    /// @param beneficiary Recipient of the payout (either local transfer or bridged recipient).
    function _emitClaimCreated(
        uint256 leaseId,
        uint256 claimId,
        address targetToken,
        uint256 queueIndex,
        uint256 amountUsdt,
        uint256 targetChainId,
        address beneficiary
    ) internal {
        _appendEventChain(
            ClaimCreated.selector,
            abi.encode(leaseId, claimId, targetToken, queueIndex, amountUsdt, targetChainId, beneficiary)
        );
        emit ClaimCreated(leaseId, claimId, targetToken, queueIndex, amountUsdt, targetChainId, beneficiary);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    /// @notice Emits {ClaimFilled} and appends it to the event chain.
    /// @param leaseId The lease id.
    /// @param claimId The per-lease claim identifier (0-indexed).
    /// @param targetToken The target token used for settlement of the claim queue.
    /// @param queueIndex The claim index within the per-`targetToken` queue.
    /// @param amountUsdt The filled amount in USDT units.
    /// @param targetChainId Destination chain for this claim's payout.
    /// @param beneficiary Recipient of the payout (either local transfer or bridged recipient).
    function _emitClaimFilled(
        uint256 leaseId,
        uint256 claimId,
        address targetToken,
        uint256 queueIndex,
        uint256 amountUsdt,
        uint256 targetChainId,
        address beneficiary
    ) internal {
        _appendEventChain(
            ClaimFilled.selector,
            abi.encode(leaseId, claimId, targetToken, queueIndex, amountUsdt, targetChainId, beneficiary)
        );
        emit ClaimFilled(leaseId, claimId, targetToken, queueIndex, amountUsdt, targetChainId, beneficiary);
    }

    /// @notice Emits {DepositPreEntitled} and appends it to the event chain.
    /// @param txId The deposit transaction id.
    /// @param leaseId The lease id.
    /// @param rawAmount The raw deposit amount (before fees).
    /// @param netOut The net amount out (after fees).
    function _emitDepositPreEntitled(bytes32 txId, uint256 leaseId, uint256 rawAmount, uint256 netOut) internal {
        _appendEventChain(DepositPreEntitled.selector, abi.encode(txId, leaseId, rawAmount, netOut));
        emit DepositPreEntitled(txId, leaseId, rawAmount, netOut);
    }

    /// @notice Emits {LpDeposited} and appends it to the event chain.
    /// @param lp The LP address.
    /// @param amount The amount deposited.
    function _emitLpDeposited(address lp, uint256 amount) internal {
        _appendEventChain(LpDeposited.selector, abi.encode(lp, amount));
        emit LpDeposited(lp, amount);
    }

    /// @notice Emits {LpWithdrawn} and appends it to the event chain.
    /// @param lp The LP address.
    /// @param amount The amount withdrawn.
    function _emitLpWithdrawn(address lp, uint256 amount) internal {
        _appendEventChain(LpWithdrawn.selector, abi.encode(lp, amount));
        emit LpWithdrawn(lp, amount);
    }

    /// @notice Emits {ProtocolPnlUpdated} and appends it to the event chain.
    /// @param pnl The current total protocol PnL.
    /// @param delta The change applied to the PnL.
    /// @param reason The reason code for the update.
    function _emitProtocolPnlUpdated(int256 pnl, int256 delta, PnlReason reason) internal {
        _appendEventChain(ProtocolPnlUpdated.selector, abi.encode(pnl, delta, reason));
        emit ProtocolPnlUpdated(pnl, delta, reason);
    }

    /// @notice Emits {LeaseNonceUpdated} and appends it to the event chain.
    /// @param leaseId The lease id.
    /// @param nonce The new nonce.
    function _emitLeaseNonceUpdated(uint256 leaseId, uint256 nonce) internal {
        _appendEventChain(LeaseNonceUpdated.selector, abi.encode(leaseId, nonce));
        emit LeaseNonceUpdated(leaseId, nonce);
    }

    /// @notice Emits {TokensRescued} and appends it to the event chain.
    /// @param token The token address.
    /// @param amount The amount rescued.
    function _emitTokensRescued(address token, uint256 amount) internal {
        _appendEventChain(TokensRescued.selector, abi.encode(token, amount));
        emit TokensRescued(token, amount);
    }

    /// @notice Emits {OwnershipTransferred} and appends it to the event chain.
    /// @param oldOwner The previous owner.
    /// @param newOwner The new owner.
    function _emitOwnershipTransferred(address oldOwner, address newOwner) internal {
        _appendEventChain(OwnershipTransferred.selector, abi.encode(oldOwner, newOwner));
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}
