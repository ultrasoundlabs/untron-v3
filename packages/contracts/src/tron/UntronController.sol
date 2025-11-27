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

    /// @notice Contract owner, can set executor and bridger configuration.
    /// @dev    Used by _onlyOwner (and thus the onlyOwner-protected admin functions).
    ///         Written in constructor and setOwner function.
    address public owner;
    /// @notice Executor, can transfer tokens from controller's balance to arbitrary recipients.
    /// @dev    Used by _onlyExecutor (and thus the onlyExecutor-protected external functions).
    ///         Written in setExecutor function.
    ///         This is a future-proof feature that can be used to implement a protocol
    ///         for swaps into Tron that would reuse liquidity from Untron V3's controller.
    address public executor;

    /// @notice Whitelist of allowed bridger contracts
    /// @dev    Only used in setBridgerAllowed, setPayload and bridge functions.
    mapping(address => bool) public isBridgerAllowed;

    /// @notice token => bridger => bridger-specific payload
    /// @dev    Only used in setPayload and bridge functions.
    mapping(address => mapping(address => bytes)) public payloadFor;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the owner of the contract is changed.
    /// @dev    Only used in setOwner function and emitted in constructor on deployment.
    event OwnerChanged(address indexed newOwner);
    /// @notice Emitted when the executor of the contract is changed.
    /// @dev    Only used in setExecutor function.
    event ExecutorChanged(address indexed newExecutor);
    /// @notice Emitted when a particular bridger is allowed or disallowed.
    /// @dev    Only used in setBridgerAllowed function.
    event BridgerAllowed(address indexed bridger, bool allowed);
    /// @notice Emitted when a payload is set for a particular token+bridger pair.
    /// @dev    Only used in setPayload function.
    event PayloadSet(address indexed token, address indexed bridger);
    /// @notice Emitted when a receiver is deployed.
    /// @dev    Only used in _callReceiver function.
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    /// @notice Emitted when a receiver is called to transfer tokens to a recipient.
    /// @dev    Only used in _callReceiver function.
    event ReceiverCalled(address indexed receiver, address indexed token, address indexed recipient, uint256 amount);
    /// @notice Emitted when tokens are dumped from receivers to the controller.
    /// @dev    Only used in pullFromReceivers function.
    event TokensPulled(address indexed token, uint256 totalAmount);
    /// @notice Emitted when tokens are bridged via a particular bridger.
    /// @dev    Only used in bridge function.
    event TokensBridged(address indexed token, uint256 inAmount, uint256 outAmount, address indexed bridger);
    /// @notice Emitted when tokens are transferred from the controller to a recipient.
    /// @dev    Only used in transferFromController function.
    event ControllerTransfer(address indexed token, address indexed recipient, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                                  ERRORS
    //////////////////////////////////////////////////////////////*/

    /// @notice Error thrown when setting the owner to the zero address.
    /// @dev    Only used in setOwner function.
    error ZeroOwnerAddress();
    /// @notice Error thrown when a function restricted to the contract's owner is called by another address.
    /// @dev    Used by _onlyOwner (and thus the onlyOwner-protected admin functions).
    error OnlyOwner();
    /// @notice Error thrown when a function restricted to the executor is called by another address.
    /// @dev    Used by _onlyExecutor (and thus the onlyExecutor-protected external functions).
    error OnlyExecutor();
    /// @notice Error thrown when the specified bridger is not allowed.
    /// @dev    Only used in setPayload and bridge functions.
    error BridgerNotAllowed();
    /// @notice Error thrown when trying to bridge with an unset route/payload.
    /// @dev    Only used in bridge function.
    error RouteNotSet();
    /// @notice Error thrown when the amount to be swept from receiver does not match the expected value.
    /// @dev    Only used in pullFromReceivers function.
    error IncorrectSweepAmount();
    /// @notice Error thrown when provided lengths of receiverSalts and amounts arrays do not match.
    /// @dev    Only used in pullFromReceivers function.
    error LengthMismatch();
    /// @notice Error thrown when the expected out amount does not match bridger-computed out amount.
    /// @dev    Only used in bridge function.
    error OutAmountMismatch();

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Modifier that restricts a function to be called only by the owner.
    /// @dev    Only used in setExecutor, setBridgerAllowed, setPayload, and setOwner functions.
    modifier onlyOwner() {
        _onlyOwner();
        _;
    }

    /// @notice Modifier that restricts a function to be called only by the executor.
    /// @dev    Only used in transferFromController function.
    modifier onlyExecutor() {
        _onlyExecutor();
        _;
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    /// @notice Constructor.
    /// @param create2Prefix Chain-specific byte prefix used in CREATE2 address calculation.
    /// @dev Initializes CREATE2 utils, sets the owner to the caller, and emits an OwnerChanged event.
    constructor(bytes1 create2Prefix) Create2Utils(create2Prefix) {
        owner = msg.sender;
        emit OwnerChanged(msg.sender);
    }

    /*//////////////////////////////////////////////////////////////
                             ADMIN FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Set the executor.
    /// @param _executor New executor address.
    /// @dev Set to address(0) to disable executor functionality.
    ///      Callable only by the owner.
    function setExecutor(address _executor) external onlyOwner {
        executor = _executor;
        emit ExecutorChanged(_executor);
    }

    /// @notice Set whether a bridger is allowed to bridge tokens.
    /// @param _bridger Bridger address.
    /// @param _allowed Whether the bridger is allowed to bridge tokens.
    /// @dev Callable only by the owner.
    function setBridgerAllowed(address _bridger, bool _allowed) external onlyOwner {
        isBridgerAllowed[_bridger] = _allowed;
        emit BridgerAllowed(_bridger, _allowed);
    }

    /// @notice Set the bridger payload for a particular token+bridger pair.
    /// @param _token Token address.
    /// @param _bridger Bridger address.
    /// @param _payload Bridger-specific payload.
    /// @dev Callable only by the owner.
    function setPayload(address _token, address _bridger, bytes calldata _payload) external onlyOwner {
        if (!isBridgerAllowed[_bridger]) revert BridgerNotAllowed();
        payloadFor[_token][_bridger] = _payload;
        emit PayloadSet(_token, _bridger);
    }

    /// @notice Set the owner of the contract.
    /// @param _newOwner New owner address.
    /// @dev Callable only by the owner. Zero address owner is disallowed.
    function setOwner(address _newOwner) external onlyOwner {
        if (_newOwner == address(0)) revert ZeroOwnerAddress();
        owner = _newOwner;
        emit OwnerChanged(_newOwner);
    }

    /*//////////////////////////////////////////////////////////////
                          EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Pulls tokens from multiple receiver contracts.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @param amounts Expected token amounts to be swept from each receiver.
    ///                amounts[i] must equal the actual sweep amount,
    ///                which is `balance - 1` if `balance > 0`, otherwise `0`.
    /// @dev Callable by anyone.
    ///      In this function, the controller only requests tokens to be sent into *its own balance*.
    ///      Sweeps all but one base unit from each non-zero-balance receiver,
    ///      in order to keep its balance slot non-zero for TRC-20 gas optimization.
    function pullFromReceivers(address token, bytes32[] calldata receiverSalts, uint256[] calldata amounts) external {
        if (receiverSalts.length != amounts.length) revert LengthMismatch();

        uint256 total = 0;
        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            address receiver = predictReceiverAddress(address(this), receiverSalts[i]);
            uint256 balance = TokenUtils.getBalanceOf(token, receiver);

            uint256 sweepAmount = balance;
            if (balance > 0) {
                unchecked {
                    // Sweep all but one base unit to keep the receiver's balance slot non-zero.
                    //
                    // Sending a TRC-20 token to a Tron address which already has some is ~2x cheaper
                    // than sending them to an empty balance slot (65k vs 130k energy for Tron USDT
                    // at the time of writing).
                    //
                    // This is a minor optimization that doesn't change the protocol's
                    // correctness or security.
                    sweepAmount -= 1;
                }
            }

            // Verify the expected amount of tokens to be swept matches the actual amount.
            // — why is this explicit in the calldata?
            // in Tron light client, we can only prove success flag + tx calldata,
            // so we have to pass things we need to prove there explicitly in calldata
            // and revert if it's not equivalent to what's happening in the blockchain state.
            if (amounts[i] != sweepAmount) revert IncorrectSweepAmount();

            if (sweepAmount != 0) {
                _callReceiver(receiverSalts[i], token, sweepAmount, address(this));
                total += sweepAmount;
            }
        }

        emit TokensPulled(token, total);
    }

    /// @notice Bridges specified amount of tokens via the provided bridger using stored payload.
    /// @param token Token address.
    /// @param bridger Bridger address.
    /// @param inAmount Amount of tokens to bridge.
    /// @param outAmount Expected output amount returned by the bridger implementation.
    /// @dev Callable by anyone; uses tokens already held by the controller
    ///      (including TRX value attached to the call, if any).
    ///      Bridgers are DELEGATECALLed in the controller's context
    ///      and are thus strongly encouraged to be stateless.
    function bridge(address token, address bridger, uint256 inAmount, uint256 outAmount) external payable {
        // Verify bridger is allowed
        if (!isBridgerAllowed[bridger]) revert BridgerNotAllowed();

        // Load payload for (token, bridger)
        bytes memory payload = payloadFor[token][bridger];
        if (payload.length == 0) revert RouteNotSet();

        // If the caller attached value, keep it in the controller;
        // the underlying bridger will be able to use it to pay for the bridge call.

        // Execute the bridger via DELEGATECALL.
        // The bridger implementation returns the expected out amount, which we
        // compare against the caller-provided value to enforce invariants at
        // the controller layer.
        bytes memory data = abi.encodeWithSelector(IBridger.bridge.selector, token, inAmount, payload);
        (bool ok, bytes memory ret) = bridger.delegatecall(data);
        if (!ok) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }

        // Verify the expected amount of tokens to be received from the bridge
        // matches the actual amount asserted by the bridge used.
        // — why is this explicit in the calldata?
        // in Tron light client, we can only prove success flag + tx calldata,
        // so we have to pass things we need to prove there explicitly in calldata
        // and revert if it's not equivalent to what's happening in the blockchain state.
        uint256 bridgerOutAmount = abi.decode(ret, (uint256));
        if (bridgerOutAmount != outAmount) revert OutAmountMismatch();

        emit TokensBridged(token, inAmount, outAmount, bridger);
    }

    /// @notice Transfers tokens from the controller to a specified recipient.
    /// @param token Token address.
    /// @param recipient Recipient address.
    /// @param amount Amount of tokens to transfer.
    /// @dev Callable only by the executor.
    function transferFromController(address token, address recipient, uint256 amount) external onlyExecutor {
        TokenUtils.transfer(token, payable(recipient), amount);
        emit ControllerTransfer(token, recipient, amount);
    }

    // Accept native token for bridging fees
    receive() external payable {}

    /*//////////////////////////////////////////////////////////////
                           INTERNAL  HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Deploys receiver contract if missing and performs pull from receiver.
    /// @param salt Salt used for deterministic receiver deployment (CREATE2).
    ///             Receiver salts are used as a canonical identifier for receivers.
    /// @param token Token address.
    /// @param amount Amount of tokens to pull from receiver.
    /// @param recipient Recipient address.
    /// @return receiver Deployed receiver contract address.
    function _callReceiver(bytes32 salt, address token, uint256 amount, address recipient)
        internal
        returns (address payable receiver)
    {
        receiver = payable(predictReceiverAddress(address(this), salt));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            receiver = deployReceiver(salt);
            emit ReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).onControllerCall(token, amount, payable(recipient));

        emit ReceiverCalled(receiver, token, recipient, amount);
    }

    /// @dev Reverts if the caller is not the owner.
    ///      Used in an onlyOwner modifier only.
    function _onlyOwner() internal view {
        if (msg.sender != owner) revert OnlyOwner();
    }

    /// @dev Reverts if the caller is not the executor.
    ///      Used in an onlyExecutor modifier only.
    function _onlyExecutor() internal view {
        if (msg.sender != executor) revert OnlyExecutor();
    }
}
