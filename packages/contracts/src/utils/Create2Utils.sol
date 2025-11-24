// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronReceiver} from "../tron/UntronReceiver.sol";

/// @title Create2Utils
/// @notice Shared logic for deterministic deployments of Untron contracts.
/// @author Ultrasound Labs
/// @dev Functions below are forked from Solady's CloneLib.
///      The only functional difference here is the use of an immutable CREATE2_PREFIX
///      to allow for chain-specific CREATE2 address calculation.
contract Create2Utils {
    // Chain-specific byte prefix used in CREATE2 address calculation (0xff for EVM, 0x41 for Tron).
    bytes1 private immutable CREATE2_PREFIX;

    constructor(bytes1 create2Prefix) {
        CREATE2_PREFIX = create2Prefix;
    }

    /// @dev Deploys the receiver contract using CREATE2 and the provided salt.
    function deployReceiver(address controller, bytes32 salt) public returns (address payable receiver) {
        bytes memory bytecode = receiverBytecode(controller);
        // solhint-disable-next-line no-inline-assembly
        assembly {
            receiver := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
            if iszero(receiver) {
                // Forward the revert reason if deployment failed.
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }
    }

    /// @notice Returns the creation bytecode for a receiver with the current
    ///         controller address embedded as constructor argument.
    function receiverBytecode(address controller) public pure returns (bytes memory) {
        return abi.encodePacked(type(UntronReceiver).creationCode, abi.encode(controller));
    }

    /// @dev Predicts the deterministic address for a receiver deployed via CREATE2.
    function predictReceiverAddress(address controller, bytes32 salt) public view returns (address predicted) {
        predicted = address(
            uint160(
                uint256(
                    keccak256(
                        abi.encodePacked(CREATE2_PREFIX, controller, salt, keccak256(receiverBytecode(controller)))
                    )
                )
            )
        );
    }
}
