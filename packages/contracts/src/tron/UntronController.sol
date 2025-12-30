// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronControllerIndex} from "./UntronControllerIndex.sol";
import {UntronReceiver} from "./UntronReceiver.sol";
import {IRebalancer} from "./rebalancers/interfaces/IRebalancer.sol";
import {ReceiverDeployer} from "../utils/ReceiverDeployer.sol";
import {Multicallable} from "solady/utils/Multicallable.sol";
import {TronTokenUtils} from "../utils/TronTokenUtils.sol";

/// @title UntronController
/// @notice Receiver coordination contract for Untron protocol on Tron-like EVM chains.
/// @dev Token operations on Tron are performed via a TRC-20-specific `TronTokenUtils` library
///      (see `packages/contracts/src/utils/TronTokenUtils.sol`).
/// @author Ultrasound Labs
contract UntronController is Multicallable, ReceiverDeployer, UntronControllerIndex {
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
    /// @dev    Only used in setPayload and rebalanceUsdt functions.
    mapping(address => bytes) public payloadFor;

    /// @notice Tracks how much USDT was pulled (or swapped into) the controller and is available
    ///         for bridging or executor-controlled transfers.
    /// @dev    Increases in pullFromReceivers; decreases in rebalanceUsdt and transferUsdtFromController.
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
    /// @dev    Only used in rebalanceUsdt function.
    error RouteNotSet();
    /// @notice Error thrown when the expected out amount does not match rebalancer-computed out amount.
    /// @dev    Only used in rebalanceUsdt function.
    error OutAmountMismatch();
    /// @notice Error thrown when attempting to spend more than was pulled via receivers for a token.
    /// @dev    Used in rebalanceUsdt, transferUsdtFromController, lpWithdrawTokens, and pullFromReceivers functions.
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
    constructor(bytes1 create2Prefix) ReceiverDeployer(create2Prefix) {
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

    /// @notice Approve USDT for spending by a spender.
    /// @param spender Address of the spender.
    /// @param amount Amount of USDT to approve.
    /// @dev Callable only by the owner.
    function approveUsdt(address spender, uint256 amount) external onlyOwner {
        TronTokenUtils.approve(usdt, spender, amount);
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
            maxWithdraw = TronTokenUtils.getBalanceOf(token, address(this));
        }

        if (amount > maxWithdraw) revert InsufficientPulledAmount();

        TronTokenUtils.transfer(token, payable(msg.sender), amount);
        _emitLpTokensWithdrawn(token, amount);
    }

    /*//////////////////////////////////////////////////////////////
                      PERMISSIONLESS FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Pulls tokens from multiple receiver contracts and swaps them into accounting token.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @dev Callable by anyone.
    ///      In this function, the controller only requests tokens to be sent into *its own balance*.
    ///      Sweeps all but one base unit from each non-zero-balance receiver,
    ///      in order to keep its balance slot non-zero for TRC-20 gas optimization.
    function pullFromReceivers(address token, bytes32[] calldata receiverSalts) external {
        bool isUsdt = token == usdt;
        uint256 rateUsed;

        if (isUsdt) {
            rateUsed = _RATE_SCALE;
        } else {
            uint256 configuredRate = lpExchangeRateFor[token];
            if (configuredRate == 0) revert ExchangeRateMismatch();
            rateUsed = configuredRate;
        }

        uint256 totalToken = 0;
        uint256 totalUsdt = 0;
        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            bytes32 receiverSalt = receiverSalts[i];
            uint256 sweepAmount = _computeSweepAmount(token, receiverSalt);

            if (sweepAmount != 0) {
                _pullFromReceiver(receiverSalt, token, sweepAmount);

                uint256 usdtAmount;
                if (isUsdt) {
                    usdtAmount = sweepAmount;
                } else {
                    usdtAmount = TronTokenUtils.mulDiv(sweepAmount, rateUsed, _RATE_SCALE);
                }
                totalToken += sweepAmount;
                totalUsdt += usdtAmount;

                // we're not interested in logging zero amount pulls
                // and they'd make the event chain system kinda vulnerable to spam of PulledFromReceiver events
                // so the event is only emitted if the call did indeed pull something
                _emitPulledFromReceiver(receiverSalt, token, sweepAmount, rateUsed, usdtAmount);
            }
        }

        if (totalToken != 0) {
            if (isUsdt) {
                // Canonical USDT: pulled amount directly increases accounting balance.
                pulledUsdt += totalUsdt;
            } else {
                // Non‑USDT tokens are immediately swapped into USDT against the LP at the
                // LP-configured exchange rate, provided there is enough USDT liquidity.
                uint256 lpFreeUsdt = _maxWithdrawableUsdt();
                if (totalUsdt > lpFreeUsdt) revert InsufficientLpLiquidity();

                // Increase canonical USDT accounting.
                pulledUsdt += totalUsdt;
            }
        }
    }

    /// @notice Computes the amount of tokens to sweep from the given token to the receiver.
    /// @param token The token to sweep.
    /// @param receiverSalt The salt used to predict the receiver address.
    /// @return sweepAmount The amount of tokens to sweep.
    function _computeSweepAmount(address token, bytes32 receiverSalt) private view returns (uint256 sweepAmount) {
        address receiver = predictReceiverAddress(receiverSalt);
        sweepAmount = TronTokenUtils.getBalanceOf(token, receiver);
        if (sweepAmount > 0) {
            unchecked {
                // Sweep all but one base unit to keep the receiver's balance slot non-zero.
                //
                // Sending a TRC-20 token to a Tron address which already has some is ~2x cheaper
                // than sending them to an empty balance slot (65k vs 130k energy for Tron USDT
                // at the time of writing).
                //
                // This is a minor optimization that doesn't change the protocol's
                // correctness or security.
                --sweepAmount;
            }
        }
    }

    /// @notice Bridges specified amount of USDT via the provided rebalancer using stored payload.
    /// @param rebalancer Rebalancer address.
    /// @param inAmount Amount of tokens to bridge.
    /// @dev Callable by anyone; uses tokens already held by the controller
    ///      (including TRX value attached to the call, if any).
    ///      Rebalancers are DELEGATECALLed in the controller's context
    ///      and are thus strongly encouraged to be stateless.
    function rebalanceUsdt(address rebalancer, uint256 inAmount) external payable {
        // Load payload for this rebalancer
        bytes memory payload = payloadFor[rebalancer];
        if (payload.length == 0) revert RouteNotSet();

        // Enforce accounting: only amounts previously pulled via receivers / LP swaps can be rebalanced
        _enforceAccounting(inAmount);

        // If the caller attached value, keep it in the controller;
        // the underlying rebalancer will be able to use it to pay for the bridge call.

        // Execute the rebalancer via DELEGATECALL.
        bytes memory data = abi.encodeWithSelector(IRebalancer.rebalance.selector, usdt, inAmount, payload);

        // In UntronController, rebalancers are specified by the owner (admin), thus are trusted.
        // (See first line, where if the payload for this rebalancer is not specified, we revert with RouteNotSet).
        // One of owner's responsibilities is to ensure that the rebalancer address is correct and secure,
        // and that the rebalancer implementation is stateless and can't perform malicious actions
        // no matter what input was given by the permissionless relayer.
        // A good implementation example of such rebalancer is LegacyMeshRebalancer,
        // that uses no state and operates with exactly inAmount of USDT in a trusted OFT contract
        // defined in owner-specified payload.
        /* solhint-disable avoid-low-level-calls */
        // slither-disable-next-line controlled-delegatecall
        (bool ok, bytes memory ret) = rebalancer.delegatecall(data);
        /* solhint-enable avoid-low-level-calls */
        if (!ok) {
            // solhint-disable-next-line no-inline-assembly
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }

        uint256 rebalancerOutAmount = abi.decode(ret, (uint256));

        _emitUsdtRebalanced(inAmount, rebalancerOutAmount, rebalancer);
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
        TronTokenUtils.transfer(usdt, payable(recipient), amount);
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
        uint256 controllerUsdtBalance = TronTokenUtils.getBalanceOf(usdt, address(this));
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
