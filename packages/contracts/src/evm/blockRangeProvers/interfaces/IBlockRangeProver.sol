// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title IBlockRangeProver
/// @notice Interface for ZK proof-based block range provers.
/// @author Ultrasound Labs
interface IBlockRangeProver {
    /// @notice Proves a block range using ZK proof.
    /// @param srs The set of elected Super Representatives in this block cycle.
    /// @param witnessDelegatees The witness delegatees of SRs passed in the previous parameter,
    ///                          or the same addresses as SR's if an SR has no witness delegatees.
    /// @param startingBlock The starting block of the range.
    /// @param endingBlock The ending block of the range.
    /// @param endingBlockTxTrieRoot The transaction trie root of the ending block.
    /// @param endingBlockTimestamp The timestamp of the ending block.
    /// @param zkProof The ZK proof for the block range.
    function proveBlockRange(
        bytes20[27] calldata srs,
        bytes20[27] calldata witnessDelegatees,
        bytes32 startingBlock,
        bytes32 endingBlock,
        bytes32 endingBlockTxTrieRoot,
        uint32 endingBlockTimestamp,
        bytes calldata zkProof
    ) external;
}
