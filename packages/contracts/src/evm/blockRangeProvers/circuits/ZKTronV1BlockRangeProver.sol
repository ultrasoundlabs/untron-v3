// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IBlockRangeProver} from "../interfaces/IBlockRangeProver.sol";
import {HonkVerifier} from "@untron/zktron/ZKTronV1Verifier.sol";

/// @title ZKTronV1BlockRangeProver
/// @notice Block Range Prover for TronLightClient based on ZKTron V1 circuit.
/// @dev ZKTron V1 is written in Noir and uses Barretenberg proof system (HonkVerifier).
/// @author Ultrasound Labs
contract ZKTronV1BlockRangeProver is IBlockRangeProver {
    /// @inheritdoc IBlockRangeProver
    function proveBlockRange(
        bytes20[27] calldata srs,
        bytes20[27] calldata witnessDelegatees,
        bytes32 startingBlock,
        bytes32 endingBlock,
        bytes32 endingBlockTxTrieRoot,
        uint32 endingBlockTimestamp,
        bytes calldata zkProof
    ) external {}
}
