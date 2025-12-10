// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Create2Utils} from "../utils/Create2Utils.sol";
import {UntronControllerIndexGenesisEventChainHash} from "../utils/UntronControllerIndexGenesisEventChainHash.sol";
import {UntronV3IndexGenesisEventChainHash} from "../utils/UntronV3IndexGenesisEventChainHash.sol";
import {TronTxReader} from "./TronTxReader.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";
import {EIP712} from "solady/utils/EIP712.sol";
import {SignatureCheckerLib} from "solady/utils/SignatureCheckerLib.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/// @notice Minimal interface for swappers used during `fill()`.
interface ISwapper {
    // forge-lint: disable-next-line(mixed-case-variable)
    function handlePayout(uint256 amountUSDT, uint256 targetChainId, address targetToken, address beneficiary) external;
}

/// @title UntronV3Index
/// @notice Hash-chain-based event index for Untron V3 hub, friendly to offchain indexers.
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
    event SwapperSet(address indexed targetToken, uint256 indexed targetChainId, address swapper);
    event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);
    event ProtocolFloorSet(uint256 floorPpm);
    event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);

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
    event TronUsdtSet(address indexed tronUsdt);

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

    function _emitSwapperSet(address targetToken, uint256 targetChainId, address swapper) internal {
        _appendEventChain(SwapperSet.selector, abi.encode(targetToken, targetChainId, swapper));
        emit SwapperSet(targetToken, targetChainId, swapper);
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

    function _emitTronReaderSet(address reader) internal {
        _appendEventChain(TronReaderSet.selector, abi.encode(reader));
        emit TronReaderSet(reader);
    }

    function _emitTronUsdtSet(address tronUsdt) internal {
        _appendEventChain(TronUsdtSet.selector, abi.encode(tronUsdt));
        emit TronUsdtSet(tronUsdt);
    }
}

/// @title Hub contract for Untron V3 protocol.
contract UntronV3 is Create2Utils, EIP712, Ownable, UntronV3Index {
    using SignatureCheckerLib for address;

    /*//////////////////////////////////////////////////////////////
                                  TYPES
    //////////////////////////////////////////////////////////////*/

    /// @notice Per-lease payout configuration, mutable by the lessee.
    /// @dev Swapper selection is per-chain and owner-controlled; leases only
    ///      specify destination chain, target token, and beneficiary.
    struct PayoutConfig {
        uint256 targetChainId;
        address targetToken;
        address beneficiary;
    }

    /// @dev EIP-712 typehash for gasless payout config updates.
    bytes32 internal constant PAYOUT_CONFIG_UPDATE_TYPEHASH = keccak256(
        "PayoutConfigUpdate(" "uint256 leaseId," "uint256 targetChainId," "address targetToken," "address beneficiary,"
        "uint256 nonce," "uint256 deadline" ")"
    );

    /*//////////////////////////////////////////////////////////////
                                     CONSTANTS
    //////////////////////////////////////////////////////////////*/

    // Parts-per-million denominator used for fee calculations (1_000_000 = 100%).
    uint256 internal constant PPM_DENOMINATOR = 1_000_000;

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

    /// @notice LP principal tracking.
    mapping(address => uint256) public lpPrincipal;

    /// @notice Swapper per destination token + destination chain, configured by the owner.
    mapping(address => mapping(uint256 => address)) public swapperForTokenAndChain;

    /// @notice Mapping of what chains are deprecated.
    /// @dev For deprecated chains, lessees can't set them in the payout config.
    mapping(uint256 => bool) public isChainDeprecated;

    /// @notice Last processed controller event-chain tip (starts at controller genesis).
    bytes32 public lastControllerEventTip = EVENT_CHAIN_GENESIS_TIP;

    /// @notice Queue of controller events awaiting processing on EVM.
    ControllerEvent[] internal controllerEvents;
    uint256 public nextControllerEventIndex;

    /// @notice Global FIFO claim queue.
    Claim[] public claims;

    /// @notice Index of the next claim to try filling.
    uint256 public nextClaimIndex;

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
    error NoSwapper();
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
    error NoEventChainTipInMulticall();
    error EventTipMismatch();
    error EventRelayNoProgress();
    error InvalidSignature();
    error SignatureExpired();
    error InvalidChainId();
    error ChainDeprecated();

    // Tron tx decoding errors (local copy of reader-side invariants)
    error NotATrc20Transfer();
    error TronInvalidTrc20DataLength();
    error TronInvalidCalldataLength();

    /*//////////////////////////////////////////////////////////////
                              TRON CALL CONSTANTS
    //////////////////////////////////////////////////////////////*/

    // TRC-20 function selectors
    bytes4 internal constant SELECTOR_TRANSFER = bytes4(keccak256("transfer(address,uint256)"));
    bytes4 internal constant SELECTOR_TRANSFER_FROM = bytes4(keccak256("transferFrom(address,address,uint256)"));

    // UntronController selectors
    bytes4 internal constant SELECTOR_IS_EVENT_CHAIN_TIP = bytes4(keccak256("isEventChainTip(bytes32)"));
    bytes4 internal constant SELECTOR_MULTICALL = bytes4(keccak256("multicall(bytes[])"));

    // UntronController event signatures used in the event chain
    bytes32 internal constant EVENT_SIG_PULLED_FROM_RECEIVER =
        keccak256("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)");
    bytes32 internal constant EVENT_SIG_USDT_SET = keccak256("UsdtSet(address)");

    // Event chain genesis tip (matches UntronControllerIndex initial value).
    bytes32 internal constant EVENT_CHAIN_GENESIS_TIP = UntronControllerIndexGenesisEventChainHash.VALUE;

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @param controllerAddress Address of the UntronController on Tron (source chain), in EVM 20‑byte form.
    /// @param create2Prefix Chain-specific byte prefix used for CREATE2 address computation (0x41 for Tron).
    /// @param tronReader_ Address of the initial external Tron tx reader contract (can be updated by owner).
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_) Create2Utils(create2Prefix) {
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

    /// @notice Set the swapper for a given destination token and destination chain.
    /// @dev Do not ever set swapperForTokenAndChain[targetToken][chainId] = address(0) for pairs already in use;
    ///      use isChainDeprecated to block new usage instead. We don't explicitly forbid 0x00 here
    ///      because owner could as well specify 0x01 or 0xdead and brick the chain this way too.
    function setSwapper(address targetToken, uint256 targetChainId, address swapper) external onlyOwner {
        swapperForTokenAndChain[targetToken][targetChainId] = swapper;
        _emitSwapperSet(targetToken, targetChainId, swapper);
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

        if (swapperForTokenAndChain[targetToken][targetChainId] == address(0)) revert InvalidChainId();
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

        // Store payout configuration so that target chain configuration is
        // available for owner-recommended swappers or direct payouts.
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

        if (swapperForTokenAndChain[targetToken][targetChainId] == address(0)) revert InvalidChainId();
        if (isChainDeprecated[targetChainId]) revert ChainDeprecated();

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

        if (swapperForTokenAndChain[targetToken_][targetChainId_] == address(0)) revert InvalidChainId();
        if (isChainDeprecated[targetChainId_]) revert ChainDeprecated();

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
        if (callData.toTron != _evmToTronAddress(tronUsdt_)) revert NotTronUsdt();

        // Sanity-check that the TRC-20 transfer goes into the expected receiver.
        address predictedReceiver = predictReceiverAddress(CONTROLLER_ADDRESS, receiverSalt);
        bytes21 expectedToTron = _evmToTronAddress(predictedReceiver);
        (, bytes21 toTron, uint256 amountQ) = _decodeTrc20FromCalldata(callData.data, callData.senderTron);
        if (toTron != expectedToTron) revert InvalidReceiverForSalt();

        // Token is no longer part of lease uniqueness; use receiver salt only.
        leaseId = _findActiveLeaseId(receiverSalt, callData.tronBlockTimestamp);
        if (leaseId == 0) revert NoActiveLease();

        Lease storage l = leases[leaseId];

        l.recognizedRaw += amountQ;
        l.unbackedRaw += amountQ;

        netOut = _computeNetOut(l, amountQ);

        if (netOut > 0) {
            claims.push(Claim({amountUSDT: netOut, leaseId: leaseId}));
            claimIndex = claims.length - 1;
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
    ) external onlyOwner returns (bytes32 tipNew) {
        TronTxReader.TriggerSmartContract memory callData =
            tronReader.readTriggerSmartContract(tronBlockNumber, encodedTx, proof, index);

        // Validate that the call is targeting the expected UntronController contract on Tron.
        bytes21 controllerTron = _evmToTronAddress(CONTROLLER_ADDRESS);
        if (callData.toTron != controllerTron) revert NotEventChainTip();

        bytes memory data = callData.data;
        if (data.length < 4) revert TronInvalidCalldataLength();
        bytes4 sel;
        assembly ("memory-safe") {
            sel := shr(224, mload(add(data, 0x20)))
        }

        if (sel == SELECTOR_IS_EVENT_CHAIN_TIP) {
            tipNew = _decodeIsEventChainTip(data);
        } else if (sel == SELECTOR_MULTICALL) {
            tipNew = _decodeMulticallEventChainTip(data);
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
            }

            unchecked {
                ++idx;
                ++processed;
            }
        }

        nextControllerEventIndex = idx;
    }

    /// @notice Convenience wrapper to fill as many claims as gas allows.
    function fill() external {
        fill(type(uint256).max);
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

    /// @notice Fill up to `maxClaims` claims from the head of the queue.
    /// @dev For cross-chain payouts, reverts if the owner-configured
    ///      swapper for the target chain reverts for a claim.
    function fill(uint256 maxClaims) public {
        uint256 claimIdx = nextClaimIndex;
        uint256 end = claims.length;
        uint256 processed;
        uint256 available = usdtBalance();

        while (claimIdx < end && processed < maxClaims) {
            Claim storage c = claims[claimIdx];

            if (c.amountUSDT == 0) {
                unchecked {
                    ++claimIdx;
                    ++processed;
                }
                continue;
            }

            if (available < c.amountUSDT) {
                break;
            }

            Lease storage l = leases[c.leaseId];
            PayoutConfig storage p = l.payout;
            uint256 targetChainId = p.targetChainId;
            address targetToken = p.targetToken;

            if (targetChainId == block.chainid && targetToken == usdt) {
                // Same-chain payout: send USDT directly to the beneficiary.
                SafeTransferLib.safeTransfer(usdt, p.beneficiary, c.amountUSDT);
            } else {
                // Cross-chain payout: use the owner-configured swapper for the (token, chain) pair.
                address swapper = swapperForTokenAndChain[targetToken][targetChainId];
                if (swapper == address(0)) revert NoSwapper();

                SafeTransferLib.safeTransfer(usdt, swapper, c.amountUSDT);
                ISwapper(swapper).handlePayout(c.amountUSDT, targetChainId, p.targetToken, p.beneficiary);
            }

            unchecked {
                available -= c.amountUSDT;
            }

            _emitClaimFilled(claimIdx, c.leaseId, c.amountUSDT);

            unchecked {
                ++claimIdx;
                ++processed;
            }
        }

        nextClaimIndex = claimIdx;
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

    function _decodeIsEventChainTip(bytes memory data) internal pure returns (bytes32 tip) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32) revert TronInvalidCalldataLength();
        assembly ("memory-safe") {
            tip := mload(add(data, 0x24)) // selector (4) + tip (32)
        }
    }

    function _decodeMulticallEventChainTip(bytes memory data) internal pure returns (bytes32 tip) {
        uint256 dataEnd = data.length;
        if (dataEnd < 4 + 32) revert TronInvalidCalldataLength();

        uint256 base = 4;
        uint256 offArray;
        assembly ("memory-safe") {
            offArray := mload(add(data, 0x24))
        }
        uint256 arrayOffset = base + offArray;
        (uint256 arrStart, uint256 arrEnd, uint256 cursor) = _readDyn(data, arrayOffset, dataEnd);

        uint256 n = _readU256(data, arrStart);
        cursor = arrStart + 32;
        for (uint256 i = 0; i < n; ++i) {
            (uint256 callStart, uint256 callEnd, uint256 newCursor) = _readDyn(data, cursor, arrEnd);
            uint256 callDataOffset = callStart + 32;
            if (callEnd > dataEnd || callEnd < callDataOffset) revert TronInvalidCalldataLength();

            bytes4 innerSel;
            assembly ("memory-safe") {
                innerSel := shr(224, mload(add(data, add(0x20, callDataOffset))))
            }

            if (innerSel == SELECTOR_IS_EVENT_CHAIN_TIP) {
                uint256 callLen = callEnd - callDataOffset;
                if (callLen != 4 + 32) revert TronInvalidCalldataLength();
                assembly ("memory-safe") {
                    tip := mload(add(data, add(0x20, add(callDataOffset, 4))))
                }
                return tip;
            }

            cursor = newCursor;
        }

        revert NoEventChainTipInMulticall();
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
            if (netOut > 0) {
                claims.push(Claim({amountUSDT: netOut, leaseId: currentLeaseId}));
                uint256 claimIdx = claims.length - 1;
                _emitClaimCreated(claimIdx, currentLeaseId, netOut);
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

    /// @dev Decode TRC-20 transfer / transferFrom calldata (first recognizable transfer).
    function _decodeTrc20FromCalldata(bytes memory data, bytes21 senderTron)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        if (data.length < 4) revert TronInvalidCalldataLength();
        bytes4 sig;
        assembly ("memory-safe") {
            sig := shr(224, mload(add(data, 0x20)))
        }
        if (sig == SELECTOR_TRANSFER) {
            (toTron, amount) = _decodeTrc20TransferArgs(data);
            fromTron = senderTron;
        } else if (sig == SELECTOR_TRANSFER_FROM) {
            (fromTron, toTron, amount) = _decodeTrc20TransferFromArgs(data);
        } else {
            revert NotATrc20Transfer();
        }
    }

    function _decodeTrc20TransferArgs(bytes memory data) internal pure returns (bytes21 toTron, uint256 amount) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 2) revert TronInvalidTrc20DataLength();
        bytes32 word1;
        bytes32 word2;
        assembly ("memory-safe") {
            word1 := mload(add(data, 0x24)) // 0x20 (data) + 4 (selector)
            word2 := mload(add(data, 0x44)) // 0x20 (data) + 36
        }
        address toAddr = address(uint160(uint256(word1)));
        toTron = _evmToTronAddress(toAddr);
        amount = uint256(word2);
    }

    function _decodeTrc20TransferFromArgs(bytes memory data)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 3) revert TronInvalidTrc20DataLength();
        bytes32 w1;
        bytes32 w2;
        bytes32 w3;
        assembly ("memory-safe") {
            w1 := mload(add(data, 0x24)) // from
            w2 := mload(add(data, 0x44)) // to
            w3 := mload(add(data, 0x64)) // amount
        }
        address fromAddr = address(uint160(uint256(w1)));
        address toAddr2 = address(uint160(uint256(w2)));
        fromTron = _evmToTronAddress(fromAddr);
        toTron = _evmToTronAddress(toAddr2);
        amount = uint256(w3);
    }

    function _readDyn(bytes memory data, uint256 offset, uint256 limit)
        internal
        pure
        returns (uint256 start, uint256 end, uint256 newCursor)
    {
        start = offset;
        uint256 len = _readU256(data, start);
        end = start + 32 + len;
        if (end > limit) revert TronInvalidCalldataLength();
        return (start, end, end);
    }

    function _readU256(bytes memory data, uint256 offset) internal pure returns (uint256 v) {
        assembly ("memory-safe") {
            v := mload(add(data, add(0x20, offset)))
        }
    }

    /// @dev Convert an EVM address into Tron-style 21-byte address (0x41 prefix + 20-byte address).
    function _evmToTronAddress(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
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
