// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "../utils/TokenUtils.sol";

/// @title UntronReceiver
/// @notice Simple smart contract for pulling TRC-20 tokens through UntronController and transferring them to a specified recipient.
/// @author Ultrasound Labs
contract UntronReceiver {
    /*//////////////////////////////////////////////////////////////
                                STORAGE
    //////////////////////////////////////////////////////////////*/

    address internal immutable controller;

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    error NotController();

    /*//////////////////////////////////////////////////////////////
                                 FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    constructor(address controller_) {
        controller = controller_;
    }

    /// @notice Called by the controller to move `amount` of `token` held by this contract to `recipient`.
    function onControllerCall(address token, uint256 amount, address payable recipient) external {
        if (msg.sender != controller) revert NotController();
        if (amount != 0) {
            TokenUtils.transfer(token, recipient, amount);
        }
    }

    // Tron forbids sending TRX to smart contracts via TransferContract,
    // but we still keep receive() fallback (which has to be called using TriggerSmartContract)
    // for future-proofness.
    receive() external payable {}
}
