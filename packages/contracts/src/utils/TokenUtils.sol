// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {SafeTransferLib} from "solady/utils/SafeTransferLib.sol";

/// Utility functions that work for both ERC20 and native tokens.
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
}
