// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {ITronTxReader} from "../interfaces/ITronTxReader.sol";
import {IBridger} from "../bridgers/interfaces/IBridger.sol";
import {TokenUtils} from "../../utils/TokenUtils.sol";

/// @title UntronV3 admin facet
/// @notice Owner-only configuration and safety controls for UntronV3.
/// @author Ultrasound Labs
contract UntronV3AdminFacet is UntronV3Base {
    /// @notice Set the protocol accounting token (USDT) contract address on this chain.
    /// @dev This token is used for:
    /// - LP vault deposits/withdrawals,
    /// - claim accounting (`Claim.amountUsdt`),
    /// - protocol profit withdrawals (`withdrawProtocolProfit`).
    ///
    /// This is intentionally not immutable ("ship of Theseus"): leases can last for a long time and the
    /// protocol may need to migrate off a specific USDT representation.
    /// @param usdt_ ERC-20 token address used as USDT accounting unit on this chain.
    function setUsdt(address usdt_) external onlyOwner {
        // Update the accounting token address.
        usdt = usdt_;

        // Emit via UntronV3Index (and append to event chain).
        _emitUsdtSet(usdt_);
    }

    /// @notice Whitelist or un-whitelist a realtor address.
    /// @dev Realtors are the only accounts allowed to call `createLease`.
    /// @param realtor Address whose realtor permission is being updated.
    /// @param allowed Whether the address is allowed to create leases.
    function setRealtor(address realtor, bool allowed) external onlyOwner {
        // Update allowlist.
        isRealtor[realtor] = allowed;

        // Emit via UntronV3Index (and append to event chain).
        _emitRealtorSet(realtor, allowed);
    }

    /// @notice Add or remove an address from the LP allowlist.
    /// @dev This permission gates `deposit(...)` only.
    ///
    /// Safety invariant:
    /// - Delisting an LP MUST NOT prevent them from withdrawing already-deposited principal.
    ///   (Accordingly, `withdraw(...)` never checks `isLpAllowed`.)
    ///
    /// Operational notes:
    /// - Allowlisting is checked at call-time: an address can deposit while allowlisted, then later be delisted,
    ///   and it can still withdraw any remaining `lpPrincipal[lp]`.
    /// - This function does not attempt to infer or validate "LP-ness" from balances; it is purely a permission flag.
    /// @param lp Address whose LP deposit permission is being updated.
    /// @param allowed Whether the address is allowed to deposit into the LP vault.
    function setLp(address lp, bool allowed) external onlyOwner {
        isLpAllowed[lp] = allowed;
        _emitLpSet(lp, allowed);
    }

    /// @notice Mark a destination chain as deprecated or not.
    /// @dev If a chain is deprecated, lessees can no longer set it in payout configs (existing configs remain).
    /// @param targetChainId Destination chain id to mark.
    /// @param deprecated True to deprecate; false to un-deprecate.
    function setChainDeprecated(uint256 targetChainId, bool deprecated) external onlyOwner {
        // Update deprecation flag.
        isChainDeprecated[targetChainId] = deprecated;

        // Emit via UntronV3Index (and append to event chain).
        _emitChainDeprecatedSet(targetChainId, deprecated);
    }

    /// @notice Set the protocol-wide minimum fee floor, in parts-per-million (ppm).
    /// @dev Effective minimum fee for a given realtor is `max(protocolFloorPpm, realtorMinFeePpm(realtor))`.
    /// @param floorPpm Fee floor in ppm, where `1_000_000` means 100%.
    function setProtocolFloorPpm(uint256 floorPpm) external onlyOwner {
        // Bound check: fee cannot exceed 100%.
        if (floorPpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();
        // Casting to 'uint32' is safe because floorPpm <= 1_000_000.
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.floorPpm = uint32(floorPpm);

        // Emit via UntronV3Index (and append to event chain).
        _emitProtocolFloorSet(floorPpm);
    }

    /// @notice Set the protocol-wide minimum flat fee floor, in USDT units.
    /// @dev Effective minimum for a given realtor is `max(protocolFloorFlatFee, realtorMinFlatFee(realtor))`.
    /// @param floorFlatFee Fee floor in USDT units.
    function setProtocolFloorFlatFee(uint64 floorFlatFee) external onlyOwner {
        _protocolConfig.floorFlatFee = floorFlatFee;
        _emitProtocolFlatFeeFloorSet(floorFlatFee);
    }

    /// @notice Set the protocol-wide maximum lease duration in seconds.
    /// @dev Duration is measured as `nukeableAfter - startTime`. If set to 0, max duration is disabled.
    /// @param maxLeaseDurationSeconds Maximum allowed duration in seconds (0 disables).
    function setProtocolMaxLeaseDurationSeconds(uint32 maxLeaseDurationSeconds) external onlyOwner {
        _protocolConfig.maxLeaseDurationSeconds = maxLeaseDurationSeconds;
        _emitProtocolMaxLeaseDurationSet(maxLeaseDurationSeconds);
    }

    /// @notice Set the realtor-specific minimum fee floor, in parts-per-million (ppm).
    /// @dev This can only increase the effective minimum fee for leases created by `realtor`.
    /// @param realtor Realtor whose override is being set.
    /// @param minFeePpm Fee floor in ppm, where `1_000_000` means 100%.
    function setRealtorMinFeePpm(address realtor, uint256 minFeePpm) external onlyOwner {
        // Bound check: fee cannot exceed 100%.
        if (minFeePpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();
        // Casting to 'uint32' is safe because minFeePpm <= 1_000_000.
        // forge-lint: disable-next-line(unsafe-typecast)
        _realtorConfig[realtor].minFeePpm = uint32(minFeePpm);
        _emitRealtorMinFeeSet(realtor, minFeePpm);
    }

    /// @notice Set the realtor-specific minimum flat fee floor, in USDT units.
    /// @dev This can only increase the effective minimum flat fee for leases created by `realtor`.
    /// @param realtor Realtor whose override is being set.
    /// @param minFlatFee Flat fee floor in USDT units.
    function setRealtorMinFlatFee(address realtor, uint64 minFlatFee) external onlyOwner {
        _realtorConfig[realtor].minFlatFee = minFlatFee;
        _emitRealtorMinFlatFeeSet(realtor, minFlatFee);
    }

    /// @notice Set the realtor-specific maximum lease duration in seconds.
    /// @dev Duration is measured as `nukeableAfter - startTime`. If set to 0, no max duration is applied.
    /// @param realtor Realtor whose override is being set.
    /// @param maxLeaseDurationSeconds Maximum allowed duration in seconds (0 disables).
    function setRealtorMaxLeaseDurationSeconds(address realtor, uint32 maxLeaseDurationSeconds) external onlyOwner {
        _realtorConfig[realtor].maxLeaseDurationSeconds = maxLeaseDurationSeconds;
        _emitRealtorMaxLeaseDurationSet(realtor, maxLeaseDurationSeconds);
    }

    /// @notice Configure payout-config update rate limiting for lessees.
    /// @dev If either param is 0, the rate limit is disabled (both must be 0 together).
    /// @param maxUpdates Maximum number of payout config updates allowed per window.
    /// @param windowSeconds Window size in seconds.
    function setLesseePayoutConfigRateLimit(uint256 maxUpdates, uint256 windowSeconds) external onlyOwner {
        if (maxUpdates > type(uint32).max || windowSeconds > type(uint32).max) {
            revert PayoutConfigRateLimitConfigInvalid();
        }
        if ((maxUpdates == 0) != (windowSeconds == 0)) revert PayoutConfigRateLimitConfigInvalid();
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitMaxUpdates = uint32(maxUpdates);
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitWindowSeconds = uint32(windowSeconds);
        _emitLesseePayoutConfigRateLimitSet(maxUpdates, windowSeconds);
    }

    /// @notice Configure lease creation rate limiting for a realtor.
    /// @dev If either param is 0, the rate limit is disabled (both must be 0 together).
    /// @param realtor Realtor whose rate limit is being set.
    /// @param maxLeases Maximum number of lease creations allowed per window.
    /// @param windowSeconds Window size in seconds.
    function setRealtorLeaseRateLimit(address realtor, uint256 maxLeases, uint256 windowSeconds) external onlyOwner {
        if (maxLeases > type(uint32).max || windowSeconds > type(uint32).max) revert LeaseRateLimitConfigInvalid();
        if ((maxLeases == 0) != (windowSeconds == 0)) revert LeaseRateLimitConfigInvalid();

        RealtorConfig storage cfg = _realtorConfig[realtor];

        // forge-lint: disable-next-line(unsafe-typecast)
        cfg.leaseRateLimitMaxLeases = uint32(maxLeases);
        // forge-lint: disable-next-line(unsafe-typecast)
        cfg.leaseRateLimitWindowSeconds = uint32(windowSeconds);

        _emitRealtorLeaseRateLimitSet(realtor, maxLeases, windowSeconds);
    }

    /// @notice Set the Tron reader contract used to verify and decode Tron transactions.
    /// @dev The reader is expected to be bound to a Tron light client and to:
    /// - verify tx inclusion via Merkle proofs,
    /// - enforce tx success,
    /// - and expose sender / to / calldata for `TriggerSmartContract` transactions.
    /// @param reader Address of the new `ITronTxReader` implementation.
    function setTronReader(address reader) external onlyOwner {
        tronReader = ITronTxReader(reader);
        _emitTronReaderSet(reader);
    }

    /// @notice Set the swap rate for a `targetToken`, in parts-per-million of USDT.
    /// @dev The rate is interpreted as `targetToken` units per `_RATE_DENOMINATOR` USDT.
    /// @param targetToken Token to set the swap rate for.
    /// @param ratePpm Rate in ppm of USDT; must be non-zero.
    function setSwapRate(address targetToken, uint256 ratePpm) external onlyOwner {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (ratePpm == 0) revert RateNotSet();

        swapRatePpm[targetToken] = ratePpm;
        _emitSwapRateSet(targetToken, ratePpm);
    }

    /// @notice Register a bridger for a `(targetToken, targetChainId)` pair.
    /// @dev Bridgers are used when filling claims whose payout chain differs from this chain.
    /// @param targetToken Token that will be bridged.
    /// @param targetChainId Destination chain id for the bridger.
    /// @param bridger Bridger contract address implementing `IBridger`.
    function setBridger(address targetToken, uint256 targetChainId, address bridger) external onlyOwner {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (bridger == address(0)) revert NoBridger();

        bridgers[targetToken][targetChainId] = IBridger(bridger);
        _emitBridgerSet(targetToken, targetChainId, bridger);
    }

    /// @notice Pause protocol entrypoints guarded by `whenNotPaused`.
    /// @dev Owner-only circuit breaker; pausing affects lease creation, entitlement, LP operations, fills, etc.
    function pause() external onlyOwner {
        _pause();
    }

    /// @notice Unpause protocol entrypoints guarded by `whenNotPaused`.
    /// @dev Owner-only circuit breaker reset.
    function unpause() external onlyOwner {
        _unpause();
    }

    /// @notice Deposit USDT directly into protocol PnL (accounting-only top-up).
    /// @dev This pulls `usdt` from the caller and increases `protocolPnl` by `amount`.
    /// @param amount Amount of `usdt` to deposit.
    function depositToPnl(uint256 amount) external {
        if (amount == 0) revert ZeroAmount();
        if (usdt == address(0)) revert InvalidTargetToken();

        TokenUtils.transferFrom(usdt, msg.sender, payable(address(this)), amount);
        _applyPnlDelta(_toInt(amount), PnlReason.DEPOSIT);
    }

    /// @notice Withdraw positive protocol PnL (profit) to the owner.
    /// @dev This transfers `usdt` from the contract and decreases `protocolPnl` by `amount`.
    /// @param amount Amount of profit to withdraw (must be > 0 and <= current `protocolPnl`).
    function withdrawProtocolProfit(int256 amount) external onlyOwner {
        // Require a strictly-positive withdrawal request.
        if (amount < 1) revert ZeroAmount();
        // Ensure the protocol has at least this much positive PnL to withdraw.
        if (amount > protocolPnl) revert InsufficientProtocolProfit();
        // Casting to 'uint256' is safe because we reverted negative/zero values above.
        // forge-lint: disable-next-line(unsafe-typecast)
        TokenUtils.transfer(usdt, payable(msg.sender), uint256(amount));
        _applyPnlDelta(-amount, PnlReason.WITHDRAWAL);
    }

    /// @notice Rescue non-USDT tokens accidentally sent to this contract.
    /// @dev This cannot be used to withdraw `usdt` (use `withdrawProtocolProfit` / `withdraw` for that).
    /// @param token Token address to rescue (must not equal `usdt`).
    /// @param amount Amount to transfer to the owner.
    function rescueTokens(address token, uint256 amount) external onlyOwner {
        // Prevent rescuing protocol accounting token.
        if (token == usdt) revert CannotRescueUSDT();

        // Transfer requested amount to the owner.
        TokenUtils.transfer(token, payable(msg.sender), amount);

        // Emit via UntronV3Index (and append to event chain).
        _emitTokensRescued(token, amount);
    }
}
