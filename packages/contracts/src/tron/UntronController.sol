// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronReceiver} from "./UntronReceiver.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";
import {IRebalancer} from "./rebalancers/interfaces/IRebalancer.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";
import {EventChainGenesis} from "../utils/EventChainGenesis.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";

/// @title UntronControllerIndex
/// @notice Hash-chain-based event index for Untron Controller friendly to offchain indexers.
/// @author Ultrasound Labs
contract UntronControllerIndex {
    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = EventChainGenesis.UntronControllerIndex;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the owner of the contract is changed.
    /// @dev    Only used in setOwner function and emitted in constructor on deployment.
    /// @param newOwner New owner address.
    event OwnerChanged(address indexed newOwner);
    /// @notice Emitted when the executor of the contract is changed.
    /// @dev    Only used in setExecutor function.
    /// @param newExecutor New executor address.
    event ExecutorChanged(address indexed newExecutor);
    /// @notice Emitted when a payload is set for a particular rebalancer.
    /// @dev    Only used in setPayload function.
    /// @param rebalancer Rebalancer address.
    /// @param payload Rebalancer-specific payload.
    event PayloadSet(address indexed rebalancer, bytes payload);
    /// @notice Emitted when a receiver is deployed.
    /// @dev    Only used in _pullFromReceiver function.
    /// @param receiver Receiver address.
    /// @param salt Salt used for deterministic deployment (CREATE2).
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    /// @notice Emitted for each receiver when tokens are swept into accounting.
    /// @dev    Includes source token amount, exchange rate (1 for USDT), and USDT-equivalent amount.
    /// @param receiverSalt Salt used for deterministic receiver deployment (CREATE2).
    /// @param token Token address.
    /// @param tokenAmount Amount of token pulled from receiver.
    /// @param exchangeRate Scaled exchange rate used for conversion.
    /// @param usdtAmount USDT-equivalent amount accounted for this pull.
    event PulledFromReceiver(
        bytes32 indexed receiverSalt,
        address indexed token,
        uint256 tokenAmount,
        uint256 exchangeRate,
        uint256 usdtAmount
    );
    /// @notice Emitted when USDT is rebalanced via a particular rebalancer.
    /// @dev    Only used in bridgeUsdt function.
    /// @param inAmount Amount of USDT bridged in.
    /// @param outAmount Amount of USDT expected out (as asserted by the rebalancer).
    /// @param rebalancer Rebalancer used.
    event UsdtRebalanced(uint256 inAmount, uint256 outAmount, address indexed rebalancer);
    /// @notice Emitted when USDT is transferred from the controller to a recipient.
    /// @dev    Only used in transferUsdtFromController function.
    /// @param recipient Recipient address.
    /// @param amount Amount of USDT transferred.
    event ControllerUsdtTransfer(address indexed recipient, uint256 amount);
    /// @notice Emitted when the canonical USDT token is set or updated.
    /// @dev    Only used in setUsdt function.
    /// @param newUsdt New USDT token address.
    event UsdtSet(address indexed newUsdt);
    /// @notice Emitted when the LP address is set or updated.
    /// @dev    Only used in setLp function.
    /// @param newLp New LP address.
    event LpSet(address indexed newLp);
    /// @notice Emitted when the LP updates the exchange rate for a token.
    /// @dev    Only used in setLpExchangeRate function.
    /// @param token Token address.
    /// @param exchangeRate Scaled exchange rate set by the LP.
    event LpExchangeRateSet(address indexed token, uint256 exchangeRate);
    /// @notice Emitted when the LP withdraws purchased tokens from the controller.
    /// @dev    Only used in lpWithdrawTokens function.
    /// @param token Token withdrawn.
    /// @param amount Amount withdrawn.
    event LpTokensWithdrawn(address indexed token, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                TRON-SPECIFIC CALLDATA GETTER
    //////////////////////////////////////////////////////////////*/

    /// @notice Checks if the provided event chain tip matches the current event chain tip.
    /// @param eventChainTip_ The event chain tip to check.
    /// @return True if the event chain tips match, reverts otherwise.
    /// @dev This is needed because Tron light client doesn't have access to the state root,
    ///      only successful transactions in the block. So we can expose the event chain tip
    ///      to the light client by allowing users to send transactions
    ///      where the event chain tip is in transaction's calldata.
    function isEventChainTip(bytes32 eventChainTip_) external view returns (bool) {
        require(eventChainTip == eventChainTip_, "no");
        return true;
    }

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature of the event.
    /// @param abiEncodedEventData The ABI-encoded data of the event.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        // we use sha256 here instead of keccak256 for future-proofness
        // in case we ZK prove this smart contract. sha256 is cheaper to prove than keccak256.
        eventChainTip =
            sha256(abi.encodePacked(eventChainTip, block.number, block.timestamp, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emits {OwnerChanged} and appends it to the event chain.
    /// @param newOwner New owner address.
    function _emitOwnerChanged(address newOwner) internal {
        _appendEventChain(OwnerChanged.selector, abi.encode(newOwner));
        emit OwnerChanged(newOwner);
    }

    /// @notice Emits {ExecutorChanged} and appends it to the event chain.
    /// @param newExecutor New executor address.
    function _emitExecutorChanged(address newExecutor) internal {
        _appendEventChain(ExecutorChanged.selector, abi.encode(newExecutor));
        emit ExecutorChanged(newExecutor);
    }

    /// @notice Emits {PayloadSet} and appends it to the event chain.
    /// @param rebalancer Rebalancer address.
    /// @param payload Rebalancer-specific payload.
    function _emitPayloadSet(address rebalancer, bytes memory payload) internal {
        _appendEventChain(PayloadSet.selector, abi.encode(rebalancer, payload));
        emit PayloadSet(rebalancer, payload);
    }

    /// @notice Emits {ReceiverDeployed} and appends it to the event chain.
    /// @param receiver Receiver address.
    /// @param salt Salt used for deterministic deployment (CREATE2).
    function _emitReceiverDeployed(address receiver, bytes32 salt) internal {
        _appendEventChain(ReceiverDeployed.selector, abi.encode(receiver, salt));
        emit ReceiverDeployed(receiver, salt);
    }

    /// @notice Emits {PulledFromReceiver} and appends it to the event chain.
    /// @param receiverSalt Salt used for deterministic receiver deployment (CREATE2).
    /// @param token Token address.
    /// @param tokenAmount Amount of token pulled from receiver.
    /// @param exchangeRate Scaled exchange rate used for conversion.
    /// @param usdtAmount USDT-equivalent amount accounted for this pull.
    function _emitPulledFromReceiver(
        bytes32 receiverSalt,
        address token,
        uint256 tokenAmount,
        uint256 exchangeRate,
        uint256 usdtAmount
    ) internal {
        _appendEventChain(
            PulledFromReceiver.selector, abi.encode(receiverSalt, token, tokenAmount, exchangeRate, usdtAmount)
        );
        emit PulledFromReceiver(receiverSalt, token, tokenAmount, exchangeRate, usdtAmount);
    }

    /// @notice Emits {UsdtRebalanced} and appends it to the event chain.
    /// @param inAmount Amount of USDT bridged in.
    /// @param outAmount Amount of USDT expected out (as asserted by the rebalancer).
    /// @param rebalancer Rebalancer used.
    function _emitUsdtRebalanced(uint256 inAmount, uint256 outAmount, address rebalancer) internal {
        _appendEventChain(UsdtRebalanced.selector, abi.encode(inAmount, outAmount, rebalancer));
        emit UsdtRebalanced(inAmount, outAmount, rebalancer);
    }

    /// @notice Emits {ControllerUsdtTransfer} and appends it to the event chain.
    /// @param recipient Recipient address.
    /// @param amount Amount of USDT transferred.
    function _emitControllerUsdtTransfer(address recipient, uint256 amount) internal {
        _appendEventChain(ControllerUsdtTransfer.selector, abi.encode(recipient, amount));
        emit ControllerUsdtTransfer(recipient, amount);
    }

    /// @notice Emits {UsdtSet} and appends it to the event chain.
    /// @param newUsdt New USDT token address.
    function _emitUsdtSet(address newUsdt) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(newUsdt));
        emit UsdtSet(newUsdt);
    }

    /// @notice Emits {LpSet} and appends it to the event chain.
    /// @param newLp New LP address.
    function _emitLpSet(address newLp) internal {
        _appendEventChain(LpSet.selector, abi.encode(newLp));
        emit LpSet(newLp);
    }

    /// @notice Emits {LpExchangeRateSet} and appends it to the event chain.
    /// @param token Token address.
    /// @param exchangeRate Scaled exchange rate set by the LP.
    function _emitLpExchangeRateSet(address token, uint256 exchangeRate) internal {
        _appendEventChain(LpExchangeRateSet.selector, abi.encode(token, exchangeRate));
        emit LpExchangeRateSet(token, exchangeRate);
    }

    /// @notice Emits {LpTokensWithdrawn} and appends it to the event chain.
    /// @param token Token withdrawn.
    /// @param amount Amount withdrawn.
    function _emitLpTokensWithdrawn(address token, uint256 amount) internal {
        _appendEventChain(LpTokensWithdrawn.selector, abi.encode(token, amount));
        emit LpTokensWithdrawn(token, amount);
    }
}

/// @title UntronController
/// @notice Receiver coordination contract for Untron protocol on Tron-like EVM chains.
/// @author Ultrasound Labs
contract UntronController is Multicallable, Create2Utils, UntronControllerIndex {
    /*//////////////////////////////////////////////////////////////
                                 STORAGE
    //////////////////////////////////////////////////////////////*/

    /// @notice Contract owner, can set executor and rebalancer configuration.
    /// @dev    Used by _onlyOwner (and thus the onlyOwner-protected admin functions).
    ///         Written in constructor and setOwner function.
    address public owner;

    /// @notice Executor, can transfer tokens from controller's balance to arbitrary recipients.
    /// @dev    Used by _onlyExecutor (and thus the onlyExecutor-protected external functions).
    ///         Written in setExecutor function.
    ///         This is a future-proof feature that can be used to implement a protocol
    ///         for swaps into Tron that would reuse liquidity from Untron V3's controller.
    address public executor;

    /// @notice Canonical accounting token (expected to be Tron USDT).
    /// @dev    All controller accounting, bridging, and executor transfers are done in this token.
    address public usdt;

    /// @notice LP address that provides USDT liquidity for swaps from non‑USDT tokens.
    /// @dev    Can be set and changed by the owner; swap configuration is controlled by the LP.
    address public lp;

    /// @notice rebalancer => rebalancer-specific payload for bridging USDT
    /// @dev    Only used in setPayload and bridgeUsdt functions.
    mapping(address => bytes) public payloadFor;

    /// @notice Tracks how much USDT was pulled (or swapped into) the controller and is available
    ///         for bridging or executor-controlled transfers.
    /// @dev    Increases in pullFromReceivers; decreases in bridgeUsdt and transferUsdtFromController.
    uint256 public pulledUsdt;

    /// @notice Per-token exchange rate configured by the LP, scaled by RATE_SCALE.
    /// @dev    For token with T decimals, rate = priceInUsdt * 10^T * RATE_SCALE.
    ///         token => scaled USDT-per-token rate; only used in pullFromReceivers and set by LP.
    mapping(address => uint256) public lpExchangeRateFor;

    /// @notice Fixed scale for exchange rates: USDT-per-tokenUnit is expressed per RATE_SCALE of token units.
    /// @dev For token with T decimals, rate = priceInUsdt * 10^T * RATE_SCALE.
    uint256 internal constant _RATE_SCALE = 1e18;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Error thrown when setting the owner to the zero address.
    /// @dev    Only used in setOwner function.
    error ZeroOwnerAddress();
    /// @notice Error thrown when a function restricted to the contract's owner is called by another address.
    /// @dev    Used by _onlyOwner (and thus the onlyOwner-protected admin functions).
    error OnlyOwner();
    /// @notice Error thrown when a function restricted to the executor is called by another address.
    /// @dev    Used by _onlyExecutor (and thus the onlyExecutor-protected external functions).
    error OnlyExecutor();
    /// @notice Error thrown when trying to bridge with an unset route/payload.
    /// @dev    Only used in bridgeUsdt function.
    error RouteNotSet();
    /// @notice Error thrown when the amount to be swept from receiver does not match the expected value.
    /// @dev    Only used in pullFromReceivers function.
    error IncorrectSweepAmount();
    /// @notice Error thrown when provided lengths of receiverSalts and amounts arrays do not match.
    /// @dev    Only used in pullFromReceivers function.
    error LengthMismatch();
    /// @notice Error thrown when the expected out amount does not match rebalancer-computed out amount.
    /// @dev    Only used in bridgeUsdt function.
    error OutAmountMismatch();
    /// @notice Error thrown when attempting to spend more than was pulled via receivers for a token.
    /// @dev    Used in bridgeUsdt, transferUsdtFromController, lpWithdrawTokens, and pullFromReceivers functions.
    error InsufficientPulledAmount();

    /// @notice Error thrown when a function restricted to the LP is called by another address.
    /// @dev    Used by _onlyLp (and thus LP-protected functions).
    error OnlyLp();

    /// @notice Error thrown when the calldata-provided exchange rate does not match the LP-configured rate.
    /// @dev    Only used in pullFromReceivers function for non‑USDT tokens.
    error ExchangeRateMismatch();

    /// @notice Error thrown when the LP does not have enough unaccounted USDT deposited
    ///         to buy swept non‑USDT tokens at the configured exchange rate.
    /// @dev    Only used in pullFromReceivers function for non‑USDT tokens.
    error InsufficientLpLiquidity();

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Modifier that restricts a function to be called only by the owner.
    /// @dev    Only used in setExecutor, setPayload, setOwner, setUsdt, and setLp functions.
    modifier onlyOwner() {
        _onlyOwner();
        _;
    }

    /// @notice Modifier that restricts a function to be called only by the executor.
    /// @dev    Only used in transferUsdtFromController function.
    modifier onlyExecutor() {
        _onlyExecutor();
        _;
    }

    /// @notice Modifier that restricts a function to be called only by the LP.
    /// @dev    Used in LP configuration and withdrawal functions.
    modifier onlyLp() {
        _onlyLp();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Constructor.
    /// @param create2Prefix Chain-specific byte prefix used in CREATE2 address calculation.
    ///                      For Tron deployments this should be 0x41; for standard EVM 0xff.
    /// @dev Initializes CREATE2 utils, sets the owner to the caller, and emits an OwnerChanged event.
    constructor(bytes1 create2Prefix) Create2Utils(create2Prefix) {
        owner = msg.sender;
        _emitOwnerChanged(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                             ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the executor.
    /// @param _executor New executor address.
    /// @dev Set to address(0) to disable executor functionality.
    ///      Callable only by the owner.
    function setExecutor(address _executor) external onlyOwner {
        executor = _executor;
        _emitExecutorChanged(_executor);
    }

    /// @notice Set the canonical accounting token (expected to be Tron USDT).
    /// @param _usdt New USDT token address (can be set to address(0) to disable bridging/accounting).
    /// @dev Callable only by the owner.
    function setUsdt(address _usdt) external onlyOwner {
        usdt = _usdt;
        _emitUsdtSet(_usdt);
    }

    /// @notice Set the LP address that provides USDT liquidity for swaps.
    /// @param _lp New LP address (can be set to address(0) to disable LP functionality).
    /// @dev Callable only by the owner.
    function setLp(address _lp) external onlyOwner {
        lp = _lp;
        _emitLpSet(_lp);
    }

    /// @notice Set the rebalancer payload for a particular USDT rebalancer.
    /// @param _rebalancer Rebalancer address.
    /// @param _payload Rebalancer-specific payload.
    /// @dev Callable only by the owner.
    function setPayload(address _rebalancer, bytes calldata _payload) external onlyOwner {
        payloadFor[_rebalancer] = _payload;
        _emitPayloadSet(_rebalancer, _payload);
    }

    /// @notice Set the owner of the contract.
    /// @param _newOwner New owner address.
    /// @dev Callable only by the owner. Zero address owner is disallowed.
    function setOwner(address _newOwner) external onlyOwner {
        if (_newOwner == address(0)) revert ZeroOwnerAddress();
        owner = _newOwner;
        _emitOwnerChanged(_newOwner);
    }

    /// @notice Set the LP-configured exchange rate for a token.
    /// @param token Token address.
    /// @param exchangeRate Scaled rate: USDT (smallest units) per RATE_SCALE token units.
    /// @dev Callable only by the LP.
    function setLpExchangeRate(address token, uint256 exchangeRate) external onlyLp {
        lpExchangeRateFor[token] = exchangeRate;
        _emitLpExchangeRateSet(token, exchangeRate);
    }

    /*//////////////////////////////////////////////////////////////
                          LP FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Withdraw tokens purchased by the LP from the controller.
    /// @param token Token address.
    /// @param amount Amount of tokens to withdraw.
    /// @dev Callable only by the LP. Does not allow withdrawing the canonical USDT accounting balance.
    function lpWithdrawTokens(address token, uint256 amount) external onlyLp {
        if (amount == 0) {
            return;
        }

        uint256 maxWithdraw;
        if (token == usdt) {
            // For USDT, protect canonical accounting balance: only the surplus
            // over pulledUsdt can be withdrawn by the LP.
            maxWithdraw = _maxWithdrawableUsdt();
        } else {
            // For non‑USDT tokens, LP can withdraw up to the full controller balance.
            maxWithdraw = TokenUtils.getBalanceOf(token, address(this));
        }

        if (amount > maxWithdraw) revert InsufficientPulledAmount();

        TokenUtils.transfer(token, payable(msg.sender), amount);
        _emitLpTokensWithdrawn(token, amount);
    }

    /*//////////////////////////////////////////////////////////////
                      PERMISSIONLESS FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Pulls tokens from multiple receiver contracts and swaps them into accounting token.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @param amounts Expected token amounts to be swept from each receiver.
    ///                amounts[i] must equal the actual sweep amount,
    ///                which is `balance - 1` if `balance > 0`, otherwise `0`.
    /// @param exchangeRate Scaled exchange rate: USDT (smallest units) per RATE_SCALE token units.
    ///                     Must match LP-configured rate when `token != usdt`. Ignored for USDT pulls.
    /// @dev Callable by anyone.
    ///      In this function, the controller only requests tokens to be sent into *its own balance*.
    ///      Sweeps all but one base unit from each non-zero-balance receiver,
    ///      in order to keep its balance slot non-zero for TRC-20 gas optimization.
    function pullFromReceivers(
        address token,
        bytes32[] calldata receiverSalts,
        uint256[] calldata amounts,
        uint256 exchangeRate
    ) external {
        if (receiverSalts.length != amounts.length) revert LengthMismatch();

        bool isUsdt = token == usdt;
        uint256 rateUsed;

        if (isUsdt) {
            rateUsed = _RATE_SCALE;
        } else {
            uint256 configuredRate = lpExchangeRateFor[token];
            if (configuredRate == 0 || configuredRate != exchangeRate) revert ExchangeRateMismatch();
            rateUsed = configuredRate;
        }

        uint256 totalToken = 0;
        uint256 totalUsdt = 0;
        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            address receiver = predictReceiverAddress(receiverSalts[i]);
            uint256 balance = TokenUtils.getBalanceOf(token, receiver);

            uint256 sweepAmount = balance;
            if (balance > 0) {
                unchecked {
                    // Sweep all but one base unit to keep the receiver's balance slot non-zero.
                    //
                    // Sending a TRC-20 token to a Tron address which already has some is ~2x cheaper
                    // than sending them to an empty balance slot (65k vs 130k energy for Tron USDT
                    // at the time of writing).
                    //
                    // This is a minor optimization that doesn't change the protocol's
                    // correctness or security.
                    sweepAmount -= 1;
                }
            }

            // Verify the expected amount of tokens to be swept matches the actual amount.
            // — why is this explicit in the calldata?
            // in Tron light client, we can only prove success flag + tx calldata,
            // so we have to pass things we need to prove there explicitly in calldata
            // and revert if it's not equivalent to what's happening in the blockchain state.
            if (amounts[i] != sweepAmount) revert IncorrectSweepAmount();

            if (sweepAmount != 0) {
                _pullFromReceiver(receiverSalts[i], token, sweepAmount);

                uint256 usdtAmount;
                if (isUsdt) {
                    usdtAmount = sweepAmount;
                } else {
                    usdtAmount = TokenUtils.mulDiv(sweepAmount, rateUsed, _RATE_SCALE);
                }
                totalToken += sweepAmount;
                totalUsdt += usdtAmount;

                // we're not interested in logging zero amount pulls
                // and they'd make the event chain system kinda vulnerable to spam of PulledFromReceiver events
                // so the event is only emitted if the call did indeed pull something
                _emitPulledFromReceiver(receiverSalts[i], token, sweepAmount, rateUsed, usdtAmount);
            }
        }

        if (totalToken != 0) {
            if (isUsdt) {
                // Canonical USDT: pulled amount directly increases accounting balance.
                pulledUsdt += totalUsdt;
            } else {
                // Non‑USDT tokens are immediately swapped into USDT against the LP at the
                // LP-configured exchange rate, provided there is enough USDT liquidity.

                // Compute LP's free USDT liquidity as controller's USDT balance minus
                // already-accounted pulledUsdt.
                uint256 lpFreeUsdt = _maxWithdrawableUsdt();
                if (totalUsdt > lpFreeUsdt) revert InsufficientLpLiquidity();

                // Increase canonical USDT accounting.
                pulledUsdt += totalUsdt;
            }
        }
    }

    /// @notice Bridges specified amount of USDT via the provided rebalancer using stored payload.
    /// @param rebalancer Rebalancer address.
    /// @param inAmount Amount of tokens to bridge.
    /// @param outAmount Expected output amount returned by the rebalancer implementation.
    /// @dev Callable by anyone; uses tokens already held by the controller
    ///      (including TRX value attached to the call, if any).
    ///      Rebalancers are DELEGATECALLed in the controller's context
    ///      and are thus strongly encouraged to be stateless.
    function bridgeUsdt(address rebalancer, uint256 inAmount, uint256 outAmount) external payable {
        // Load payload for this rebalancer
        bytes memory payload = payloadFor[rebalancer];
        if (payload.length == 0) revert RouteNotSet();

        // Enforce accounting: only amounts previously pulled via receivers / LP swaps can be rebalanced
        _enforceAccounting(inAmount);

        // If the caller attached value, keep it in the controller;
        // the underlying rebalancer will be able to use it to pay for the bridge call.

        // Execute the rebalancer via DELEGATECALL.
        // The rebalancer implementation returns the expected out amount, which we
        // compare against the caller-provided value to enforce invariants at
        // the controller layer.
        bytes memory data = abi.encodeWithSelector(IRebalancer.rebalance.selector, usdt, inAmount, payload);
        (bool ok, bytes memory ret) = rebalancer.delegatecall(data);
        if (!ok) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }

        // Verify the expected amount of tokens to be received from the bridge
        // matches the actual amount asserted by the bridge used.
        // — why is this explicit in the calldata?
        // in Tron light client, we can only prove success flag + tx calldata,
        // so we have to pass things we need to prove there explicitly in calldata
        // and revert if it's not equivalent to what's happening in the blockchain state.
        uint256 rebalancerOutAmount = abi.decode(ret, (uint256));
        if (rebalancerOutAmount != outAmount) revert OutAmountMismatch();

        _emitUsdtRebalanced(inAmount, outAmount, rebalancer);
    }

    /// @notice Accepts native token for bridging fees.
    receive() external payable {}

    /*//////////////////////////////////////////////////////////////
                          EXECUTOR FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Transfers USDT from the controller to a specified recipient.
    /// @param recipient Recipient address.
    /// @param amount Amount of tokens to transfer.
    /// @dev Callable only by the executor.
    function transferUsdtFromController(address recipient, uint256 amount) external onlyExecutor {
        _enforceAccounting(amount);
        TokenUtils.transfer(usdt, payable(recipient), amount);
        _emitControllerUsdtTransfer(recipient, amount);
    }

    /*//////////////////////////////////////////////////////////////
                           INTERNAL  HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Deploys receiver contract if missing and performs pull from receiver.
    /// @param salt Salt used for deterministic receiver deployment (CREATE2).
    ///             Receiver salts are used as a canonical identifier for receivers.
    /// @param token Token address.
    /// @param amount Amount of tokens to pull from receiver.
    /// @dev DOES NOT call _emitPulledFromReceiver, the calling function must emit it
    function _pullFromReceiver(bytes32 salt, address token, uint256 amount) internal {
        address payable receiver = payable(predictReceiverAddress(salt));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            receiver = deployReceiver(salt);
            _emitReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).pull(token, amount);
    }

    /// @notice Enforces accounting for USDT spending from controller balance.
    /// @param amount Amount of USDT to spend.
    function _enforceAccounting(uint256 amount) internal {
        uint256 pulled = pulledUsdt;
        if (amount > pulled) revert InsufficientPulledAmount();
        unchecked {
            pulledUsdt = pulled - amount;
        }
    }

    /// @notice Computes USDT that can be spent without violating accounting invariants.
    /// @return Amount of USDT that can be withdrawn/spent without dipping into accounted `pulledUsdt`.
    function _maxWithdrawableUsdt() internal view returns (uint256) {
        uint256 controllerUsdtBalance = TokenUtils.getBalanceOf(usdt, address(this));
        if (controllerUsdtBalance < pulledUsdt) revert InsufficientPulledAmount();
        return controllerUsdtBalance - pulledUsdt;
    }

    /// @notice Reverts if the caller is not the owner.
    /// @dev    Used in an onlyOwner modifier only.
    function _onlyOwner() internal view {
        if (msg.sender != owner) revert OnlyOwner();
    }

    /// @notice Reverts if the caller is not the executor.
    /// @dev    Used in an onlyExecutor modifier only.
    function _onlyExecutor() internal view {
        if (msg.sender != executor) revert OnlyExecutor();
    }

    /// @notice Reverts if the caller is not the LP.
    /// @dev    Used in an onlyLp modifier only.
    function _onlyLp() internal view {
        if (msg.sender != lp) revert OnlyLp();
    }
}
