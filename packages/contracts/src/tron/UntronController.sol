// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronReceiver} from "./UntronReceiver.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";
import {IBridger} from "./bridgers/interfaces/IBridger.sol";

/// @title UntronController
/// @notice Central coordination contract for Untron protocol on Tron-like EVM chains.
/// @author Ultrasound Labs
contract UntronController {
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

    // Pre-computed hash of the UntronReceiver creation bytecode with this controller
    bytes32 private immutable RECEIVER_BYTECODE_HASH;
    // Chain-specific byte prefix used in CREATE2 address calculation (0xff for EVM, 0x41 for Tron).
    bytes1 private immutable CREATE2_PREFIX;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    event OwnerChanged(address indexed newOwner);
    event ExecutorChanged(address indexed newExecutor);
    event BridgerAllowed(address indexed bridger, bool allowed);
    event PayloadSet(address indexed token, address indexed bridger);
    event PayloadCleared(address indexed token, address indexed bridger);
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    event ReceiverCalled(address indexed receiver, address indexed token, address indexed recipient, uint256 amount);
    event TokensDumped(address indexed token, uint256 totalAmount);
    event TokensBridged(address indexed token, uint256 amount, address indexed bridger);

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

    constructor(bytes1 create2Prefix) {
        CREATE2_PREFIX = create2Prefix;
        // Compute and cache the bytecode hash for CREATE2 address derivation
        RECEIVER_BYTECODE_HASH = keccak256(receiverBytecode());
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

    /// @dev Deploys the receiver contract using CREATE2 and the provided salt.
    function deployReceiver(bytes32 salt) public returns (address payable receiver) {
        bytes memory bytecode = receiverBytecode();
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
    function receiverBytecode() public view returns (bytes memory) {
        return abi.encodePacked(type(UntronReceiver).creationCode, abi.encode(address(this)));
    }

    /// @dev Predicts the deterministic address for a receiver deployed via CREATE2.
    function predictReceiverAddress(bytes32 salt) public view returns (address predicted) {
        predicted = address(
            uint160(uint256(keccak256(abi.encodePacked(CREATE2_PREFIX, address(this), salt, RECEIVER_BYTECODE_HASH))))
        );
    }

    /*//////////////////////////////////////////////////////////////
                          EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Dumps tokens into multiple receiver contracts.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @param amounts Corresponding token amounts for each receiver.
    /// @return total Total amount transferred.
    function dumpReceivers(
        address token,
        bytes32[] calldata receiverSalts,
        uint256[] calldata amounts,
        address bridger
    ) external payable returns (uint256 total) {
        require(receiverSalts.length == amounts.length);

        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            _callReceiver(receiverSalts[i], token, amounts[i], address(this));
            total += amounts[i];
        }

        // Initiate bridging if a bridger is provided
        if (bridger != address(0)) {
            _bridge(token, bridger, total);
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
        receiver = payable(predictReceiverAddress(salt));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            receiver = deployReceiver(salt);
            emit ReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).onControllerCall(token, amount, payable(recipient));

        emit ReceiverCalled(receiver, token, recipient, amount);
    }

    /// @dev Bridges specified amount of tokens via the provided bridger using stored payload.
    function _bridge(address token, address bridger, uint256 amount) internal {
        // Verify bridger is still allowed
        require(isBridgerAllowed[bridger], "UntronController: bridger not allowed");

        // Load payload for (token, bridger)
        bytes memory payload = payloadFor[token][bridger];
        require(payload.length != 0, "UntronController: route not set");

        // Quote the native fee
        uint256 fee = IBridger(bridger).quoteFee(token, amount, payload);

        // Ensure the contract has enough native balance to cover the fee
        require(address(this).balance >= fee, "UntronController: insufficient native for fee");

        // If the caller attached value, keep it in the controller;
        // the underlying bridger will be able to use it to pay for the bridge call.

        // Execute the bridger via DELEGATECALL, forwarding the required native fee
        bytes memory data = abi.encodeWithSelector(IBridger.bridge.selector, token, amount, payload);
        (bool ok, bytes memory ret) = bridger.delegatecall(data);
        if (!ok) {
            assembly {
                revert(add(ret, 32), mload(ret))
            }
        }

        emit TokensBridged(token, amount, bridger);
    }

    function _onlyOwner() internal view {
        require(msg.sender == owner, "UntronController: only owner can call this function");
    }

    function _onlyExecutor() internal view {
        require(msg.sender == executor, "UntronController: only executor can call this function");
    }
}
