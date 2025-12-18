// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {MockERC20} from "./MockERC20.sol";

contract ReturnFalseERC20 is MockERC20 {
    constructor(string memory name_, string memory symbol_, uint8 decimals_) MockERC20(name_, symbol_, decimals_) {}

    function transfer(address to, uint256 amount) public override returns (bool) {
        super.transfer(to, amount);
        return false;
    }

    function transferFrom(address from, address to, uint256 amount) public override returns (bool) {
        super.transferFrom(from, to, amount);
        return false;
    }
}
