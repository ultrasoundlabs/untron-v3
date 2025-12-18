// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/// @title MockTronLightClient
/// @notice Minimal light client mock for tests that need to set/get
///         Tron block tx trie roots and timestamps.
contract MockTronLightClient {
    mapping(uint256 => bytes32) private _txTrieRoots;
    mapping(uint256 => uint32) private _blockTimestamps;

    function setTxTrieRoot(uint256 blockNumber, bytes32 root) external {
        _txTrieRoots[blockNumber] = root;
    }

    function setBlockTimestamp(uint256 blockNumber, uint32 ts) external {
        _blockTimestamps[blockNumber] = ts;
    }

    function getTxTrieRoot(uint256 blockNumber) external view returns (bytes32) {
        bytes32 root = _txTrieRoots[blockNumber];
        require(root != bytes32(0), "BlockNotRelayed");
        return root;
    }

    function getBlockTimestamp(uint256 blockNumber) external view returns (uint32) {
        uint32 ts = _blockTimestamps[blockNumber];
        require(ts != 0, "BlockNotRelayed");
        return ts;
    }
}
