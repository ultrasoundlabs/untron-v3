// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";

/// @title UntronV3 lease facet
/// @notice Lease creation and payout configuration management.
/// @author Ultrasound Labs
contract UntronV3LeaseFacet is UntronV3Base {
    /// @notice Create a new lease for a given receiver salt.
    /// @dev Requirements:
    /// - Caller must be a whitelisted realtor (`isRealtor[msg.sender] == true`).
    /// - Realtor must satisfy the effective lease creation rate limit.
    /// - `leaseFeePpm` must be within `[minFee, 1_000_000]`, where `minFee = max(protocol floor, realtor min)`.
    /// - `flatFee` must be within `[minFlatFee, type(uint64).max]`, where `minFlatFee = max(protocol floor, realtor min)`.
    /// - `targetChainId` must not be deprecated.
    /// - `nukeableAfter` must be >= current timestamp.
    /// - Lease duration (`nukeableAfter - startTime`) must not exceed the effective configured maximum.
    /// - For a reused `receiverSalt`, the previous lease must already be nukeable.
    /// - The payout config must be routable (rate set if `targetToken != usdt`; bridger set if `targetChainId != block.chainid`).
    ///
    /// This function does not deploy the receiver; it only registers lease metadata. Deterministic receiver
    /// addresses are derived on Tron using CREATE2 with `receiverSalt`.
    /// @param receiverSalt CREATE2 salt used to derive the Tron receiver address.
    /// @param lessee Account that will control `payout` configuration for this lease.
    /// @param nukeableAfter Earliest time at which a new lease for this `receiverSalt` may be created.
    /// @param leaseFeePpm Percentage fee in ppm applied to recognized raw volume.
    /// @param flatFee Flat fee subtracted after percentage fee.
    /// @param targetChainId Destination chain for payouts for claims created by this lease.
    /// @param targetToken Token on THIS chain used for settlement of claims created by this lease.
    /// @param beneficiary Recipient address for payouts for claims created by this lease.
    /// @return leaseId Newly created global lease identifier.
    /// @return leaseNumber Position of the lease in the receiver's lease array.
    function createLease(
        bytes32 receiverSalt,
        address lessee,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee,
        uint256 targetChainId,
        address targetToken,
        address beneficiary
    ) external whenNotPaused returns (uint256 leaseId, uint256 leaseNumber) {
        _enforceCreateLeasePreconditions(msg.sender, receiverSalt, nukeableAfter, leaseFeePpm, flatFee, targetChainId);

        // Validate that the payout route is currently supported/configured.
        // This makes lease creation fail fast if rate/bridger isn't configured yet.
        _enforcePayoutConfigRoutable(targetChainId, targetToken);

        // Allocate the new lease id.
        leaseId = ++nextLeaseId;
        uint64 startTime = _leaseStartTime();

        // Populate lease storage and append to the receiver's lease timeline.
        leaseNumber = _storeLease(
            leaseId,
            receiverSalt,
            msg.sender,
            lessee,
            startTime,
            nukeableAfter,
            leaseFeePpm,
            flatFee,
            targetChainId,
            targetToken,
            beneficiary
        );

        // Emit lease creation and initial payout config via UntronV3Index.
        _emitLeaseCreated(
            leaseId, receiverSalt, leaseNumber, msg.sender, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee
        );
        // this is slightly crutchy because we technically enshrine the initial config
        // at creation time, but this simplifies indexing logic quite a bunch
        _emitPayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    /// @notice Update payout configuration for an existing lease.
    /// @dev Callable only by the current lessee.
    /// Requirements:
    /// - `leaseId` must exist.
    /// - Caller must be the current lessee.
    /// - Caller must satisfy the payout-config update rate limit (if enabled).
    /// - `targetChainId` must not be deprecated.
    /// - The payout config must be routable (rate set if `targetToken != usdt`; bridger set if `targetChainId != block.chainid`).
    /// @param leaseId Lease to update.
    /// @param targetChainId New destination chain id.
    /// @param targetToken New settlement token on THIS chain.
    /// @param beneficiary New payout recipient.
    function setPayoutConfig(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        external
        whenNotPaused
    {
        // Load and validate lease.
        Lease storage lease = _leaseStorage(leaseId);
        // Only the current lessee can update payout config.
        if (msg.sender != lease.lessee) revert NotLessee();

        // Apply per-lessee rate limiting for payout config updates.
        _enforcePayoutConfigRateLimit(msg.sender);

        // this technically makes changing beneficiaries but not chains revert too
        // but i think it's fine because hey mf you're the one who stops us from deprecating it
        // Disallow setting deprecated chains even if only changing beneficiary.
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();
        // Validate chain + route configuration (rate/bridger availability).
        _enforcePayoutConfigRoutable(targetChainId, targetToken);

        // Persist new payout config for future claims created by this lease.
        lease.payout = PayoutConfig({targetChainId: targetChainId, targetToken: targetToken, beneficiary: beneficiary});
        // Emit via UntronV3Index (and append to event chain).
        _emitPayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    /// @notice Gasless payout config update using an EIP-712 signature by the lessee.
    /// @dev Anyone can relay this; signer must be the current lessee of `leaseId`.
    /// The signed struct is:
    /// `PayoutConfigUpdate(uint256 leaseId,uint256 targetChainId,address targetToken,address beneficiary,uint256 nonce,uint256 deadline)`
    ///
    /// Signature verification:
    /// - Uses Solady `SignatureCheckerLib`, supporting both EOA signatures (ECDSA) and smart contract wallets (ERC-1271).
    /// - Uses per-lease nonces (`leaseNonces[leaseId]`) to prevent replay.
    ///
    /// Requirements:
    /// - `deadline` must be in the future.
    /// - `leaseId` must exist.
    /// - Lessee must satisfy payout-config update rate limit (if enabled).
    /// - `targetChainId` must not be deprecated.
    /// - The payout config must be routable (rate set if `targetToken != usdt`; bridger set if `targetChainId != block.chainid`).
    /// @param leaseId Lease to update.
    /// @param config New payout configuration.
    /// @param deadline Timestamp after which the signature is invalid.
    /// @param signature EIP-712 signature by the lease's current lessee.
    function setPayoutConfigWithSig(
        uint256 leaseId,
        PayoutConfig calldata config,
        uint256 deadline,
        bytes calldata signature
    ) external whenNotPaused {
        // Deadline check to avoid accepting stale signatures.
        if (block.timestamp > deadline) revert SignatureExpired();

        // Load and validate lease existence.
        Lease storage lease = _leaseStorage(leaseId);
        _enforcePayoutConfigRateLimit(lease.lessee);

        // Snapshot nonce and config fields to avoid repeated calldata reads.
        uint256 nonce = leaseNonces[leaseId];
        uint256 targetChainId_ = config.targetChainId;
        address targetToken_ = config.targetToken;
        address beneficiary_ = config.beneficiary;

        // Validate chain + route configuration (rate/bridger availability).
        _enforcePayoutConfigRoutable(targetChainId_, targetToken_);

        bytes32 digest = _payoutConfigUpdateDigest(leaseId, targetChainId_, targetToken_, beneficiary_, nonce, deadline);
        _enforceValidSignature(lease.lessee, digest, signature);
        _consumeLeaseNonce(leaseId, nonce);
        _setLeasePayoutConfig(lease, config);

        _emitPayoutConfigUpdated(leaseId, config.targetChainId, config.targetToken, config.beneficiary);
    }

    /// @notice Return lease data for an external `leaseId`.
    /// @param leaseId The ID of the lease to retrieve.
    /// @return receiverSalt The salt used to generate the receiver address.
    /// @return realtor The address of the realtor.
    /// @return lessee The address of the lessee.
    /// @return startTime The start time of the lease.
    /// @return nukeableAfter The time after which the lease can be nuked.
    /// @return leaseFeePpm The fee in parts per million.
    /// @return flatFee The flat fee.
    /// @return recognizedRaw The recognized raw amount.
    /// @return backedRaw The backed raw amount.
    /// @return unbackedRaw The unbacked raw amount.
    /// @return payout The payout configuration.
    function leases(uint256 leaseId)
        external
        view
        returns (
            bytes32 receiverSalt,
            address realtor,
            address lessee,
            uint64 startTime,
            uint64 nukeableAfter,
            uint32 leaseFeePpm,
            uint64 flatFee,
            uint256 recognizedRaw,
            uint256 backedRaw,
            uint256 unbackedRaw,
            PayoutConfig memory payout
        )
    {
        LeaseLocator storage loc = _leaseLocatorById[leaseId];
        uint256 leaseNumberPlusOne = loc.leaseNumberPlusOne;
        if (leaseNumberPlusOne == 0) {
            return (
                receiverSalt,
                realtor,
                lessee,
                startTime,
                nukeableAfter,
                leaseFeePpm,
                flatFee,
                recognizedRaw,
                backedRaw,
                unbackedRaw,
                payout
            );
        }

        Lease storage lease = leasesByReceiver[loc.receiverSalt][leaseNumberPlusOne - 1];
        receiverSalt = lease.receiverSalt;
        realtor = lease.realtor;
        lessee = lease.lessee;
        startTime = lease.startTime;
        nukeableAfter = lease.nukeableAfter;
        leaseFeePpm = lease.leaseFeePpm;
        flatFee = lease.flatFee;
        recognizedRaw = lease.recognizedRaw;
        backedRaw = lease.backedRaw;
        unbackedRaw = lease.unbackedRaw;
        payout = lease.payout;
    }

    /// @notice Returns the protocol-wide payout config update rate limit for lessees.
    /// @dev If either returned value is 0, the payout-config update rate limit is disabled.
    /// @return maxUpdates Max number of payout config updates allowed per window.
    /// @return windowSeconds Window size in seconds.
    function lesseePayoutConfigRateLimit() external view returns (uint256 maxUpdates, uint256 windowSeconds) {
        ProtocolConfig storage cfg = _protocolConfig;
        return (cfg.payoutConfigRateLimitMaxUpdates, cfg.payoutConfigRateLimitWindowSeconds);
    }

    /// @notice Returns the raw realtor lease creation rate limit config.
    /// @param realtor Realtor to query.
    /// @return maxLeases Maximum number of lease creations allowed per window.
    /// @return windowSeconds Window size in seconds.
    function realtorLeaseRateLimit(address realtor) external view returns (uint256 maxLeases, uint256 windowSeconds) {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        return (cfg.leaseRateLimitMaxLeases, cfg.leaseRateLimitWindowSeconds);
    }

    /// @notice Returns the effective realtor lease creation rate limit config.
    /// @dev If either returned value is 0, rate limiting is disabled.
    /// @param realtor Realtor to query.
    /// @return enabled Whether the rate limit is enabled.
    /// @return maxLeases Maximum number of lease creations allowed per window.
    /// @return windowSeconds Window size in seconds.
    function effectiveLeaseRateLimit(address realtor)
        external
        view
        returns (bool enabled, uint256 maxLeases, uint256 windowSeconds)
    {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        maxLeases = cfg.leaseRateLimitMaxLeases;
        windowSeconds = cfg.leaseRateLimitWindowSeconds;
        enabled = (maxLeases != 0 && windowSeconds != 0);
    }

    /// @notice Returns the next lease number at a receiver.
    /// @param receiverSalt CREATE2 salt used to derive the receiver address.
    /// @return nextLeaseNumber The next lease number (i.e., current `leasesByReceiver[receiverSalt].length`).
    function nextLeaseNumberAtReceiver(bytes32 receiverSalt) external view returns (uint256 nextLeaseNumber) {
        return leasesByReceiver[receiverSalt].length;
    }

    /// @notice Returns the protocol-wide minimum fee floor in ppm.
    /// @return floorPpm Fee floor in ppm.
    function protocolFloorPpm() public view returns (uint256 floorPpm) {
        return uint256(_protocolConfig.floorPpm);
    }

    /// @notice Returns the protocol-wide minimum flat fee floor in USDT units.
    /// @return floorFlatFee Flat fee floor in USDT units.
    function protocolFloorFlatFee() public view returns (uint256 floorFlatFee) {
        return uint256(_protocolConfig.floorFlatFee);
    }

    /// @notice Returns the protocol-wide maximum lease duration in seconds (0 disables).
    /// @return maxLeaseDurationSeconds Max lease duration in seconds.
    function protocolMaxLeaseDurationSeconds() public view returns (uint256 maxLeaseDurationSeconds) {
        return uint256(_protocolConfig.maxLeaseDurationSeconds);
    }

    /// @notice Returns the realtor-specific minimum fee floor in ppm.
    /// @param realtor Realtor to query.
    /// @return minFeePpm Minimum fee floor in ppm.
    function realtorMinFeePpm(address realtor) public view returns (uint256 minFeePpm) {
        return uint256(_realtorConfig[realtor].minFeePpm);
    }

    /// @notice Returns the realtor-specific minimum flat fee floor in USDT units.
    /// @param realtor Realtor to query.
    /// @return minFlatFee Minimum flat fee floor in USDT units.
    function realtorMinFlatFee(address realtor) public view returns (uint256 minFlatFee) {
        return uint256(_realtorConfig[realtor].minFlatFee);
    }

    /// @notice Returns the realtor-specific maximum lease duration in seconds (0 disables).
    /// @param realtor Realtor to query.
    /// @return maxLeaseDurationSeconds Max lease duration in seconds.
    function realtorMaxLeaseDurationSeconds(address realtor) public view returns (uint256 maxLeaseDurationSeconds) {
        return uint256(_realtorConfig[realtor].maxLeaseDurationSeconds);
    }

    /// @notice Returns the effective max lease duration in seconds for a realtor.
    /// @dev This is the minimum non-zero of protocol and realtor configured maximums, or 0 if both disabled.
    /// @param realtor Realtor to query.
    /// @return maxLeaseDurationSeconds Effective max lease duration in seconds.
    function effectiveMaxLeaseDurationSeconds(address realtor) public view returns (uint256 maxLeaseDurationSeconds) {
        return _maxLeaseDurationSeconds(realtor);
    }
}
