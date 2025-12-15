// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title IBlockRangeProver
/// @notice Interface for ZK proof-based block range provers.
/// @author Ultrasound Labs
interface IBlockRangeProver {
    /// @notice Proves a block range using ZK proof.
    /// @param srDataHash The hash of the SR data (see TronLightClient).
    /// @param startingBlock The starting block of the range.
    /// @param endingBlock The ending block of the range.
    /// @param endingBlockTxTrieRoot The transaction trie root of the ending block.
    /// @param endingBlockTimestamp The timestamp of the ending block.
    /// @param zkProof The ZK proof for the block range.
    function proveBlockRange(
        bytes32 srDataHash,
        bytes32 startingBlock,
        bytes32 endingBlock,
        bytes32 endingBlockTxTrieRoot,
        uint32 endingBlockTimestamp,
        bytes calldata zkProof
    ) external;
}
