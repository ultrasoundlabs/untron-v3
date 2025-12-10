// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Value used as genesis event chain hash in UntronControllerIndex contract
/// @dev It's a library because it's used in two places (UntronControllerIndex and UntronV3)
///      and my autistic ass couldn't just copypaste the value in two contracts and call it a day
/// @author Ultrasound Labs
library UntronControllerIndexGenesisEventChainHash {
    bytes32 internal constant VALUE = sha256(
        "UntronControllerIndex\nJustin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People."
    );
}
