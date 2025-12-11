// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ReentrancyGuard} from "solady/utils/ReentrancyGuard.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";

// things below are defined outside the contract so parents can use them

/// @notice Represents a low-level contract call used during swap execution.
/// @dev The call is executed with `call.value` native tokens and `call.data`
/// forwarded to the target address `to`.
struct Call {
    /// @notice Address of the contract to call.
    address to;
    /// @notice Native token amount (e.g. ETH) to forward with the call, or 0.
    uint256 value;
    /// @notice Calldata to send to the target contract.
    bytes data;
}

/// @notice Thrown when the owner address attempts to call a restricted function.
/// @dev Used to prevent the `OWNER` from calling `execute`.
error OnlyOwner();
/// @notice Thrown when one of the low-level calls in `execute` fails.
/// @param callIndex Index of the call in the `calls` array that failed.
error CallFailed(uint256 callIndex);
/// @notice Thrown when the resulting token balance is less than the expected output amount.
/// @dev This guards against incomplete or unfavorable swaps.
error InsufficientOutput();

/// @title SwapExecutor
/// @notice Executes a sequence of arbitrary contract calls and settles the resulting token balance.
/// @dev
/// - Intended to be deployed and called via the main UntronV3 contract.
/// - Executes calls using `call` and enforces a minimum output amount for a single ERC-20 token.
/// Inspired by Daimo Pay's `DaimoPayExecutor`.
/// @author Ultrasound Labs
contract SwapExecutor is ReentrancyGuard {
    /// @notice Immutable address representing the owner/controller of this executor.
    /// @dev This address is not allowed to call `execute` directly.
    address public immutable OWNER;

    /// @notice Initializes the SwapExecutor with an immutable owner address.
    /// @param owner Address treated as the owner/controller of this executor.
    constructor(address owner) {
        OWNER = owner;
    }

    /// @notice Executes a batch of arbitrary calls and settles token outputs.
    /// @dev
    /// - Reverts with {OnlyOwner} if `msg.sender` equals `OWNER`.
    /// - Reverts with {CallFailed} if any underlying call fails.
    /// - Reverts with {InsufficientOutput} if the post-call token balance is less than `expectedAmount`.
    /// - Uses `TokenUtils` to safely transfer the expected and surplus token amounts.
    /// @param calls Array of low-level calls that will be executed in order.
    /// @param token Address of the ERC-20 token whose balance is checked and distributed.
    /// @param expectedAmount Minimum amount of `token` that must be present after executing `calls`.
    /// @param recipient Address receiving exactly `expectedAmount` of `token`.
    /// @param surplusRecipient Address receiving any surplus `token` balance above `expectedAmount`.
    function execute(
        Call[] calldata calls,
        address token,
        uint256 expectedAmount,
        address payable recipient,
        address payable surplusRecipient
    ) external nonReentrant {
        if (msg.sender != OWNER) revert OnlyOwner();

        // Execute provided calls.
        uint256 callsLength = calls.length;
        for (uint256 i = 0; i < callsLength; ++i) {
            Call calldata call = calls[i];
            (bool success,) = call.to.call{value: call.value}(call.data);
            if (!success) revert CallFailed(i);
        }

        /// Check that at least one of the expectedOutput tokens is present
        /// with enough balance.
        uint256 balance = TokenUtils.getBalanceOf(token, address(this));
        if (balance < expectedAmount) revert InsufficientOutput();

        // Transfer the expected amount of the token to the recipient.
        TokenUtils.transfer({token: token, recipient: recipient, amount: expectedAmount});

        // Transfer any surplus tokens to the surplus recipient.
        TokenUtils.transfer({token: token, recipient: surplusRecipient, amount: balance - expectedAmount});
    }

    /// @notice Accepts native token (e.g. ETH) deposits used by swap calls.
    /// @dev This function enables the executor to receive ETH for subsequent low-level calls.
    receive() external payable {}
}
