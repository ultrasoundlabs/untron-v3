// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {MockERC20} from "./MockERC20.sol";

contract RevertERC20 is MockERC20 {
    constructor(string memory name_, string memory symbol_, uint8 decimals_) MockERC20(name_, symbol_, decimals_) {}

    function transfer(address, uint256) public pure override returns (bool) {
        revert("REVERT_ERC20_TRANSFER");
    }

    function transferFrom(address, address, uint256) public pure override returns (bool) {
        revert("REVERT_ERC20_TRANSFER_FROM");
    }
}

