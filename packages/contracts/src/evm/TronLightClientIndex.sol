// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {EventChainGenesis} from "../utils/EventChainGenesis.sol";

/// @title  TronLightClientIndex
/// @notice Hash-chain-based event index for TronLightClient, friendly to offchain indexers.
/// @dev    TronLightClient must not emit events itself. All events must be defined and emitted through TronLightClientIndex.
/// @author Ultrasound Labs
contract TronLightClientIndex {
    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = EventChainGenesis.TronLightClientIndex;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted once at deployment to expose light client configuration.
    /// @param blockRangeProver External verifier for `proveBlockRange`.
    /// @param srDataHash Hash of the SR data for the epoch.
    /// @param initialBlockId Initial Tron `blockId` checkpoint (height embedded in the top 8 bytes).
    /// @param initialTxTrieRoot Transaction trie root corresponding to `initialBlockId`.
    /// @param initialTimestamp Block timestamp for `initialBlockId` (seconds).
    /// @param srs SR owner accounts for the epoch.
    /// @param witnessDelegatees SR signing keys for the epoch.
    event TronLightClientConfigured(
        address indexed blockRangeProver,
        bytes32 indexed srDataHash,
        bytes32 indexed initialBlockId,
        bytes32 initialTxTrieRoot,
        uint32 initialTimestamp,
        bytes20[27] srs,
        bytes20[27] witnessDelegatees
    );

    /// @notice Emitted for each stored Tron checkpoint block.
    /// @param blockNumber Tron block height.
    /// @param blockId Stored Tron `blockId` at that height.
    /// @param txTrieRoot Transaction trie root at that height.
    /// @param timestamp Stored block timestamp in seconds.
    event TronBlockStored(uint256 indexed blockNumber, bytes32 indexed blockId, bytes32 txTrieRoot, uint32 timestamp);

    /// @notice Emitted whenever `latestProvenBlock` is updated to a newer value.
    /// @param previousLatest Prior `latestProvenBlock` value.
    /// @param newLatest New `latestProvenBlock` value.
    /// @param newBlockNumber Tron block height for `newLatest`.
    event LatestProvenBlockUpdated(bytes32 indexed previousLatest, bytes32 indexed newLatest, uint256 newBlockNumber);

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature hash (topic0) of the event.
    /// @param abiEncodedEventData ABI-encoded event arguments.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        eventChainTip =
            sha256(abi.encodePacked(eventChainTip, block.number, block.timestamp, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emits {TronLightClientConfigured} and appends it to the event chain.
    /// @param blockRangeProver The address of the block range prover.
    /// @param srDataHash The hash of the SR data.
    /// @param initialBlockId The initial block ID.
    /// @param initialTxTrieRoot The initial transaction trie root.
    /// @param initialTimestamp The initial timestamp.
    /// @param srs The array of SRS.
    /// @param witnessDelegatees The array of witness delegatees.
    function _emitTronLightClientConfigured(
        address blockRangeProver,
        bytes32 srDataHash,
        bytes32 initialBlockId,
        bytes32 initialTxTrieRoot,
        uint32 initialTimestamp,
        bytes20[27] memory srs,
        bytes20[27] memory witnessDelegatees
    ) internal {
        _appendEventChain(
            TronLightClientConfigured.selector,
            abi.encode(
                blockRangeProver,
                srDataHash,
                initialBlockId,
                initialTxTrieRoot,
                initialTimestamp,
                srs,
                witnessDelegatees
            )
        );
        emit TronLightClientConfigured(
            blockRangeProver, srDataHash, initialBlockId, initialTxTrieRoot, initialTimestamp, srs, witnessDelegatees
        );
    }

    /// @notice Emits {TronBlockStored} and appends it to the event chain.
    /// @param blockNumber The block number of the stored block.
    /// @param blockId The block ID of the stored block.
    /// @param txTrieRoot The transaction trie root of the stored block.
    /// @param timestamp The timestamp of the stored block.
    function _emitTronBlockStored(uint256 blockNumber, bytes32 blockId, bytes32 txTrieRoot, uint32 timestamp) internal {
        _appendEventChain(TronBlockStored.selector, abi.encode(blockNumber, blockId, txTrieRoot, timestamp));
        emit TronBlockStored(blockNumber, blockId, txTrieRoot, timestamp);
    }

    /// @notice Emits {LatestProvenBlockUpdated} and appends it to the event chain.
    /// @param previousLatest The previous latest block ID.
    /// @param newLatest The new latest block ID.
    /// @param newBlockNumber The new block number.
    function _emitLatestProvenBlockUpdated(bytes32 previousLatest, bytes32 newLatest, uint256 newBlockNumber) internal {
        _appendEventChain(LatestProvenBlockUpdated.selector, abi.encode(previousLatest, newLatest, newBlockNumber));
        emit LatestProvenBlockUpdated(previousLatest, newLatest, newBlockNumber);
    }
}
