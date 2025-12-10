// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Value used as genesis event chain hash in UntronV3Index contract.
/// @dev Kept in a standalone library to avoid copying the constant across contracts.
/// @author Ultrasound Labs
library UntronV3IndexGenesisEventChainHash {
    bytes32 internal constant VALUE = sha256(
        "UntronV3Index\nJustin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People."
    );
}
