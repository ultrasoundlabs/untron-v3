// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3IndexedOwnable} from "./index/UntronV3IndexedOwnable.sol";
import {ITronTxReader} from "../interfaces/ITronTxReader.sol";
import {SwapExecutor, Call} from "../SwapExecutor.sol";
import {IBridger} from "../bridgers/interfaces/IBridger.sol";
import {EventChainGenesis} from "../../utils/EventChainGenesis.sol";
import {TronCalldataUtils} from "../../utils/TronCalldataUtils.sol";
import {TokenUtils} from "../../utils/TokenUtils.sol";

import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

import {EIP712} from "solady/utils/EIP712.sol";
import {SignatureCheckerLib} from "solady/utils/SignatureCheckerLib.sol";
import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";

/* solhint-disable max-states-count */
/* solhint-disable var-name-mixedcase */

/// @title Untron V3 hub
/// @notice Turns provable Tron-side controller activity into EVM-side payouts.
/// @dev High-level responsibilities:
/// - Maintain a timeline of per-receiver `Lease`s (created by whitelisted "realtors") that define:
///   - who the active lessee is,
///   - which fees apply to recognized volume,
///   - and where payouts should go (chain/token/beneficiary).
/// - Allow anyone to prove a recognizable Tron USDT TRC-20 transfer into a deterministic receiver and
///   "pre-entitle" that deposit to the correct active lease at the Tron tx timestamp (`preEntitle`).
/// - Maintain per-`targetToken` FIFO claim queues; claims are denominated in USDT and later settled either:
///   - locally in USDT,
///   - locally by swapping USDT -> `targetToken`,
///   - or swapping and then bridging via a configured `IBridger`.
/// - Relay and process a Tron-side controller event hash-chain to reconcile backing (receiver pulls) and
///   capture rebalancing PnL (`relayControllerEventChain` / `processControllerEvents`).
/// - Provide a "fast-fill" LP vault (principal accounting only; 0% APY by design) used to fund fills.
///
/// Eventing model:
/// - UntronV3 MUST NOT emit events directly.
/// - All event emissions MUST go through `UntronV3Index`, which also appends events into an onchain
///   hash-chain (`eventChainTip`) to make offchain indexing and integrity checks easier.
///
/// Trust model (important):
/// - The owner is highly trusted: can change `usdt`, `tronReader`, `swapRatePpm`, bridgers, realtor list, etc.
/// - Bridgers are trusted for delivery: UntronV3 does not verify cross-chain settlement.
/// @author Ultrasound Labs
abstract contract UntronV3Base is EIP712, ReentrancyGuard, Pausable, UntronV3IndexedOwnable {
    using SignatureCheckerLib for address;

    /*//////////////////////////////////////////////////////////////
                                  TYPES
    //////////////////////////////////////////////////////////////*/

    /// @notice Protocol-wide configuration set by the owner.
    /// @dev Stored as small ints to reduce storage and because values are naturally bounded.
    struct ProtocolConfig {
        /// @notice Protocol-wide minimum fee, in parts-per-million (ppm) of recognized raw volume.
        /// @dev Applied as a floor for every lease; realtor-specific `minFeePpm` can raise it.
        uint32 floorPpm;
        /// @notice Max number of payout-config updates allowed per window per lessee.
        uint32 payoutConfigRateLimitMaxUpdates;
        /// @notice Sliding window length (seconds) for payout-config update rate limiting.
        uint32 payoutConfigRateLimitWindowSeconds;
        /// @notice Protocol-wide minimum flat fee (in USDT units) applied as a floor for every lease.
        /// @dev Realtor-specific `minFlatFee` can raise it.
        uint64 floorFlatFee;
        /// @notice Protocol-wide maximum lease duration (seconds), measured as `nukeableAfter - startTime`.
        /// @dev If 0, max duration is disabled at protocol level.
        uint32 maxLeaseDurationSeconds;
    }

    /// @notice Realtor-specific configuration set by the owner.
    struct RealtorConfig {
        /// @notice Realtor-specific minimum fee floor (ppm). Effective minimum is `max(protocolFloorPpm, minFeePpm)`.
        uint32 minFeePpm;
        /// @notice Realtor-specific lease creation max leases.
        uint32 leaseRateLimitMaxLeases;
        /// @notice Realtor-specific lease creation window seconds.
        uint32 leaseRateLimitWindowSeconds;
        /// @notice Realtor-specific minimum flat fee (in USDT units). Effective minimum is `max(protocolFloorFlatFee, minFlatFee)`.
        uint64 minFlatFee;
        /// @notice Realtor-specific maximum lease duration (seconds), measured as `nukeableAfter - startTime`.
        /// @dev If 0, no realtor-specific max is applied.
        uint32 maxLeaseDurationSeconds;
    }

    /// @notice Per-lease payout configuration, mutable by the lessee.
    /// @dev Owner controls bridge pair availability; leases specify destination chain, target token, and beneficiary.
    struct PayoutConfig {
        /// @notice Destination chain ID for the payout.
        /// @dev If equal to `block.chainid`, payout is local and no bridging occurs.
        uint256 targetChainId;
        /// @notice Target token address on THIS chain (not the destination chain).
        /// @dev Claims are always denominated in USDT internally; this token determines how they are settled.
        address targetToken;
        /// @notice Recipient address for the payout.
        address beneficiary;
    }

    /// @notice Lease scoped by receiver salt (not token).
    /// @dev A lease defines who controls payouts for a receiver and what fee schedule applies to recognized volume.
    struct Lease {
        /// @notice Salt used to derive the deterministic Tron receiver address.
        /// @dev Receiver address is `predictReceiverAddress(CONTROLLER_ADDRESS, receiverSalt)` using Tron CREATE2 prefix.
        bytes32 receiverSalt;
        /// @notice Whitelisted realtor that created this lease.
        address realtor;
        /// @notice Lessee that can update `payout` for this lease.
        address lessee;
        /// @notice Lease start time on this chain (seconds since epoch).
        /// @dev Used together with Tron timestamps to find the "active lease at time T".
        uint64 startTime;
        /// @notice Earliest timestamp at which a subsequent lease for the same `receiverSalt` can be created.
        /// @dev Prevents "nuking" or replacing the receiver's lessee too early.
        uint64 nukeableAfter;
        /// @notice Percentage fee, ppm of recognized raw volume.
        /// @dev Net payout before flat fee is `amount * (1e6 - leaseFeePpm) / 1e6`.
        uint32 leaseFeePpm;
        /// @notice Flat fee subtracted after the percentage fee is applied.
        /// @dev If the percentage net is <= flat fee, payout becomes 0.
        uint64 flatFee;
        /// @notice Total recognized raw USDT volume attributed to this lease.
        /// @dev Increased by `preEntitle` and by `_processReceiverPulled` (for remaining volume).
        uint256 recognizedRaw;
        /// @notice Portion of `recognizedRaw` that has been backed by actual receiver pulls (controller events).
        uint256 backedRaw;
        /// @notice Portion of `recognizedRaw` not yet backed by receiver pulls.
        /// @dev This tracks "float" until the controller reports the receiver has been pulled/rebalanced.
        uint256 unbackedRaw;
        /// @notice Current payout configuration for newly created claims.
        /// @dev This is mutable by the lessee (directly or via signature).
        PayoutConfig payout;
    }

    /// @notice Internal lease identifier: receiver salt + sequential lease number.
    /// @dev `leaseNumberPlusOne == 0` indicates that a given external `leaseId` does not exist.
    struct LeaseLocator {
        bytes32 receiverSalt;
        uint256 leaseNumberPlusOne;
    }

    /// @notice FIFO claim queue element.
    /// @dev Claim amounts are denominated in USDT; settlement may pay USDT or `targetToken` depending on route.
    struct Claim {
        /// @notice Per-lease claim identifier (0-indexed).
        /// @dev Increments monotonically per `leaseId`. `(leaseId, claimId)` uniquely identifies a claim.
        uint256 claimId;
        /// @notice USDT-denominated claim amount.
        /// @dev
        /// - This is ALWAYS denominated in the protocol's USDT accounting unit, regardless of how the claim is paid.
        /// - For `targetToken == usdt`, this is the exact token amount transferred.
        /// - For `targetToken != usdt`, this is converted into `targetToken` via `swapRatePpm` at fill time.
        /// - Slots are deleted when filled.
        // forge-lint: disable-next-line(mixed-case-variable)
        uint256 amountUsdt;
        /// @notice Lease that produced this claim (for indexing/analytics).
        uint256 leaseId;
        // target token is the key of the queue the claim is sitting in;
        // not including it here thus feels more concise (and storage-efficient!)
        /// @notice Destination chain for this claim's payout.
        uint256 targetChainId;
        /// @notice Recipient of the payout (either local transfer or bridged recipient).
        address beneficiary;
    }

    /// @notice Location of a claim within a per-`targetToken` queue.
    /// @dev Used to map `(leaseId, claimId)` -> `(targetToken, queueIndex)` for lookup/indexing.
    struct ClaimLocator {
        address targetToken;
        uint256 queueIndex;
    }

    /// @notice Subjective pre-entitlement record (LP-sponsored early payout) keyed by anticipated Tron `txId`.
    /// @dev Reimbursed only if a later `preEntitle` proves the exact same `(txId, leaseId, rawAmount)`.
    struct SubjectivePreEntitlement {
        address sponsor;
        uint256 leaseId;
        uint256 rawAmount;
        uint256 queueIndex;
        uint256 claimId;
    }

    /// @notice Raw controller event reconstructed from the Tron event chain.
    /// @dev Stores raw data; processing happens later based on `sig`.
    struct ControllerEvent {
        /// @notice Keccak256 hash of the Tron event signature string.
        bytes32 sig;
        /// @notice ABI-encoded event data payload (EVM ABI encoding).
        bytes data;
        /// @notice Tron block number associated with the event.
        uint64 blockNumber;
        /// @notice Tron block timestamp associated with the event.
        uint64 blockTimestamp;
    }

    /*//////////////////////////////////////////////////////////////
                             STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /* solhint-disable gas-small-strings */
    // keccak of string literals is evaluated at compile time

    /// @dev EIP-712 typehash for gasless payout config updates.
    /// @dev Used to build `PayoutConfigUpdate(...)` struct hashes inside `setPayoutConfigWithSig`.
    bytes32 internal constant _PAYOUT_CONFIG_UPDATE_TYPEHASH = keccak256(
        "PayoutConfigUpdate(uint256 leaseId,uint256 targetChainId,address targetToken,address beneficiary,uint256 nonce,uint256 deadline)"
    );

    /// @dev Parts-per-million denominator used for fee calculations (1_000_000 = 100%).
    uint256 internal constant _PPM_DENOMINATOR = 1_000_000;
    /// @notice Denominator for target token rate tables.
    /// @dev `swapRatePpm[targetToken]` is interpreted as `targetToken` units per `_RATE_DENOMINATOR` USDT.
    uint256 internal constant _RATE_DENOMINATOR = 1_000_000;

    /// @dev Tron controller function selector for `isEventChainTip(bytes32)`.
    bytes4 internal constant _SELECTOR_IS_EVENT_CHAIN_TIP = bytes4(keccak256("isEventChainTip(bytes32)"));
    /// @dev Tron controller function selector for `multicall(bytes[])` (used as wrapper for `isEventChainTip`).
    bytes4 internal constant _SELECTOR_MULTICALL = bytes4(keccak256("multicall(bytes[])"));

    /// @dev Tron controller event signature hash for `PulledFromReceiver(bytes32,address,uint256,uint256,uint256)`.
    ///      Used to reconcile receiver pulls and backing on the EVM side.
    bytes32 internal constant _EVENT_SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
    /// @dev Tron controller event signature hash for `UsdtSet(address)`.
    ///      Updates `tronUsdt` (the only TRC-20 accepted by `preEntitle`).
    bytes32 internal constant _EVENT_SIG_USDT_SET = keccak256("UsdtSet(address)");
    /// @dev Tron controller event signature hash for `UsdtRebalanced(uint256,uint256,address)`.
    ///      Updates `protocolPnl` to account for rebalance drift.
    bytes32 internal constant _EVENT_SIG_USDT_REBALANCED = keccak256("UsdtRebalanced(uint256,uint256,address)");
    /// @dev Tron controller event signature hash for `ControllerUsdtTransfer(address,uint256)`.
    ///      Used to account for controller executor USDT spends as negative protocol PnL.
    bytes32 internal constant _EVENT_SIG_CONTROLLER_USDT_TRANSFER =
        keccak256("ControllerUsdtTransfer(address,uint256)");

    /* solhint-enable gas-small-strings */

    /// @notice Address embedded into the receiver proxy init code.
    /// @dev For Tron-side controllers this is the deployed `UntronReceiver` implementation.
    ///      For EVM-side contracts that only need to *predict* Tron receiver addresses, this should be set
    ///      to the Tron controller's `RECEIVER_IMPL()` (it does not need to exist on the local chain).
    address public RECEIVER_IMPL;

    // Chain-specific byte prefix used in CREATE2 address calculation (0xff for EVM, 0x41 for Tron).
    bytes1 internal _CREATE2_PREFIX;

    /// @notice The address of the UntronController contract on Tron (source chain), in EVM format.
    /// @dev Stored as a 20-byte EVM address; converted to Tron 21-byte format when comparing Tron calldata.
    address public CONTROLLER_ADDRESS;

    /// @notice External Tron reader used to verify and decode Tron transactions.
    /// @dev The reader is expected to be bound to a Tron light client and to:
    /// - verify tx inclusion via Merkle proofs,
    /// - enforce tx success,
    /// - and expose sender / to / calldata for `TriggerSmartContract` transactions.
    ITronTxReader public tronReader;

    /// @notice Address of USDT on destination (EVM) chain used for all accounting.
    /// @dev    It's not immutable in case there's some crazy situation and we have
    ///         to move off USDT0. Since leases can last for years, the general engineering
    ///         philosophy of UntronV3 contract is ship-of-Theseusian.
    address public usdt;

    /// @notice Address of Tron USDT token (20-byte EVM-form of TRC‑20 contract address on Tron).
    /// @dev Used to enforce that preEntitle only recognizes Tron USDT transfers.
    address public tronUsdt;

    /// @notice Swap executor used for batched swaps before bridging.
    /// @dev Deployed in the constructor; only this UntronV3 instance can call `SwapExecutor.execute`.
    SwapExecutor public SWAP_EXECUTOR;

    /*//////////////////////////////////////////////////////////////
                           RECEIVER UTILS
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns the EIP-1167 creation bytecode for a receiver proxy.
    /// @dev `UntronReceiver` has no constructor args; the controller binding is via `msg.sender` at impl deploy time.
    /// @return The EIP-1167 creation bytecode for a receiver proxy.
    function receiverBytecode() public view returns (bytes memory) {
        return abi.encodePacked(
            hex"3d602d80600a3d3981f3363d3d373d3d3d363d73", RECEIVER_IMPL, hex"5af43d82803e903d91602b57fd5bf3"
        );
    }

    /// @notice Predict the deterministic address for a receiver deployed via CREATE2.
    /// @param controller The UntronController (deployer) address in 20-byte EVM form.
    /// @param salt The CREATE2 salt.
    /// @return predicted The predicted address of the receiver.
    function predictReceiverAddress(address controller, bytes32 salt) public view returns (address predicted) {
        predicted = address(
            uint160(
                uint256(keccak256(abi.encodePacked(_CREATE2_PREFIX, controller, salt, keccak256(receiverBytecode()))))
            )
        );
    }

    /// @notice Predict the deterministic address for a receiver deployed via CREATE2 by this contract.
    /// @param salt The CREATE2 salt.
    /// @return predicted The predicted address of the receiver.
    function predictReceiverAddress(bytes32 salt) public view returns (address predicted) {
        predicted = predictReceiverAddress(address(this), salt);
    }

    /// @notice Returns the current USDT balance held by this contract.
    /// @return The USDT balance held by this contract.
    /// @dev Returns 0 if `usdt` is not set.
    function usdtBalance() public view returns (uint256) {
        address usdt_ = usdt; // not sure if the compiler would optimize it into this anyway
        if (usdt_ == address(0)) return 0;
        return TokenUtils.getBalanceOf(usdt_, address(this));
    }

    /// @notice Next lease identifier.
    uint256 public nextLeaseId = 0;

    /// @notice Lease storage, keyed by receiver salt and sequential lease number.
    /// @dev The lease number is the index of the lease within `leasesByReceiver[receiverSalt]`.
    mapping(bytes32 => Lease[]) public leasesByReceiver;

    /// @notice Mapping from external lease id to internal lease locator.
    /// @dev External `leaseId` is assigned monotonically starting from 1.
    mapping(uint256 => LeaseLocator) internal _leaseLocatorById;

    // Slither may misdetect that _leaseIdsByReceiver is never initialized.
    // This is not true: in createLease, the storage element is taken as an "ids"
    // variable and is used to append a new lease ID to receiver salt-specific
    // leases array.
    // If you believe that this comment is no longer true under status quo code,
    // please update the code respectively.
    // slither-disable-start uninitialized-state

    /// @notice Timeline of leases per receiver salt.
    /// @dev The array is append-only. The "active lease at time T" is the last lease with `startTime <= T`.
    mapping(bytes32 => uint256[]) internal _leaseIdsByReceiver;

    // slither-disable-end uninitialized-state

    /// @notice Whitelisted realtors.
    mapping(address => bool) public isRealtor;

    /// @notice LP allowlist used to permission deposits into the fast-fill vault.
    /// @dev This mapping gates `deposit(...)` only:
    /// - If `isLpAllowed[lp] == false`, `deposit(...)` MUST revert for `lp`.
    /// - `withdraw(...)` MUST NOT check this mapping so that delisted LPs can still exit.
    ///
    /// Rationale:
    /// - The protocol may want to restrict who can provide fast-fill liquidity (e.g. compliance, KYC, risk).
    /// - Once principal has been deposited, the protocol must not "trap" LP funds by delisting them.
    mapping(address => bool) public isLpAllowed;

    /// @notice Protocol-wide configuration, managed by the owner.
    /// @dev Includes global fee floor and rate limit parameters.
    ProtocolConfig internal _protocolConfig;

    /// @notice Realtor-specific configuration overrides, managed by the owner.
    /// @dev Includes realtor-specific fee floors and rate limit override settings.
    mapping(address => RealtorConfig) internal _realtorConfig;

    // Slither may misdetect that _leaseCreationTimestampsByRealtor is never initialized.
    // This is not true: in _enforceLeaseRateLimit, the storage element is taken as a
    // "timestamps" variable and is used to enforce lease creation rate limits.
    // If you believe that this comment is no longer true under status quo code,
    // please update the code respectively.
    // slither-disable-start uninitialized-state

    /// @notice Timeline of lease creations per realtor for rate limiting.
    mapping(address => uint64[]) internal _leaseCreationTimestampsByRealtor;

    // slither-disable-end uninitialized-state

    // Slither may misdetect that _payoutConfigUpdateTimestampsByLessee is never initialized.
    // This is not true: in _enforcePayoutConfigRateLimit, the storage element is taken as a
    // "timestamps" variable and is used to enforce payout configuration update rate limits.
    // If you believe that this comment is no longer true under status quo code,
    // please update the code respectively.
    // slither-disable-start uninitialized-state

    /// @notice Timeline of payout config updates per lessee for rate limiting.
    mapping(address => uint64[]) internal _payoutConfigUpdateTimestampsByLessee;

    // slither-disable-end uninitialized-state

    /// @notice Signed protocol profit-and-loss (fees earned minus rebalance drift).
    /// @dev Positive deltas come from lease fees and favorable rebalances.
    ///      Negative deltas come from owner withdrawals and unfavorable rebalances.
    int256 public protocolPnl;

    /// @notice LP principal tracking.
    mapping(address => uint256) public lpPrincipal;

    /// @notice Swap rate per token, in parts-per-million of USDT.
    /// @dev Rate is expressed in swap destination token units per _RATE_DENOMINATOR of USDT.
    mapping(address => uint256) public swapRatePpm;

    /// @notice Bridger registry used when settling cross-chain claims.
    /// @dev Keyed by `(targetToken, targetChainId)`; missing entries make payout configs unroutable.
    mapping(address => mapping(uint256 => IBridger)) public bridgers;

    /// @notice Mapping of what chains are deprecated.
    /// @dev For deprecated chains, lessees can't set them in the payout config.
    mapping(uint256 => bool) public isChainDeprecated;

    /// @notice Last processed controller event-chain tip (starts at controller genesis).
    /// @dev Updated by `relayControllerEventChain` after validating a hash chain of provided events.
    bytes32 public lastControllerEventTip = EventChainGenesis.UntronControllerIndex;

    /// @notice Last processed controller event-chain sequence number (starts at 0 at controller genesis).
    /// @dev Needed because the controller tip hash includes a monotonically increasing `eventSeq`.
    uint256 public lastControllerEventSeq;

    /// @notice Queue of controller events awaiting processing on EVM.
    /// @dev Events are appended by `relayControllerEventChain` and consumed by `processControllerEvents`.
    ControllerEvent[] internal _controllerEvents;
    /// @notice Cursor into `_controllerEvents` indicating the next event to process.
    uint256 public nextControllerEventIndex;

    // Slither may misdetect that claimsByTargetToken is never initialized.
    // This is not true: in _enqueueClaimForTargetToken, the storage element is taken as a
    // "queue" variable and is used to add a claim to the target-token-specific claim queue.
    // If you believe that this comment is no longer true under status quo code,
    // please update the code respectively.
    // slither-disable-start uninitialized-state

    /// @notice Per-target-token FIFO claim queues for grouped swap+bridge fills.
    /// @dev Each `targetToken` has its own queue so fills can amortize swaps/bridges.
    mapping(address => Claim[]) public claimsByTargetToken;

    // slither-disable-end uninitialized-state

    /// @notice Per-target-token head index (cursor) for grouped queues.
    /// @dev We do not pop from arrays; instead we advance this cursor and delete filled claim slots.
    mapping(address => uint256) public nextIndexByTargetToken;

    /// @notice Next per-lease claim identifier to assign.
    mapping(uint256 => uint256) public nextClaimIdByLease;

    /// @notice Lookup from `(leaseId, claimId)` to the claim's current queue location.
    /// @dev `targetToken == address(0)` indicates missing / already filled.
    mapping(uint256 => mapping(uint256 => ClaimLocator)) public claimLocatorByLease;

    /// @notice Guard against double-processing of recognizable Tron deposits.
    /// @dev Keyed by the Tron transaction ID as computed by `ITronTxReader` (sha256 of `raw_data` bytes).
    mapping(bytes32 => bool) public depositProcessed;

    /// @notice Lookup from `txId` to a subjective pre-entitlement record, if any.
    mapping(bytes32 => SubjectivePreEntitlement) public subjectivePreEntitlementByTxId;

    /// @notice Latest processed Tron timestamp of a `PulledFromReceiver` event for a receiver salt and token.
    /// @dev Used to enforce that `preEntitle` cannot recognize deposits that occurred at/before the latest known pull
    ///      for the deposit token (currently `tronUsdt`).
    mapping(bytes32 => mapping(address => uint64)) public lastReceiverPullTimestampByToken;

    /// @notice Nonces per lease for gasless payout config updates.
    /// @dev Incremented in `setPayoutConfigWithSig` after a successful signature validation.
    mapping(uint256 => uint256) public leaseNonces;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when an operation is requested with `amount == 0` (where 0 is not meaningful).
    error ZeroAmount();
    /// @notice Thrown when the owner attempts to withdraw more profit than `protocolPnl` currently allows.
    error InsufficientProtocolProfit();
    /// @notice Thrown when attempting to rescue the protocol's accounting token via `rescueTokens`.
    error CannotRescueUSDT();
    /// @notice Thrown when a non-whitelisted address attempts a realtor-only operation.
    error NotRealtor();
    /// @notice Thrown when a caller attempts to deposit into the LP vault without being allowlisted.
    /// @dev LP allowlisting is an explicit owner-managed permission gate for `deposit(...)` only.
    ///      Even if an LP is later delisted, it MUST still be able to withdraw any remaining principal.
    error LpNotAllowlisted();
    /// @notice Thrown when a proposed lease fee is outside the allowed bounds or below configured floors.
    error LeaseFeeTooLow();
    /// @notice Thrown when a proposed lease flat fee is below configured floors.
    error LeaseFlatFeeTooLow();
    /// @notice Thrown when a proposed lease timeframe is invalid (e.g. nukeableAfter in the past).
    error InvalidLeaseTimeframe();
    /// @notice Thrown when a proposed lease duration exceeds configured maximums.
    error LeaseDurationTooLong();
    /// @notice Thrown when trying to create a new lease for a receiver before the previous one is nukeable.
    error LeaseNotNukeableYet();
    /// @notice Thrown when referencing a lease id that does not exist.
    error InvalidLeaseId();
    /// @notice Thrown when a caller is not the current lessee of a lease.
    error NotLessee();
    /// @notice Thrown when attempting to pre-entitle or attribute volume without any lease active at a timestamp.
    error NoActiveLease();
    /// @notice Thrown when a recognizable Tron deposit tx is submitted more than once.
    error DepositAlreadyProcessed();
    /// @notice Thrown when a Tron deposit's block timestamp is not strictly after the latest known receiver pull.
    error DepositNotAfterLastReceiverPull();
    /// @notice Thrown when a proved Tron transfer is not sent to the predicted receiver for a given salt.
    error InvalidReceiverForSalt();
    /// @notice Thrown when an LP attempts to withdraw more than their accounted principal.
    error WithdrawExceedsPrincipal();
    /// @notice Thrown when the contract does not have enough USDT balance for a withdrawal or fill.
    error InsufficientUsdtBalance();
    /// @notice Thrown when attempting to subjectively pre-entitle a tx id that already has a record.
    error SubjectivePreEntitlementAlreadyExists();
    /// @notice Thrown when an LP does not have enough principal to fund a subjective pre-entitlement.
    error InsufficientLpPrincipal();
    /// @notice Thrown when a subjective pre-entitlement would pay `netOut == 0`.
    error SubjectiveNetOutZero();
    /// @notice Thrown when `preEntitle` is called for a TRC-20 transfer that is not Tron USDT.
    error NotTronUsdt();
    /// @notice Thrown when `relayControllerEventChain` is not proving an `isEventChainTip` call to the controller.
    error NotEventChainTip();
    /// @notice Thrown when the provided controller events do not hash-link to the proved tip.
    error EventTipMismatch();
    /// @notice Thrown when `relayControllerEventChain` would not advance `lastControllerEventTip`.
    error EventRelayNoProgress();
    /// @notice Thrown when an EIP-712 / ERC-1271 signature validation fails.
    error InvalidSignature();
    /// @notice Thrown when a signature-based update is submitted after its `deadline`.
    error SignatureExpired();
    /// @notice Thrown when a payout config targets a chain that has been deprecated by the owner.
    error ChainDeprecated();
    /// @notice Thrown when a required swap/bridge rate is not configured (`swapRatePpm[targetToken] == 0`).
    error RateNotSet();
    /// @notice Thrown when a required bridger is not configured for a `(targetToken, targetChainId)` pair.
    error NoBridger();
    /// @notice Thrown when a required token address is the zero address.
    error InvalidTargetToken();
    /// @notice Thrown when a `uint256` value cannot be safely cast to `int256`.
    error AmountTooLargeForInt();
    /// @notice Thrown when configuring lease rate limits with invalid parameters.
    error LeaseRateLimitConfigInvalid();
    /// @notice Thrown when a realtor exceeds its effective lease creation rate limit.
    error LeaseRateLimitExceeded();
    /// @notice Thrown when configuring payout-config update rate limits with invalid parameters.
    error PayoutConfigRateLimitConfigInvalid();
    /// @notice Thrown when a lessee exceeds the payout-config update rate limit.
    error PayoutConfigRateLimitExceeded();

    /// @notice Thrown when Tron calldata decoding fails basic length invariants.
    /// @dev Local copy of reader-side invariants to preserve revert selectors across libraries.
    error TronInvalidCalldataLength();

    /*//////////////////////////////////////////////////////////////
                           INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Load a lease by external `leaseId` as a storage pointer.
    /// @param leaseId The external lease ID.
    /// @return lease The lease storage pointer.
    /// @dev Reverts with `InvalidLeaseId()` if the lease does not exist.
    function _leaseStorage(uint256 leaseId) internal view returns (Lease storage lease) {
        LeaseLocator storage loc = _leaseLocatorById[leaseId];
        uint256 leaseNumberPlusOne = loc.leaseNumberPlusOne;
        if (leaseNumberPlusOne == 0) revert InvalidLeaseId();
        lease = leasesByReceiver[loc.receiverSalt][leaseNumberPlusOne - 1];
    }

    /// @notice Enforces the effective lease creation rate limit for `realtor`, if enabled.
    /// @dev Uses an append-only timestamp array per realtor and checks the timestamp at index `len - maxLeases`
    ///      to determine whether the oldest of the last `maxLeases` creations is outside the window.
    /// @param realtor Realtor whose lease creation is being rate-limited.
    function _enforceLeaseRateLimit(address realtor) internal {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        uint256 maxLeases = cfg.leaseRateLimitMaxLeases;
        uint256 windowSeconds = cfg.leaseRateLimitWindowSeconds;
        if (maxLeases == 0 || windowSeconds == 0) return;

        // Record current timestamp (fits into uint64 for the foreseeable future).
        uint64 nowTs = uint64(block.timestamp);
        uint64[] storage timestamps = _leaseCreationTimestampsByRealtor[realtor];
        uint256 len = timestamps.length;

        // solhint-disable-next-line gas-strict-inequalities
        if (len >= maxLeases) {
            // Oldest timestamp among the most recent `maxLeases` creations.
            uint64 oldest = timestamps[len - maxLeases];
            // If oldest is still within the window, we're over the rate limit.
            if (uint256(nowTs) < uint256(oldest) + windowSeconds) revert LeaseRateLimitExceeded();
        }

        // Append timestamp for this creation (arrays grow unbounded by design).
        timestamps.push(nowTs);
    }

    /// @notice Enforces the payout-config update rate limit for `lessee`, if enabled.
    /// @param lessee Lessee being rate-limited.
    function _enforcePayoutConfigRateLimit(address lessee) internal {
        ProtocolConfig storage cfg = _protocolConfig;
        uint256 maxUpdates = cfg.payoutConfigRateLimitMaxUpdates;
        uint256 windowSeconds = cfg.payoutConfigRateLimitWindowSeconds;
        if (maxUpdates == 0 || windowSeconds == 0) return;

        // Record current timestamp (fits into uint64 for the foreseeable future).
        uint64 nowTs = uint64(block.timestamp);
        uint64[] storage timestamps = _payoutConfigUpdateTimestampsByLessee[lessee];
        uint256 len = timestamps.length;

        // solhint-disable-next-line gas-strict-inequalities
        if (len >= maxUpdates) {
            // Oldest timestamp among the most recent `maxUpdates` updates.
            uint64 oldest = timestamps[len - maxUpdates];
            // If oldest is still within the window, we're over the rate limit.
            if (uint256(nowTs) < uint256(oldest) + windowSeconds) revert PayoutConfigRateLimitExceeded();
        }

        // Append timestamp for this update (arrays grow unbounded by design).
        timestamps.push(nowTs);
    }

    /// @notice Enforce all non-route preconditions for `createLease`.
    /// @param realtor Realtor creating the lease (must be whitelisted).
    /// @param receiverSalt CREATE2 receiver salt for the deterministic Tron receiver.
    /// @param nukeableAfter Earliest time at which a subsequent lease for this receiver may be created.
    /// @param leaseFeePpm Lease fee in ppm to validate against configured floors/bounds.
    /// @param flatFee Lease flat fee in USDT units to validate against configured floors.
    /// @param targetChainId Destination chain id to validate against deprecations.
    function _enforceCreateLeasePreconditions(
        address realtor,
        bytes32 receiverSalt,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee,
        uint256 targetChainId
    ) internal {
        // Realtors are the only actors allowed to create leases.
        if (!isRealtor[realtor]) revert NotRealtor();

        // Apply effective (protocol-wide or realtor override) rate limiting.
        _enforceLeaseRateLimit(realtor);

        // Enforce fee bounds and floors.
        uint256 minFee = _minLeaseFeePpm(realtor);
        if (leaseFeePpm < minFee || leaseFeePpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();

        // Enforce flat fee floors.
        uint256 minFlatFee = _minLeaseFlatFee(realtor);
        if (uint256(flatFee) < minFlatFee) revert LeaseFlatFeeTooLow();

        // Disallow creating leases that immediately target a deprecated chain.
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

        uint64 startTime = _leaseStartTime();
        // Prevent leases that are already nukeable (nukeableAfter must be in the future/present).
        if (nukeableAfter < startTime) revert InvalidLeaseTimeframe();

        uint256 maxDuration = _maxLeaseDurationSeconds(realtor);
        if (maxDuration != 0) {
            uint256 duration = uint256(nukeableAfter) - uint256(startTime);
            if (duration > maxDuration) revert LeaseDurationTooLong();
        }

        // Uniqueness is enforced per receiver salt regardless of token.
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        if (ids.length != 0) {
            Lease storage last = leasesByReceiver[receiverSalt][ids.length - 1];
            // Disallow nuking before previous lease becomes nukeable.
            if (block.timestamp < last.nukeableAfter) revert LeaseNotNukeableYet();
        }
    }

    /// @notice Persist a newly created lease and append it to the receiver's lease timeline.
    /// @param leaseId Newly allocated lease id.
    /// @param receiverSalt CREATE2 receiver salt for the deterministic Tron receiver.
    /// @param realtor Realtor creating the lease.
    /// @param lessee Lessee that can later update payout config.
    /// @param startTime Lease start time on this chain.
    /// @param nukeableAfter Earliest time at which a subsequent lease for this receiver may be created.
    /// @param leaseFeePpm Lease fee in ppm.
    /// @param flatFee Flat fee component of the lease fee schedule.
    /// @param targetChainId Destination chain for payouts.
    /// @param targetToken Settlement token on this chain.
    /// @param beneficiary Recipient of the payout.
    /// @return leaseNumber Index of the newly created lease in the receiver's lease array.
    function _storeLease(
        uint256 leaseId,
        bytes32 receiverSalt,
        address realtor,
        address lessee,
        uint64 startTime,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee,
        uint256 targetChainId,
        address targetToken,
        address beneficiary
    ) internal returns (uint256 leaseNumber) {
        // Lease number is "how many leases this receiver has had so far".
        leaseNumber = leasesByReceiver[receiverSalt].length;

        // Store locator by external id (plus-one encodes existence).
        _leaseLocatorById[leaseId] = LeaseLocator({receiverSalt: receiverSalt, leaseNumberPlusOne: leaseNumber + 1});

        // Persist the lease under (receiverSalt, leaseNumber).
        leasesByReceiver[receiverSalt].push(
            Lease({
                receiverSalt: receiverSalt,
                realtor: realtor,
                lessee: lessee,
                startTime: startTime,
                nukeableAfter: nukeableAfter,
                leaseFeePpm: leaseFeePpm,
                flatFee: flatFee,
                recognizedRaw: 0,
                backedRaw: 0,
                unbackedRaw: 0,
                payout: PayoutConfig({targetChainId: targetChainId, targetToken: targetToken, beneficiary: beneficiary})
            })
        );

        // Append to the receiver's lease timeline.
        _leaseIdsByReceiver[receiverSalt].push(leaseId);
    }

    /// @notice Consume a lease nonce and emit the updated value.
    /// @param leaseId Lease whose nonce is being consumed.
    /// @param nonce Current nonce value.
    function _consumeLeaseNonce(uint256 leaseId, uint256 nonce) internal {
        // Consume nonce (unchecked is safe: 2^256 wrap is not realistically reachable).
        unchecked {
            leaseNonces[leaseId] = nonce + 1;
        }
        _emitLeaseNonceUpdated(leaseId, nonce + 1);
    }

    /// @notice Persist payout config for future claims created by a lease.
    /// @param lease Lease being updated.
    /// @param config New payout configuration.
    function _setLeasePayoutConfig(Lease storage lease, PayoutConfig calldata config) internal {
        lease.payout = PayoutConfig({
            targetChainId: config.targetChainId, targetToken: config.targetToken, beneficiary: config.beneficiary
        });
    }

    /// @notice Hash-link a list of controller events starting from `tip`, emitting an index event per hop.
    /// @param tip Starting tip (current `lastControllerEventTip`).
    /// @param seq Starting sequence number (current `lastControllerEventSeq`).
    /// @param events Controller events to hash-link.
    /// @return New tip after hashing all events.
    /// @return New sequence number after hashing all events.
    function _hashLinkControllerEventsAndEmit(bytes32 tip, uint256 seq, ControllerEvent[] calldata events)
        internal
        returns (bytes32, uint256)
    {
        uint256 n = events.length;
        for (uint256 i = 0; i < n; ++i) {
            ControllerEvent calldata ev = events[i];
            // Important: Tron-side UntronControllerIndex hashes block fields as uint256 via abi.encodePacked.
            // We keep calldata compact with uint64 fields, but must cast here to preserve cross-chain tip equality.
            _emitControllerEventChainTipUpdated(
                tip, uint256(ev.blockNumber), uint256(ev.blockTimestamp), ev.sig, ev.data
            );
            unchecked {
                ++seq;
            }
            tip = sha256(
                abi.encodePacked(tip, seq, uint256(ev.blockNumber), uint256(ev.blockTimestamp), ev.sig, ev.data)
            );
        }
        return (tip, seq);
    }

    /// @notice Enqueue controller events for later processing.
    /// @param events Controller events to append to the processing queue.
    function _enqueueControllerEvents(ControllerEvent[] calldata events) internal {
        uint256 n = events.length;
        for (uint256 i = 0; i < n; ++i) {
            ControllerEvent calldata ev = events[i];
            _controllerEvents.push(
                ControllerEvent({
                    sig: ev.sig, data: ev.data, blockNumber: ev.blockNumber, blockTimestamp: ev.blockTimestamp
                })
            );
        }
    }

    /// @notice Apply a delta to `protocolPnl` and emit the updated value via `UntronV3Index`.
    /// @param delta Signed change to apply.
    /// @param reason Reason code for indexing/analytics.
    function _applyPnlDelta(int256 delta, PnlReason reason) internal {
        if (delta == 0) return;
        protocolPnl += delta;
        _emitProtocolPnlUpdated(protocolPnl, delta, reason);
    }

    /// @notice Book protocol fee revenue for a recognized raw amount.
    /// @param raw Raw recognized volume (USDT units).
    /// @param netOut Net amount that will be paid out to the beneficiary (USDT units).
    function _bookFee(uint256 raw, uint256 netOut) internal {
        _applyPnlDelta(_toInt(raw - netOut), PnlReason.FEE);
    }

    /// @notice Append a claim to `claimsByTargetToken[targetToken]`.
    /// @param targetToken Queue key.
    /// @param amountUsdt USDT-denominated claim amount.
    /// @param leaseId Lease that produced the claim.
    /// @param targetChainId Destination chain id for the claim payout.
    /// @param beneficiary Recipient of the claim payout.
    /// @return queueIndex Index of the appended claim within the queue.
    /// @return claimId Per-lease claim identifier.
    function _enqueueClaimForTargetToken(
        address targetToken,
        uint256 amountUsdt,
        uint256 leaseId,
        uint256 targetChainId,
        address beneficiary
    ) internal returns (uint256 queueIndex, uint256 claimId) {
        // solhint-disable-next-line gas-increment-by-one
        claimId = nextClaimIdByLease[leaseId]++;
        // Append claim to the per-token queue.
        Claim[] storage queue = claimsByTargetToken[targetToken];
        queue.push(
            Claim({
                claimId: claimId,
                amountUsdt: amountUsdt,
                leaseId: leaseId,
                targetChainId: targetChainId,
                beneficiary: beneficiary
            })
        );
        // Claim index is the array index of the appended element.
        queueIndex = queue.length - 1;
        claimLocatorByLease[leaseId][claimId] = ClaimLocator({targetToken: targetToken, queueIndex: queueIndex});
        return (queueIndex, claimId);
    }

    /// @notice Emit a `ClaimCreated` event for a claim created by `preEntitle`.
    /// @param leaseId Lease id producing the claim.
    /// @param claimId Per-lease claim id assigned to the new claim.
    /// @param p Payout config used at claim creation time (lease-local config snapshot).
    /// @param queueIndex Index in `claimsByTargetToken[p.targetToken]` where the claim was appended.
    /// @param amountUsdt Claim amount in USDT units (net after lease fees).
    /// @param txId Proven Tron tx id (sha256(raw_data)).
    /// @param tronBlockTimestamp Tron block timestamp for the proved tx.
    /// @param rawAmount Raw TRC-20 amount recognized (before fees).
    function _emitPreEntitleClaimCreated(
        uint256 leaseId,
        uint256 claimId,
        PayoutConfig storage p,
        uint256 queueIndex,
        uint256 amountUsdt,
        bytes32 txId,
        uint32 tronBlockTimestamp,
        uint256 rawAmount
    ) internal {
        _emitClaimCreated(
            ClaimCreatedArgs({
                leaseId: leaseId,
                claimId: claimId,
                targetToken: p.targetToken,
                queueIndex: queueIndex,
                amountUsdt: amountUsdt,
                targetChainId: p.targetChainId,
                beneficiary: p.beneficiary,
                origin: ClaimOrigin.PRE_ENTITLE,
                originId: txId,
                originActor: address(0),
                originToken: address(0),
                originTimestamp: uint64(tronBlockTimestamp),
                originRawAmount: rawAmount
            })
        );
    }

    /// @notice Emit a `ClaimCreated` event for a claim created by `subjectivePreEntitle`.
    /// @dev This claim is indistinguishable to the `fill()` pipeline from any other claim; only the `origin` metadata differs.
    /// @param leaseId Lease id producing the claim.
    /// @param claimId Per-lease claim id assigned to the new claim.
    /// @param p Payout config used at claim creation time (lease-local config snapshot).
    /// @param queueIndex Index in `claimsByTargetToken[p.targetToken]` where the claim was appended.
    /// @param amountUsdt Claim amount in USDT units (net after lease fees).
    /// @param txId Anticipated Tron tx id (sha256(raw_data)).
    /// @param sponsor LP address whose principal was debited to create this claim.
    /// @param rawAmount Sponsor-provided raw amount guess (before fees).
    function _emitSubjectivePreEntitleClaimCreated(
        uint256 leaseId,
        uint256 claimId,
        PayoutConfig storage p,
        uint256 queueIndex,
        uint256 amountUsdt,
        bytes32 txId,
        address sponsor,
        uint256 rawAmount
    ) internal {
        _emitClaimCreated(
            ClaimCreatedArgs({
                leaseId: leaseId,
                claimId: claimId,
                targetToken: p.targetToken,
                queueIndex: queueIndex,
                amountUsdt: amountUsdt,
                targetChainId: p.targetChainId,
                beneficiary: p.beneficiary,
                origin: ClaimOrigin.SUBJECTIVE_PRE_ENTITLE,
                originId: txId,
                originActor: sponsor,
                originToken: address(0),
                originTimestamp: 0,
                originRawAmount: rawAmount
            })
        );
    }

    /* solhint-enable function-max-lines */

    /// @notice Enqueue and emit a claim created when processing a `PulledFromReceiver` event's remaining profit volume.
    /// @dev This is only used for the "remaining" (profit) part, not for backing repayments.
    /// @param leaseId Lease id active at `dumpTimestamp`.
    /// @param cur Lease storage pointer (same as `_leaseStorage(leaseId)` but already loaded by caller).
    /// @param amountUsdt Net claim amount in USDT units (after lease fees).
    /// @param receiverSalt Receiver salt affected by the pull.
    /// @param token Token address reported by the controller event.
    /// @param dumpTimestamp Tron timestamp at which the pull occurred (seconds since epoch).
    /// @param rawAmount Raw USDT-equivalent amount being treated as newly recognized profit volume (before fees).
    function _enqueueReceiverPullClaim(
        uint256 leaseId,
        Lease storage cur,
        uint256 amountUsdt,
        bytes32 receiverSalt,
        address token,
        uint64 dumpTimestamp,
        uint256 rawAmount
    ) internal {
        // Enqueue claim using the current payout config.
        PayoutConfig storage p = cur.payout;
        (uint256 queueIndex, uint256 claimId) =
            _enqueueClaimForTargetToken(p.targetToken, amountUsdt, leaseId, p.targetChainId, p.beneficiary);
        _emitClaimCreated(
            ClaimCreatedArgs({
                leaseId: leaseId,
                claimId: claimId,
                targetToken: p.targetToken,
                queueIndex: queueIndex,
                amountUsdt: amountUsdt,
                targetChainId: p.targetChainId,
                beneficiary: p.beneficiary,
                origin: ClaimOrigin.RECEIVER_PULL,
                originId: receiverSalt,
                originActor: address(0),
                originToken: token,
                originTimestamp: dumpTimestamp,
                originRawAmount: rawAmount
            })
        );
    }

    /* solhint-disable function-max-lines */

    /// @notice The internal function for processing PulledFromReceiver controller events.
    /// @dev Handle a `PulledFromReceiver` controller event by reconciling unbacked volume and/or creating new claims.
    ///
    /// The controller reports a USDT amount pulled out of the receiver(s) for a given `receiverSalt`.
    ///
    /// If `token == tronUsdt`, we treat this as "backing" previously recognized but unbacked pre-entitled volume,
    /// oldest-first across the lease timeline.
    ///
    /// If `token != tronUsdt`, the pull is treated as additional USDT-equivalent "profit volume" and does NOT back
    /// any pre-entitled volume (to avoid mixing accounting between USDT deposits and non-USDT sweeps).
    ///
    /// If the pulled amount exceeds total unbacked volume at/through `dumpTimestamp`, the remaining amount is treated
    /// as new recognized volume for the lease active at `dumpTimestamp` ("profit volume") and is subject to fees and
    /// claim creation like `preEntitle`. If there is no active lease at that time, the remainder is booked as protocol
    /// PnL with reason `RECEIVER_PULL_NO_LEASE`.
    ///
    /// @param receiverSalt Receiver salt whose lease timeline is affected.
    /// @param token Token address pulled from the receiver (as reported by the controller event).
    /// @param usdtAmount Total USDT amount reported as pulled from receivers by the controller.
    /// @param dumpTimestamp Tron timestamp at which the pull occurred (used to find active lease).
    function _processReceiverPulled(bytes32 receiverSalt, address token, uint256 usdtAmount, uint64 dumpTimestamp)
        internal
    {
        // Track the latest observed pull timestamp for this receiver salt and token (monotonic by design).
        uint64 prevToken = lastReceiverPullTimestampByToken[receiverSalt][token];
        if (dumpTimestamp > prevToken) lastReceiverPullTimestampByToken[receiverSalt][token] = dumpTimestamp;

        if (usdtAmount == 0) {
            return;
        }

        // Remaining amount to allocate between backing repayment and (possibly) new profit volume.
        uint256 remaining = usdtAmount;

        // Only canonical USDT pulls can back pre-entitled (USDT-denominated) unbacked volume.
        if (token == tronUsdt) {
            // Repay historical unbacked volume across leases for receiverSalt.
            uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
            uint256 len = ids.length;
            for (uint256 j = 0; j < len && remaining != 0; ++j) {
                Lease storage oldL = leasesByReceiver[receiverSalt][j];
                // Stop if we reached leases that start after the pull timestamp.
                if (oldL.startTime > dumpTimestamp) break;
                uint256 unbacked = oldL.unbackedRaw;
                if (unbacked == 0) continue;
                // Repay up to the unbacked amount for this lease.
                uint256 repay = remaining < unbacked ? remaining : unbacked;
                oldL.backedRaw += repay;
                oldL.unbackedRaw = unbacked - repay;
                remaining -= repay;
            }
        }

        // Any remaining volume becomes profit for the lease active at dump time.
        if (remaining != 0) {
            // Find lease active at the pull timestamp.
            uint256 currentLeaseId = _findActiveLeaseId(receiverSalt, dumpTimestamp);
            if (currentLeaseId == 0) {
                // If no lease is active, attribute remainder to protocol PnL.
                _applyPnlDelta(_toInt(remaining), PnlReason.RECEIVER_PULL_NO_LEASE);
                return;
            }

            Lease storage cur = _leaseStorage(currentLeaseId);
            // Treat remainder as newly recognized & backed volume for the current lease.
            cur.recognizedRaw += remaining;
            cur.backedRaw += remaining;

            // Compute net payout and book fees.
            uint256 netOut = _computeNetOut(cur, remaining);
            _bookFee(remaining, netOut);
            if (netOut > 0) {
                _enqueueReceiverPullClaim(currentLeaseId, cur, netOut, receiverSalt, token, dumpTimestamp, remaining);
            }
        }
    }

    /* solhint-enable function-max-lines */

    /// @notice Execute a swap for a batch of claims.
    /// @param targetToken Token to swap into.
    /// @param totalUsdt Total USDT to swap.
    /// @param expectedOutTotal Total expected output.
    /// @param calls Swap calls.
    /// @return surplusOut Amount of urplus output.
    function _swapForBatch(address targetToken, uint256 totalUsdt, uint256 expectedOutTotal, Call[] calldata calls)
        internal
        returns (uint256 surplusOut)
    {
        TokenUtils.transfer(usdt, payable(address(SWAP_EXECUTOR)), totalUsdt);
        uint256 actualOut = SWAP_EXECUTOR.execute(calls, targetToken, expectedOutTotal, payable(address(this)));
        if (actualOut > expectedOutTotal) {
            surplusOut = actualOut - expectedOutTotal;
        }
    }

    /// @notice Settle a contiguous range of claim slots for a given queue token.
    /// @param targetToken The target token to settle.
    /// @param ratePpm The rate in parts per million.
    /// @param queue The claim queue.
    /// @param start The first index (inclusive) to settle.
    /// @param end The end index (exclusive) to settle.
    /// @dev Assumes `ratePpm != 0` if `targetToken != usdt`.
    function _settleClaimRange(address targetToken, uint256 ratePpm, Claim[] storage queue, uint256 start, uint256 end)
        internal
    {
        for (uint256 idx = start; idx < end; ++idx) {
            Claim memory c = queue[idx];
            uint256 amountUsdt = c.amountUsdt;

            // Delete before any external interaction.
            delete queue[idx];
            delete claimLocatorByLease[c.leaseId][c.claimId];

            uint256 outAmount =
                targetToken == usdt ? amountUsdt : TokenUtils.mulDiv(amountUsdt, ratePpm, _RATE_DENOMINATOR);

            if (outAmount != 0) {
                if (c.targetChainId != block.chainid) {
                    IBridger bridger = bridgers[targetToken][c.targetChainId];
                    if (address(bridger) == address(0)) revert NoBridger();

                    TokenUtils.transfer(targetToken, payable(address(bridger)), outAmount);
                    bridger.bridge(targetToken, outAmount, c.targetChainId, c.beneficiary);
                } else {
                    TokenUtils.transfer(targetToken, payable(c.beneficiary), outAmount);
                }
            }

            _emitClaimFilled(c.leaseId, c.claimId, targetToken, idx, amountUsdt, c.targetChainId, c.beneficiary);
        }
    }

    /*//////////////////////////////////////////////////////////////
                          INTERNAL VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Return the current lease start time (EVM `block.timestamp`) as `uint64`.
    /// @return startTime Current timestamp as `uint64`.
    function _leaseStartTime() internal view returns (uint64 startTime) {
        // Record the lease start time as the EVM chain timestamp.
        startTime = uint64(block.timestamp);
    }

    /// @notice Decode a proved Tron `TriggerSmartContract` call as a recognizable TRC-20 USDT deposit into a receiver.
    /// @dev This is used by `preEntitle` to:
    /// - enforce the called contract is `tronUsdt_`,
    /// - enforce the decoded recipient matches the deterministic receiver for `receiverSalt`,
    /// - and extract the raw transferred amount (`amountQ`) in USDT units (6 decimals expected, but treated as raw).
    /// @param receiverSalt CREATE2 salt used to derive the receiver address on Tron.
    /// @param toTron Tron contract address being called (must match `tronUsdt_`).
    /// @param senderTron Tron sender address for calldata decoding context.
    /// @param data Tron calldata for the `TriggerSmartContract` call.
    /// @param tronUsdt_ Expected Tron USDT address (as an EVM address representation).
    /// @return amountQ Decoded raw TRC-20 transfer amount (before fees).
    function _decodeRecognizableTronUsdtDepositAmount(
        bytes32 receiverSalt,
        bytes21 toTron,
        bytes21 senderTron,
        bytes memory data,
        address tronUsdt_
    ) internal view returns (uint256 amountQ) {
        if (toTron != TronCalldataUtils.evmToTronAddress(tronUsdt_)) revert NotTronUsdt();

        // Sanity-check that the TRC-20 transfer goes into the expected receiver.
        address predictedReceiver = predictReceiverAddress(CONTROLLER_ADDRESS, receiverSalt);
        bytes21 expectedToTron = TronCalldataUtils.evmToTronAddress(predictedReceiver);
        (, bytes21 decodedToTron, uint256 decodedAmountQ) = TronCalldataUtils.decodeTrc20FromCalldata(data, senderTron);
        if (decodedToTron != expectedToTron) revert InvalidReceiverForSalt();
        return decodedAmountQ;
    }

    /// @notice Enforce that a proved deposit timestamp is strictly after the latest observed receiver pull for this receiver+token.
    /// @dev Prevents recognizing deposits that occurred at/before a known pull, which would break backing ordering assumptions.
    /// @param receiverSalt CREATE2 salt used to derive the receiver address on Tron.
    /// @param tronUsdt_ Expected Tron USDT token address (as an EVM address representation).
    /// @param tronBlockTimestamp Tron block timestamp for the proved tx.
    function _enforceDepositNotAfterLastReceiverPull(bytes32 receiverSalt, address tronUsdt_, uint32 tronBlockTimestamp)
        internal
        view
    {
        // Do not recognize deposits that occurred at/before the latest known `PulledFromReceiver` timestamp
        // for this receiver salt and token.
        uint64 lastPullTs = lastReceiverPullTimestampByToken[receiverSalt][tronUsdt_];
        // solhint-disable-next-line gas-strict-inequalities
        if (lastPullTs != 0 && uint64(tronBlockTimestamp) <= lastPullTs) {
            revert DepositNotAfterLastReceiverPull();
        }
    }

    /// @notice Enforce that a payout config is valid to set (not deprecated + routable).
    /// @param targetChainId_ Destination chain id.
    /// @param targetToken_ Settlement token on this chain.
    function _enforcePayoutConfigRoutable(uint256 targetChainId_, address targetToken_) internal view {
        // this technically makes changing beneficiaries but not chains revert too
        // but i think it's fine because hey mf you're the one who stops us from deprecating it
        // Disallow setting deprecated chains even if only changing beneficiary.
        if (isChainDeprecated[targetChainId_]) revert ChainDeprecated();

        if (targetToken_ == address(0)) revert InvalidTargetToken();

        bool needsSwap = targetToken_ != usdt;
        bool needsBridge = targetChainId_ != block.chainid;

        if (needsSwap) {
            if (swapRatePpm[targetToken_] == 0) revert RateNotSet();
        }
        if (needsBridge) {
            if (address(bridgers[targetToken_][targetChainId_]) == address(0)) revert NoBridger();
        }
    }

    /// @notice Compute the EIP-712 digest for a payout config update.
    /// @param leaseId Lease being updated.
    /// @param targetChainId_ Destination chain id.
    /// @param targetToken_ Settlement token on this chain.
    /// @param beneficiary_ Recipient of the payout.
    /// @param nonce Current per-lease nonce.
    /// @param deadline Timestamp after which the signature is invalid.
    /// @return digest EIP-712 digest to validate against the signature.
    function _payoutConfigUpdateDigest(
        uint256 leaseId,
        uint256 targetChainId_,
        address targetToken_,
        address beneficiary_,
        uint256 nonce,
        uint256 deadline
    ) internal view returns (bytes32 digest) {
        bytes32 typehash = _PAYOUT_CONFIG_UPDATE_TYPEHASH;

        bytes32 structHash;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            // Load free memory pointer
            let ptr := mload(0x40)

            // abi.encode(
            //   _PAYOUT_CONFIG_UPDATE_TYPEHASH,
            //   leaseId,
            //   targetChainId,
            //   targetToken,
            //   beneficiary,
            //   nonce,
            //   deadline
            // )
            mstore(ptr, typehash)
            mstore(add(ptr, 0x20), leaseId)
            mstore(add(ptr, 0x40), targetChainId_)
            mstore(add(ptr, 0x60), targetToken_)
            mstore(add(ptr, 0x80), beneficiary_)
            mstore(add(ptr, 0xa0), nonce)
            mstore(add(ptr, 0xc0), deadline)

            structHash := keccak256(ptr, 0xe0)

            // Update free memory pointer
            mstore(0x40, add(ptr, 0xe0))
        }

        digest = _hashTypedData(structHash);
    }

    /// @notice Enforce a valid ECDSA/ERC-1271 signature.
    /// @param signer Expected signer (lessee).
    /// @param digest EIP-712 digest that was signed.
    /// @param signature Signature bytes.
    function _enforceValidSignature(address signer, bytes32 digest, bytes calldata signature) internal view {
        bool ok = signer.isValidSignatureNow(digest, signature);
        if (!ok) revert InvalidSignature();
    }

    /// @notice Compute the effective minimum lease fee (ppm) for a given realtor.
    /// @param realtor Realtor to compute minimum for.
    /// @return minFeePpm The maximum of protocol-wide and realtor-specific fee floors.
    function _minLeaseFeePpm(address realtor) internal view returns (uint256) {
        uint256 minFee = uint256(_protocolConfig.floorPpm);
        uint256 realtorMin = uint256(_realtorConfig[realtor].minFeePpm);
        if (realtorMin > minFee) minFee = realtorMin;
        return minFee;
    }

    /// @notice Compute the effective minimum lease flat fee for a given realtor.
    /// @param realtor Realtor to compute minimum for.
    /// @return minFlatFee The maximum of protocol-wide and realtor-specific flat fee floors.
    function _minLeaseFlatFee(address realtor) internal view returns (uint256) {
        uint256 minFlatFee = uint256(_protocolConfig.floorFlatFee);
        uint256 realtorMin = uint256(_realtorConfig[realtor].minFlatFee);
        if (realtorMin > minFlatFee) minFlatFee = realtorMin;
        return minFlatFee;
    }

    /// @notice Compute the effective maximum lease duration for a given realtor.
    /// @dev If both protocol and realtor configs are 0, max duration is disabled (returns 0).
    /// @param realtor Realtor to compute max duration for.
    /// @return maxDurationSeconds Effective max duration in seconds (0 disables).
    function _maxLeaseDurationSeconds(address realtor) internal view returns (uint256) {
        uint256 protocolMax = uint256(_protocolConfig.maxLeaseDurationSeconds);
        uint256 realtorMax = uint256(_realtorConfig[realtor].maxLeaseDurationSeconds);

        if (protocolMax == 0) return realtorMax;
        if (realtorMax == 0) return protocolMax;
        return (realtorMax < protocolMax) ? realtorMax : protocolMax;
    }

    /// @notice Find the lease that was active for `receiverSalt` at timestamp `ts`.
    /// @dev This walks the receiver's lease timeline backwards and returns the last lease whose `startTime <= ts`.
    /// @param receiverSalt Receiver salt whose timeline is queried.
    /// @param ts Timestamp to query (seconds since epoch; may be an EVM or Tron timestamp depending on caller).
    /// @return leaseId Active lease id at `ts`, or 0 if none.
    function _findActiveLeaseId(bytes32 receiverSalt, uint64 ts) internal view returns (uint256 leaseId) {
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        uint256 len = ids.length;
        if (len == 0) return 0;

        // Walk backwards until we find the last lease with startTime <= ts.
        for (uint256 i = len; i != 0;) {
            unchecked {
                --i;
            }
            Lease storage lease = leasesByReceiver[receiverSalt][i];
            // solhint-disable-next-line gas-strict-inequalities
            if (lease.startTime <= ts) {
                leaseId = ids[i];
                break;
            }
        }
    }

    /// @notice Compute the net payout for `amountQ` raw volume under lease `l` after applying percentage and flat fees.
    /// @param lease Lease whose fees apply.
    /// @param amountQ Raw recognized amount.
    /// @return netOut Net amount after fees (floored at 0).
    function _computeNetOut(Lease storage lease, uint256 amountQ) internal view returns (uint256 netOut) {
        uint256 feePpm = lease.leaseFeePpm;
        // Percentage-based payout after ppm fee.
        uint256 percentageOut = amountQ * (_PPM_DENOMINATOR - feePpm) / _PPM_DENOMINATOR;
        uint256 flat = lease.flatFee;
        if (percentageOut > flat) {
            unchecked {
                netOut = percentageOut - flat;
            }
        } else {
            netOut = 0;
        }
    }

    /// @notice Plan filling a batch of claims.
    /// @param targetToken Token to fill.
    /// @param queue Queue of claims.
    /// @param head Head of the queue.
    /// @param maxClaims Maximum number of claims to fill.
    /// @param ratePpm Rate per million.
    /// @return end Index of the last claim filled.
    /// @return totalUsdt Total USDT filled.
    /// @return expectedOutTotal Total expected output.
    function _planFillBatch(
        address targetToken,
        Claim[] storage queue,
        uint256 head,
        uint256 maxClaims,
        uint256 ratePpm
    ) internal view returns (uint256 end, uint256 totalUsdt, uint256 expectedOutTotal) {
        uint256 availableUsdt = usdtBalance();
        end = head;

        // solhint-disable-next-line gas-strict-inequalities
        while (end < queue.length && end - head < maxClaims) {
            uint256 amountUsdt = queue[end].amountUsdt;
            if (availableUsdt < amountUsdt) break;

            // Ensure any required bridge route exists before we swap for this batch.
            totalUsdt += amountUsdt;
            if (targetToken != usdt) {
                uint256 targetChainId = queue[end].targetChainId;
                if (targetChainId != block.chainid) {
                    if (address(bridgers[targetToken][targetChainId]) == address(0)) revert NoBridger();
                }
                expectedOutTotal += TokenUtils.mulDiv(amountUsdt, ratePpm, _RATE_DENOMINATOR);
            }

            unchecked {
                availableUsdt -= amountUsdt;
                ++end;
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                          INTERNAL PURE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Decode an `isEventChainTip` call (plain or multicall-wrapped) and return the tip.
    /// @param data Tron calldata bytes from the proved transaction.
    /// @return tipNew The decoded event-chain tip.
    function _decodeEventChainTip(bytes memory data) internal pure returns (bytes32 tipNew) {
        // Extract the 4-byte selector from the Tron call data.
        if (data.length < 4) revert TronInvalidCalldataLength();
        // Casting to `bytes4` takes the first 4 bytes (the function selector).
        // This matches Solidity's `bytes4` left-aligned representation, avoiding selector alignment bugs.
        // forge-lint: disable-next-line(unsafe-typecast)
        bytes4 sel = bytes4(data);

        if (sel == _SELECTOR_IS_EVENT_CHAIN_TIP) {
            return TronCalldataUtils.decodeIsEventChainTip(data);
        }
        if (sel == _SELECTOR_MULTICALL) {
            return TronCalldataUtils.decodeMulticallEventChainTip(data, _SELECTOR_IS_EVENT_CHAIN_TIP);
        }
        revert NotEventChainTip();
    }

    /// @notice Convert a `uint256` into `int256` with bounds checking.
    /// @param x Unsigned value to cast.
    /// @return Signed integer representation of `x`.
    function _toInt(uint256 x) internal pure returns (int256) {
        if (x > uint256(type(int256).max)) revert AmountTooLargeForInt();
        // casting to 'int256' is safe because we check if x is greater than max int256 value
        // and revert if so in the line above
        // forge-lint: disable-next-line(unsafe-typecast)
        return int256(x);
    }

    /// @notice EIP-712 domain name and version for signature-based payout config updates.
    /// @return name EIP-712 domain name.
    /// @return version EIP-712 domain version string.
    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
        // decided not to do UntronV3 just to appear cleaner in users' wallets.
        // Signature request: "Untron". Sexy.
        // (Untron V1 and Untron V2 didn't use EIP-712 signatures,
        // and Untron Intents is probably gonna use a different name/version, so this is safe.)
        name = "Untron";
        version = "1";
    }
}
