// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Create2Utils} from "../utils/Create2Utils.sol";
import {UntronControllerIndexGenesisEventChainHash} from "../utils/UntronControllerIndexGenesisEventChainHash.sol";
import {UntronV3IndexGenesisEventChainHash} from "../utils/UntronV3IndexGenesisEventChainHash.sol";
import {TronTxReader} from "./TronTxReader.sol";
import {TronCalldataLib} from "./TronCalldataLib.sol";
import {SwapExecutor, Call} from "./SwapExecutor.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";

import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {EIP712} from "solady/utils/EIP712.sol";
import {SignatureCheckerLib} from "solady/utils/SignatureCheckerLib.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/// @notice Interface for contracts bridging payout tokens cross-chain.
interface IBridger {
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external;
}

/// @title  UntronV3Index
/// @notice Hash-chain-based event index for Untron V3 hub, friendly to offchain indexers.
/// @dev    UntronV3 must not emit events itself. All events must be defined and emitted through UntronV3Index.
/// @author Ultrasound Labs
contract UntronV3Index {
    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = UntronV3IndexGenesisEventChainHash.VALUE;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    event UsdtSet(address indexed usdt);
    event RealtorSet(address indexed realtor, bool allowed);
    event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);
    event ProtocolFloorSet(uint256 floorPpm);
    event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);
    event TronUsdtSet(address indexed tronUsdt);
    event BridgeRateSet(address indexed bridgeToken, uint256 ratePpm);
    event BridgerSet(address indexed bridgeToken, uint256 indexed targetChainId, address bridger);

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

    event PayoutConfigUpdated(uint256 indexed leaseId, uint256 targetChainId, address targetToken, address beneficiary);

    // forge-lint: disable-next-line(mixed-case-variable)
    event ClaimCreated(uint256 indexed claimIndex, uint256 indexed leaseId, uint256 amountUSDT);
    // forge-lint: disable-next-line(mixed-case-variable)
    event ClaimFilled(uint256 indexed claimIndex, uint256 indexed leaseId, uint256 amountUSDT);

    event DepositPreEntitled(bytes32 indexed txLeaf, uint256 indexed leaseId, uint256 rawAmount, uint256 netOut);

    event LpDeposited(address indexed lp, uint256 amount);
    event LpWithdrawn(address indexed lp, uint256 amount);
    event TronReaderSet(address indexed reader);
    event ProtocolPnlUpdated(int256 pnl, int256 delta, uint8 reason);

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature of the event.
    /// @param abiEncodedEventData The ABI-encoded data of the event.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        eventChainTip =
            sha256(abi.encodePacked(eventChainTip, block.number, block.timestamp, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    function _emitUsdtSet(address usdt_) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(usdt_));
        emit UsdtSet(usdt_);
    }

    function _emitRealtorSet(address realtor, bool allowed) internal {
        _appendEventChain(RealtorSet.selector, abi.encode(realtor, allowed));
        emit RealtorSet(realtor, allowed);
    }

    function _emitChainDeprecatedSet(uint256 targetChainId, bool deprecated) internal {
        _appendEventChain(ChainDeprecatedSet.selector, abi.encode(targetChainId, deprecated));
        emit ChainDeprecatedSet(targetChainId, deprecated);
    }

    function _emitProtocolFloorSet(uint256 floorPpm) internal {
        _appendEventChain(ProtocolFloorSet.selector, abi.encode(floorPpm));
        emit ProtocolFloorSet(floorPpm);
    }

    function _emitRealtorMinFeeSet(address realtor, uint256 minFeePpm) internal {
        _appendEventChain(RealtorMinFeeSet.selector, abi.encode(realtor, minFeePpm));
        emit RealtorMinFeeSet(realtor, minFeePpm);
    }

    function _emitTronReaderSet(address reader) internal {
        _appendEventChain(TronReaderSet.selector, abi.encode(reader));
        emit TronReaderSet(reader);
    }

    function _emitTronUsdtSet(address tronUsdt) internal {
        _appendEventChain(TronUsdtSet.selector, abi.encode(tronUsdt));
        emit TronUsdtSet(tronUsdt);
    }

    function _emitBridgeRateSet(address bridgeToken, uint256 ratePpm) internal {
        _appendEventChain(BridgeRateSet.selector, abi.encode(bridgeToken, ratePpm));
        emit BridgeRateSet(bridgeToken, ratePpm);
    }

    function _emitBridgerSet(address bridgeToken, uint256 targetChainId, address bridger) internal {
        _appendEventChain(BridgerSet.selector, abi.encode(bridgeToken, targetChainId, bridger));
        emit BridgerSet(bridgeToken, targetChainId, bridger);
    }

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

    function _emitPayoutConfigUpdated(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        internal
    {
        _appendEventChain(PayoutConfigUpdated.selector, abi.encode(leaseId, targetChainId, targetToken, beneficiary));
        emit PayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _emitClaimCreated(uint256 claimIndex, uint256 leaseId, uint256 amountUSDT) internal {
        _appendEventChain(ClaimCreated.selector, abi.encode(claimIndex, leaseId, amountUSDT));
        emit ClaimCreated(claimIndex, leaseId, amountUSDT);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _emitClaimFilled(uint256 claimIndex, uint256 leaseId, uint256 amountUSDT) internal {
        _appendEventChain(ClaimFilled.selector, abi.encode(claimIndex, leaseId, amountUSDT));
        emit ClaimFilled(claimIndex, leaseId, amountUSDT);
    }

    function _emitDepositPreEntitled(bytes32 txLeaf, uint256 leaseId, uint256 rawAmount, uint256 netOut) internal {
        _appendEventChain(DepositPreEntitled.selector, abi.encode(txLeaf, leaseId, rawAmount, netOut));
        emit DepositPreEntitled(txLeaf, leaseId, rawAmount, netOut);
    }

    function _emitLpDeposited(address lp, uint256 amount) internal {
        _appendEventChain(LpDeposited.selector, abi.encode(lp, amount));
        emit LpDeposited(lp, amount);
    }

    function _emitLpWithdrawn(address lp, uint256 amount) internal {
        _appendEventChain(LpWithdrawn.selector, abi.encode(lp, amount));
        emit LpWithdrawn(lp, amount);
    }

    function _emitProtocolPnlUpdated(int256 pnl, int256 delta, uint8 reason) internal {
        _appendEventChain(ProtocolPnlUpdated.selector, abi.encode(pnl, delta, reason));
        emit ProtocolPnlUpdated(pnl, delta, reason);
    }
}

/// @title Hub contract for Untron V3 protocol.
contract UntronV3 is Create2Utils, EIP712, Ownable, UntronV3Index {
    using SignatureCheckerLib for address;

    /*//////////////////////////////////////////////////////////////
                                  TYPES
    //////////////////////////////////////////////////////////////*/

    enum RouteKind {
        LocalUsdt,
        LocalSwap,
        Bridge
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
    /// @dev Owner controls bridge pair availability; leases specify destination chain, bridge token, and beneficiary.
    struct PayoutConfig {
        uint256 targetChainId;
        // bridge token on THIS chain, not destination chain.
        address targetToken;
        address beneficiary;
    }

    /// @dev EIP-712 typehash for gasless payout config updates.
    bytes32 internal constant PAYOUT_CONFIG_UPDATE_TYPEHASH = keccak256(
        "PayoutConfigUpdate(uint256 leaseId,uint256 targetChainId,address targetToken,address beneficiary,uint256 nonce,uint256 deadline)"
    );

    /*//////////////////////////////////////////////////////////////
                                     CONSTANTS
    //////////////////////////////////////////////////////////////*/

    // Parts-per-million denominator used for fee calculations (1_000_000 = 100%).
    uint256 internal constant PPM_DENOMINATOR = 1_000_000;
    /// @notice Denominator for bridge token rate tables.
    uint256 internal constant RATE_DENOMINATOR = 1_000_000;

    // Protocol PnL update reason codes.
    uint8 internal constant PNL_REASON_FEE = 1;
    uint8 internal constant PNL_REASON_REBALANCE = 2;

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

    /// @notice Bridge configuration for a token and destination chain.
    struct BridgePair {
        address bridger;
        bool deprecated;
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
                                 STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice The address of the UntronController contract on Tron (source chain), in EVM format.
    address public immutable CONTROLLER_ADDRESS;

    /// @notice External Tron reader used to verify and decode Tron transactions.
    TronTxReader public tronReader;

    /// @notice Address of USDT on destination (EVM) chain used for all accounting.
    address public usdt;

    /// @notice Address of Tron USDT token (20-byte EVM-form of TRC‑20 contract address on Tron).
    /// @dev Used to enforce that preEntitle only recognizes Tron USDT transfers.
    address public tronUsdt;

    /// @notice Swap executor used for batched swaps before bridging.
    SwapExecutor public swapExecutor;

    /// @notice Next lease identifier (starts from 1 so 0 can mean "missing").
    uint256 public nextLeaseId = 1;

    /// @notice Mapping from lease id to lease data.
    mapping(uint256 => Lease) public leases;

    /// @notice Timeline of leases per receiver salt.
    mapping(bytes32 => uint256[]) internal leaseIdsByReceiver;

    /// @notice Whitelisted realtors.
    mapping(address => bool) public isRealtor;

    /// @notice Global protocol minimum fee in parts per million (applies to all Tron USDT volume).
    uint256 public protocolFloorPpm;

    /// @notice Realtor-specific minimum fee in parts per million (applies to all Tron USDT volume).
    mapping(address => uint256) public realtorMinFeePpm;

    /// @notice Signed protocol profit-and-loss (fees earned minus rebalance drift).
    int256 public protocolPnl;

    /// @notice LP principal tracking.
    mapping(address => uint256) public lpPrincipal;

    /// @notice Bridge pair configuration per bridge token and destination chain.
    mapping(address => mapping(uint256 => BridgePair)) public bridgePairs;

    /// @notice Bridge rate per bridge token, in parts-per-million of USDT.
    /// @dev Rate is expressed in bridge token units per RATE_DENOMINATOR of USDT.
    mapping(address => uint256) public bridgeRatePpm;

    /// @notice Mapping of what chains are deprecated.
    /// @dev For deprecated chains, lessees can't set them in the payout config.
    mapping(uint256 => bool) public isChainDeprecated;

    /// @notice Track known bridge token and chain pairs for enumeration.
    mapping(address => bool) internal isBridgeTokenKnown;
    mapping(address => mapping(uint256 => bool)) internal isBridgeTokenChainKnown;

    /// @notice Last processed controller event-chain tip (starts at controller genesis).
    bytes32 public lastControllerEventTip = EVENT_CHAIN_GENESIS_TIP;

    /// @notice Queue of controller events awaiting processing on EVM.
    ControllerEvent[] internal controllerEvents;
    uint256 public nextControllerEventIndex;

    /// @notice Per-bridge-token FIFO claim queues for grouped swap+bridge fills.
    mapping(address => Claim[]) public claimsByBridgeToken;

    /// @notice Per-bridge-token head index for grouped queues.
    mapping(address => uint256) public nextIndexByBridgeToken;

    /// @notice Guard against double-processing of recognizable Tron deposits.
    /// @dev Keyed by Tron tx leaf (sha256(Transaction.encode(tx))).
    mapping(bytes32 => bool) public depositProcessed;

    /// @notice Nonces per lease for gasless payout config updates.
    mapping(uint256 => uint256) public leaseNonces;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    error ZeroAmount();
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
    error InvalidBridgeToken();
    error AmountTooLargeForInt();

    // Tron tx decoding errors (local copy of reader-side invariants)
    error TronInvalidCalldataLength();

    /*//////////////////////////////////////////////////////////////
                              TRON CALL CONSTANTS
    //////////////////////////////////////////////////////////////*/

    // UntronController selectors
    bytes4 internal constant SELECTOR_IS_EVENT_CHAIN_TIP = bytes4(keccak256("isEventChainTip(bytes32)"));
    bytes4 internal constant SELECTOR_MULTICALL = bytes4(keccak256("multicall(bytes[])"));

    // UntronController event signatures used in the event chain
    bytes32 internal constant EVENT_SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
    bytes32 internal constant EVENT_SIG_USDT_SET = keccak256("UsdtSet(address)");
    bytes32 internal constant EVENT_SIG_USDT_REBALANCED = keccak256("UsdtRebalanced(uint256,uint256,address)");

    // Event chain genesis tip (matches UntronControllerIndex initial value).
    bytes32 internal constant EVENT_CHAIN_GENESIS_TIP = UntronControllerIndexGenesisEventChainHash.VALUE;

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @param controllerAddress Address of the UntronController on Tron (source chain), in EVM 20‑byte form.
    /// @param create2Prefix Chain-specific byte prefix used for CREATE2 address computation (0x41 for Tron).
    /// @param tronReader_ Address of the initial external Tron tx reader contract (can be updated by owner).
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_) Create2Utils(create2Prefix) {
        swapExecutor = new SwapExecutor(); // its address is gonna be keccak256(rlp([address(this), 1]))
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
        if (floorPpm > PPM_DENOMINATOR) revert LeaseFeeTooLow();
        protocolFloorPpm = floorPpm;
        _emitProtocolFloorSet(floorPpm);
    }

    /// @notice Set realtor-specific minimum fee in parts per million (applies to all Tron USDT volume).
    function setRealtorMinFeePpm(address realtor, uint256 minFeePpm) external onlyOwner {
        if (minFeePpm > PPM_DENOMINATOR) revert LeaseFeeTooLow();
        realtorMinFeePpm[realtor] = minFeePpm;
        _emitRealtorMinFeeSet(realtor, minFeePpm);
    }

    /// @notice Set or update the external Tron tx reader contract address.
    function setTronReader(address reader) external onlyOwner {
        tronReader = TronTxReader(reader);
        _emitTronReaderSet(reader);
    }

    /// @notice Set bridge rate for a bridge token (independent of destination chain).
    /// @dev Rate is expressed in bridge token units per RATE_DENOMINATOR of USDT.
    function setBridgeRate(address bridgeToken, uint256 ratePpm) external onlyOwner {
        if (bridgeToken == address(0)) revert InvalidBridgeToken();
        if (ratePpm == 0) revert RateNotSet();
        bridgeRatePpm[bridgeToken] = ratePpm;
        _emitBridgeRateSet(bridgeToken, ratePpm);
    }

    /// @notice Set the bridger for a bridge token and destination chain.
    function setBridger(address bridgeToken, uint256 targetChainId, address bridger) external onlyOwner {
        if (bridgeToken == address(0)) revert InvalidBridgeToken();
        if (bridger == address(0)) revert NoBridger();
        bridgePairs[bridgeToken][targetChainId].bridger = bridger;
        _emitBridgerSet(bridgeToken, targetChainId, bridger);
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
    ) external returns (uint256 leaseId) {
        if (!isRealtor[msg.sender]) revert NotRealtor();

        uint256 minFee = protocolFloorPpm;
        uint256 realtorMin = realtorMinFeePpm[msg.sender];
        if (realtorMin > minFee) minFee = realtorMin;
        if (leaseFeePpm < minFee || leaseFeePpm > PPM_DENOMINATOR) revert LeaseFeeTooLow();
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

        uint64 startTime = uint64(block.timestamp);
        if (nukeableAfter < startTime) revert InvalidLeaseTimeframe();

        // Uniqueness is enforced per receiver salt regardless of token.
        uint256[] storage ids = leaseIdsByReceiver[receiverSalt];
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
    }

    /// @notice Update payout configuration for an existing lease.
    /// @dev Callable only by the current lessee.
    function setPayoutConfig(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        external
    {
        Lease storage l = leases[leaseId];
        if (l.lessee == address(0)) revert InvalidLeaseId();
        if (msg.sender != l.lessee) revert NotLessee();

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
    ) external {
        if (block.timestamp > deadline) revert SignatureExpired();

        Lease storage l = leases[leaseId];
        if (l.lessee == address(0)) revert InvalidLeaseId();

        uint256 nonce = leaseNonces[leaseId];
        uint256 targetChainId_ = config.targetChainId;
        address targetToken_ = config.targetToken;
        address beneficiary_ = config.beneficiary;
        bytes32 typehash = PAYOUT_CONFIG_UPDATE_TYPEHASH;

        // this technically makes changing beneficiaries but not chains revert too
        // but i think it's fine because hey mf you're the one who stops us from deprecating it
        if (isChainDeprecated[targetChainId_]) revert ChainDeprecated();
        _resolveRoute(targetChainId_, targetToken_);

        bytes32 structHash;
        assembly {
            // Load free memory pointer
            let ptr := mload(0x40)

            // abi.encode(
            //   PAYOUT_CONFIG_UPDATE_TYPEHASH,
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
    ) external returns (uint256 claimIndex, uint256 leaseId, uint256 netOut) {
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        if (depositProcessed[callData.txLeaf]) revert DepositAlreadyProcessed();
        depositProcessed[callData.txLeaf] = true;

        // Enforce that the TRC-20 contract called is exactly Tron USDT.
        address tronUsdt_ = tronUsdt;
        if (callData.toTron != TronCalldataLib.evmToTronAddress(tronUsdt_)) revert NotTronUsdt();

        // Sanity-check that the TRC-20 transfer goes into the expected receiver.
        address predictedReceiver = predictReceiverAddress(CONTROLLER_ADDRESS, receiverSalt);
        bytes21 expectedToTron = TronCalldataLib.evmToTronAddress(predictedReceiver);
        (, bytes21 toTron, uint256 amountQ) =
            TronCalldataLib.decodeTrc20FromCalldata(callData.data, callData.senderTron);
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
            claimIndex = _enqueueClaimForBridgeToken(p.targetToken, netOut, leaseId, p.targetChainId, p.beneficiary);
            _emitClaimCreated(claimIndex, leaseId, netOut);
        }

        _emitDepositPreEntitled(callData.txLeaf, leaseId, amountQ, netOut);
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
    ) external returns (bytes32 tipNew) {
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        // Validate that the call is targeting the expected UntronController contract on Tron.
        bytes21 controllerTron = TronCalldataLib.evmToTronAddress(CONTROLLER_ADDRESS);
        if (callData.toTron != controllerTron) revert NotEventChainTip();

        bytes memory data = callData.data;
        if (data.length < 4) revert TronInvalidCalldataLength();
        bytes4 sel;
        assembly ("memory-safe") {
            sel := shr(224, mload(add(data, 0x20)))
        }

        if (sel == SELECTOR_IS_EVENT_CHAIN_TIP) {
            tipNew = TronCalldataLib.decodeIsEventChainTip(data);
        } else if (sel == SELECTOR_MULTICALL) {
            tipNew = TronCalldataLib.decodeMulticallEventChainTip(data, SELECTOR_IS_EVENT_CHAIN_TIP);
        } else {
            revert NotEventChainTip();
        }

        bytes32 tip = lastControllerEventTip;
        if (tipNew == tip) revert EventRelayNoProgress();

        uint256 n = events.length;
        for (uint256 i = 0; i < n; ++i) {
            ControllerEvent calldata ev = events[i];
            tip = sha256(abi.encodePacked(tip, ev.blockNumber, ev.blockTimestamp, ev.sig, ev.data));
        }
        if (tip != tipNew) revert EventTipMismatch();

        lastControllerEventTip = tipNew;

        for (uint256 j = 0; j < n; ++j) {
            ControllerEvent calldata ev = events[j];
            controllerEvents.push(
                ControllerEvent({
                    sig: ev.sig, data: ev.data, blockNumber: ev.blockNumber, blockTimestamp: ev.blockTimestamp
                })
            );
        }
    }

    /// @notice Process up to `maxEvents` queued controller events.
    /// @dev Applies only events UntronV3 cares about; unknown events are skipped but still advance the cursor.
    function processControllerEvents(uint256 maxEvents) external {
        uint256 idx = nextControllerEventIndex;
        uint256 end = controllerEvents.length;
        uint256 processed;

        while (idx < end && processed < maxEvents) {
            ControllerEvent storage ev = controllerEvents[idx];
            bytes32 sig = ev.sig;

            if (sig == EVENT_SIG_PULLED_FROM_RECEIVER) {
                (bytes32 receiverSalt,/*token*/,/*tokenAmount*/,/* exchangeRate */, uint256 usdtAmount) =
                    abi.decode(ev.data, (bytes32, address, uint256, uint256, uint256));
                _processReceiverPulled(receiverSalt, usdtAmount, ev.blockTimestamp);
            } else if (sig == EVENT_SIG_USDT_SET) {
                address newTronUsdt = abi.decode(ev.data, (address));
                tronUsdt = newTronUsdt;
                _emitTronUsdtSet(newTronUsdt);
            } else if (sig == EVENT_SIG_USDT_REBALANCED) {
                (
                    uint256 inAmount,
                    uint256 outAmount, /*rebalancer*/
                ) = abi.decode(ev.data, (uint256, uint256, address));
                int256 delta = outAmount >= inAmount ? _toInt(outAmount - inAmount) : -_toInt(inAmount - outAmount);
                _applyPnlDelta(delta, PNL_REASON_REBALANCE);
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
    function deposit(uint256 amount) external {
        if (amount == 0) revert ZeroAmount();

        SafeTransferLib.safeTransferFrom(usdt, msg.sender, address(this), amount);
        lpPrincipal[msg.sender] += amount;

        _emitLpDeposited(msg.sender, amount);
    }

    /// @notice Withdraw USDT from the fast-fill vault.
    function withdraw(uint256 amount) external {
        if (amount == 0) revert ZeroAmount();

        uint256 principal = lpPrincipal[msg.sender];
        if (amount > principal) revert WithdrawExceedsPrincipal();
        if (amount > usdtBalance()) revert InsufficientUsdtBalance();

        lpPrincipal[msg.sender] = principal - amount;
        SafeTransferLib.safeTransfer(usdt, msg.sender, amount);

        _emitLpWithdrawn(msg.sender, amount);
    }

    /*//////////////////////////////////////////////////////////////
                             CLAIM QUEUE
    //////////////////////////////////////////////////////////////*/

    /// @notice Fill up to `maxClaims` claims for a bridge token, optionally swapping once then bridging.
    /// @dev `calls` may be empty if no swap is needed (e.g. USDT -> USDT bridging).
    ///      Any swap output above `expectedOutTotal` is paid to the relayer (`msg.sender`).
    function fill(address bridgeToken, uint256 maxClaims, Call[] calldata calls) external {
        if (bridgeToken == address(0)) revert InvalidBridgeToken();
        if (maxClaims == 0) return;

        Claim[] storage queue = claimsByBridgeToken[bridgeToken];
        uint256 surplusOut;
        uint256 startHead = nextIndexByBridgeToken[bridgeToken];

        FillPlan memory plan = _buildFillPlan(bridgeToken, maxClaims, queue, startHead, usdtBalance());

        if (plan.processedSlots == 0) {
            return;
        }
        nextIndexByBridgeToken[bridgeToken] = plan.newHead;

        if (plan.totalUsdt != 0) {
            SwapExecutor executor = swapExecutor;
            SafeTransferLib.safeTransfer(usdt, address(executor), plan.totalUsdt);
            uint256 actualOut = executor.execute(calls, bridgeToken, plan.expectedOutTotal, payable(address(this)));

            if (actualOut > plan.expectedOutTotal) {
                surplusOut = actualOut - plan.expectedOutTotal;
            }
        }

        _executeFillPlan(bridgeToken, queue, startHead, plan);

        if (surplusOut != 0) {
            TokenUtils.transfer(bridgeToken, payable(msg.sender), surplusOut);
        }
    }

    /// @notice Returns the current USDT balance held by this contract.
    function usdtBalance() public view returns (uint256) {
        address usdt_ = usdt; // not sure if the compiler would optimize it into this anyway
        if (usdt_ == address(0)) return 0;
        return SafeTransferLib.balanceOf(usdt_, address(this));
    }

    /*//////////////////////////////////////////////////////////////
                           INTERNAL HELPERS
    //////////////////////////////////////////////////////////////*/

    function _resolveRoute(uint256 targetChainId, address targetToken) internal view returns (Route memory r) {
        if (targetToken == address(0)) revert InvalidBridgeToken();

        if (targetChainId == block.chainid) {
            if (targetToken == usdt) {
                return Route({kind: RouteKind.LocalUsdt, ratePpm: 0, bridger: address(0)});
            }

            uint256 rate = bridgeRatePpm[targetToken];
            if (rate == 0) revert RateNotSet();

            return Route({kind: RouteKind.LocalSwap, ratePpm: rate, bridger: address(0)});
        }

        BridgePair storage pair = bridgePairs[targetToken][targetChainId];

        uint256 rate2 = bridgeRatePpm[targetToken];
        if (rate2 == 0) revert RateNotSet();

        address bridger = pair.bridger;
        if (bridger == address(0)) revert NoBridger();

        return Route({kind: RouteKind.Bridge, ratePpm: rate2, bridger: bridger});
    }

    function _buildFillPlan(
        address bridgeToken,
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

            Route memory rt = _resolveRoute(c.targetChainId, bridgeToken);
            plan.kinds[processedSlots] = rt.kind;
            plan.ratesPpm[processedSlots] = rt.ratePpm;
            plan.bridgers[processedSlots] = rt.bridger;

            uint256 amountUsdt = c.amountUSDT;
            if (available < amountUsdt) break;

            if (rt.kind == RouteKind.LocalUsdt) {
                c.amountUSDT = 0;
                SafeTransferLib.safeTransfer(usdt, c.beneficiary, amountUsdt);
                _emitClaimFilled(head, c.leaseId, amountUsdt);
            } else {
                uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, rt.ratePpm, RATE_DENOMINATOR);
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

    function _executeFillPlan(address bridgeToken, Claim[] storage queue, uint256 startHead, FillPlan memory plan)
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
                    uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, plan.ratesPpm[slot], RATE_DENOMINATOR);
                    if (expectedOut != 0) {
                        TokenUtils.transfer(bridgeToken, payable(plan.beneficiaries[slot]), expectedOut);
                    }
                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                } else if (kind == RouteKind.Bridge) {
                    uint256 expectedOut = TokenUtils.mulDiv(amountUsdt, plan.ratesPpm[slot], RATE_DENOMINATOR);
                    address bridger = plan.bridgers[slot];

                    if (expectedOut != 0) {
                        TokenUtils.transfer(bridgeToken, payable(bridger), expectedOut);
                        IBridger(bridger)
                            .bridge(bridgeToken, expectedOut, plan.targetChainIds[slot], plan.beneficiaries[slot]);
                    }

                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                } else {
                    SafeTransferLib.safeTransfer(usdt, plan.beneficiaries[slot], amountUsdt);
                    _emitClaimFilled(secondHead, c2.leaseId, amountUsdt);
                }
            }

            unchecked {
                ++secondHead;
                --remaining;
            }
        }
    }

    function _applyPnlDelta(int256 delta, uint8 reason) internal {
        if (delta == 0) return;
        protocolPnl += delta;
        _emitProtocolPnlUpdated(protocolPnl, delta, reason);
    }

    function _bookFee(uint256 raw, uint256 netOut) internal {
        _applyPnlDelta(_toInt(raw - netOut), PNL_REASON_FEE);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _enqueueClaimForBridgeToken(
        address bridgeToken,
        uint256 amountUsdt,
        uint256 leaseId,
        uint256 targetChainId,
        address beneficiary
    ) internal returns (uint256 claimIndex) {
        Claim[] storage queue = claimsByBridgeToken[bridgeToken];
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
        uint256[] storage ids = leaseIdsByReceiver[receiverSalt];
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
            if (currentLeaseId == 0) revert NoActiveLease();

            Lease storage cur = leases[currentLeaseId];
            cur.recognizedRaw += remaining;
            cur.backedRaw += remaining;

            uint256 netOut = _computeNetOut(cur, remaining);
            _bookFee(remaining, netOut);
            if (netOut > 0) {
                PayoutConfig storage p = cur.payout;
                uint256 claimIndex =
                    _enqueueClaimForBridgeToken(p.targetToken, netOut, currentLeaseId, p.targetChainId, p.beneficiary);
                _emitClaimCreated(claimIndex, currentLeaseId, netOut);
            }
        }
    }

    function _findActiveLeaseId(bytes32 receiverSalt, uint64 ts) internal view returns (uint256 leaseId) {
        uint256[] storage ids = leaseIdsByReceiver[receiverSalt];
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
        uint256 percentageOut = amountQ * (PPM_DENOMINATOR - feePpm) / PPM_DENOMINATOR;
        uint256 flat = l.flatFee;
        if (percentageOut > flat) {
            unchecked {
                netOut = percentageOut - flat;
            }
        } else {
            netOut = 0;
        }
    }

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
