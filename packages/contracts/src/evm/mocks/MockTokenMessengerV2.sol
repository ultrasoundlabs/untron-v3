// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

/// @title IERC20Burnable
/// @notice Minimal interface for ERC20 tokens that support `burn(address,uint256)`.
/// @author Ultrasound Labs
interface IERC20Burnable {
    /// @notice Burns `amount` tokens from `from`.
    /// @param from Address to burn from.
    /// @param amount Amount to burn.
    function burn(address from, uint256 amount) external;
}

/// @title MockTokenMessengerV2
/// @notice Minimal mock for Circle CCTP V2 TokenMessengerV2.
/// @dev Pulls `burnToken` from caller and optionally burns it if the token supports `burn(address,uint256)`.
/// @author Ultrasound Labs
contract MockTokenMessengerV2 {
    /// @notice Emitted after successfully debiting tokens in `depositForBurn`.
    /// @param caller Original caller.
    /// @param burnToken Token debited.
    /// @param amount Amount debited.
    /// @param destinationDomain Destination domain (as in CCTP).
    /// @param mintRecipient Recipient on the destination domain.
    event DepositForBurn(
        address indexed caller,
        address indexed burnToken,
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient
    );

    error MockTokenMessengerV2_TransferFromFailed();

    /// @notice Debits `burnToken` from the caller and (best-effort) burns it.
    /// @param amount Amount to debit.
    /// @param destinationDomain Destination domain.
    /// @param mintRecipient Recipient on destination.
    /// @param burnToken Token address.
    /// @param destinationCaller Unused.
    /// @param maxFee Unused.
    /// @param minFinalityThreshold Unused.
    function depositForBurn(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient,
        address burnToken,
        bytes32 destinationCaller,
        uint256 maxFee,
        uint32 minFinalityThreshold
    ) external {
        destinationCaller;
        maxFee;
        minFinalityThreshold;

        bool okTf = IERC20(burnToken).transferFrom(msg.sender, address(this), amount);
        if (!okTf) revert MockTokenMessengerV2_TransferFromFailed();

        // Best-effort burn if supported.
        bool burned;
        try IERC20Burnable(burnToken).burn(address(this), amount) {
            burned = true;
        } catch {
            burned = false;
        }
        burned;

        emit DepositForBurn(msg.sender, burnToken, amount, destinationDomain, mintRecipient);
    }
}
