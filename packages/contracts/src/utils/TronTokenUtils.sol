// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Math} from "openzeppelin-contracts/contracts/utils/math/Math.sol";

/// @title TronTokenUtils
/// @notice Token helpers for Tron-like EVM chains.
/// @dev This library matches the API and high-level semantics of `TokenUtils`:
///      - `token == address(0)` means native TRX.
///      - `approve` and `transferFrom` are no-ops for native TRX.
///
///      Tron-specific caveat:
///      Some TRC-20 tokens on Tron (notably USDT) are non-compliant and may return `false`
///      even when the transfer/approve succeeds, while reverting on failure.
///
///      For TRC-20 operations, this library intentionally treats **only a revert** as failure:
///      - If the low-level call succeeds, the operation is treated as successful regardless of returndata.
///      - Therefore, Tron-side Untron V3 contracts only support TRC-20 tokens that **revert on failure**
///        (tokens that signal failure by returning `false` without reverting are unsupported).
/// @author Ultrasound Labs
library TronTokenUtils {
    /* solhint-disable avoid-low-level-calls */
    /*//////////////////////////////////////////////////////////////
                                 ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Thrown when a TRC-20 low-level call reverts.
    error Trc20CallFailed();
    /// @notice Thrown when a TRC-20 `staticcall` (e.g. `balanceOf`) reverts.
    error Trc20StaticCallFailed();
    /// @notice Thrown when TRC-20 returndata is malformed (e.g. missing 32-byte return for `balanceOf`).
    error Trc20BadReturnData();
    /// @notice Thrown when sending native TRX via `call{value: ...}("")` fails.
    error TrxTransferFailed();

    /*//////////////////////////////////////////////////////////////
                              CONSTANTS
    //////////////////////////////////////////////////////////////*/

    bytes4 internal constant _TRANSFER = 0xa9059cbb;
    bytes4 internal constant _TRANSFER_FROM = 0x23b872dd;
    bytes4 internal constant _APPROVE = 0x095ea7b3;
    bytes4 internal constant _BALANCE_OF = 0x70a08231;

    /*//////////////////////////////////////////////////////////////
                              VIEW HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Returns TRC-20 or TRX balance of `account`.
    /// @param token The token address to query (`address(0)` = native TRX).
    /// @param account The account to query.
    /// @return balance The balance, denominated in the smallest unit (TRX "sun" for TRX).
    function getBalanceOf(address token, address account) internal view returns (uint256) {
        if (token == address(0)) return account.balance;
        return balanceOf(token, account);
    }

    /// @notice Returns TRC-20 balance of `account`.
    /// @param token The token address to query.
    /// @param account The account to query.
    /// @return bal The token balance.
    /// @dev Reverts if the token call itself reverts or returns malformed data.
    function balanceOf(address token, address account) internal view returns (uint256 bal) {
        (bool ok, bytes memory data) = token.staticcall(abi.encodeWithSelector(_BALANCE_OF, account));
        if (!ok) revert Trc20StaticCallFailed();
        if (data.length < 32) revert Trc20BadReturnData();
        // solhint-disable-next-line no-inline-assembly
        assembly {
            bal := mload(add(data, 0x20))
        }
    }

    /*//////////////////////////////////////////////////////////////
                              TRX HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Transfers native TRX.
    /// @param to Recipient address.
    /// @param amount Amount of TRX in the smallest unit ("sun").
    /// @dev Uses a low-level call because Tron forbids sending TRX to contracts via TransferContract;
    ///      contracts must be funded via TriggerSmartContract / fallback.
    function _transferTrx(address payable to, uint256 amount) private {
        (bool ok,) = to.call{value: amount}("");
        if (!ok) revert TrxTransferFailed();
    }

    /*//////////////////////////////////////////////////////////////
                              MATH HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Full-precision mulDiv: floor(x * y / denominator).
    /// @param x The first operand.
    /// @param y The second operand.
    /// @param denominator The denominator.
    /// @return result The multiplication and division result.
    function mulDiv(uint256 x, uint256 y, uint256 denominator) internal pure returns (uint256) {
        return Math.mulDiv(x, y, denominator);
    }

    /*//////////////////////////////////////////////////////////////
                               TRC-20
    //////////////////////////////////////////////////////////////*/

    /// @notice Transfers TRC-20 tokens or TRX.
    /// @param token The token address to transfer (`address(0)` = native TRX).
    /// @param recipient The recipient address.
    /// @param amount The amount to transfer in the smallest unit.
    /// @dev For TRC-20 tokens this treats only a revert as failure; successful calls are accepted
    ///      regardless of returndata.
    function transfer(address token, address payable recipient, uint256 amount) internal {
        if (token == address(0)) {
            _transferTrx(recipient, amount);
            return;
        }
        (bool ok,) = token.call(abi.encodeWithSelector(_TRANSFER, recipient, amount));
        if (!ok) revert Trc20CallFailed();
    }

    /// @notice Transfers TRC-20 tokens from `from` to `recipient`.
    /// @param token The token address to transfer (`address(0)` = native TRX; no-op).
    /// @param from The token holder address.
    /// @param recipient The recipient address.
    /// @param amount The amount to transfer in the smallest unit.
    /// @dev No-op for native TRX, matching `TokenUtils.transferFrom`.
    function transferFrom(address token, address from, address payable recipient, uint256 amount) internal {
        if (token == address(0)) {
            // No-op for native TRX (same as TokenUtils.transferFrom).
            return;
        }
        (bool ok,) = token.call(abi.encodeWithSelector(_TRANSFER_FROM, from, recipient, amount));
        if (!ok) revert Trc20CallFailed();
    }

    /// @notice Approves `spender` to spend `amount` of `token`.
    /// @param token The token address to approve (`address(0)` = native TRX; no-op).
    /// @param spender The spender address.
    /// @param amount The allowance amount.
    /// @dev This uses a USDT-like retry flow: `approve(amount) -> approve(0) -> approve(amount)`.
    ///      For TRC-20 tokens this treats only a revert as failure; successful calls are accepted
    ///      regardless of returndata.
    function approve(address token, address spender, uint256 amount) internal {
        if (token == address(0)) {
            // No-op for native TRX (same as TokenUtils.approve).
            return;
        }

        (bool ok,) = token.call(abi.encodeWithSelector(_APPROVE, spender, amount));
        if (ok) {
            return;
        }

        (ok,) = token.call(abi.encodeWithSelector(_APPROVE, spender, 0));
        if (!ok) revert Trc20CallFailed();

        (ok,) = token.call(abi.encodeWithSelector(_APPROVE, spender, amount));
        if (!ok) revert Trc20CallFailed();
    }

    /* solhint-enable avoid-low-level-calls */
}
