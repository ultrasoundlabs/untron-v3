// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronReceiver} from "../tron/UntronReceiver.sol";
import {ReceiverUtils} from "./ReceiverUtils.sol";

/// @title ReceiverDeployer
/// @notice Shared CREATE2 deployment logic for deterministic Untron receivers.
/// @dev This module extends {ReceiverUtils} and adds `deployReceiver`.
/// @author Ultrasound Labs
abstract contract ReceiverDeployer is ReceiverUtils {
    /// @notice Constructor for the ReceiverDeployer contract.
    /// @param create2Prefix Chain-specific CREATE2 prefix (0xff for EVM, 0x41 for Tron).
    constructor(bytes1 create2Prefix) ReceiverUtils(create2Prefix, address(new UntronReceiver())) {}

    /// @notice Deploy the receiver proxy via CREATE2.
    /// @param salt The CREATE2 salt.
    /// @return receiver The deployed proxy address.
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
}
