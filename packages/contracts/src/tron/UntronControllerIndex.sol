// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {EventChainGenesis} from "../utils/EventChainGenesis.sol";

/// @title UntronControllerIndex
/// @notice Hash-chain-based event index for Untron Controller friendly to offchain indexers.
/// @author Ultrasound Labs
contract UntronControllerIndex {
    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = EventChainGenesis.UntronControllerIndex;

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emitted when the owner of the contract is changed.
    /// @dev    Only used in setOwner function and emitted in constructor on deployment.
    /// @param newOwner New owner address.
    event OwnerChanged(address indexed newOwner);
    /// @notice Emitted when the executor of the contract is changed.
    /// @dev    Only used in setExecutor function.
    /// @param newExecutor New executor address.
    event ExecutorChanged(address indexed newExecutor);
    /// @notice Emitted when a payload is set for a particular rebalancer.
    /// @dev    Only used in setPayload function.
    /// @param rebalancer Rebalancer address.
    /// @param payload Rebalancer-specific payload.
    event PayloadSet(address indexed rebalancer, bytes payload);
    /// @notice Emitted when a receiver is deployed.
    /// @dev    Only used in _pullFromReceiver function.
    /// @param receiver Receiver address.
    /// @param salt Salt used for deterministic deployment (CREATE2).
    event ReceiverDeployed(address indexed receiver, bytes32 salt);
    /// @notice Emitted for each receiver when tokens are swept into accounting.
    /// @dev    Includes source token amount, exchange rate (1 for USDT), and USDT-equivalent amount.
    /// @param receiverSalt Salt used for deterministic receiver deployment (CREATE2).
    /// @param token Token address.
    /// @param tokenAmount Amount of token pulled from receiver.
    /// @param exchangeRate Scaled exchange rate used for conversion.
    /// @param usdtAmount USDT-equivalent amount accounted for this pull.
    event PulledFromReceiver(
        bytes32 indexed receiverSalt,
        address indexed token,
        uint256 tokenAmount,
        uint256 exchangeRate,
        uint256 usdtAmount
    );
    /// @notice Emitted when USDT is rebalanced via a particular rebalancer.
    /// @dev    Only used in rebalanceUsdt function.
    /// @param inAmount Amount of USDT bridged in.
    /// @param outAmount Amount of USDT expected out (as asserted by the rebalancer).
    /// @param rebalancer Rebalancer used.
    event UsdtRebalanced(uint256 inAmount, uint256 outAmount, address indexed rebalancer);
    /// @notice Emitted when USDT is transferred from the controller to a recipient.
    /// @dev    Only used in transferUsdtFromController function.
    /// @param recipient Recipient address.
    /// @param amount Amount of USDT transferred.
    event ControllerUsdtTransfer(address indexed recipient, uint256 amount);
    /// @notice Emitted when the canonical USDT token is set or updated.
    /// @dev    Only used in setUsdt function.
    /// @param newUsdt New USDT token address.
    event UsdtSet(address indexed newUsdt);
    /// @notice Emitted when the LP address is set or updated.
    /// @dev    Only used in setLp function.
    /// @param newLp New LP address.
    event LpSet(address indexed newLp);
    /// @notice Emitted when the LP updates the exchange rate for a token.
    /// @dev    Only used in setLpExchangeRate function.
    /// @param token Token address.
    /// @param exchangeRate Scaled exchange rate set by the LP.
    event LpExchangeRateSet(address indexed token, uint256 exchangeRate);
    /// @notice Emitted when the LP withdraws purchased tokens from the controller.
    /// @dev    Only used in lpWithdrawTokens function.
    /// @param token Token withdrawn.
    /// @param amount Amount withdrawn.
    event LpTokensWithdrawn(address indexed token, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                TRON-SPECIFIC CALLDATA GETTER
    //////////////////////////////////////////////////////////////*/

    /// @notice Checks if the provided event chain tip matches the current event chain tip.
    /// @param eventChainTip_ The event chain tip to check.
    /// @return True if the event chain tips match, reverts otherwise.
    /// @dev This is needed because Tron light client doesn't have access to the state root,
    ///      only successful transactions in the block. So we can expose the event chain tip
    ///      to the light client by allowing users to send transactions
    ///      where the event chain tip is in transaction's calldata.
    function isEventChainTip(bytes32 eventChainTip_) external view returns (bool) {
        // i don't think this one needs a custom error
        // solhint-disable-next-line gas-custom-errors
        require(eventChainTip == eventChainTip_, "no");
        return true;
    }

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature of the event.
    /// @param abiEncodedEventData The ABI-encoded data of the event.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        // we use sha256 here instead of keccak256 for future-proofness
        // in case we ZK prove this smart contract. sha256 is cheaper to prove than keccak256.
        eventChainTip =
            sha256(abi.encodePacked(eventChainTip, block.number, block.timestamp, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    /// @notice Emits {OwnerChanged} and appends it to the event chain.
    /// @param newOwner New owner address.
    function _emitOwnerChanged(address newOwner) internal {
        _appendEventChain(OwnerChanged.selector, abi.encode(newOwner));
        emit OwnerChanged(newOwner);
    }

    /// @notice Emits {ExecutorChanged} and appends it to the event chain.
    /// @param newExecutor New executor address.
    function _emitExecutorChanged(address newExecutor) internal {
        _appendEventChain(ExecutorChanged.selector, abi.encode(newExecutor));
        emit ExecutorChanged(newExecutor);
    }

    /// @notice Emits {PayloadSet} and appends it to the event chain.
    /// @param rebalancer Rebalancer address.
    /// @param payload Rebalancer-specific payload.
    function _emitPayloadSet(address rebalancer, bytes memory payload) internal {
        _appendEventChain(PayloadSet.selector, abi.encode(rebalancer, payload));
        emit PayloadSet(rebalancer, payload);
    }

    /// @notice Emits {ReceiverDeployed} and appends it to the event chain.
    /// @param receiver Receiver address.
    /// @param salt Salt used for deterministic deployment (CREATE2).
    function _emitReceiverDeployed(address receiver, bytes32 salt) internal {
        _appendEventChain(ReceiverDeployed.selector, abi.encode(receiver, salt));
        emit ReceiverDeployed(receiver, salt);
    }

    /// @notice Emits {PulledFromReceiver} and appends it to the event chain.
    /// @param receiverSalt Salt used for deterministic receiver deployment (CREATE2).
    /// @param token Token address.
    /// @param tokenAmount Amount of token pulled from receiver.
    /// @param exchangeRate Scaled exchange rate used for conversion.
    /// @param usdtAmount USDT-equivalent amount accounted for this pull.
    function _emitPulledFromReceiver(
        bytes32 receiverSalt,
        address token,
        uint256 tokenAmount,
        uint256 exchangeRate,
        uint256 usdtAmount
    ) internal {
        _appendEventChain(
            PulledFromReceiver.selector, abi.encode(receiverSalt, token, tokenAmount, exchangeRate, usdtAmount)
        );
        emit PulledFromReceiver(receiverSalt, token, tokenAmount, exchangeRate, usdtAmount);
    }

    /// @notice Emits {UsdtRebalanced} and appends it to the event chain.
    /// @param inAmount Amount of USDT bridged in.
    /// @param outAmount Amount of USDT expected out (as asserted by the rebalancer).
    /// @param rebalancer Rebalancer used.
    function _emitUsdtRebalanced(uint256 inAmount, uint256 outAmount, address rebalancer) internal {
        _appendEventChain(UsdtRebalanced.selector, abi.encode(inAmount, outAmount, rebalancer));
        emit UsdtRebalanced(inAmount, outAmount, rebalancer);
    }

    /// @notice Emits {ControllerUsdtTransfer} and appends it to the event chain.
    /// @param recipient Recipient address.
    /// @param amount Amount of USDT transferred.
    function _emitControllerUsdtTransfer(address recipient, uint256 amount) internal {
        _appendEventChain(ControllerUsdtTransfer.selector, abi.encode(recipient, amount));
        emit ControllerUsdtTransfer(recipient, amount);
    }

    /// @notice Emits {UsdtSet} and appends it to the event chain.
    /// @param newUsdt New USDT token address.
    function _emitUsdtSet(address newUsdt) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(newUsdt));
        emit UsdtSet(newUsdt);
    }

    /// @notice Emits {LpSet} and appends it to the event chain.
    /// @param newLp New LP address.
    function _emitLpSet(address newLp) internal {
        _appendEventChain(LpSet.selector, abi.encode(newLp));
        emit LpSet(newLp);
    }

    /// @notice Emits {LpExchangeRateSet} and appends it to the event chain.
    /// @param token Token address.
    /// @param exchangeRate Scaled exchange rate set by the LP.
    function _emitLpExchangeRateSet(address token, uint256 exchangeRate) internal {
        _appendEventChain(LpExchangeRateSet.selector, abi.encode(token, exchangeRate));
        emit LpExchangeRateSet(token, exchangeRate);
    }

    /// @notice Emits {LpTokensWithdrawn} and appends it to the event chain.
    /// @param token Token withdrawn.
    /// @param amount Amount withdrawn.
    function _emitLpTokensWithdrawn(address token, uint256 amount) internal {
        _appendEventChain(LpTokensWithdrawn.selector, abi.encode(token, amount));
        emit LpTokensWithdrawn(token, amount);
    }
}
