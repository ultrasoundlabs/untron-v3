// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

/// @title MockERC20
/// @notice Minimal ERC20 mock with configurable decimals and mint/burn helpers.
/// @author Ultrasound Labs
contract MockERC20 is ERC20 {
    uint8 private immutable _DECIMALS;

    /// @notice Creates a mock ERC20 with the given metadata.
    /// @param name_ Token name.
    /// @param symbol_ Token symbol.
    /// @param decimals_ Token decimals.
    constructor(string memory name_, string memory symbol_, uint8 decimals_) ERC20(name_, symbol_) {
        _DECIMALS = decimals_;
    }

    /// @notice Returns the configured decimals value.
    /// @return The token decimals.
    function decimals() public view override returns (uint8) {
        return _DECIMALS;
    }

    /// @notice Mints `amount` tokens to `to`.
    /// @param to Recipient address.
    /// @param amount Amount to mint.
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }

    /// @notice Burns `amount` tokens from `from`.
    /// @param from Address to burn from.
    /// @param amount Amount to burn.
    function burn(address from, uint256 amount) external {
        _burn(from, amount);
    }
}
