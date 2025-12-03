// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

interface IBlockRangeProver {
    function proveBlockRange(
        bytes20[27] calldata srs,
        bytes20[27] calldata witnessDelegatees,
        bytes32 startingBlock,
        bytes32 endingBlock,
        bytes calldata zkProof
    ) external;
}
