// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronV3IndexedOwnable} from "./UntronV3IndexedOwnable.sol";
import {TronTxReader} from "./TronTxReader.sol";
import {SwapExecutor, Call} from "./SwapExecutor.sol";
import {IBridger} from "./bridgers/interfaces/IBridger.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";
import {EventChainGenesis} from "../utils/EventChainGenesis.sol";
import {TronCalldataUtils} from "../utils/TronCalldataUtils.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";

import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";

import {EIP712} from "solady/utils/EIP712.sol";
import {SignatureCheckerLib} from "solady/utils/SignatureCheckerLib.sol";
import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";

/* solhint-disable max-states-count */

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
contract UntronV3 is Create2Utils, EIP712, ReentrancyGuard, Pausable, UntronV3IndexedOwnable {
    using SignatureCheckerLib for address;

    /*//////////////////////////////////////////////////////////////
                                  TYPES
    //////////////////////////////////////////////////////////////*/

    /// @notice How the lease creation rate limit is chosen for a given realtor.
    enum LeaseRateLimitMode {
        /// @dev Use the protocol-wide config stored in `_protocolConfig`.
        Inherit,
        /// @dev Use the realtor-specific override stored in `_realtorConfig[realtor]`.
        Override,
        /// @dev Disable rate limiting for this realtor.
        Disabled
    }

    /// @notice Protocol-wide configuration set by the owner.
    /// @dev Stored as `uint32` to reduce storage and because values are naturally bounded.
    struct ProtocolConfig {
        /// @notice Protocol-wide minimum fee, in parts-per-million (ppm) of recognized raw volume.
        /// @dev Applied as a floor for every lease; realtor-specific `minFeePpm` can raise it.
        uint32 floorPpm;
        /// @notice Max number of lease creations allowed per window for all realtors (unless overridden).
        uint32 leaseRateLimitMaxLeases;
        /// @notice Sliding window length (seconds) for protocol-wide lease creation rate limiting.
        uint32 leaseRateLimitWindowSeconds;
        /// @notice Max number of payout-config updates allowed per window per lessee.
        uint32 payoutConfigRateLimitMaxUpdates;
        /// @notice Sliding window length (seconds) for payout-config update rate limiting.
        uint32 payoutConfigRateLimitWindowSeconds;
    }

    /// @notice Realtor-specific configuration set by the owner.
    struct RealtorConfig {
        /// @notice Realtor-specific minimum fee floor (ppm). Effective minimum is `max(protocolFloorPpm, minFeePpm)`.
        uint32 minFeePpm;
        /// @notice Realtor-specific lease creation max leases (used only when mode == Override).
        uint32 leaseRateLimitMaxLeases;
        /// @notice Realtor-specific lease creation window seconds (used only when mode == Override).
        uint32 leaseRateLimitWindowSeconds;
        /// @notice How to interpret the rate limit fields for this realtor.
        LeaseRateLimitMode leaseRateLimitMode;
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

    /// @notice FIFO claim queue element.
    /// @dev Claim amounts are denominated in USDT; settlement may pay USDT or `targetToken` depending on route.
    struct Claim {
        /// @notice USDT-denominated claim amount.
        /// @dev Slots are deleted when filled.
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

    /* solhint-enable gas-small-strings */

    /// @notice The address of the UntronController contract on Tron (source chain), in EVM format.
    /// @dev Stored as a 20-byte EVM address; converted to Tron 21-byte format when comparing Tron calldata.
    address public immutable CONTROLLER_ADDRESS;

    /// @notice External Tron reader used to verify and decode Tron transactions.
    /// @dev The reader is expected to be bound to a Tron light client and to:
    /// - verify tx inclusion via Merkle proofs,
    /// - enforce tx success,
    /// - and expose sender / to / calldata for `TriggerSmartContract` transactions.
    TronTxReader public tronReader;

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
    SwapExecutor public immutable SWAP_EXECUTOR;

    /// @notice Next lease identifier.
    uint256 public nextLeaseId = 0;

    /// @notice Mapping from lease id to lease data.
    /// @dev `leaseId` is assigned monotonically starting from 1.
    mapping(uint256 => Lease) public leases;

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

    /// @notice Guard against double-processing of recognizable Tron deposits.
    /// @dev Keyed by the Tron transaction ID as computed by `TronTxReader` (sha256 of `raw_data` bytes).
    mapping(bytes32 => bool) public depositProcessed;

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
    /// @notice Thrown when a proposed lease fee is outside the allowed bounds or below configured floors.
    error LeaseFeeTooLow();
    /// @notice Thrown when a proposed lease timeframe is invalid (e.g. nukeableAfter in the past).
    error InvalidLeaseTimeframe();
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
    /// @notice Thrown when a proved Tron transfer is not sent to the predicted receiver for a given salt.
    error InvalidReceiverForSalt();
    /// @notice Thrown when an LP attempts to withdraw more than their accounted principal.
    error WithdrawExceedsPrincipal();
    /// @notice Thrown when the contract does not have enough USDT balance for a withdrawal or fill.
    error InsufficientUsdtBalance();
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
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Deploy the UntronV3 hub.
    /// @dev Side effects:
    /// - Deploys a dedicated `SwapExecutor` instance and stores it in `SWAP_EXECUTOR`.
    /// - Sets immutable `CONTROLLER_ADDRESS` (Tron-side UntronController address in EVM 20-byte form).
    /// - Sets initial `tronReader` (can be updated later by the owner).
    /// - Initializes ownership (events emitted via `UntronV3Index`).
    /// @param controllerAddress Address of the UntronController on Tron (source chain), in EVM 20‑byte form.
    /// @param create2Prefix Chain-specific byte prefix used for CREATE2 address computation (0x41 for Tron).
    /// @param tronReader_ Address of the initial external Tron tx reader contract (can be updated by owner).
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_) Create2Utils(create2Prefix) {
        // Deploy an isolated executor for swaps. Only this UntronV3 instance can call `execute(...)`.
        // NOTE: This is intentionally deployed via `new` (not CREATE2), so its address is derived from nonce.
        SWAP_EXECUTOR = new SwapExecutor(); // its address is gonna be keccak256(rlp([address(this), 1]))

        // Set the Tron controller address used for deterministic receiver derivation and event-chain relay checks.
        CONTROLLER_ADDRESS = controllerAddress;

        // Set initial Tron reader (bound to a Tron light client) used to verify and decode Tron transactions.
        tronReader = TronTxReader(tronReader_);

        // Initialize owner and emit OwnershipTransferred via UntronV3Index.
        _initializeOwner(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                               ADMIN CONFIG
    //////////////////////////////////////////////////////////////*/

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

        // Emit via UntronV3Index (and append to event chain).
        _emitRealtorMinFeeSet(realtor, minFeePpm);
    }

    /// @notice Configure the protocol-wide lease creation rate limit for all realtors.
    /// @dev The effective rate limit for a given realtor is computed by `_effectiveLeaseRateLimit` which can:
    /// - inherit from this config,
    /// - override per realtor,
    /// - or be disabled for a realtor.
    ///
    /// Setting both values to 0 disables the protocol-wide rate limit (but realtor overrides can still apply).
    /// @param maxLeases Maximum number of leases allowed per realtor per window.
    /// @param windowSeconds Window size in seconds.
    function setProtocolLeaseRateLimit(uint256 maxLeases, uint256 windowSeconds) external onlyOwner {
        // Ensure values fit into storage types.
        if (maxLeases > type(uint32).max || windowSeconds > type(uint32).max) revert LeaseRateLimitConfigInvalid();
        // Either both are 0 (disabled) or both are non-zero (enabled).
        if ((maxLeases == 0) != (windowSeconds == 0)) revert LeaseRateLimitConfigInvalid();
        // Casting to 'uint32' is safe because we capped to type(uint32).max above.
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.leaseRateLimitMaxLeases = uint32(maxLeases);
        // Casting to 'uint32' is safe because we capped to type(uint32).max above.
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.leaseRateLimitWindowSeconds = uint32(windowSeconds);

        // Emit via UntronV3Index (and append to event chain).
        _emitProtocolLeaseRateLimitSet(maxLeases, windowSeconds);
    }

    /// @notice Configure the protocol-wide lessee payout-config update rate limit.
    /// @dev Rate limiting applies to both `setPayoutConfig` and `setPayoutConfigWithSig`.
    /// Setting both values to 0 disables the rate limit.
    /// @param maxUpdates Maximum number of payout config updates per lessee per window.
    /// @param windowSeconds Window size in seconds.
    function setLesseePayoutConfigRateLimit(uint256 maxUpdates, uint256 windowSeconds) external onlyOwner {
        // Ensure values fit into storage types.
        if (maxUpdates > type(uint32).max || windowSeconds > type(uint32).max) {
            revert PayoutConfigRateLimitConfigInvalid();
        }
        // Either both are 0 (disabled) or both are non-zero (enabled).
        if ((maxUpdates == 0) != (windowSeconds == 0)) revert PayoutConfigRateLimitConfigInvalid();
        // Casting to 'uint32' is safe because we capped to type(uint32).max above.
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitMaxUpdates = uint32(maxUpdates);
        // Casting to 'uint32' is safe because we capped to type(uint32).max above.
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitWindowSeconds = uint32(windowSeconds);

        // Emit via UntronV3Index (and append to event chain).
        _emitLesseePayoutConfigRateLimitSet(maxUpdates, windowSeconds);
    }

    /// @notice Configure lease creation rate limiting behavior for a specific realtor.
    /// @dev Modes:
    /// - `Inherit`: use protocol-wide parameters from `_protocolConfig`.
    /// - `Override`: use the provided `maxLeases` and `windowSeconds` for this realtor (must be non-zero).
    /// - `Disabled`: skip rate limiting for this realtor entirely (provided values must be 0).
    /// @param realtor Realtor whose lease rate limit settings are being updated.
    /// @param mode Rate limiting mode.
    /// @param maxLeases Maximum number of lease creations allowed per window (only if mode == Override).
    /// @param windowSeconds Window size in seconds (only if mode == Override).
    function setRealtorLeaseRateLimit(
        address realtor,
        LeaseRateLimitMode mode,
        uint256 maxLeases,
        uint256 windowSeconds
    ) external onlyOwner {
        RealtorConfig storage cfg = _realtorConfig[realtor];

        // Store the mode regardless of branch.
        cfg.leaseRateLimitMode = mode;

        if (mode == LeaseRateLimitMode.Override) {
            // Override mode requires explicit, non-zero parameters that fit into uint32.
            if (maxLeases > type(uint32).max || windowSeconds > type(uint32).max) revert LeaseRateLimitConfigInvalid();
            if (maxLeases == 0 || windowSeconds == 0) revert LeaseRateLimitConfigInvalid();
            // Casting to 'uint32' is safe because we capped to type(uint32).max above.
            // forge-lint: disable-next-line(unsafe-typecast)
            cfg.leaseRateLimitMaxLeases = uint32(maxLeases);
            // Casting to 'uint32' is safe because we capped to type(uint32).max above.
            // forge-lint: disable-next-line(unsafe-typecast)
            cfg.leaseRateLimitWindowSeconds = uint32(windowSeconds);
        } else {
            // Inherit/Disabled modes require zero parameters to avoid ambiguous configuration.
            if (maxLeases != 0 || windowSeconds != 0) revert LeaseRateLimitConfigInvalid();
            cfg.leaseRateLimitMaxLeases = 0;
            cfg.leaseRateLimitWindowSeconds = 0;
        }

        // Emit via UntronV3Index (and append to event chain).
        _emitRealtorLeaseRateLimitSet(realtor, uint8(mode), maxLeases, windowSeconds);
    }

    /// @notice Set or update the external Tron tx reader contract address.
    /// @dev This contract is trusted to verify and decode Tron transactions correctly.
    /// @param reader Address of the new `TronTxReader`.
    function setTronReader(address reader) external onlyOwner {
        // Update reader.
        tronReader = TronTxReader(reader);

        // Emit via UntronV3Index (and append to event chain).
        _emitTronReaderSet(reader);
    }

    /// @notice Set the expected swap/bridge rate for a target token.
    /// @dev This rate is used to compute `expectedOut` for claims:
    /// `expectedOut = amountUsdt * swapRatePpm[targetToken] / _RATE_DENOMINATOR`.
    ///
    /// The rate is independent of the destination chain. Cross-chain routes additionally require a bridger.
    /// @param targetToken Token that claims will be settled in (locally or bridged).
    /// @param ratePpm Expected output rate in ppm of USDT (`targetToken` units per `_RATE_DENOMINATOR` USDT).
    function setSwapRate(address targetToken, uint256 ratePpm) external onlyOwner {
        // Basic input validation.
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (ratePpm == 0) revert RateNotSet();

        // Store the rate used by fill-time swapping.
        swapRatePpm[targetToken] = ratePpm;

        // Emit via UntronV3Index (and append to event chain).
        _emitSwapRateSet(targetToken, ratePpm);
    }

    /// @notice Set the bridger implementation for a `(targetToken, targetChainId)` pair.
    /// @dev Required for cross-chain payouts (`targetChainId != block.chainid`). When settling a bridged claim, UntronV3 will:
    /// 1) `transfer(targetToken, bridger, amount)`, then
    /// 2) call `IBridger(bridger).bridge(targetToken, amount, targetChainId, beneficiary)`.
    /// @param targetToken Token that will be bridged.
    /// @param targetChainId Destination chain id.
    /// @param bridger Bridger contract address implementing `IBridger`.
    function setBridger(address targetToken, uint256 targetChainId, address bridger) external onlyOwner {
        // Basic input validation.
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (bridger == address(0)) revert NoBridger();

        // Store bridger used by fill-time bridging.
        bridgers[targetToken][targetChainId] = IBridger(bridger);

        // Emit via UntronV3Index (and append to event chain).
        _emitBridgerSet(targetToken, targetChainId, bridger);
    }

    /// @notice Pause the protocol.
    /// @dev When paused, all `whenNotPaused` entrypoints revert (including lease creation, preEntitle, fill, etc.).
    function pause() external onlyOwner {
        _pause();
    }

    /// @notice Unpause the protocol.
    function unpause() external onlyOwner {
        _unpause();
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

        // Book the negative PnL delta and emit.
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

    /*//////////////////////////////////////////////////////////////
                                 LEASES
    //////////////////////////////////////////////////////////////*/

    /// @notice Create a new lease for a given receiver salt.
    /// @dev Requirements:
    /// - Caller must be a whitelisted realtor (`isRealtor[msg.sender] == true`).
    /// - Realtor must satisfy the effective lease creation rate limit.
    /// - `leaseFeePpm` must be within `[minFee, 1_000_000]`, where `minFee = max(protocol floor, realtor min)`.
    /// - `targetChainId` must not be deprecated.
    /// - `nukeableAfter` must be >= current timestamp.
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
    /// @return leaseId Newly created lease identifier.
    function createLease(
        bytes32 receiverSalt,
        address lessee,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee,
        uint256 targetChainId,
        address targetToken,
        address beneficiary
    ) external whenNotPaused returns (uint256 leaseId) {
        address realtor = msg.sender;
        _enforceCreateLeasePreconditions(realtor, receiverSalt, nukeableAfter, leaseFeePpm, targetChainId);

        // Validate that the payout route is currently supported/configured.
        // This makes lease creation fail fast if rate/bridger isn't configured yet.
        _enforcePayoutConfigRoutable(targetChainId, targetToken);

        // Allocate the new lease id.
        leaseId = ++nextLeaseId;
        uint64 startTime = _leaseStartTime();

        // Populate lease storage and append to the receiver's lease timeline.
        _storeLease(
            leaseId,
            receiverSalt,
            realtor,
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
        _emitLeaseCreated(leaseId, receiverSalt, realtor, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee);
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
        Lease storage lease = leases[leaseId];
        if (lease.lessee == address(0)) revert InvalidLeaseId();
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
        Lease storage lease = leases[leaseId];
        if (lease.lessee == address(0)) revert InvalidLeaseId();

        // Apply per-lessee rate limiting (based on the signer/lessee, not the relayer).
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

        // Emit via UntronV3Index (and append to event chain).
        _emitPayoutConfigUpdated(leaseId, config.targetChainId, config.targetToken, config.beneficiary);
    }

    /*//////////////////////////////////////////////////////////////
                        PRE-ENTITLEMENT FROM TRON
    //////////////////////////////////////////////////////////////*/

    /// @notice Prove and pre-entitle a recognizable TRC-20 deposit on Tron to a lease.
    /// @dev This wires together:
    /// - `TronTxReader` (verifies inclusion + decodes a `TriggerSmartContract` tx),
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
    ///
    /// Accounting:
    /// - Increments `lease.recognizedRaw` and `lease.unbackedRaw` by the raw TRC-20 amount.
    /// - Computes net payout (`netOut`) using the lease fee schedule and books protocol fees into `protocolPnl`.
    /// - Enqueues a claim for `netOut` (if `netOut > 0`) under the lease's current `payout.targetToken`.
    ///
    /// @param receiverSalt CREATE2 salt used to derive the receiver address on Tron.
    /// @param tronBlockNumber Tron block number where the tx is included (for light client verification).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @return claimIndex Index in `claimsByTargetToken[payout.targetToken]` where the claim was appended (0 if none).
    /// @return leaseId Lease id that the deposit was attributed to.
    /// @return netOut USDT-denominated net payout after fees (0 if fees exceed the amount).
    function preEntitle(
        bytes32 receiverSalt,
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external whenNotPaused returns (uint256 claimIndex, uint256 leaseId, uint256 netOut) {
        // Verify inclusion + success and decode into a generic TriggerSmartContract view.
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        // Prevent double-processing of the same recognizable tx.
        if (depositProcessed[callData.txId]) revert DepositAlreadyProcessed();
        depositProcessed[callData.txId] = true;

        // Enforce that the TRC-20 contract called is exactly Tron USDT.
        address tronUsdt_ = tronUsdt;
        if (callData.toTron != TronCalldataUtils.evmToTronAddress(tronUsdt_)) revert NotTronUsdt();

        // Sanity-check that the TRC-20 transfer goes into the expected receiver.
        address predictedReceiver = predictReceiverAddress(CONTROLLER_ADDRESS, receiverSalt);
        bytes21 expectedToTron = TronCalldataUtils.evmToTronAddress(predictedReceiver);
        (, bytes21 toTron, uint256 amountQ) =
            TronCalldataUtils.decodeTrc20FromCalldata(callData.data, callData.senderTron);
        if (toTron != expectedToTron) revert InvalidReceiverForSalt();

        // Token is no longer part of lease uniqueness; use receiver salt only.
        // Attribute to the lease that was active at the Tron tx timestamp.
        leaseId = _findActiveLeaseId(receiverSalt, callData.tronBlockTimestamp);
        if (leaseId == 0) revert NoActiveLease();
        Lease storage lease = leases[leaseId];

        // Recognize raw volume and mark it as unbacked until the controller reports receiver pulls.
        lease.recognizedRaw += amountQ;
        lease.unbackedRaw += amountQ;

        // Compute net payout after lease fee schedule.
        netOut = _computeNetOut(lease, amountQ);

        // Book protocol fee revenue immediately as PnL (raw - netOut).
        _bookFee(amountQ, netOut);

        // Enqueue a claim for later settlement if there is a positive net payout.
        if (netOut > 0) {
            PayoutConfig storage p = lease.payout;
            claimIndex = _enqueueClaimForTargetToken(p.targetToken, netOut, leaseId, p.targetChainId, p.beneficiary);
            _emitClaimCreated(claimIndex, leaseId, netOut);
        }

        // Emit pre-entitlement event for offchain reconciliation.
        _emitDepositPreEntitled(callData.txId, leaseId, amountQ, netOut);
    }

    /*//////////////////////////////////////////////////////////////
                         CONTROLLER EVENT RELAY
    //////////////////////////////////////////////////////////////*/

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
    /// @param tronBlockNumber Tron block number where the `isEventChainTip` tx is included (for light client verification).
    /// @param encodedTx Raw protobuf-encoded Tron transaction bytes.
    /// @param proof Merkle proof for tx inclusion in the Tron block's tx trie.
    /// @param index Merkle leaf index for the tx within the block.
    /// @param events Controller events that should extend the local tip to `tipNew`.
    /// @return tipNew The new controller event-chain tip proven by the Tron transaction.
    function relayControllerEventChain(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index,
        ControllerEvent[] calldata events
    ) external whenNotPaused returns (bytes32 tipNew) {
        // Verify inclusion + success and decode into a generic TriggerSmartContract view.
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        // Validate that the call is targeting the expected UntronController contract on Tron.
        bytes21 controllerTron = TronCalldataUtils.evmToTronAddress(CONTROLLER_ADDRESS);
        if (callData.toTron != controllerTron) revert NotEventChainTip();

        // Decode the new tip from either a direct `isEventChainTip` call or a multicall wrapper.
        tipNew = _decodeEventChainTip(callData.data);

        // Ensure progress (avoid accepting a no-op relay).
        bytes32 tip = lastControllerEventTip;
        if (tipNew == tip) revert EventRelayNoProgress();

        bytes32 computedTip = _hashLinkControllerEventsAndEmit(tip, events);
        if (computedTip != tipNew) revert EventTipMismatch();

        // Commit the new tip.
        lastControllerEventTip = tipNew;

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
                (bytes32 receiverSalt,/*token*/,/*tokenAmount*/,/* exchangeRate */, uint256 usdtAmount) =
                    abi.decode(ev.data, (bytes32, address, uint256, uint256, uint256));
                _processReceiverPulled(receiverSalt, usdtAmount, ev.blockTimestamp);
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
            }

            unchecked {
                // Cursor and processed counters only increase; unchecked saves gas.
                ++idx;
                ++processed;
            }
        }

        // Commit updated cursor.
        nextControllerEventIndex = idx;
    }

    /*//////////////////////////////////////////////////////////////
                              LP VAULT
    //////////////////////////////////////////////////////////////*/

    /// @notice Deposit USDT into the fast-fill vault.
    /// @dev fast-fill vaults are 0% APY by design. All incentivized LPing logic
    ///      must be handled by external contracts.
    /// Funds deposited here increase `usdtBalance()` and can be used to fill claims.
    /// @param amount Amount of `usdt` to transfer from the caller into the contract.
    function deposit(uint256 amount) external whenNotPaused {
        // Disallow no-op deposits.
        if (amount == 0) revert ZeroAmount();

        // Pull USDT from the LP and increase their principal balance.
        TokenUtils.transferFrom(usdt, msg.sender, payable(address(this)), amount);
        lpPrincipal[msg.sender] += amount;

        // Emit via UntronV3Index (and append to event chain).
        _emitLpDeposited(msg.sender, amount);
    }

    /// @notice Withdraw USDT from the fast-fill vault.
    /// @dev Requirements:
    /// - `amount` must be > 0.
    /// - `amount` must be <= caller's `lpPrincipal`.
    /// - Contract must currently hold at least `amount` USDT.
    /// @param amount Amount of `usdt` to withdraw.
    function withdraw(uint256 amount) external whenNotPaused {
        // Disallow no-op withdrawals.
        if (amount == 0) revert ZeroAmount();

        // Enforce principal accounting: LPs can only withdraw what they put in.
        uint256 principal = lpPrincipal[msg.sender];
        if (amount > principal) revert WithdrawExceedsPrincipal();
        // Also enforce that the contract has enough liquid USDT to satisfy the withdrawal.
        if (amount > usdtBalance()) revert InsufficientUsdtBalance();

        // Update principal before transferring (best practice for external calls).
        lpPrincipal[msg.sender] = principal - amount;
        TokenUtils.transfer(usdt, payable(msg.sender), amount);

        // Emit via UntronV3Index (and append to event chain).
        _emitLpWithdrawn(msg.sender, amount);
    }

    /*//////////////////////////////////////////////////////////////
                             CLAIM QUEUE
    //////////////////////////////////////////////////////////////*/

    /// @notice Fill up to `maxClaims` claims for a target token, swapping once if needed then settling sequentially.
    /// @dev `calls` may be empty if no swap is needed (e.g. when `targetToken == usdt`).
    ///      Any swap output above `expectedOutTotal` is paid to the relayer (`msg.sender`).
    ///      This function is non-reentant because it calls executor that performs arbitrary onchain calls.
    ///
    /// Fill mechanics:
    /// - Claims are stored in a per-`targetToken` FIFO queue `claimsByTargetToken[targetToken]`.
    /// - For `targetToken != usdt`, this function:
    ///   - scans forward from the head to determine how many claims fit under current `usdtBalance()`,
    ///   - swaps USDT -> `targetToken` once for the batch,
    ///   - then fills those claims sequentially, either locally or via a configured bridger.
    ///
    /// @param targetToken The queue key: claims to be filled are `claimsByTargetToken[targetToken]`.
    /// @param maxClaims Maximum number of non-empty claims to fill in this call.
    /// @param calls Arbitrary swap calls executed by `SwapExecutor` if the plan requires swapping.
    function fill(address targetToken, uint256 maxClaims, Call[] calldata calls) external nonReentrant whenNotPaused {
        // Basic input validation.
        if (targetToken == address(0)) revert InvalidTargetToken();
        // Explicitly allow early return for maxClaims == 0 to save gas.
        if (maxClaims == 0) return;

        Claim[] storage queue = claimsByTargetToken[targetToken];

        uint256 head = nextIndexByTargetToken[targetToken];

        uint256 ratePpm;
        if (targetToken != usdt) {
            ratePpm = swapRatePpm[targetToken];
            if (ratePpm == 0) revert RateNotSet();
        }

        (uint256 end, uint256 totalUsdt, uint256 expectedOutTotal) =
            _planFillBatch(targetToken, queue, head, maxClaims, ratePpm);

        uint256 surplusOut;
        if (targetToken != usdt) {
            surplusOut = _swapForBatch(targetToken, totalUsdt, expectedOutTotal, calls);
        }

        _settleClaimRange({targetToken: targetToken, ratePpm: ratePpm, queue: queue, start: head, end: end});

        nextIndexByTargetToken[targetToken] = end;

        // Transfer swap surplus output token to the filler.
        if (surplusOut != 0) {
            TokenUtils.transfer(targetToken, payable(msg.sender), surplusOut);
        }
    }

    /*//////////////////////////////////////////////////////////////
                             EXTERNAL VIEW
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns the protocol-wide lease rate limit config.
    /// @dev If either returned value is 0, the protocol-wide limit is effectively disabled.
    /// @return maxLeases Max number of leases allowed per window.
    /// @return windowSeconds Window size in seconds.
    function protocolLeaseRateLimit() external view returns (uint256 maxLeases, uint256 windowSeconds) {
        ProtocolConfig storage cfg = _protocolConfig;
        return (cfg.leaseRateLimitMaxLeases, cfg.leaseRateLimitWindowSeconds);
    }

    /// @notice Returns the protocol-wide payout config update rate limit for lessees.
    /// @dev If either returned value is 0, the payout-config update rate limit is disabled.
    /// @return maxUpdates Max number of payout config updates allowed per window.
    /// @return windowSeconds Window size in seconds.
    function lesseePayoutConfigRateLimit() external view returns (uint256 maxUpdates, uint256 windowSeconds) {
        ProtocolConfig storage cfg = _protocolConfig;
        return (cfg.payoutConfigRateLimitMaxUpdates, cfg.payoutConfigRateLimitWindowSeconds);
    }

    /// @notice Returns the raw realtor lease rate limit config (mode + values).
    /// @dev This returns the raw stored realtor config. Use `effectiveLeaseRateLimit` to get the applied config.
    /// @param realtor Realtor to query.
    /// @return mode The realtor's configured mode.
    /// @return maxLeases The stored maxLeases (meaningful only if mode == Override).
    /// @return windowSeconds The stored windowSeconds (meaningful only if mode == Override).
    function realtorLeaseRateLimit(address realtor)
        external
        view
        returns (LeaseRateLimitMode mode, uint256 maxLeases, uint256 windowSeconds)
    {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        return (cfg.leaseRateLimitMode, cfg.leaseRateLimitMaxLeases, cfg.leaseRateLimitWindowSeconds);
    }

    /// @notice Returns the effective lease rate limit config for a realtor after applying overrides.
    /// @param realtor Realtor to query.
    /// @return enabled Whether the effective limit is enabled.
    /// @return maxLeases Effective maxLeases.
    /// @return windowSeconds Effective windowSeconds.
    function effectiveLeaseRateLimit(address realtor)
        external
        view
        returns (bool enabled, uint256 maxLeases, uint256 windowSeconds)
    {
        (enabled, maxLeases, windowSeconds) = _effectiveLeaseRateLimit(realtor);
    }

    /*//////////////////////////////////////////////////////////////
                              PUBLIC VIEW
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns the protocol-wide minimum fee in parts per million.
    /// @dev Preserves the legacy public getter name.
    /// @return floorPpm Protocol-wide fee floor in ppm.
    function protocolFloorPpm() public view returns (uint256) {
        return uint256(_protocolConfig.floorPpm);
    }

    /// @notice Returns the realtor-specific minimum fee override in parts per million.
    /// @dev Preserves the legacy public getter name.
    /// @param realtor Realtor to query.
    /// @return minFeePpm Realtor-specific fee floor in ppm.
    function realtorMinFeePpm(address realtor) public view returns (uint256) {
        return uint256(_realtorConfig[realtor].minFeePpm);
    }

    /// @notice Returns the current USDT balance held by this contract.
    /// @return The USDT balance held by this contract.
    /// @dev Returns 0 if `usdt` is not set.
    function usdtBalance() public view returns (uint256) {
        address usdt_ = usdt; // not sure if the compiler would optimize it into this anyway
        if (usdt_ == address(0)) return 0;
        return TokenUtils.getBalanceOf(usdt_, address(this));
    }

    /*//////////////////////////////////////////////////////////////
                           INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Enforces the effective lease creation rate limit for `realtor`, if enabled.
    /// @dev Uses an append-only timestamp array per realtor and checks the timestamp at index `len - maxLeases`
    ///      to determine whether the oldest of the last `maxLeases` creations is outside the window.
    /// @param realtor Realtor whose lease creation is being rate-limited.
    function _enforceLeaseRateLimit(address realtor) internal {
        // Resolve effective config after applying mode/overrides.
        (bool enabled, uint256 maxLeases, uint256 windowSeconds) = _effectiveLeaseRateLimit(realtor);
        if (!enabled) return;

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
    /// @param targetChainId Destination chain id to validate against deprecations.
    function _enforceCreateLeasePreconditions(
        address realtor,
        bytes32 receiverSalt,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint256 targetChainId
    ) internal {
        // Realtors are the only actors allowed to create leases.
        if (!isRealtor[realtor]) revert NotRealtor();

        // Apply effective (protocol-wide or realtor override) rate limiting.
        _enforceLeaseRateLimit(realtor);

        // Enforce fee bounds and floors.
        uint256 minFee = _minLeaseFeePpm(realtor);
        if (leaseFeePpm < minFee || leaseFeePpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();

        // Disallow creating leases that immediately target a deprecated chain.
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

        // Prevent leases that are already nukeable (nukeableAfter must be in the future/present).
        if (nukeableAfter < _leaseStartTime()) revert InvalidLeaseTimeframe();

        // Uniqueness is enforced per receiver salt regardless of token.
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        if (ids.length != 0) {
            Lease storage last = leases[ids[ids.length - 1]];
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
    ) internal {
        Lease storage lease = leases[leaseId];
        lease.receiverSalt = receiverSalt;
        lease.realtor = realtor;
        lease.lessee = lessee;
        lease.startTime = startTime;
        lease.nukeableAfter = nukeableAfter;
        lease.leaseFeePpm = leaseFeePpm;
        lease.flatFee = flatFee;

        // Store payout configuration so that target chain configuration is
        // available for owner-recommended bridging or direct payouts.
        lease.payout = PayoutConfig({targetChainId: targetChainId, targetToken: targetToken, beneficiary: beneficiary});

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
    /// @param events Controller events to hash-link.
    /// @return New tip after hashing all events.
    function _hashLinkControllerEventsAndEmit(bytes32 tip, ControllerEvent[] calldata events)
        internal
        returns (bytes32)
    {
        uint256 n = events.length;
        for (uint256 i = 0; i < n; ++i) {
            ControllerEvent calldata ev = events[i];
            _emitControllerEventChainTipUpdated(tip, ev.blockNumber, ev.blockTimestamp, ev.sig, ev.data);
            tip = sha256(abi.encodePacked(tip, ev.blockNumber, ev.blockTimestamp, ev.sig, ev.data));
        }
        return tip;
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
    /// @return claimIndex Index of the appended claim within the queue.
    function _enqueueClaimForTargetToken(
        address targetToken,
        uint256 amountUsdt,
        uint256 leaseId,
        uint256 targetChainId,
        address beneficiary
    ) internal returns (uint256 claimIndex) {
        // Append claim to the per-token queue.
        Claim[] storage queue = claimsByTargetToken[targetToken];
        queue.push(
            Claim({amountUsdt: amountUsdt, leaseId: leaseId, targetChainId: targetChainId, beneficiary: beneficiary})
        );
        // Claim index is the array index of the appended element.
        return queue.length - 1;
    }

    /// @notice The internal function for processing PulledFromReceiver controller events.
    /// @dev Handle a `PulledFromReceiver` controller event by reconciling unbacked volume and/or creating new claims.
    ///
    /// The controller reports a USDT amount pulled out of the receiver(s) for a given `receiverSalt`.
    /// We treat this as "backing" previously recognized but unbacked volume, oldest-first across the lease timeline.
    ///
    /// If the pulled amount exceeds total unbacked volume at/through `dumpTimestamp`, the remaining amount is treated
    /// as new recognized volume for the lease active at `dumpTimestamp` ("profit volume") and is subject to fees and
    /// claim creation like `preEntitle`. If there is no active lease at that time, the remainder is booked as protocol
    /// PnL with reason `RECEIVER_PULL_NO_LEASE`.
    ///
    /// @param receiverSalt Receiver salt whose lease timeline is affected.
    /// @param usdtAmount Total USDT amount reported as pulled from receivers by the controller.
    /// @param dumpTimestamp Tron timestamp at which the pull occurred (used to find active lease).
    function _processReceiverPulled(bytes32 receiverSalt, uint256 usdtAmount, uint64 dumpTimestamp) internal {
        if (usdtAmount == 0) {
            return;
        }

        // Remaining amount to allocate between backing repayment and (possibly) new profit volume.
        uint256 remaining = usdtAmount;

        // Repay historical unbacked volume across leases for receiverSalt.
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        uint256 len = ids.length;
        for (uint256 j = 0; j < len && remaining != 0; ++j) {
            Lease storage oldL = leases[ids[j]];
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

        // Any remaining volume becomes profit for the lease active at dump time.
        if (remaining != 0) {
            // Find lease active at the pull timestamp.
            uint256 currentLeaseId = _findActiveLeaseId(receiverSalt, dumpTimestamp);
            if (currentLeaseId == 0) {
                // If no lease is active, attribute remainder to protocol PnL.
                _applyPnlDelta(_toInt(remaining), PnlReason.RECEIVER_PULL_NO_LEASE);
                return;
            }

            Lease storage cur = leases[currentLeaseId];
            // Treat remainder as newly recognized & backed volume for the current lease.
            cur.recognizedRaw += remaining;
            cur.backedRaw += remaining;

            // Compute net payout and book fees.
            uint256 netOut = _computeNetOut(cur, remaining);
            _bookFee(remaining, netOut);
            if (netOut > 0) {
                // Enqueue claim using the current payout config.
                PayoutConfig storage p = cur.payout;
                uint256 claimIndex =
                    _enqueueClaimForTargetToken(p.targetToken, netOut, currentLeaseId, p.targetChainId, p.beneficiary);
                _emitClaimCreated(claimIndex, currentLeaseId, netOut);
            }
        }
    }

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
            Claim storage c = queue[idx];
            uint256 amountUsdt = c.amountUsdt;

            uint256 leaseId = c.leaseId;
            uint256 targetChainId = c.targetChainId;
            address beneficiary = c.beneficiary;
            bool needsBridge = targetChainId != block.chainid;

            uint256 outAmount =
                targetToken == usdt ? amountUsdt : TokenUtils.mulDiv(amountUsdt, ratePpm, _RATE_DENOMINATOR);

            IBridger bridger;
            if (needsBridge) {
                bridger = bridgers[targetToken][targetChainId];
                if (address(bridger) == address(0)) revert NoBridger();
            }

            // Delete before any external interaction.
            delete queue[idx];

            if (outAmount != 0) {
                if (needsBridge) {
                    TokenUtils.transfer(targetToken, payable(address(bridger)), outAmount);
                    bridger.bridge(targetToken, outAmount, targetChainId, beneficiary);
                } else {
                    TokenUtils.transfer(targetToken, payable(beneficiary), outAmount);
                }
            }

            _emitClaimFilled(idx, leaseId, amountUsdt);
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

    /// @notice Compute the effective lease creation rate limit for `realtor` after applying `LeaseRateLimitMode`.
    /// @param realtor Realtor to query.
    /// @return enabled Whether rate limiting is enabled.
    /// @return maxLeases Effective max leases allowed per window.
    /// @return windowSeconds Effective window size in seconds.
    function _effectiveLeaseRateLimit(address realtor)
        internal
        view
        returns (bool enabled, uint256 maxLeases, uint256 windowSeconds)
    {
        RealtorConfig storage rcfg = _realtorConfig[realtor];
        LeaseRateLimitMode mode = rcfg.leaseRateLimitMode;
        if (mode == LeaseRateLimitMode.Disabled) return (false, 0, 0);

        if (mode == LeaseRateLimitMode.Override) {
            // Realtor explicitly overrides protocol-wide config.
            maxLeases = rcfg.leaseRateLimitMaxLeases;
            windowSeconds = rcfg.leaseRateLimitWindowSeconds;
        } else {
            // Default behavior: inherit protocol-wide config.
            ProtocolConfig storage pcfg = _protocolConfig;
            maxLeases = pcfg.leaseRateLimitMaxLeases;
            windowSeconds = pcfg.leaseRateLimitWindowSeconds;
        }

        // Enabled iff both parameters are non-zero.
        enabled = (maxLeases != 0 && windowSeconds != 0);
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
            Lease storage lease = leases[ids[i]];
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
        bool needsSwap = targetToken != usdt;

        uint256 availableUsdt = usdtBalance();
        uint256 queueLen = queue.length;
        end = head;

        uint256 plannedClaims;
        while (end < queueLen && plannedClaims < maxClaims) {
            Claim storage cScan = queue[end];
            uint256 amountUsdt = cScan.amountUsdt;

            if (availableUsdt < amountUsdt) break;

            // Ensure any required bridge route exists before we swap for this batch.
            if (needsSwap && cScan.targetChainId != block.chainid) {
                if (address(bridgers[targetToken][cScan.targetChainId]) == address(0)) revert NoBridger();
            }

            totalUsdt += amountUsdt;
            if (needsSwap) {
                expectedOutTotal += TokenUtils.mulDiv(amountUsdt, ratePpm, _RATE_DENOMINATOR);
            }

            unchecked {
                availableUsdt -= amountUsdt;
                ++end;
                ++plannedClaims;
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
        bytes4 sel;
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            sel := shr(224, mload(add(data, 0x20)))
        }

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
