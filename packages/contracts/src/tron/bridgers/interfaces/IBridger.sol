// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title IBridger
/// @notice Minimal interface for pluggable bridge implementations.
/// @dev Each bridger handles token bridging for a specific bridge protocol.
interface IBridger {
    /// @notice Bridge `inAmount` of `token` into `outAmount` according to `payload`.
    /// @dev MUST revert on failure. The payload format is bridger-specific.
    /// @param token Token address to bridge.
    /// @param inAmount Amount to bridge.
    /// @param outAmount Expected amount of tokens to be bridged.
    /// @param payload Bridger-specific encoded parameters.
    function bridge(address token, uint256 inAmount, uint256 outAmount, bytes calldata payload) external;
}
