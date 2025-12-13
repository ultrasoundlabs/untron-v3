// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {TokenUtils} from "../utils/TokenUtils.sol";

/// @title UntronReceiver
/// @notice Simple smart contract controlled by UntronController that holds TRC-20 tokens
///         and native TRX and lets the controller transfer them to a specified recipient.
/// @author Ultrasound Labs
contract UntronReceiver {
    /*//////////////////////////////////////////////////////////////
                                STORAGE
    //////////////////////////////////////////////////////////////*/

    address payable internal immutable _CONTROLLER;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    error NotController();

    /*//////////////////////////////////////////////////////////////
                                 CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor() {
        _CONTROLLER = payable(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                                 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Called by the controller to move `amount` of `token` held by this contract to `recipient`.
    function pull(address token, uint256 amount) external {
        if (msg.sender != _CONTROLLER) revert NotController();
        if (amount != 0) {
            TokenUtils.transfer(token, _CONTROLLER, amount);
        }
    }

    // Tron forbids sending TRX to smart contracts via TransferContract,
    // but we still keep receive() fallback (which has to be called using TriggerSmartContract)
    // for future-proofness.
    receive() external payable {}
}
