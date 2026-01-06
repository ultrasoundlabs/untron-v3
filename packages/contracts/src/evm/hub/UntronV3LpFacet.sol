// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {TokenUtils} from "../../utils/TokenUtils.sol";

/// @title UntronV3 LP facet
/// @notice Fast-fill vault deposit/withdrawal (principal accounting only).
/// @author Ultrasound Labs
contract UntronV3LpFacet is UntronV3Base {
    /// @notice Deposit USDT into the fast-fill vault.
    /// @dev Permissioning:
    /// - Caller MUST be allowlisted (`isLpAllowed[msg.sender] == true`), otherwise this reverts.
    ///
    /// Economics:
    /// - Fast-fill vaults are 0% APY by design; any incentive program must be implemented externally.
    ///
    /// Accounting:
    /// - On success, `lpPrincipal[msg.sender]` increases by `amount`.
    /// - These funds increase `usdtBalance()` and can be used to fill claims.
    /// @param amount Amount of `usdt` to transfer from the caller into the contract.
    function deposit(uint256 amount) external whenNotPaused {
        // Enforce LP allowlist. (Delisting does not affect `withdraw(...)`.)
        if (!isLpAllowed[msg.sender]) revert LpNotAllowlisted();
        if (amount == 0) revert ZeroAmount();

        TokenUtils.transferFrom(usdt, msg.sender, payable(address(this)), amount);
        lpPrincipal[msg.sender] += amount;

        _emitLpDeposited(msg.sender, amount);
    }

    /// @notice Withdraw USDT from the fast-fill vault.
    /// @dev Requirements:
    /// - `amount` must be > 0.
    /// - `amount` must be <= caller's `lpPrincipal`.
    /// - Contract must currently hold at least `amount` USDT.
    ///
    /// Permissioning:
    /// - This function is intentionally NOT gated by `isLpAllowed`. LPs must always be able to withdraw
    ///   already-deposited principal, even if later delisted.
    /// @param amount Amount of `usdt` to withdraw.
    function withdraw(uint256 amount) external whenNotPaused {
        // Disallow no-op withdrawals.
        if (amount == 0) revert ZeroAmount();

        // Enforce principal accounting: LPs can only withdraw what they put in.
        uint256 principal = lpPrincipal[msg.sender];
        if (amount > principal) revert WithdrawExceedsPrincipal();
        // Also enforce that the contract has enough liquid USDT to satisfy the withdrawal.
        if (amount > usdtBalance()) revert InsufficientUsdtBalance();

        // Update principal before transferring (best practice for external calls).
        lpPrincipal[msg.sender] = principal - amount;
        TokenUtils.transfer(usdt, payable(msg.sender), amount);

        _emitLpWithdrawn(msg.sender, amount);
    }
}
