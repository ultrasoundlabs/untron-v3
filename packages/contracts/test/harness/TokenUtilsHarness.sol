// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {TokenAmount, TokenUtils} from "../../src/utils/TokenUtils.sol";

contract TokenUtilsHarness {
    using TokenUtils for *;

    // Accept ETH
    receive() external payable {}

    // ---- Thin wrappers that just forward to the library ----

    function getBalanceOfToken(address token, address addr) external view returns (uint256) {
        return TokenUtils.getBalanceOf(token, addr);
    }

    function approveToken(address token, address spender, uint256 amount) external {
        TokenUtils.approve(token, spender, amount);
    }

    function transferToken(address token, address payable recipient, uint256 amount) external {
        TokenUtils.transfer(token, recipient, amount);
    }

    function transferFromToken(address token, address from, address to, uint256 amount) external {
        TokenUtils.transferFrom(token, from, to, amount);
    }

    function transferBalanceToken(address token, address payable recipient) external returns (uint256) {
        return TokenUtils.transferBalance(token, recipient);
    }

    function checkBalanceHarness(TokenAmount[] calldata tokenAmounts) external view returns (uint256) {
        return TokenUtils.checkBalance(tokenAmounts);
    }
}
