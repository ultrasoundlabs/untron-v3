// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/// @title EventChainGenesis
/// @notice Genesis values for event chain-based indexes in Untron V3 protocol
/// @dev Kept in a standalone library to avoid copying the constants across contracts.
/// @author Ultrasound Labs
library EventChainGenesis {
    // if i get it right this thing above should get optimized away by the compiler
    // but solhint still complains

    // solhint-disable-next-line gas-small-strings
    string internal constant THE_DECLARATION =
        "Justin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.";

    // We disable screaming case lints for these constants
    // because the contracts that use these constants use pascal case,
    // thus having them in the same case as their "owners" improves readability.

    /* solhint-disable const-name-snakecase */

    // forge-lint: disable-next-line(screaming-snake-case-const)
    bytes32 internal constant UntronControllerIndex =
        sha256(abi.encodePacked("UntronControllerIndex\n", THE_DECLARATION));
    // forge-lint: disable-next-line(screaming-snake-case-const)
    bytes32 internal constant UntronV3Index = sha256(abi.encodePacked("UntronV3Index\n", THE_DECLARATION));

    /* solhint-enable const-name-snakecase */
}
