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

    address internal immutable CONTROLLER;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    error NotController();

    /*//////////////////////////////////////////////////////////////
                                 CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor() {
        CONTROLLER = msg.sender;
    }

    /*//////////////////////////////////////////////////////////////
                                 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Called by the controller to move `amount` of `token` held by this contract to `recipient`.
    function onControllerCall(address token, uint256 amount, address payable recipient) external {
        if (msg.sender != CONTROLLER) revert NotController();
        if (amount != 0) {
            TokenUtils.transfer(token, recipient, amount);
        }
    }

    // Tron forbids sending TRX to smart contracts via TransferContract,
    // but we still keep receive() fallback (which has to be called using TriggerSmartContract)
    // for future-proofness.
    receive() external payable {}
}
