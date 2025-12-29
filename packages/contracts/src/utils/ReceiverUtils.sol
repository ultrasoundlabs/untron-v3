// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/// @title ReceiverUtils
/// @notice Shared *read-only* logic for deterministic Untron receiver addresses.
/// @dev This module intentionally contains no CREATE2 deployment logic.
///
/// Receiver addresses are computed as:
/// `address(uint160(uint256(keccak256(CREATE2_PREFIX ++ controller ++ salt ++ keccak256(receiverBytecode())))))`
///
/// `receiverBytecode()` is the EIP-1167 minimal proxy creation bytecode with `RECEIVER_IMPL` embedded.
/// @author Ultrasound Labs
abstract contract ReceiverUtils {
    /// @notice Address embedded into the receiver proxy init code.
    /// @dev For Tron-side controllers this is the deployed `UntronReceiver` implementation.
    ///      For EVM-side contracts that only need to *predict* Tron receiver addresses, this should be set
    ///      to the Tron controller's `RECEIVER_IMPL()` (it does not need to exist on the local chain).
    address public immutable RECEIVER_IMPL;

    // Chain-specific byte prefix used in CREATE2 address calculation (0xff for EVM, 0x41 for Tron).
    bytes1 private immutable _CREATE2_PREFIX;

    /// @notice Constructor for the ReceiverUtils contract.
    /// @param create2Prefix Chain-specific CREATE2 prefix (0xff for EVM, 0x41 for Tron).
    /// @param receiverImpl Address embedded into the EIP-1167 receiver init code.
    constructor(bytes1 create2Prefix, address receiverImpl) {
        _CREATE2_PREFIX = create2Prefix;
        RECEIVER_IMPL = receiverImpl;
    }

    /// @notice Returns the EIP-1167 creation bytecode for a receiver proxy.
    /// @dev `UntronReceiver` has no constructor args; the controller binding is via `msg.sender` at impl deploy time.
    /// @return The EIP-1167 creation bytecode for a receiver proxy.
    function receiverBytecode() public view returns (bytes memory) {
        return abi.encodePacked(
            hex"3d602d80600a3d3981f3363d3d373d3d3d363d73", RECEIVER_IMPL, hex"5af43d82803e903d91602b57fd5bf3"
        );
    }

    /// @notice Predict the deterministic address for a receiver deployed via CREATE2.
    /// @param controller The UntronController (deployer) address in 20-byte EVM form.
    /// @param salt The CREATE2 salt.
    /// @return predicted The predicted address of the receiver.
    function predictReceiverAddress(address controller, bytes32 salt) public view returns (address predicted) {
        predicted = address(
            uint160(
                uint256(keccak256(abi.encodePacked(_CREATE2_PREFIX, controller, salt, keccak256(receiverBytecode()))))
            )
        );
    }

    /// @notice Predict the deterministic address for a receiver deployed via CREATE2 by this contract.
    /// @param salt The CREATE2 salt.
    /// @return predicted The predicted address of the receiver.
    function predictReceiverAddress(bytes32 salt) public view returns (address predicted) {
        predicted = predictReceiverAddress(address(this), salt);
    }
}
