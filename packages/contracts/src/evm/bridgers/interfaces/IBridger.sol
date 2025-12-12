// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Adapter interface for cross-chain payouts.
/// @dev UntronV3 uses this as a plug-in point for bridging. When a claim’s route is `Bridge`,
///      UntronV3 will first `transfer(token, bridger, amount)` and then call `bridge(...)`.
///      Implementations MUST assume they already custody `amount` of `token` on the current chain,
///      and SHOULD initiate a bridge to `targetChainId` that delivers funds to `beneficiary` on
///      the destination chain. UntronV3 does not verify delivery; correctness/trust is delegated
///      to the configured bridger implementation.
interface IBridger {
    /// @notice Bridge `amount` of `token` to `beneficiary` on `targetChainId`.
    /// @dev Called by UntronV3 after transferring `amount` of `token` to this contract.
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external;
}
