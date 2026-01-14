// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IRebalancer} from "./interfaces/IRebalancer.sol";
import {TronTokenUtils} from "../../utils/TronTokenUtils.sol";

/// @title NearIntentsRebalancer
/// @notice Rebalancer implementation for the receiver-based NEAR Intents bridge.
/// @author Ultrasound Labs
contract NearIntentsRebalancer is IRebalancer {
    /// @notice Thrown when the provided pool address is zero.
    error InvalidPool();

    /// @notice Bridge tokens via NEAR Intents.
    /// @dev Payload must be ABI-encoded: (address payable pool)
    ///      Runs via DELEGATECALL in the controller context; controller holds funds.
    /// @param token USDT contract address. The rebalancer only works with USDT.
    /// @param inAmount Amount to bridge.
    /// @param payload ABI-encoded (address payable pool).
    /// @return outAmount Expected amount of tokens to be rebalanced. Assumes 0 bps fee.
    function rebalance(address token, uint256 inAmount, bytes calldata payload)
        external
        payable
        returns (uint256 outAmount)
    {
        address pool_ = abi.decode(payload, (address));
        if (pool_ == address(0)) revert InvalidPool();
        address payable pool = payable(pool_);

        TronTokenUtils.transfer(token, pool, inAmount);
        outAmount = inAmount;
    }
}
