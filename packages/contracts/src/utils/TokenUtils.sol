// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.12;

import "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import "solady/utils/SafeTransferLib.sol";

/// Asset amount, e.g. $100 USDC or 0.1 ETH
struct TokenAmount {
    /// Zero address = native asset, e.g. ETH
    address token;
    uint256 amount;
}

/// Utility functions that work for both ERC20 and native tokens.
///
/// Guarantees and semantics:
/// - Supports standard ERC-20s that either return a boolean or no return value; tokens
///   that return `false` are treated as failure paths for `tryTransfer/transferFrom`.
/// - Fee-on-transfer/deflationary tokens: functions return based on call success;
///   no guarantee that the recipient received the full `amount`.
/// - `approve` is a no-op for the native token (token == address(0)).
/// - `transferFrom` forbids the native token and will revert.
/// - `transferBalance` returns the amount swept from this contract; returns 0 if nothing to sweep.
/// - `checkBalance` returns the first index whose balance >= amount, or `n` if none (including empty arrays).
library TokenUtils {
    /// Returns ERC20 or ETH balance.
    function getBalanceOf(address token, address addr) internal view returns (uint256) {
        if (token == address(0)) {
            return addr.balance;
        } else {
            return IERC20(token).balanceOf(addr);
        }
    }

    /// Approves a token transfer.
    function approve(address token, address spender, uint256 amount) internal {
        if (token != address(0)) {
            SafeTransferLib.safeApproveWithRetry({token: token, to: spender, amount: amount});
        } // Do nothing for native token.
    }

    /// Sends an ERC20 or ETH transfer. For ETH, verify call success.
    function transfer(address token, address payable recipient, uint256 amount) internal {
        if (token != address(0)) {
            SafeTransferLib.safeTransfer({token: token, to: recipient, amount: amount});
        } else {
            SafeTransferLib.safeTransferETH({to: recipient, amount: amount});
        }
    }

    /// Sends an ERC20 or ETH transfer. Returns true if the call succeeded.
    /// For ERC-20 fee-on-transfer tokens, the recipient may receive less than `amount`.
    function tryTransfer(address token, address payable recipient, uint256 amount) internal returns (bool) {
        if (token != address(0)) {
            return
                SafeTransferLib.trySafeTransferFrom({token: token, from: address(this), to: recipient, amount: amount});
        } else {
            // Attempt native transfer with limited gas stipend.
            return SafeTransferLib.trySafeTransferETH({
                to: recipient,
                amount: amount,
                gasStipend: SafeTransferLib.GAS_STIPEND_NO_GRIEF
            });
        }
    }

    /// Sends an ERC20 transfer.
    function transferFrom(address token, address from, address to, uint256 amount) internal {
        require(token != address(0), "TokenUtils: ETH transferFrom must be caller");
        SafeTransferLib.safeTransferFrom({token: token, from: from, to: to, amount: amount});
    }

    /// Sends any token balance in the contract to the recipient.
    function transferBalance(address token, address payable recipient) internal returns (uint256) {
        uint256 balance = getBalanceOf({token: token, addr: address(this)});
        if (balance > 0) {
            transfer({token: token, recipient: recipient, amount: balance});
        }
        return balance;
    }

    /// Check that the address has enough of at least one of the tokenAmounts.
    /// Returns the index of the first token that has sufficient balance, or
    /// the length of the tokenAmounts array if no token has sufficient balance.
    function checkBalance(TokenAmount[] calldata tokenAmounts) internal view returns (uint256) {
        uint256 n = tokenAmounts.length;
        for (uint256 i = 0; i < n; ++i) {
            TokenAmount calldata tokenAmount = tokenAmounts[i];
            uint256 balance = getBalanceOf({token: tokenAmount.token, addr: address(this)});
            if (balance >= tokenAmount.amount) {
                return i;
            }
        }
        return n;
    }
}
