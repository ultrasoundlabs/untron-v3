// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {LibClone} from "solady/utils/LibClone.sol";
import {UntronReceiver} from "./UntronReceiver.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";
import {IOFT, SendParam, OFTReceipt} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

/// @title UntronController
/// @notice Central coordination contract for Untron protocol on Tron-like EVM chains.
/// @author Ultrasound Labs
contract UntronController {
    /*//////////////////////////////////////////////////////////////
                                STRUCTS
    //////////////////////////////////////////////////////////////*/

    struct LZBridgeConfig {
        address bridge; // OFT (pure) or OFT-Adapter UntronController will call
        uint32 dstEid; // LayerZero dst endpoint id
        bytes32 recipient; // e.g. 0x000... + EVM address, as per LZ spec
        uint16 slippageBps; // minOut guard
        address refundAddress; // refund for leftover native fee
        bytes extraOptions; // LZ extra options
    }

    /*//////////////////////////////////////////////////////////////
                                 STORAGE
    //////////////////////////////////////////////////////////////*/

    // Contract owner, can set executor and bridge data
    address public owner;
    // Executor, can transfer tokens from receivers to arbitrary recipients
    address public executor;

    // Token -> how it should be bridged via an LZ-compatible bridge
    mapping(address => LZBridgeConfig) public bridgeConfig;

    // Implementation to clone for receivers
    address public immutable receiverImplementation;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    event OwnerChanged(address indexed newOwner);
    event ExecutorChanged(address indexed newExecutor);
    event BridgeSet(address indexed token, LZBridgeConfig bridgeConfig_);
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    event ReceiverCalled(address indexed receiver, address indexed token, address indexed recipient, uint256 amount);
    event TokensDumped(address indexed token, uint256 totalAmount);

    /*//////////////////////////////////////////////////////////////
                                MODIFIERS
    //////////////////////////////////////////////////////////////*/

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    modifier onlyExecutor() {
        require(msg.sender == executor);
        _;
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor() {
        receiverImplementation = address(new UntronReceiver(address(this)));
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

    function setBridge(address _token, LZBridgeConfig calldata _bridgeConfig) external onlyOwner {
        // TODO: automatically max approve to save gas
        bridgeConfig[_token] = _bridgeConfig;
        emit BridgeSet(_token, _bridgeConfig);
    }

    function setOwner(address _newOwner) external onlyOwner {
        require(_newOwner != address(0));
        owner = _newOwner;
        emit OwnerChanged(_newOwner);
    }

    function drain() external onlyOwner {
        payable(owner).transfer(address(this).balance);
    }

    /*//////////////////////////////////////////////////////////////
                          PUBLIC/EXTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Dumps tokens into multiple receiver contracts.
    /// @param token Token address.
    /// @param receiverSalts Array of salts used for deterministic receiver
    ///                      deployment (CREATE2).
    /// @param amounts Corresponding token amounts for each receiver.
    /// @return total Total amount transferred.
    function dumpReceivers(address token, bytes32[] calldata receiverSalts, uint256[] calldata amounts)
        external
        payable
        returns (uint256 total)
    {
        require(receiverSalts.length == amounts.length);

        for (uint256 i = 0; i < receiverSalts.length; ++i) {
            _callReceiver(receiverSalts[i], token, amounts[i], address(this));
            total += amounts[i];
        }

        // initiate bridging if bridge config is present for this token
        if (bridgeConfig[token].bridge != address(0)) {
            _bridge(token, total, bridgeConfig[token]);
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

    /*//////////////////////////////////////////////////////////////
                           INTERNAL  HELPERS
    //////////////////////////////////////////////////////////////*/

    /// @dev Deploys receiver contract if missing and performs token transfer.
    function _callReceiver(bytes32 salt, address token, uint256 amount, address recipient)
        internal
        returns (address payable receiver)
    {
        bytes memory args = abi.encode(address(this));
        receiver = payable(LibClone.predictDeterministicAddress(receiverImplementation, args, salt, address(this)));

        // Deploy if not already deployed
        if (receiver.code.length == 0) {
            require(receiverImplementation != address(0));
            receiver = payable(LibClone.cloneDeterministic(receiverImplementation, args, salt));
            emit ReceiverDeployed(receiver, salt);
        }

        UntronReceiver(receiver).onControllerCall(token, amount, payable(recipient));

        emit ReceiverCalled(receiver, token, recipient, amount);
    }

    /// @dev Bridges specified amount of tokens through LayerZero-compliant bridge.
    function _bridge(address token, uint256 amount, LZBridgeConfig memory cfg) internal {
        // Approve tokens to the bridge if necessary
        TokenUtils.approve(token, cfg.bridge, amount);

        // Calculate minimum amount after slippage
        uint256 minAmount = amount * (10000 - cfg.slippageBps) / 10000;

        // Build LayerZero OFT SendParam
        SendParam memory sp = SendParam({
            dstEid: cfg.dstEid,
            to: cfg.recipient,
            amountLD: amount,
            minAmountLD: minAmount,
            extraOptions: cfg.extraOptions,
            composeMsg: "",
            oftCmd: ""
        });

        // Quote fee (native token)
        MessagingFee memory fee = IOFT(cfg.bridge).quoteSend(sp, false);

        // Ensure the contract has enough native balance to cover the fee
        require(address(this).balance >= fee.nativeFee, "UntronController: insufficient native for fee");

        // Perform the send; forward the exact fee.nativeFee as msg.value
        IOFT(cfg.bridge).send{value: fee.nativeFee}(sp, fee, cfg.refundAddress);
    }

    // Accept native token for LayerZero messaging
    receive() external payable {}
}
