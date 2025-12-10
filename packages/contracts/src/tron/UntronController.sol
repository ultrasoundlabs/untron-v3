// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronReceiver} from "./UntronReceiver.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";
import {IBridger} from "./bridgers/interfaces/IBridger.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";
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
    ///         The hashed string that you see is the genesis hash. No events go before it.
    bytes32 public eventChainTip = sha256(
        "UntronControllerIndex\nJustin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People."
    );

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the owner of the contract is changed.
    /// @dev    Only used in setOwner function and emitted in constructor on deployment.
    event OwnerChanged(address indexed newOwner);
    /// @notice Emitted when the executor of the contract is changed.
    /// @dev    Only used in setExecutor function.
    event ExecutorChanged(address indexed newExecutor);
    /// @notice Emitted when a payload is set for a particular bridger.
    /// @dev    Only used in setPayload function.
    event PayloadSet(address indexed bridger, bytes payload);
    /// @notice Emitted when a receiver is deployed.
    /// @dev    Only used in _callReceiver function.
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    /// @notice Emitted when a receiver is called to transfer tokens to a recipient.
    /// @dev    Only used in _callReceiver function.
    event ReceiverCalled(address indexed receiver, address indexed token, address indexed recipient, uint256 amount);
    /// @notice Emitted when tokens are dumped from receivers to the controller.
    /// @dev    Only used in pullFromReceivers function.
    event TokensPulled(address indexed token, uint256 totalAmount);
    /// @notice Emitted when USDT is bridged via a particular bridger.
    /// @dev    Only used in bridgeUsdt function.
    event UsdtBridged(uint256 inAmount, uint256 outAmount, address indexed bridger);
    /// @notice Emitted when USDT is transferred from the controller to a recipient.
    /// @dev    Only used in transferUsdtFromController function.
    event ControllerUsdtTransfer(address indexed recipient, uint256 amount);
    /// @notice Emitted when the canonical USDT token is set or updated.
    /// @dev    Only used in setUsdt function.
    event UsdtSet(address indexed newUsdt);
    /// @notice Emitted when the LP address is set or updated.
    /// @dev    Only used in setLp function.
    event LpSet(address indexed newLp);
    /// @notice Emitted when the LP updates the exchange rate for a token.
    /// @dev    Only used in setLpExchangeRate function.
    event LpExchangeRateSet(address indexed token, uint256 exchangeRate);
    /// @notice Emitted when the LP withdraws purchased tokens from the controller.
    /// @dev    Only used in lpWithdrawTokens function.
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
    function isEventChainTip(bytes32 eventChainTip_) public view returns (bool) {
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
        eventChainTip = sha256(abi.encodePacked(eventChainTip, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    function _emitOwnerChanged(address newOwner) internal {
        _appendEventChain(OwnerChanged.selector, abi.encode(newOwner));
        emit OwnerChanged(newOwner);
    }

    function _emitExecutorChanged(address newExecutor) internal {
        _appendEventChain(ExecutorChanged.selector, abi.encode(newExecutor));
        emit ExecutorChanged(newExecutor);
    }

    function _emitPayloadSet(address bridger, bytes calldata payload) internal {
        _appendEventChain(PayloadSet.selector, abi.encode(bridger, payload));
        emit PayloadSet(bridger, payload);
    }

    function _emitReceiverDeployed(address receiver, bytes32 salt) internal {
        _appendEventChain(ReceiverDeployed.selector, abi.encode(receiver, salt));
        emit ReceiverDeployed(receiver, salt);
    }

    function _emitReceiverCalled(address receiver, address token, address recipient, uint256 amount) internal {
        _appendEventChain(ReceiverCalled.selector, abi.encode(receiver, token, recipient, amount));
        emit ReceiverCalled(receiver, token, recipient, amount);
    }

    function _emitTokensPulled(address token, uint256 totalAmount) internal {
        _appendEventChain(TokensPulled.selector, abi.encode(token, totalAmount));
        emit TokensPulled(token, totalAmount);
    }

    function _emitUsdtBridged(uint256 inAmount, uint256 outAmount, address bridger) internal {
        _appendEventChain(UsdtBridged.selector, abi.encode(inAmount, outAmount, bridger));
        emit UsdtBridged(inAmount, outAmount, bridger);
    }

    function _emitControllerUsdtTransfer(address recipient, uint256 amount) internal {
        _appendEventChain(ControllerUsdtTransfer.selector, abi.encode(recipient, amount));
        emit ControllerUsdtTransfer(recipient, amount);
    }

    function _emitUsdtSet(address newUsdt) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(newUsdt));
        emit UsdtSet(newUsdt);
    }

    function _emitLpSet(address newLp) internal {
        _appendEventChain(LpSet.selector, abi.encode(newLp));
        emit LpSet(newLp);
    }

    function _emitLpExchangeRateSet(address token, uint256 exchangeRate) internal {
        _appendEventChain(LpExchangeRateSet.selector, abi.encode(token, exchangeRate));
        emit LpExchangeRateSet(token, exchangeRate);
    }

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

    /// @notice Contract owner, can set executor and bridger configuration.
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

    /// @notice bridger => bridger-specific payload for bridging USDT
    /// @dev    Only used in setPayload and bridgeUsdt functions.
    mapping(address => bytes) public payloadFor;

    /// @notice Tracks how much USDT was pulled (or swapped into) the controller and is available
    ///         for bridging or executor-controlled transfers.
    /// @dev    Increases in pullFromReceivers; decreases in bridgeUsdt and transferUsdtFromController.
    uint256 public pulledUsdt;

    /// @notice Per-token exchange rate configured by the LP: how much USDT (in smallest units)
    ///         the LP is willing to pay per 1 smallest unit of `token`.
    /// @dev    token => USDT-per-token rate; only used in pullFromReceivers and set by LP.
    mapping(address => uint256) public lpExchangeRateFor;

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
    /// @notice Error thrown when the expected out amount does not match bridger-computed out amount.
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

    /// @notice Set the bridger payload for a particular USDT bridger.
    /// @param _bridger Bridger address.
    /// @param _payload Bridger-specific payload.
    /// @dev Callable only by the owner.
    function setPayload(address _bridger, bytes calldata _payload) external onlyOwner {
        payloadFor[_bridger] = _payload;
        _emitPayloadSet(_bridger, _payload);
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
    /// @param exchangeRate USDT amount (in smallest units) the LP is willing to pay
    ///                     per 1 smallest unit of `token`.
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
            uint256 controllerUsdtBalance = TokenUtils.getBalanceOf(usdt, address(this));
            if (controllerUsdtBalance < pulledUsdt) revert InsufficientPulledAmount();
            maxWithdraw = controllerUsdtBalance - pulledUsdt;
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
    /// @param exchangeRate USDT amount (in smallest units) that the LP is expected to pay per
    ///                     1 smallest unit of `token` for this pull. Must match LP-configured rate
    ///                     when `token != usdt`. WARNING: not used when token == usdt
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

        uint256 total = 0;
        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            address receiver = predictReceiverAddress(address(this), receiverSalts[i]);
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
                _callReceiver(receiverSalts[i], token, sweepAmount, address(this));
                total += sweepAmount;
            }
        }

        if (total != 0) {
            if (token == usdt) {
                // Canonical USDT: pulled amount directly increases accounting balance.
                pulledUsdt += total;
            } else {
                // Non‑USDT tokens are immediately swapped into USDT against the LP at the
                // LP-configured exchange rate, provided there is enough USDT liquidity.

                uint256 configuredRate = lpExchangeRateFor[token];
                if (configuredRate == 0 || configuredRate != exchangeRate) revert ExchangeRateMismatch();

                // USDT required from LP to buy `total` units of `token`.
                uint256 usdtRequired = total * exchangeRate;

                // Compute LP's free USDT liquidity as controller's USDT balance minus
                // already-accounted pulledUsdt.
                uint256 controllerUsdtBalance = TokenUtils.getBalanceOf(usdt, address(this));
                if (controllerUsdtBalance < pulledUsdt) revert InsufficientPulledAmount();
                uint256 lpFreeUsdt = controllerUsdtBalance - pulledUsdt;
                if (usdtRequired > lpFreeUsdt) revert InsufficientLpLiquidity();

                // Increase canonical USDT accounting.
                pulledUsdt += usdtRequired;
            }

            // we're not interested in logging zero amount pulls
            // and they'd make the event chain system kinda vulnerable to spam of TokensPulled events
            // so the event is only emitted if the call did indeed pull something
            _emitTokensPulled(token, total);
        }
    }

    /// @notice Bridges specified amount of USDT via the provided bridger using stored payload.
    /// @param bridger Bridger address.
    /// @param inAmount Amount of tokens to bridge.
    /// @param outAmount Expected output amount returned by the bridger implementation.
    /// @dev Callable by anyone; uses tokens already held by the controller
    ///      (including TRX value attached to the call, if any).
    ///      Bridgers are DELEGATECALLed in the controller's context
    ///      and are thus strongly encouraged to be stateless.
    function bridgeUsdt(address bridger, uint256 inAmount, uint256 outAmount) external payable {
        // Load payload for this bridger
        bytes memory payload = payloadFor[bridger];
        if (payload.length == 0) revert RouteNotSet();

        // Enforce accounting: only amounts previously pulled via receivers / LP swaps can be bridged
        _enforceAccounting(inAmount);

        // If the caller attached value, keep it in the controller;
        // the underlying bridger will be able to use it to pay for the bridge call.

        // Execute the bridger via DELEGATECALL.
        // The bridger implementation returns the expected out amount, which we
        // compare against the caller-provided value to enforce invariants at
        // the controller layer.
        bytes memory data = abi.encodeWithSelector(IBridger.bridge.selector, usdt, inAmount, payload);
        (bool ok, bytes memory ret) = bridger.delegatecall(data);
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
        uint256 bridgerOutAmount = abi.decode(ret, (uint256));
        if (bridgerOutAmount != outAmount) revert OutAmountMismatch();

        _emitUsdtBridged(inAmount, outAmount, bridger);
    }

    // Accept native token for bridging fees
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
    /// @param recipient Recipient address.
    function _callReceiver(bytes32 salt, address token, uint256 amount, address recipient) internal {
        address payable receiver = payable(predictReceiverAddress(address(this), salt));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            receiver = deployReceiver(salt);
            _emitReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).onControllerCall(token, amount, payable(recipient));

        _emitReceiverCalled(receiver, token, recipient, amount);
    }

    /// @dev Enforces accounting for token transfers in the canonical accounting token (USDT).
    /// @param amount Amount of tokens to transfer.
    function _enforceAccounting(uint256 amount) internal {
        uint256 pulled = pulledUsdt;
        if (amount > pulled) revert InsufficientPulledAmount();
        unchecked {
            pulledUsdt = pulled - amount;
        }
    }

    /// @dev Reverts if the caller is not the owner.
    ///      Used in an onlyOwner modifier only.
    function _onlyOwner() internal view {
        if (msg.sender != owner) revert OnlyOwner();
    }

    /// @dev Reverts if the caller is not the executor.
    ///      Used in an onlyExecutor modifier only.
    function _onlyExecutor() internal view {
        if (msg.sender != executor) revert OnlyExecutor();
    }

    /// @dev Reverts if the caller is not the LP.
    ///      Used in an onlyLp modifier only.
    function _onlyLp() internal view {
        if (msg.sender != lp) revert OnlyLp();
    }
}
