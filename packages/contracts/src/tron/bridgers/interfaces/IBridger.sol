// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title IBridger
/// @notice Minimal interface for pluggable bridge implementations.
/// @dev Each bridger handles token bridging for a specific bridge protocol.
interface IBridger {
    /// @notice Bridge `amount` of `token` according to `payload`.
    /// @dev MUST revert on failure. The payload format is bridger-specific.
    /// @param token Token address to bridge.
    /// @param amount Amount to bridge.
    /// @param payload Bridger-specific encoded parameters.
    /// @return bridgerReceipt Opaque receipt that caller can log/ignore.
    function bridge(address token, uint256 amount, bytes calldata payload)
        external
        returns (bytes memory bridgerReceipt);

    /// @notice Quote the native fee required for the bridge call.
    /// @dev Bridgers that do not require a native fee MUST return 0.
    /// @param token Token address to bridge.
    /// @param amount Amount to bridge.
    /// @param payload Bridger-specific encoded parameters.
    /// @return nativeFee Native token fee required.
    function quoteFee(address token, uint256 amount, bytes calldata payload) external view returns (uint256 nativeFee);
}

