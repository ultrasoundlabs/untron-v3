// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronV3IndexedOwnable} from "./UntronV3Index.sol";
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

/// @title Hub contract for Untron V3 protocol.
/// @dev UntronV3 must not emit events directly; all event emissions must go through UntronV3Index.
/// @author Ultrasound Labs
contract UntronV3 is Create2Utils, EIP712, ReentrancyGuard, Pausable, UntronV3IndexedOwnable {
    using SignatureCheckerLib for address;

    /*//////////////////////////////////////////////////////////////
                                  TYPES
    //////////////////////////////////////////////////////////////*/

    enum RouteKind {
        LocalUsdt,
        LocalSwap,
        Bridge
    }

    enum LeaseRateLimitMode {
        Inherit,
        Override,
        Disabled
    }

    struct ProtocolConfig {
        uint32 floorPpm;
        uint32 leaseRateLimitMaxLeases;
        uint32 leaseRateLimitWindowSeconds;
        uint32 payoutConfigRateLimitMaxUpdates;
        uint32 payoutConfigRateLimitWindowSeconds;
    }

    struct RealtorConfig {
        uint32 minFeePpm;
        uint32 leaseRateLimitMaxLeases;
        uint32 leaseRateLimitWindowSeconds;
        LeaseRateLimitMode leaseRateLimitMode;
    }

    struct Route {
        RouteKind kind;
        uint256 ratePpm;
        address bridger;
    }

    struct FillPlan {
        uint256 newHead;
        uint256 processedSlots;
        uint256 totalUsdt;
        uint256 expectedOutTotal;
        RouteKind[] kinds;
        uint256[] ratesPpm;
        address[] bridgers;
        address[] beneficiaries;
        uint256[] targetChainIds;
    }

    /// @notice Per-lease payout configuration, mutable by the lessee.
    /// @dev Owner controls bridge pair availability; leases specify destination chain, target token, and beneficiary.
    struct PayoutConfig {
        uint256 targetChainId;
        // target token on THIS chain, not destination chain.
        address targetToken;
        address beneficiary;
    }

    /// @notice Lease scoped by receiver salt (not token).
    struct Lease {
        // Identity
        bytes32 receiverSalt;
        // Economics / parties
        address realtor;
        address lessee;
        // Timeline
        uint64 startTime;
        uint64 nukeableAfter;
        // Fees (parts per million)
        uint32 leaseFeePpm;
        uint64 flatFee;
        // Raw volume accounting
        uint256 recognizedRaw;
        uint256 backedRaw;
        uint256 unbackedRaw;
        // Live payout configuration
        PayoutConfig payout;
    }

    /// @notice FIFO claim queue element.
    struct Claim {
        // forge-lint: disable-next-line(mixed-case-variable)
        uint256 amountUSDT;
        uint256 leaseId;
        // target token is the key of the queue the claim is sitting in;
        // not including it here thus feels more concise (and storage-efficient!)
        uint256 targetChainId;
        address beneficiary;
    }

    /// @notice Raw controller event reconstructed from the Tron event chain.
    /// @dev Stores raw data; processing happens later based on `sig`.
    struct ControllerEvent {
        bytes32 sig;
        bytes data;
        uint64 blockNumber;
        uint64 blockTimestamp;
    }

    /*//////////////////////////////////////////////////////////////
                             STATE VARIABLES
    //////////////////////////////////////////////////////////////*/

    /// @dev EIP-712 typehash for gasless payout config updates.
    bytes32 internal constant _PAYOUT_CONFIG_UPDATE_TYPEHASH = keccak256(
        "PayoutConfigUpdate(uint256 leaseId,uint256 targetChainId,address targetToken,address beneficiary,uint256 nonce,uint256 deadline)"
    );

    // Parts-per-million denominator used for fee calculations (1_000_000 = 100%).
    uint256 internal constant _PPM_DENOMINATOR = 1_000_000;
    /// @notice Denominator for target token rate tables.
    uint256 internal constant _RATE_DENOMINATOR = 1_000_000;

    // UntronController _SELECTORs
    bytes4 internal constant _SELECTOR_IS_EVENT_CHAIN_TIP = bytes4(keccak256("isEventChainTip(bytes32)"));
    bytes4 internal constant _SELECTOR_MULTICALL = bytes4(keccak256("multicall(bytes[])"));

    // UntronController event signatures used in the event chain
    bytes32 internal constant _EVENT_SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
    bytes32 internal constant _EVENT_SIG_USDT_SET = keccak256("UsdtSet(address)");
    bytes32 internal constant _EVENT_SIG_USDT_REBALANCED = keccak256("UsdtRebalanced(uint256,uint256,address)");

    /// @notice The address of the UntronController contract on Tron (source chain), in EVM format.
    address public immutable CONTROLLER_ADDRESS;

    /// @notice External Tron reader used to verify and decode Tron transactions.
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
    SwapExecutor public immutable SWAP_EXECUTOR;

    /// @notice Next lease identifier (starts from 1 so 0 can mean "missing").
    uint256 public nextLeaseId = 1;

    /// @notice Mapping from lease id to lease data.
    mapping(uint256 => Lease) public leases;

    /// @notice Timeline of leases per receiver salt.
    mapping(bytes32 => uint256[]) internal _leaseIdsByReceiver;

    /// @notice Whitelisted realtors.
    mapping(address => bool) public isRealtor;

    /// @notice Protocol-wide configuration, managed by the owner.
    ProtocolConfig internal _protocolConfig;

    /// @notice Realtor-specific configuration overrides, managed by the owner.
    mapping(address => RealtorConfig) internal _realtorConfig;

    /// @notice Timeline of lease creations per realtor for rate limiting.
    mapping(address => uint64[]) internal _leaseCreationTimestampsByRealtor;

    /// @notice Timeline of payout config updates per lessee for rate limiting.
    mapping(address => uint64[]) internal _payoutConfigUpdateTimestampsByLessee;

    /// @notice Signed protocol profit-and-loss (fees earned minus rebalance drift).
    int256 public protocolPnl;

    /// @notice LP principal tracking.
    mapping(address => uint256) public lpPrincipal;

    /// @notice Swap rate per token, in parts-per-million of USDT.
    /// @dev Rate is expressed in swap destination token units per _RATE_DENOMINATOR of USDT.
    mapping(address => uint256) public swapRatePpm;

    /// @notice Officially used bridgers for bridging target tokens in claims
    ///         to chains defined by lessees.
    mapping(address => mapping(uint256 => IBridger)) public bridgers;

    /// @notice Mapping of what chains are deprecated.
    /// @dev For deprecated chains, lessees can't set them in the payout config.
    mapping(uint256 => bool) public isChainDeprecated;

    /// @notice Last processed controller event-chain tip (starts at controller genesis).
    bytes32 public lastControllerEventTip = EventChainGenesis.UntronControllerIndex;

    /// @notice Queue of controller events awaiting processing on EVM.
    ControllerEvent[] internal _controllerEvents;
    uint256 public nextControllerEventIndex;

    /// @notice Per-target-token FIFO claim queues for grouped swap+bridge fills.
    mapping(address => Claim[]) public claimsByTargetToken;

    /// @notice Per-target-token head index for grouped queues.
    mapping(address => uint256) public nextIndexByTargetToken;

    /// @notice Guard against double-processing of recognizable Tron deposits.
    /// @dev Keyed by Tron tx leaf (sha256(Transaction.encode(tx))).
    mapping(bytes32 => bool) public depositProcessed;

    /// @notice Nonces per lease for gasless payout config updates.
    mapping(uint256 => uint256) public leaseNonces;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroAmount();
    error InsufficientProtocolProfit();
    error CannotRescueUSDT();
    error NotRealtor();
    error LeaseFeeTooLow();
    error InvalidLeaseTimeframe();
    error LeaseNotNukeableYet();
    error InvalidLeaseId();
    error NotLessee();
    error NoActiveLease();
    error DepositAlreadyProcessed();
    error InvalidReceiverForSalt();
    error WithdrawExceedsPrincipal();
    error InsufficientUsdtBalance();
    error NotTronUsdt();
    error NotEventChainTip();
    error EventTipMismatch();
    error EventRelayNoProgress();
    error InvalidSignature();
    error SignatureExpired();
    error ChainDeprecated();
    error RateNotSet();
    error NoBridger();
    error InvalidTargetToken();
    error AmountTooLargeForInt();
    error LeaseRateLimitConfigInvalid();
    error LeaseRateLimitExceeded();
    error PayoutConfigRateLimitConfigInvalid();
    error PayoutConfigRateLimitExceeded();

    // Tron tx decoding errors (local copy of reader-side invariants)
    error TronInvalidCalldataLength();

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @param controllerAddress Address of the UntronController on Tron (source chain), in EVM 20‑byte form.
    /// @param create2Prefix Chain-specific byte prefix used for CREATE2 address computation (0x41 for Tron).
    /// @param tronReader_ Address of the initial external Tron tx reader contract (can be updated by owner).
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_) Create2Utils(create2Prefix) {
        SWAP_EXECUTOR = new SwapExecutor(); // its address is gonna be keccak256(rlp([address(this), 1]))
        CONTROLLER_ADDRESS = controllerAddress;
        tronReader = TronTxReader(tronReader_);
        _initializeOwner(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                               ADMIN CONFIG
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the accounting token (USDT) contract address.
    /// @dev Callable by the owner; can be reconfigured if needed in v1.
    function setUsdt(address usdt_) external onlyOwner {
        usdt = usdt_;
        _emitUsdtSet(usdt_);
    }

    /// @notice Whitelist or un-whitelist a realtor.
    function setRealtor(address realtor, bool allowed) external onlyOwner {
        isRealtor[realtor] = allowed;
        _emitRealtorSet(realtor, allowed);
    }

    /// @notice Set the deprecated status for a given destination chain.
    /// @dev A true value marks the chain as deprecated.
    function setChainDeprecated(uint256 targetChainId, bool deprecated) external onlyOwner {
        isChainDeprecated[targetChainId] = deprecated;
        _emitChainDeprecatedSet(targetChainId, deprecated);
    }

    /// @notice Set global protocol minimum fee in parts per million (applies to all Tron USDT volume).
    function setProtocolFloorPpm(uint256 floorPpm) external onlyOwner {
        if (floorPpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();
        // casting to 'uint32' is safe because floorPpm <= _PPM_DENOMINATOR (1_000_000)
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.floorPpm = uint32(floorPpm);
        _emitProtocolFloorSet(floorPpm);
    }

    /// @notice Set realtor-specific minimum fee in parts per million (applies to all Tron USDT volume).
    function setRealtorMinFeePpm(address realtor, uint256 minFeePpm) external onlyOwner {
        if (minFeePpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();
        // casting to 'uint32' is safe because minFeePpm <= _PPM_DENOMINATOR (1_000_000)
        // forge-lint: disable-next-line(unsafe-typecast)
        _realtorConfig[realtor].minFeePpm = uint32(minFeePpm);
        _emitRealtorMinFeeSet(realtor, minFeePpm);
    }

    /// @notice Sets the protocol-wide lease creation rate limit for all realtors.
    /// @dev Setting both values to 0 disables the protocol-wide rate limit.
    function setProtocolLeaseRateLimit(uint256 maxLeases, uint256 windowSeconds) external onlyOwner {
        if (maxLeases > type(uint32).max || windowSeconds > type(uint32).max) revert LeaseRateLimitConfigInvalid();
        if ((maxLeases == 0) != (windowSeconds == 0)) revert LeaseRateLimitConfigInvalid();
        // casting to 'uint32' is safe because we cap values to type(uint32).max above
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.leaseRateLimitMaxLeases = uint32(maxLeases);
        // casting to 'uint32' is safe because we cap values to type(uint32).max above
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.leaseRateLimitWindowSeconds = uint32(windowSeconds);
        _emitProtocolLeaseRateLimitSet(maxLeases, windowSeconds);
    }

    /// @notice Sets the protocol-wide lessee payout config update rate limit.
    /// @dev Setting both values to 0 disables the lessee payout config update rate limit.
    function setLesseePayoutConfigRateLimit(uint256 maxUpdates, uint256 windowSeconds) external onlyOwner {
        if (maxUpdates > type(uint32).max || windowSeconds > type(uint32).max) {
            revert PayoutConfigRateLimitConfigInvalid();
        }
        if ((maxUpdates == 0) != (windowSeconds == 0)) revert PayoutConfigRateLimitConfigInvalid();
        // casting to 'uint32' is safe because we cap values to type(uint32).max above
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitMaxUpdates = uint32(maxUpdates);
        // casting to 'uint32' is safe because we cap values to type(uint32).max above
        // forge-lint: disable-next-line(unsafe-typecast)
        _protocolConfig.payoutConfigRateLimitWindowSeconds = uint32(windowSeconds);
        _emitLesseePayoutConfigRateLimitSet(maxUpdates, windowSeconds);
    }

    /// @notice Configure lease creation rate limiting for a specific realtor.
    /// @dev `Inherit` uses the protocol-wide config, `Override` sets realtor-specific values, `Disabled` skips the check.
    function setRealtorLeaseRateLimit(
        address realtor,
        LeaseRateLimitMode mode,
        uint256 maxLeases,
        uint256 windowSeconds
    ) external onlyOwner {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        cfg.leaseRateLimitMode = mode;

        if (mode == LeaseRateLimitMode.Override) {
            if (maxLeases > type(uint32).max || windowSeconds > type(uint32).max) revert LeaseRateLimitConfigInvalid();
            if (maxLeases == 0 || windowSeconds == 0) revert LeaseRateLimitConfigInvalid();
            // casting to 'uint32' is safe because we cap values to type(uint32).max above
            // forge-lint: disable-next-line(unsafe-typecast)
            cfg.leaseRateLimitMaxLeases = uint32(maxLeases);
            // casting to 'uint32' is safe because we cap values to type(uint32).max above
            // forge-lint: disable-next-line(unsafe-typecast)
            cfg.leaseRateLimitWindowSeconds = uint32(windowSeconds);
        } else {
            if (maxLeases != 0 || windowSeconds != 0) revert LeaseRateLimitConfigInvalid();
            cfg.leaseRateLimitMaxLeases = 0;
            cfg.leaseRateLimitWindowSeconds = 0;
        }

        _emitRealtorLeaseRateLimitSet(realtor, uint8(mode), maxLeases, windowSeconds);
    }

    /// @notice Set or update the external Tron tx reader contract address.
    function setTronReader(address reader) external onlyOwner {
        tronReader = TronTxReader(reader);
        _emitTronReaderSet(reader);
    }

    /// @notice Set bridge rate for a target token (independent of destination chain).
    /// @dev Rate is expressed in target token units per _RATE_DENOMINATOR of USDT.
    function setSwapRate(address targetToken, uint256 ratePpm) external onlyOwner {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (ratePpm == 0) revert RateNotSet();
        swapRatePpm[targetToken] = ratePpm;
        _emitSwapRateSet(targetToken, ratePpm);
    }

    /// @notice Set the bridger for a target token and destination chain.
    function setBridger(address targetToken, uint256 targetChainId, address bridger) external onlyOwner {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (bridger == address(0)) revert NoBridger();
        bridgers[targetToken][targetChainId] = IBridger(bridger);
        _emitBridgerSet(targetToken, targetChainId, bridger);
    }

    /// @notice Pauses the protocol.
    function pause() external onlyOwner {
        _pause();
    }

    /// @notice Unpauses the protocol.
    function unpause() external onlyOwner {
        _unpause();
    }

    /// @notice Withdraws positive protocol PnL to the owner.
    function withdrawProtocolProfit(int256 amount) external onlyOwner {
        if (amount <= 0) revert ZeroAmount();
        if (amount > protocolPnl) revert InsufficientProtocolProfit();
        // casting to 'uint256' is safe because we revert negative values in the first line
        // forge-lint: disable-next-line(unsafe-typecast)
        TokenUtils.transfer(usdt, payable(msg.sender), uint256(amount));
        _applyPnlDelta(-amount, PnlReason.WITHDRAWAL);
    }

    function rescueTokens(address token, uint256 amount) external onlyOwner {
        if (token == usdt) revert CannotRescueUSDT();
        TokenUtils.transfer(token, payable(msg.sender), amount);
        _emitTokensRescued(token, amount);
    }

    /*//////////////////////////////////////////////////////////////
                                 LEASES
    //////////////////////////////////////////////////////////////*/

    /// @notice Create a new lease for a given receiver salt.
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
        if (!isRealtor[msg.sender]) revert NotRealtor();

        _enforceLeaseRateLimit(msg.sender);

        uint256 minFee = _minLeaseFeePpm(msg.sender);
        if (leaseFeePpm < minFee || leaseFeePpm > _PPM_DENOMINATOR) revert LeaseFeeTooLow();
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

        uint64 startTime = uint64(block.timestamp);
        if (nukeableAfter < startTime) revert InvalidLeaseTimeframe();

        // Uniqueness is enforced per receiver salt regardless of token.
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        if (ids.length != 0) {
            Lease storage last = leases[ids[ids.length - 1]];
            // Disallow nuking before previous lease becomes nukeable.
            if (block.timestamp < last.nukeableAfter) revert LeaseNotNukeableYet();
        }

        leaseId = nextLeaseId++;

        Lease storage l = leases[leaseId];
        l.receiverSalt = receiverSalt;
        l.realtor = msg.sender;
        l.lessee = lessee;
        l.startTime = startTime;
        l.nukeableAfter = nukeableAfter;
        l.leaseFeePpm = leaseFeePpm;
        l.flatFee = flatFee;

        _resolveRoute(targetChainId, targetToken);

        // Store payout configuration so that target chain configuration is
        // available for owner-recommended bridging or direct payouts.
        l.payout = PayoutConfig({targetChainId: targetChainId, targetToken: targetToken, beneficiary: beneficiary});

        ids.push(leaseId);

        _emitLeaseCreated(leaseId, receiverSalt, msg.sender, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee);
        // this is slightly crutchy because we technically enshrine the initial config
        // at creation time, but this simplifies indexing logic quite a bunch
        _emitPayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    /// @notice Update payout configuration for an existing lease.
    /// @dev Callable only by the current lessee.
    function setPayoutConfig(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        external
        whenNotPaused
    {
        Lease storage l = leases[leaseId];
        if (l.lessee == address(0)) revert InvalidLeaseId();
        if (msg.sender != l.lessee) revert NotLessee();

        _enforcePayoutConfigRateLimit(msg.sender);

        // this technically makes changing beneficiaries but not chains revert too
        // but i think it's fine because hey mf you're the one who stops us from deprecating it
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();
        _resolveRoute(targetChainId, targetToken);

        l.payout = PayoutConfig({targetChainId: targetChainId, targetToken: targetToken, beneficiary: beneficiary});

        _emitPayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    /// @notice Gasless payout config update using an EIP-712 signature by the lessee.
    /// @dev Anyone can relay this; signer must be the current lessee of `leaseId`.
    function setPayoutConfigWithSig(
        uint256 leaseId,
        PayoutConfig calldata config,
        uint256 deadline,
        bytes calldata signature
    ) external whenNotPaused {
        if (block.timestamp > deadline) revert SignatureExpired();

        Lease storage l = leases[leaseId];
        if (l.lessee == address(0)) revert InvalidLeaseId();

        _enforcePayoutConfigRateLimit(l.lessee);

        uint256 nonce = leaseNonces[leaseId];
        uint256 targetChainId_ = config.targetChainId;
        address targetToken_ = config.targetToken;
        address beneficiary_ = config.beneficiary;
        bytes32 typehash = _PAYOUT_CONFIG_UPDATE_TYPEHASH;

        // this technically makes changing beneficiaries but not chains revert too
        // but i think it's fine because hey mf you're the one who stops us from deprecating it
        if (isChainDeprecated[targetChainId_]) revert ChainDeprecated();
        _resolveRoute(targetChainId_, targetToken_);

        bytes32 structHash;
        assembly {
            // Load free memory pointer
            let ptr := mload(0x40)

            // abi.encode(
            //   _PAYOUT_CONFIG_UPDATE_TYPEHASH,
            //   leaseId,
            //   config.targetChainId,
            //   config.targetToken,
            //   config.beneficiary,
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

        bytes32 digest = _hashTypedData(structHash);

        // ECDSA or ERC1271 depending on `lessee` code length.
        bool ok = l.lessee.isValidSignatureNow(digest, signature);
        if (!ok) revert InvalidSignature();

        unchecked {
            leaseNonces[leaseId] = nonce + 1;
        }
        _emitLeaseNonceUpdated(leaseId, nonce + 1);

        l.payout = PayoutConfig({
            targetChainId: config.targetChainId, targetToken: config.targetToken, beneficiary: config.beneficiary
        });

        _emitPayoutConfigUpdated(leaseId, config.targetChainId, config.targetToken, config.beneficiary);
    }

    /*//////////////////////////////////////////////////////////////
                        PRE-ENTITLEMENT FROM TRON
    //////////////////////////////////////////////////////////////*/

    /// @notice Prove and pre-entitle a recognizable TRC-20 deposit on Tron to a lease.
    /// @dev This function wires together the external TronTxReader with the lease timeline logic.
    function preEntitle(
        bytes32 receiverSalt,
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external whenNotPaused returns (uint256 claimIndex, uint256 leaseId, uint256 netOut) {
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

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
        leaseId = _findActiveLeaseId(receiverSalt, callData.tronBlockTimestamp);
        if (leaseId == 0) revert NoActiveLease();

        Lease storage l = leases[leaseId];

        l.recognizedRaw += amountQ;
        l.unbackedRaw += amountQ;

        netOut = _computeNetOut(l, amountQ);

        _bookFee(amountQ, netOut);

        if (netOut > 0) {
            PayoutConfig storage p = l.payout;
            claimIndex = _enqueueClaimForTargetToken(p.targetToken, netOut, leaseId, p.targetChainId, p.beneficiary);
            _emitClaimCreated(claimIndex, leaseId, netOut);
        }

        _emitDepositPreEntitled(callData.txId, leaseId, amountQ, netOut);
    }

    /*//////////////////////////////////////////////////////////////
                         CONTROLLER EVENT RELAY
    //////////////////////////////////////////////////////////////*/

    /// @notice Verify a Tron tx containing isEventChainTip (plain or inside multicall) and enqueue controller events.
    /// @dev Only hashes the provided events to check they link lastControllerEventTip -> tipNew; processing is separate.
    function relayControllerEventChain(
        uint256 tronBlockNumber,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index,
        ControllerEvent[] calldata events
    ) external whenNotPaused returns (bytes32 tipNew) {
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        // Validate that the call is targeting the expected UntronController contract on Tron.
        bytes21 controllerTron = TronCalldataUtils.evmToTronAddress(CONTROLLER_ADDRESS);
        if (callData.toTron != controllerTron) revert NotEventChainTip();

        bytes memory data = callData.data;
        if (data.length < 4) revert TronInvalidCalldataLength();
        bytes4 sel;
        assembly ("memory-safe") {
            sel := shr(224, mload(add(data, 0x20)))
        }

        if (sel == _SELECTOR_IS_EVENT_CHAIN_TIP) {
            tipNew = TronCalldataUtils.decodeIsEventChainTip(data);
        } else if (sel == _SELECTOR_MULTICALL) {
            tipNew = TronCalldataUtils.decodeMulticallEventChainTip(data, _SELECTOR_IS_EVENT_CHAIN_TIP);
        } else {
            revert NotEventChainTip();
        }

        bytes32 tip = lastControllerEventTip;
        if (tipNew == tip) revert EventRelayNoProgress();

        uint256 n = events.length;
        for (uint256 i = 0; i < n; ++i) {
            ControllerEvent calldata ev = events[i];
            _emitControllerEventChainTipUpdated(tip, ev.blockNumber, ev.blockTimestamp, ev.sig, ev.data);
            tip = sha256(abi.encodePacked(tip, ev.blockNumber, ev.blockTimestamp, ev.sig, ev.data));
        }
        if (tip != tipNew) revert EventTipMismatch();

        lastControllerEventTip = tipNew;

        for (uint256 j = 0; j < n; ++j) {
            ControllerEvent calldata ev = events[j];
            _controllerEvents.push(
                ControllerEvent({
                    sig: ev.sig, data: ev.data, blockNumber: ev.blockNumber, blockTimestamp: ev.blockTimestamp
                })
            );
        }
    }

    /// @notice Process up to `maxEvents` queued controller events.
    /// @dev Applies only events UntronV3 cares about; unknown events are skipped but still advance the cursor.
    function processControllerEvents(uint256 maxEvents) external whenNotPaused {
        uint256 idx = nextControllerEventIndex;
        uint256 end = _controllerEvents.length;
        uint256 processed;

        while (idx < end && processed < maxEvents) {
            ControllerEvent storage ev = _controllerEvents[idx];
            bytes32 sig = ev.sig;

            if (sig == _EVENT_SIG_PULLED_FROM_RECEIVER) {
                (bytes32 receiverSalt,/*token*/,/*tokenAmount*/,/* exchangeRate */, uint256 usdtAmount) =
                    abi.decode(ev.data, (bytes32, address, uint256, uint256, uint256));
                _processReceiverPulled(receiverSalt, usdtAmount, ev.blockTimestamp);
            } else if (sig == _EVENT_SIG_USDT_SET) {
                address newTronUsdt = abi.decode(ev.data, (address));
                tronUsdt = newTronUsdt;
                _emitTronUsdtSet(newTronUsdt);
            } else if (sig == _EVENT_SIG_USDT_REBALANCED) {
                (
                    uint256 inAmount,
                    uint256 outAmount, /*rebalancer*/
                ) = abi.decode(ev.data, (uint256, uint256, address));
                int256 delta = outAmount >= inAmount ? _toInt(outAmount - inAmount) : -_toInt(inAmount - outAmount);
                _applyPnlDelta(delta, PnlReason.REBALANCE);
            }

            unchecked {
                ++idx;
                ++processed;
            }
        }

        nextControllerEventIndex = idx;
    }

    /*//////////////////////////////////////////////////////////////
                              LP VAULT
    //////////////////////////////////////////////////////////////*/

    /// @notice Deposit USDT into the fast-fill vault.
    /// @dev fast-fill vaults are 0% APY by design. All incentivized LPing logic
    ///      must be handled by external contracts.
    function deposit(uint256 amount) external whenNotPaused {
        if (amount == 0) revert ZeroAmount();

        TokenUtils.transferFrom(usdt, msg.sender, payable(address(this)), amount);
        lpPrincipal[msg.sender] += amount;

        _emitLpDeposited(msg.sender, amount);
    }

    /// @notice Withdraw USDT from the fast-fill vault.
    function withdraw(uint256 amount) external whenNotPaused {
        if (amount == 0) revert ZeroAmount();

        uint256 principal = lpPrincipal[msg.sender];
        if (amount > principal) revert WithdrawExceedsPrincipal();
        if (amount > usdtBalance()) revert InsufficientUsdtBalance();

        lpPrincipal[msg.sender] = principal - amount;
        TokenUtils.transfer(usdt, payable(msg.sender), amount);

        _emitLpWithdrawn(msg.sender, amount);
    }

    /*//////////////////////////////////////////////////////////////
                             CLAIM QUEUE
    //////////////////////////////////////////////////////////////*/

    /// @notice Fill up to `maxClaims` claims for a target token, optionally swapping once then bridging.
    /// @dev `calls` may be empty if no swap is needed (e.g. USDT -> USDT bridging).
    ///      Any swap output above `expectedOutTotal` is paid to the relayer (`msg.sender`).
    ///      This function is non-reentant because it calls executor that performs arbitrary onchain calls.
    function fill(address targetToken, uint256 maxClaims, Call[] calldata calls) external nonReentrant whenNotPaused {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (maxClaims == 0) return;

        Claim[] storage queue = claimsByTargetToken[targetToken];
        uint256 surplusOut;
        uint256 startHead = nextIndexByTargetToken[targetToken];

        FillPlan memory plan = _buildFillPlan(targetToken, maxClaims, queue, startHead, usdtBalance());

        if (plan.processedSlots == 0) {
            return;
        }

        if (plan.totalUsdt != 0) {
            TokenUtils.transfer(usdt, payable(address(SWAP_EXECUTOR)), plan.totalUsdt);
            uint256 actualOut = SWAP_EXECUTOR.execute(calls, targetToken, plan.expectedOutTotal, payable(address(this)));

            if (actualOut > plan.expectedOutTotal) {
                surplusOut = actualOut - plan.expectedOutTotal;
            }
        }

        _executeFillPlan(targetToken, queue, startHead, plan);

        nextIndexByTargetToken[targetToken] = plan.newHead;

        if (surplusOut != 0) {
            TokenUtils.transfer(targetToken, payable(msg.sender), surplusOut);
        }
    }

    /*//////////////////////////////////////////////////////////////
                             EXTERNAL VIEW
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns the protocol-wide lease rate limit config.
    function protocolLeaseRateLimit() external view returns (uint256 maxLeases, uint256 windowSeconds) {
        ProtocolConfig storage cfg = _protocolConfig;
        return (cfg.leaseRateLimitMaxLeases, cfg.leaseRateLimitWindowSeconds);
    }

    /// @notice Returns the protocol-wide payout config update rate limit for lessees.
    function lesseePayoutConfigRateLimit() external view returns (uint256 maxUpdates, uint256 windowSeconds) {
        ProtocolConfig storage cfg = _protocolConfig;
        return (cfg.payoutConfigRateLimitMaxUpdates, cfg.payoutConfigRateLimitWindowSeconds);
    }

    /// @notice Returns the raw realtor lease rate limit config (mode + values).
    function realtorLeaseRateLimit(address realtor)
        external
        view
        returns (LeaseRateLimitMode mode, uint256 maxLeases, uint256 windowSeconds)
    {
        RealtorConfig storage cfg = _realtorConfig[realtor];
        return (cfg.leaseRateLimitMode, cfg.leaseRateLimitMaxLeases, cfg.leaseRateLimitWindowSeconds);
    }

    /// @notice Returns the effective lease rate limit config for a realtor after applying overrides.
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
    function protocolFloorPpm() public view returns (uint256) {
        return uint256(_protocolConfig.floorPpm);
    }

    /// @notice Returns the realtor-specific minimum fee override in parts per million.
    /// @dev Preserves the legacy public getter name.
    function realtorMinFeePpm(address realtor) public view returns (uint256) {
        return uint256(_realtorConfig[realtor].minFeePpm);
    }

    /// @notice Returns the current USDT balance held by this contract.
    function usdtBalance() public view returns (uint256) {
        address usdt_ = usdt; // not sure if the compiler would optimize it into this anyway
        if (usdt_ == address(0)) return 0;
        return TokenUtils.getBalanceOf(usdt_, address(this));
    }

    /*//////////////////////////////////////////////////////////////
                           INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _enforceLeaseRateLimit(address realtor) internal {
        (bool enabled, uint256 maxLeases, uint256 windowSeconds) = _effectiveLeaseRateLimit(realtor);
        if (!enabled) return;

        uint64 nowTs = uint64(block.timestamp);
        uint64[] storage timestamps = _leaseCreationTimestampsByRealtor[realtor];
        uint256 len = timestamps.length;

        if (len >= maxLeases) {
            uint64 oldest = timestamps[len - maxLeases];
            if (uint256(nowTs) < uint256(oldest) + windowSeconds) revert LeaseRateLimitExceeded();
        }

        timestamps.push(nowTs);
    }

    function _enforcePayoutConfigRateLimit(address lessee) internal {
        ProtocolConfig storage cfg = _protocolConfig;
        uint256 maxUpdates = cfg.payoutConfigRateLimitMaxUpdates;
        uint256 windowSeconds = cfg.payoutConfigRateLimitWindowSeconds;
        if (maxUpdates == 0 || windowSeconds == 0) return;

        uint64 nowTs = uint64(block.timestamp);
        uint64[] storage timestamps = _payoutConfigUpdateTimestampsByLessee[lessee];
        uint256 len = timestamps.length;

        if (len >= maxUpdates) {
            uint64 oldest = timestamps[len - maxUpdates];
            if (uint256(nowTs) < uint256(oldest) + windowSeconds) revert PayoutConfigRateLimitExceeded();
        }

        timestamps.push(nowTs);
    }

    function _buildFillPlan(
        address targetToken,
        uint256 maxClaims,
        Claim[] storage queue,
        uint256 head,
        uint256 available
    ) internal returns (FillPlan memory plan) {
        uint256 queueLen = queue.length;

        plan.kinds = new RouteKind[](maxClaims);
        plan.ratesPpm = new uint256[](maxClaims);
        plan.bridgers = new address[](maxClaims);
        plan.beneficiaries = new address[](maxClaims);
        plan.targetChainIds = new uint256[](maxClaims);

        uint256 processedSlots;
        uint256 totalUsdt;
        uint256 expectedOutTotal;

        while (head < queueLen && processedSlots < maxClaims) {
            Claim storage c = queue[head];

            if (c.amountUSDT == 0) {
                unchecked {
                    ++head;
                    ++processedSlots;
                }
                continue;
            }

            plan.beneficiaries[processedSlots] = c.beneficiary;
            plan.targetChainIds[processedSlots] = c.targetChainId;

            Route memory rt = _resolveRoute(c.targetChainId, targetToken);
            plan.kinds[processedSlots] = rt.kind;
            plan.ratesPpm[processedSlots] = rt.ratePpm;
            plan.bridgers[processedSlots] = rt.bridger;

            uint256 amountUsdt = c.amountUSDT;
            if (available < amountUsdt) break;

            if (rt.kind == RouteKind.LocalUsdt) {
                c.amountUSDT = 0;
                TokenUtils.transfer(usdt, payable(c.beneficiary), amountUsdt);
                _emitClaimFilled(head, c.leaseId, amountUsdt);
            } else {
                uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, rt.ratePpm, _RATE_DENOMINATOR);
                totalUsdt += amountUsdt;
                expectedOutTotal += expectedOut;
            }

            unchecked {
                available -= amountUsdt;
                ++head;
                ++processedSlots;
            }
        }

        plan.newHead = head;
        plan.processedSlots = processedSlots;
        plan.totalUsdt = totalUsdt;
        plan.expectedOutTotal = expectedOutTotal;
    }

    function _executeFillPlan(address targetToken, Claim[] storage queue, uint256 startHead, FillPlan memory plan)
        internal
    {
        uint256 secondHead = startHead;
        uint256 remaining = plan.processedSlots;

        while (remaining != 0) {
            Claim storage c2 = queue[secondHead];
            uint256 amountUsdt = c2.amountUSDT;

            if (amountUsdt != 0) {
                c2.amountUSDT = 0;
                uint256 slot = plan.processedSlots - remaining;
                RouteKind kind = plan.kinds[slot];

                if (kind == RouteKind.LocalSwap) {
                    uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, plan.ratesPpm[slot], _RATE_DENOMINATOR);
                    if (expectedOut != 0) {
                        TokenUtils.transfer(targetToken, payable(plan.beneficiaries[slot]), expectedOut);
                    }
                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                } else if (kind == RouteKind.Bridge) {
                    uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, plan.ratesPpm[slot], _RATE_DENOMINATOR);
                    address bridger = plan.bridgers[slot];

                    if (expectedOut != 0) {
                        TokenUtils.transfer(targetToken, payable(bridger), expectedOut);
                        IBridger(bridger)
                            .bridge(targetToken, expectedOut, plan.targetChainIds[slot], plan.beneficiaries[slot]);
                    }

                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                } else {
                    TokenUtils.transfer(usdt, payable(plan.beneficiaries[slot]), amountUsdt);
                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                }
            }

            unchecked {
                ++secondHead;
                --remaining;
            }
        }
    }

    function _applyPnlDelta(int256 delta, PnlReason reason) internal {
        if (delta == 0) return;
        protocolPnl += delta;
        _emitProtocolPnlUpdated(protocolPnl, delta, reason);
    }

    function _bookFee(uint256 raw, uint256 netOut) internal {
        _applyPnlDelta(_toInt(raw - netOut), PnlReason.FEE);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _enqueueClaimForTargetToken(
        address targetToken,
        uint256 amountUsdt,
        uint256 leaseId,
        uint256 targetChainId,
        address beneficiary
    ) internal returns (uint256 claimIndex) {
        Claim[] storage queue = claimsByTargetToken[targetToken];
        queue.push(
            Claim({amountUSDT: amountUsdt, leaseId: leaseId, targetChainId: targetChainId, beneficiary: beneficiary})
        );
        return queue.length - 1;
    }

    function _processReceiverPulled(bytes32 receiverSalt, uint256 usdtAmount, uint64 dumpTimestamp) internal {
        if (usdtAmount == 0) {
            return;
        }

        uint256 remaining = usdtAmount;

        // Repay historical unbacked volume across leases for receiverSalt.
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        uint256 len = ids.length;
        for (uint256 j = 0; j < len && remaining != 0; ++j) {
            Lease storage oldL = leases[ids[j]];
            if (oldL.startTime > dumpTimestamp) break;
            uint256 unbacked = oldL.unbackedRaw;
            if (unbacked == 0) continue;
            uint256 repay = remaining < unbacked ? remaining : unbacked;
            oldL.backedRaw += repay;
            oldL.unbackedRaw = unbacked - repay;
            remaining -= repay;
        }

        // Any remaining volume becomes profit for the lease active at dump time.
        if (remaining != 0) {
            uint256 currentLeaseId = _findActiveLeaseId(receiverSalt, dumpTimestamp);
            if (currentLeaseId == 0) {
                _applyPnlDelta(_toInt(remaining), PnlReason.RECEIVER_PULL_NO_LEASE);
                return;
            }

            Lease storage cur = leases[currentLeaseId];
            cur.recognizedRaw += remaining;
            cur.backedRaw += remaining;

            uint256 netOut = _computeNetOut(cur, remaining);
            _bookFee(remaining, netOut);
            if (netOut > 0) {
                PayoutConfig storage p = cur.payout;
                uint256 claimIndex =
                    _enqueueClaimForTargetToken(p.targetToken, netOut, currentLeaseId, p.targetChainId, p.beneficiary);
                _emitClaimCreated(claimIndex, currentLeaseId, netOut);
            }
        }
    }

    /*//////////////////////////////////////////////////////////////
                          INTERNAL VIEW FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _minLeaseFeePpm(address realtor) internal view returns (uint256) {
        uint256 minFee = uint256(_protocolConfig.floorPpm);
        uint256 realtorMin = uint256(_realtorConfig[realtor].minFeePpm);
        if (realtorMin > minFee) minFee = realtorMin;
        return minFee;
    }

    function _effectiveLeaseRateLimit(address realtor)
        internal
        view
        returns (bool enabled, uint256 maxLeases, uint256 windowSeconds)
    {
        RealtorConfig storage rcfg = _realtorConfig[realtor];
        LeaseRateLimitMode mode = rcfg.leaseRateLimitMode;
        if (mode == LeaseRateLimitMode.Disabled) return (false, 0, 0);

        if (mode == LeaseRateLimitMode.Override) {
            maxLeases = rcfg.leaseRateLimitMaxLeases;
            windowSeconds = rcfg.leaseRateLimitWindowSeconds;
        } else {
            ProtocolConfig storage pcfg = _protocolConfig;
            maxLeases = pcfg.leaseRateLimitMaxLeases;
            windowSeconds = pcfg.leaseRateLimitWindowSeconds;
        }

        enabled = (maxLeases != 0 && windowSeconds != 0);
    }

    function _resolveRoute(uint256 targetChainId, address targetToken) internal view returns (Route memory r) {
        if (targetToken == address(0)) revert InvalidTargetToken();

        if (targetChainId == block.chainid) {
            if (targetToken == usdt) {
                return Route({kind: RouteKind.LocalUsdt, ratePpm: 0, bridger: address(0)});
            }

            uint256 rate = swapRatePpm[targetToken];
            if (rate == 0) revert RateNotSet();

            return Route({kind: RouteKind.LocalSwap, ratePpm: rate, bridger: address(0)});
        }

        IBridger bridger = bridgers[targetToken][targetChainId];

        uint256 rate2 = swapRatePpm[targetToken];
        if (rate2 == 0) revert RateNotSet();

        if (address(bridger) == address(0)) revert NoBridger();

        return Route({kind: RouteKind.Bridge, ratePpm: rate2, bridger: address(bridger)});
    }

    function _findActiveLeaseId(bytes32 receiverSalt, uint64 ts) internal view returns (uint256 leaseId) {
        uint256[] storage ids = _leaseIdsByReceiver[receiverSalt];
        uint256 len = ids.length;
        if (len == 0) return 0;

        // Walk backwards until we find the last lease with startTime <= ts.
        for (uint256 i = len; i != 0;) {
            unchecked {
                --i;
            }
            Lease storage l = leases[ids[i]];
            if (l.startTime <= ts) {
                leaseId = ids[i];
                break;
            }
        }
    }

    function _computeNetOut(Lease storage l, uint256 amountQ) internal view returns (uint256 netOut) {
        uint256 feePpm = l.leaseFeePpm;
        uint256 percentageOut = amountQ * (_PPM_DENOMINATOR - feePpm) / _PPM_DENOMINATOR;
        uint256 flat = l.flatFee;
        if (percentageOut > flat) {
            unchecked {
                netOut = percentageOut - flat;
            }
        } else {
            netOut = 0;
        }
    }

    /*//////////////////////////////////////////////////////////////
                          INTERNAL PURE FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function _toInt(uint256 x) internal pure returns (int256) {
        if (x > uint256(type(int256).max)) revert AmountTooLargeForInt();
        // casting to 'int256' is safe because we check if x is greater than max int256 value
        // and revert if so in the line above
        // forge-lint: disable-next-line(unsafe-typecast)
        return int256(x);
    }

    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
        // decided not to do UntronV3 just to appear cleaner in users' wallets.
        // Signature request: "Untron". Sexy.
        // (Untron V1 and Untron V2 didn't use EIP-712 signatures,
        // and Untron Intents is probably gonna use a different name/version, so this is safe.)
        name = "Untron";
        version = "1";
    }
}
