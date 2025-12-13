// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title IRebalancer
/// @notice Minimal interface for pluggable bridge implementations.
/// @dev Each rebalancer handles token bridging for a specific bridge protocol.
/// @author Ultrasound Labs
interface IRebalancer {
    /// @notice Bridge `inAmount` of `token` according to `payload`, returning the expected out amount.
    /// @dev MUST revert on failure. The payload format is rebalancer-specific.
    ///      Implementations MUST return the amount that will be (or is expected to be)
    ///      received on the destination chain, so that the caller can enforce invariants.
    /// @param token Token address to bridge.
    /// @param inAmount Amount to bridge.
    /// @param payload Rebalancer-specific encoded parameters.
    /// @return outAmount Expected amount of tokens to be rebalanced.
    function rebalance(address token, uint256 inAmount, bytes calldata payload) external returns (uint256 outAmount);
}
