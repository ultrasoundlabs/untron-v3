// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {SwapExecutor, Call} from "../SwapExecutor.sol";

/* solhint-disable no-inline-assembly */
/* solhint-disable no-unused-vars */

/// @title Untron V3 hub (size-split dispatcher)
/// @notice Deployed hub that delegates heavy entrypoints to facets to satisfy EIP-170 (24KB) runtime size limit.
/// @author Ultrasound Labs
contract UntronV3 is UntronV3Base {
    address internal immutable _ADMIN_FACET;
    address internal immutable _LEASE_FACET;
    address internal immutable _ENTITLE_FACET;
    address internal immutable _CONTROLLER_FACET;
    address internal immutable _LP_FACET;
    address internal immutable _FILL_FACET;

    /// @notice Deploy the UntronV3 hub.
    /// @dev Side effects:
    /// - Sets immutables used for deterministic receiver derivation (`CONTROLLER_ADDRESS`, `RECEIVER_IMPL`, `_CREATE2_PREFIX`).
    /// - Deploys a dedicated `SwapExecutor` instance and stores it in `SWAP_EXECUTOR`.
    /// - Initializes ownership (events emitted via `UntronV3Index`).
    /// @param controllerAddress Address of the UntronController on Tron (source chain), in EVM 20-byte form.
    /// @param create2Prefix Chain-specific byte prefix used for CREATE2 address computation (0x41 for Tron).
    /// @param receiverImpl Tron-side receiver implementation address (20-byte EVM form).
    /// @param adminFacet Facet contract implementing admin functions.
    /// @param leaseFacet Facet contract implementing lease functions.
    /// @param entitleFacet Facet contract implementing entitlement functions.
    /// @param controllerFacet Facet contract implementing controller relay functions.
    /// @param lpFacet Facet contract implementing LP vault functions.
    /// @param fillFacet Facet contract implementing fill functions.
    constructor(
        address controllerAddress,
        bytes1 create2Prefix,
        address receiverImpl,
        address adminFacet,
        address leaseFacet,
        address entitleFacet,
        address controllerFacet,
        address lpFacet,
        address fillFacet
    ) {
        _CREATE2_PREFIX = create2Prefix;
        RECEIVER_IMPL = receiverImpl;
        CONTROLLER_ADDRESS = controllerAddress;

        // Deploy an isolated executor for swaps. Only this UntronV3 instance can call `execute(...)`.
        // NOTE: This is intentionally deployed via `new` (not CREATE2), so its address is derived from nonce.
        SWAP_EXECUTOR = new SwapExecutor(); // its address is gonna be keccak256(rlp([address(this), 1]))

        _ADMIN_FACET = adminFacet;
        _LEASE_FACET = leaseFacet;
        _ENTITLE_FACET = entitleFacet;
        _CONTROLLER_FACET = controllerFacet;
        _LP_FACET = lpFacet;
        _FILL_FACET = fillFacet;

        // Initialize owner and emit OwnershipTransferred via UntronV3Index.
        _initializeOwner(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                               ADMIN CONFIG
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to admin facet: set `usdt`.
    /// @param usdt_ ERC-20 token address used as USDT accounting unit on this chain.
    function setUsdt(address usdt_) external {
        usdt_;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set realtor allowlist flag.
    /// @param realtor Realtor address.
    /// @param allowed Whether the address is allowed to create leases.
    function setRealtor(address realtor, bool allowed) external {
        realtor;
        allowed;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set LP allowlist flag.
    /// @param lp LP address.
    /// @param allowed Whether the address is allowed to deposit into the LP vault.
    function setLp(address lp, bool allowed) external {
        lp;
        allowed;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: mark a destination chain as deprecated or not.
    /// @param targetChainId Destination chain id to mark.
    /// @param deprecated True to deprecate; false to un-deprecate.
    function setChainDeprecated(uint256 targetChainId, bool deprecated) external {
        targetChainId;
        deprecated;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set protocol fee floor in ppm.
    /// @param floorPpm Fee floor in ppm (1_000_000 = 100%).
    function setProtocolFloorPpm(uint256 floorPpm) external {
        floorPpm;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set protocol flat fee floor.
    /// @param floorFlatFee Flat fee floor in USDT units.
    function setProtocolFloorFlatFee(uint64 floorFlatFee) external {
        floorFlatFee;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set protocol max lease duration.
    /// @param maxLeaseDurationSeconds Maximum allowed duration in seconds (0 disables).
    function setProtocolMaxLeaseDurationSeconds(uint32 maxLeaseDurationSeconds) external {
        maxLeaseDurationSeconds;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set realtor-specific minimum fee floor in ppm.
    /// @param realtor Realtor whose override is being set.
    /// @param minFeePpm Fee floor in ppm (1_000_000 = 100%).
    function setRealtorMinFeePpm(address realtor, uint256 minFeePpm) external {
        realtor;
        minFeePpm;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set realtor-specific minimum flat fee floor.
    /// @param realtor Realtor whose override is being set.
    /// @param minFlatFee Flat fee floor in USDT units.
    function setRealtorMinFlatFee(address realtor, uint64 minFlatFee) external {
        realtor;
        minFlatFee;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set realtor-specific max lease duration.
    /// @param realtor Realtor whose override is being set.
    /// @param maxLeaseDurationSeconds Maximum allowed duration in seconds (0 disables).
    function setRealtorMaxLeaseDurationSeconds(address realtor, uint32 maxLeaseDurationSeconds) external {
        realtor;
        maxLeaseDurationSeconds;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: configure lessee payout-config update rate limiting.
    /// @param maxUpdates Maximum number of payout config updates allowed per window.
    /// @param windowSeconds Window size in seconds.
    function setLesseePayoutConfigRateLimit(uint256 maxUpdates, uint256 windowSeconds) external {
        maxUpdates;
        windowSeconds;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: configure realtor lease creation rate limiting.
    /// @param realtor Realtor whose rate limit is being set.
    /// @param maxLeases Maximum number of lease creations allowed per window.
    /// @param windowSeconds Window size in seconds.
    function setRealtorLeaseRateLimit(address realtor, uint256 maxLeases, uint256 windowSeconds) external {
        realtor;
        maxLeases;
        windowSeconds;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set Tron reader.
    /// @param reader Address of the new `ITronTxReader` implementation.
    function setTronReader(address reader) external {
        reader;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: set swap rate for a target token.
    /// @param targetToken Token to set the swap rate for.
    /// @param ratePpm Rate in ppm of USDT; must be non-zero.
    function setSwapRate(address targetToken, uint256 ratePpm) external {
        targetToken;
        ratePpm;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: register a bridger for a `(targetToken, targetChainId)` pair.
    /// @param targetToken Token that will be bridged.
    /// @param targetChainId Destination chain id for the bridger.
    /// @param bridger Bridger contract address implementing `IBridger`.
    function setBridger(address targetToken, uint256 targetChainId, address bridger) external {
        targetToken;
        targetChainId;
        bridger;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: pause the protocol.
    function pause() external {
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: unpause the protocol.
    function unpause() external {
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: deposit USDT into protocol PnL.
    /// @param amount Amount of `usdt` to deposit.
    function depositToPnl(uint256 amount) external {
        amount;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: withdraw positive protocol PnL (profit) to the owner.
    /// @param amount Amount of profit to withdraw.
    function withdrawProtocolProfit(int256 amount) external {
        amount;
        _delegate(_ADMIN_FACET);
    }

    /// @notice Delegate to admin facet: rescue non-USDT tokens accidentally sent to the contract.
    /// @param token Token address to rescue.
    /// @param amount Amount to transfer to the owner.
    function rescueTokens(address token, uint256 amount) external {
        token;
        amount;
        _delegate(_ADMIN_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                                 LEASES
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to lease facet: create a new lease.
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
    ) external returns (uint256, uint256) {
        receiverSalt;
        lessee;
        nukeableAfter;
        leaseFeePpm;
        flatFee;
        targetChainId;
        targetToken;
        beneficiary;
        _delegate(_LEASE_FACET);
    }

    /// @notice Delegate to lease facet: update payout configuration for a lease.
    /// @param leaseId Lease to update.
    /// @param targetChainId New destination chain id.
    /// @param targetToken New settlement token on THIS chain.
    /// @param beneficiary New payout recipient.
    function setPayoutConfig(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        external
    {
        leaseId;
        targetChainId;
        targetToken;
        beneficiary;
        _delegate(_LEASE_FACET);
    }

    /// @notice Delegate to lease facet: gasless payout config update using EIP-712 signature by the lessee.
    /// @param leaseId Lease to update.
    /// @param config New payout configuration.
    /// @param deadline Timestamp after which the signature is invalid.
    /// @param signature EIP-712 signature by the lease's current lessee.
    function setPayoutConfigWithSig(
        uint256 leaseId,
        PayoutConfig calldata config,
        uint256 deadline,
        bytes calldata signature
    ) external {
        leaseId;
        config;
        deadline;
        signature;
        _delegate(_LEASE_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                        PRE-ENTITLEMENT FROM TRON
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to entitle facet: prove and pre-entitle a recognizable Tron USDT deposit.
    /// @param receiverSalt CREATE2 salt used to derive the receiver address on Tron.
    /// @param blocks 20 Protobuf-encoded Tron block headers (first contains the tx).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @return queueIndex Index in `claimsByTargetToken[payout.targetToken]` where the claim was appended (0 if none).
    /// @return leaseId Lease id that the deposit was attributed to.
    /// @return netOut USDT-denominated net payout after fees.
    function preEntitle(
        bytes32 receiverSalt,
        bytes[20] calldata blocks,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external returns (uint256, uint256, uint256) {
        receiverSalt;
        blocks;
        encodedTx;
        proof;
        index;
        _delegate(_ENTITLE_FACET);
    }

    /// @notice Delegate to entitle facet: subjective (LP-sponsored) pre-entitlement for an anticipated Tron tx.
    /// @param txId Anticipated Tron transaction id (`sha256(raw_data)`).
    /// @param leaseId Lease expected to be active at the Tron tx timestamp.
    /// @param rawAmount Expected raw TRC-20 amount (before fees).
    /// @return queueIndex Index in `claimsByTargetToken[payout.targetToken]` where the claim was appended.
    /// @return netOut USDT-denominated claim amount after fees.
    function subjectivePreEntitle(bytes32 txId, uint256 leaseId, uint256 rawAmount)
        external
        returns (uint256, uint256)
    {
        txId;
        leaseId;
        rawAmount;
        _delegate(_ENTITLE_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                         CONTROLLER EVENT RELAY
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to controller facet: relay controller event chain and enqueue events for processing.
    /// @param blocks 20 Protobuf-encoded Tron block headers (first contains the tx).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @param events Controller events that should extend the local tip.
    /// @return tipNew The new controller event-chain tip.
    function relayControllerEventChain(
        bytes[20] calldata blocks,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index,
        ControllerEvent[] calldata events
    ) external returns (bytes32 tipNew) {
        tipNew;
        blocks;
        encodedTx;
        proof;
        index;
        events;
        _delegate(_CONTROLLER_FACET);
    }

    /// @notice Delegate to controller facet: process up to `maxEvents` queued controller events.
    /// @param maxEvents Maximum number of queued events to process in this call.
    function processControllerEvents(uint256 maxEvents) external {
        maxEvents;
        _delegate(_CONTROLLER_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                              LP VAULT
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to LP facet: deposit USDT into the fast-fill vault.
    /// @param amount Amount of `usdt` to transfer from the caller into the contract.
    function deposit(uint256 amount) external {
        amount;
        _delegate(_LP_FACET);
    }

    /// @notice Delegate to LP facet: withdraw USDT from the fast-fill vault.
    /// @param amount Amount of `usdt` to withdraw.
    function withdraw(uint256 amount) external {
        amount;
        _delegate(_LP_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                             CLAIM QUEUE
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegate to fill facet: fill up to `maxClaims` claims for a target token.
    /// @param targetToken The queue key: claims to be filled are `claimsByTargetToken[targetToken]`.
    /// @param maxClaims Maximum number of non-empty claims to fill in this call.
    /// @param calls Arbitrary swap calls executed by `SwapExecutor` if the plan requires swapping.
    function fill(address targetToken, uint256 maxClaims, Call[] calldata calls) external {
        targetToken;
        maxClaims;
        calls;
        _delegate(_FILL_FACET);
    }

    /*//////////////////////////////////////////////////////////////
                             EXTERNAL VIEW
    //////////////////////////////////////////////////////////////*/

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

    /*//////////////////////////////////////////////////////////////
                              PUBLIC VIEW
    //////////////////////////////////////////////////////////////*/

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
    /// @param realtor Realtor to query.
    /// @return maxLeaseDurationSeconds Effective max lease duration in seconds.
    function effectiveMaxLeaseDurationSeconds(address realtor) public view returns (uint256 maxLeaseDurationSeconds) {
        return _maxLeaseDurationSeconds(realtor);
    }

    /*//////////////////////////////////////////////////////////////
                               DELEGATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Delegatecall into a facet, returning or bubbling the revert.
    /// @param facet The facet address to delegatecall into.
    function _delegate(address facet) internal {
        /// @solidity memory-safe-assembly
        assembly {
            calldatacopy(0x00, 0x00, calldatasize())
            let result := delegatecall(gas(), facet, 0x00, calldatasize(), 0x00, 0x00)
            returndatacopy(0x00, 0x00, returndatasize())
            switch result
            case 0 { revert(0x00, returndatasize()) }
            default { return(0x00, returndatasize()) }
        }
    }
}
