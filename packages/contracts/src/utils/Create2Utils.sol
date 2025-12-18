// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronReceiver} from "../tron/UntronReceiver.sol";

/// @title Create2Utils
/// @notice Shared logic for deterministic deployments of Untron contracts.
/// @author Ultrasound Labs
/// @dev Functions below are forked from Solady's CloneLib.
///      The only functional difference here is the use of an immutable CREATE2_PREFIX
///      to allow for chain-specific CREATE2 address calculation.
contract Create2Utils {
    /// @notice Address of the deployed UntronReceiver contract.
    /// @dev Used to deploy minimal proxies of it.
    address public immutable RECEIVER_IMPL;

    // TODO: make it deploy minimal proxies maybe?
    // Chain-specific byte prefix used in CREATE2 address calculation (0xff for EVM, 0x41 for Tron).
    bytes1 private immutable _CREATE2_PREFIX;

    /// @notice Initializes the contract with the specified CREATE2_PREFIX.
    /// @param create2Prefix The CREATE2_PREFIX of the deployment chain (0xff for EVM, 0x41 for Tron).
    constructor(bytes1 create2Prefix) {
        _CREATE2_PREFIX = create2Prefix;
        RECEIVER_IMPL = address(new UntronReceiver()); // deploy from controller => controller becomes immutable
    }

    /// @notice Deploys the receiver contract using CREATE2 and the provided salt.
    /// @param salt The salt used for CREATE2 address calculation.
    /// @return receiver The address of the deployed receiver contract.
    function deployReceiver(bytes32 salt) public returns (address payable receiver) {
        address impl = RECEIVER_IMPL;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let ptr := mload(0x40)

            mstore(ptr, 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000000000000000000000)
            mstore(add(ptr, 0x14), shl(0x60, impl))
            mstore(add(ptr, 0x28), 0x5af43d82803e903d91602b57fd5bf30000000000000000000000000000000000)

            receiver := create2(0, ptr, 0x37, salt)
            if iszero(receiver) {
                returndatacopy(0, 0, returndatasize())
                revert(0, returndatasize())
            }
        }
    }

    /// @notice Returns the creation bytecode for a receiver.
    /// @return bytes The bytecode for the receiver contract.
    /// @dev The UntronReceiver constructor has no explicit parameters and uses msg.sender
    ///      as the controller, so no controller address needs to be embedded here.
    function receiverBytecode() public view returns (bytes memory) {
        return abi.encodePacked(
            hex"3d602d80600a3d3981f3363d3d373d3d3d363d73", RECEIVER_IMPL, hex"5af43d82803e903d91602b57fd5bf3"
        );
    }

    /// @dev Predicts the deterministic address for a receiver deployed via CREATE2.
    /// @param controller The address of the UntronController (or other deployer) that will perform CREATE2.
    /// @param salt       The CREATE2 salt used for deterministic deployment.
    /// @return predicted The predicted address of the receiver contract.
    /// @notice This function is pure read logic and can be used off-chain or from other chains
    ///         to precompute receiver addresses for a given controller and salt.
    function predictReceiverAddress(address controller, bytes32 salt) public view returns (address predicted) {
        predicted = address(
            uint160(
                uint256(keccak256(abi.encodePacked(_CREATE2_PREFIX, controller, salt, keccak256(receiverBytecode()))))
            )
        );
    }

    /// @dev Predicts the deterministic address for a receiver deployed via CREATE2.
    /// @param salt       The CREATE2 salt used for deterministic deployment.
    /// @return predicted The predicted address of the receiver contract.
    /// @notice This function calls predictReceiverAddress with controller == address(this)
    ///         and is supposed to be used when you need to determine address of a receiver
    ///         the calling contract is going to deploy.
    function predictReceiverAddress(bytes32 salt) public view returns (address predicted) {
        predicted = predictReceiverAddress(address(this), salt);
    }
}
