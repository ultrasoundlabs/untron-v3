// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ITronTxReader} from "../interfaces/ITronTxReader.sol";

/// @title MockStatefulTronTxReader
/// @notice Mock implementation of StatefulTronTxReader that treats any inputs as valid.
/// @dev Intended for local testing/dev deployments; it does not verify consensus or Merkle proofs.
/// @author Ultrasound Labs
contract MockStatefulTronTxReader is ITronTxReader {
    TriggerSmartContract internal _next;
    bool internal _useNext;

    /// @notice Sets the next value returned by `readTriggerSmartContract`.
    /// @param next_ Next `TriggerSmartContract` value to return.
    function setNext(TriggerSmartContract calldata next_) external {
        _next = next_;
        _useNext = true;
    }

    /// @notice Clears the configured next return value.
    function clearNext() external {
        _useNext = false;
    }

    /// @notice Returns a "valid" TriggerSmartContract view for any inputs.
    /// @dev If `setNext` was called, returns the configured value. Otherwise:
    /// - `txId = sha256(encodedTx)` (deterministic per input)
    /// - `tronBlockTimestamp = block.timestamp`
    /// - `senderTron = 0x41 || bytes20(msg.sender)`
    /// - `data = encodedTx` (passthrough)
    /// @param blocks Unused.
    /// @param encodedTx Raw protobuf-encoded Tron `Transaction` bytes.
    /// @param proof Unused.
    /// @param index Unused.
    /// @return callData Parsed `TriggerSmartContract` subset.
    function readTriggerSmartContract(
        bytes[20] calldata blocks,
        bytes calldata encodedTx,
        bytes32[] calldata proof,
        uint256 index
    ) external view returns (TriggerSmartContract memory callData) {
        blocks;
        proof;
        index;

        if (_useNext) return _next;

        callData.txId = sha256(encodedTx);
        callData.tronBlockNumber = 0;
        if (block.timestamp > type(uint32).max) revert MockStatefulTronTxReader_TimestampTooLarge();
        callData.tronBlockTimestamp = uint32(block.timestamp);
        callData.senderTron = _evmToTron(msg.sender);
        callData.toTron = _evmToTron(address(0));
        callData.data = encodedTx;
    }

    error MockStatefulTronTxReader_TimestampTooLarge();

    /// @notice Converts an EVM address to a Tron "base58check address bytes" prefix+payload representation.
    /// @param a EVM address.
    /// @return tron Tron-formatted address bytes (`0x41 || bytes20(a)`).
    function _evmToTron(address a) internal pure returns (bytes21 tron) {
        tron = bytes21((uint168(0x41) << 160) | uint160(a));
    }
}
