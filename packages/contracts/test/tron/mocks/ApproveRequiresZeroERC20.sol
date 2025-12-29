// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {MockERC20} from "./MockERC20.sol";

/// @dev Mimics USDT-like allowance behavior: changing a non-zero allowance to a non-zero value reverts.
contract ApproveRequiresZeroERC20 is MockERC20 {
    constructor(string memory name_, string memory symbol_, uint8 decimals_) MockERC20(name_, symbol_, decimals_) {}

    function approve(address spender, uint256 amount) public override returns (bool) {
        if (amount != 0 && allowance(_msgSender(), spender) != 0) {
            revert("ALLOWANCE_NOT_ZERO");
        }
        return super.approve(spender, amount);
    }
}

