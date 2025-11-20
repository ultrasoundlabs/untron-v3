// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronReceiver} from "./UntronReceiver.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";
import {IBridger} from "./bridgers/interfaces/IBridger.sol";
import {Create2Utils} from "../utils/Create2Utils.sol";

/// @title UntronController
/// @notice Central coordination contract for Untron protocol on Tron-like EVM chains.
/// @author Ultrasound Labs
contract UntronController is Create2Utils {
    /*//////////////////////////////////////////////////////////////
                                 STORAGE
    //////////////////////////////////////////////////////////////*/

    // Contract owner, can set executor and bridger configuration
    address public owner;
    // Executor, can transfer tokens from receivers to arbitrary recipients
    address public executor;

    // Whitelist of allowed bridger contracts
    mapping(address => bool) public isBridgerAllowed;

    // token => bridger => bridger-specific payload
    mapping(address => mapping(address => bytes)) public payloadFor;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the owner of the contract is changed.
    event OwnerChanged(address indexed newOwner);
    /// @notice Emitted when the executor of the contract is changed.
    event ExecutorChanged(address indexed newExecutor);
    /// @notice Emitted when a particular bridger is allowed or disallowed.
    event BridgerAllowed(address indexed bridger, bool allowed);
    /// @notice Emitted when a payload is set for a particular token+bridger pair.
    event PayloadSet(address indexed token, address indexed bridger);
    /// @notice Emitted when a receiver is deployed.
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    /// @notice Emitted when a receiver is called to transfer tokens to a recipient.
    event ReceiverCalled(address indexed receiver, address indexed token, address indexed recipient, uint256 amount);
    /// @notice Emitted when tokens are dumped from receivers to the controller.
    event TokensDumped(address indexed token, uint256 totalAmount);
    /// @notice Emitted when tokens are bridged via a particular bridger.
    event TokensBridged(address indexed token, uint256 inAmount, uint256 outAmount, address indexed bridger);

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    modifier onlyOwner() {
        _onlyOwner();
        _;
    }

    modifier onlyExecutor() {
        _onlyExecutor();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(bytes1 create2Prefix) Create2Utils(create2Prefix) {
        owner = msg.sender;
        emit OwnerChanged(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                             ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    function setExecutor(address _executor) external onlyOwner {
        executor = _executor;
        emit ExecutorChanged(_executor);
    }

    function setBridgerAllowed(address _bridger, bool _allowed) external onlyOwner {
        isBridgerAllowed[_bridger] = _allowed;
        emit BridgerAllowed(_bridger, _allowed);
    }

    function setPayload(address _token, address _bridger, bytes calldata _payload) external onlyOwner {
        require(isBridgerAllowed[_bridger], "UntronController: bridger not allowed");
        payloadFor[_token][_bridger] = _payload;
        emit PayloadSet(_token, _bridger);
    }

    function setOwner(address _newOwner) external onlyOwner {
        require(_newOwner != address(0));
        owner = _newOwner;
        emit OwnerChanged(_newOwner);
    }

    /*//////////////////////////////////////////////////////////////
                          PUBLIC FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    // Functions below are forked from Solady's CloneLib.
    // The only functional difference here is the use of an immutable _create2Prefix
    // and immutable _receiverBytecodeHash, instead of self-computed values.

    /*//////////////////////////////////////////////////////////////
                          EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Dumps tokens into multiple receiver contracts.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @param amounts Corresponding token amounts for each receiver.
    /// @param expectedOutAmount Expected amount of tokens to be bridged.
    /// @param bridger Bridger address to bridge the tokens through.
    function dumpReceivers(
        address token,
        bytes32[] calldata receiverSalts,
        uint256[] calldata amounts,
        uint256 expectedOutAmount,
        address bridger
    ) external payable {
        require(receiverSalts.length == amounts.length);

        uint256 total = 0;
        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            address receiver = predictReceiverAddress(address(this), receiverSalts[i]);
            uint256 balance = TokenUtils.getBalanceOf(token, receiver);

            uint256 sweepAmount = 0;
            if (balance > 1) {
                unchecked {
                    // Sweep all but one base unit to keep the receiver's balance slot non-zero.
                    sweepAmount = balance - 1;
                }
            }

            require(amounts[i] == sweepAmount, "UntronController: incorrect sweep amount");

            if (sweepAmount != 0) {
                _callReceiver(receiverSalts[i], token, sweepAmount, address(this));
                total += sweepAmount;
            }
        }

        // Initiate bridging if a bridger is provided
        if (bridger != address(0)) {
            _bridge(token, bridger, total, expectedOutAmount);
        }

        emit TokensDumped(token, total);
    }

    /// @notice Transfers tokens from a receiver to a specified recipient.
    /// @dev Callable only by the designated executor.
    function transferFromReceiver(bytes32 receiverSalt, address token, address recipient, uint256 amount)
        external
        onlyExecutor
    {
        _callReceiver(receiverSalt, token, amount, recipient);
    }

    /// @notice Transfers tokens from a receiver to a specified recipient.
    /// @dev Callable only by the designated executor.
    function transferFromController(address token, address recipient, uint256 amount) external onlyExecutor {
        TokenUtils.transfer(token, payable(recipient), amount);
    }

    // Accept native token for bridging fees
    receive() external payable {}

    /*//////////////////////////////////////////////////////////////
                           INTERNAL  HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @dev Deploys receiver contract if missing and performs token transfer.
    function _callReceiver(bytes32 salt, address token, uint256 amount, address recipient)
        internal
        returns (address payable receiver)
    {
        receiver = payable(predictReceiverAddress(address(this), salt));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            receiver = deployReceiver(address(this), salt);
            emit ReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).onControllerCall(token, amount, payable(recipient));

        emit ReceiverCalled(receiver, token, recipient, amount);
    }

    /// @dev Bridges specified amount of tokens via the provided bridger using stored payload.
    function _bridge(address token, address bridger, uint256 inAmount, uint256 outAmount) internal {
        // Verify bridger is still allowed
        require(isBridgerAllowed[bridger], "UntronController: bridger not allowed");

        // Load payload for (token, bridger)
        bytes memory payload = payloadFor[token][bridger];
        require(payload.length != 0, "UntronController: route not set");

        // If the caller attached value, keep it in the controller;
        // the underlying bridger will be able to use it to pay for the bridge call.

        // Execute the bridger via DELEGATECALL, forwarding the required native fee
        bytes memory data = abi.encodeWithSelector(IBridger.bridge.selector, token, inAmount, outAmount, payload);
        (bool ok, bytes memory ret) = bridger.delegatecall(data);
        if (!ok) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }

        emit TokensBridged(token, inAmount, outAmount, bridger);
    }

    /// @dev Reverts if the caller is not the owner.
    ///      Used in an onlyOwner modifier only.
    function _onlyOwner() internal view {
        require(msg.sender == owner, "UntronController: only owner can call this function");
    }

    /// @dev Reverts if the caller is not the executor.
    ///      Used in an onlyExecutor modifier only.
    function _onlyExecutor() internal view {
        require(msg.sender == executor, "UntronController: only executor can call this function");
    }
}
