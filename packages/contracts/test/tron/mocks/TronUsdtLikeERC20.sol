// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ApproveRequiresZeroERC20} from "./ApproveRequiresZeroERC20.sol";

/// @dev Mimics Tron USDT semantics:
///      - `transfer`, `transferFrom`, `approve` return `false` even on success.
///      - changing a non-zero allowance to a non-zero value reverts (USDT-like).
contract TronUsdtLikeERC20 is ApproveRequiresZeroERC20 {
    constructor(string memory name_, string memory symbol_, uint8 decimals_)
        ApproveRequiresZeroERC20(name_, symbol_, decimals_)
    {}

    function transfer(address to, uint256 amount) public override returns (bool) {
        super.transfer(to, amount);
        return false;
    }

    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        super.transferFrom(from, to, amount);
        return false;
    }

    function approve(address spender, uint256 amount) public override returns (bool) {
        super.approve(spender, amount);
        return false;
    }
}

