// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {EventChainGenesis} from "../utils/EventChainGenesis.sol";

/// @title  UntronV3Index
/// @notice Hash-chain-based event index for Untron V3 hub, friendly to offchain indexers.
/// @dev    UntronV3 must not emit events itself. All events must be defined and emitted through UntronV3Index.
/// @author Ultrasound Labs
contract UntronV3Index {
    /*//////////////////////////////////////////////////////////////
                                INDEXES
    //////////////////////////////////////////////////////////////*/

    /// @notice The hash of the latest event in the event chain.
    /// @dev    This is used to reconstruct all events that have ever been emitted through this contract.
    bytes32 public eventChainTip = EventChainGenesis.UntronControllerIndex;

    // TODO: make per-event sig or per-object event chains

    /*//////////////////////////////////////////////////////////////
                                  EVENTS
    //////////////////////////////////////////////////////////////*/

    event UsdtSet(address indexed usdt);
    event RealtorSet(address indexed realtor, bool allowed);
    event ChainDeprecatedSet(uint256 indexed targetChainId, bool deprecated);
    event ProtocolFloorSet(uint256 floorPpm);
    event RealtorMinFeeSet(address indexed realtor, uint256 minFeePpm);
    event ProtocolLeaseRateLimitSet(uint256 maxLeases, uint256 windowSeconds);
    event RealtorLeaseRateLimitSet(address indexed realtor, uint8 mode, uint256 maxLeases, uint256 windowSeconds);
    event LesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds);
    event TronUsdtSet(address indexed tronUsdt);
    event SwapRateSet(address indexed targetToken, uint256 ratePpm);
    event BridgerSet(address indexed targetToken, uint256 indexed targetChainId, address bridger);

    event LeaseCreated(
        uint256 indexed leaseId,
        bytes32 indexed receiverSalt,
        address realtor,
        address lessee,
        uint64 startTime,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee
    );

    event PayoutConfigUpdated(uint256 indexed leaseId, uint256 targetChainId, address targetToken, address beneficiary);

    // forge-lint: disable-next-line(mixed-case-variable)
    event ClaimCreated(uint256 indexed claimIndex, uint256 indexed leaseId, uint256 amountUSDT);
    // forge-lint: disable-next-line(mixed-case-variable)
    event ClaimFilled(uint256 indexed claimIndex, uint256 indexed leaseId, uint256 amountUSDT);

    event DepositPreEntitled(bytes32 indexed txId, uint256 indexed leaseId, uint256 rawAmount, uint256 netOut);

    event LpDeposited(address indexed lp, uint256 amount);
    event LpWithdrawn(address indexed lp, uint256 amount);
    event TronReaderSet(address indexed reader);

    event ControllerEventChainTipUpdated(
        bytes32 previousTip,
        uint256 indexed blockNumber,
        uint256 blockTimestamp,
        bytes32 indexed eventSignature,
        bytes abiEncodedEventData
    );

    // Protocol PnL update reason codes.
    enum PnlReason {
        FEE, // positive
        REBALANCE, // positive
        WITHDRAWAL, // negative
        RECEIVER_PULL_NO_LEASE // positive
    }

    event ProtocolPnlUpdated(int256 pnl, int256 delta, PnlReason reason);
    event LeaseNonceUpdated(uint256 indexed leaseId, uint256 nonce);
    event TokensRescued(address token, uint256 amount);

    /// @dev The ownership is transferred from `oldOwner` to `newOwner`.
    /// This event is intentionally kept the same as OpenZeppelin's Ownable to be
    /// compatible with indexers and [EIP-173](https://eips.ethereum.org/EIPS/eip-173),
    /// despite it not being as lightweight as a single argument event.
    event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

    /*//////////////////////////////////////////////////////////////
                APPEND EVENT CHAIN IMPLEMENTATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Appends an event to the event chain.
    /// @param eventSignature The signature of the event.
    /// @param abiEncodedEventData The ABI-encoded data of the event.
    function _appendEventChain(bytes32 eventSignature, bytes memory abiEncodedEventData) internal {
        eventChainTip =
            sha256(abi.encodePacked(eventChainTip, block.number, block.timestamp, eventSignature, abiEncodedEventData));
    }

    /*//////////////////////////////////////////////////////////////
                            EMITTERS
    //////////////////////////////////////////////////////////////*/

    function _emitUsdtSet(address usdt_) internal {
        _appendEventChain(UsdtSet.selector, abi.encode(usdt_));
        emit UsdtSet(usdt_);
    }

    function _emitRealtorSet(address realtor, bool allowed) internal {
        _appendEventChain(RealtorSet.selector, abi.encode(realtor, allowed));
        emit RealtorSet(realtor, allowed);
    }

    function _emitChainDeprecatedSet(uint256 targetChainId, bool deprecated) internal {
        _appendEventChain(ChainDeprecatedSet.selector, abi.encode(targetChainId, deprecated));
        emit ChainDeprecatedSet(targetChainId, deprecated);
    }

    function _emitProtocolFloorSet(uint256 floorPpm) internal {
        _appendEventChain(ProtocolFloorSet.selector, abi.encode(floorPpm));
        emit ProtocolFloorSet(floorPpm);
    }

    function _emitRealtorMinFeeSet(address realtor, uint256 minFeePpm) internal {
        _appendEventChain(RealtorMinFeeSet.selector, abi.encode(realtor, minFeePpm));
        emit RealtorMinFeeSet(realtor, minFeePpm);
    }

    function _emitProtocolLeaseRateLimitSet(uint256 maxLeases, uint256 windowSeconds) internal {
        _appendEventChain(ProtocolLeaseRateLimitSet.selector, abi.encode(maxLeases, windowSeconds));
        emit ProtocolLeaseRateLimitSet(maxLeases, windowSeconds);
    }

    function _emitLesseePayoutConfigRateLimitSet(uint256 maxUpdates, uint256 windowSeconds) internal {
        _appendEventChain(LesseePayoutConfigRateLimitSet.selector, abi.encode(maxUpdates, windowSeconds));
        emit LesseePayoutConfigRateLimitSet(maxUpdates, windowSeconds);
    }

    function _emitRealtorLeaseRateLimitSet(address realtor, uint8 mode, uint256 maxLeases, uint256 windowSeconds)
        internal
    {
        _appendEventChain(RealtorLeaseRateLimitSet.selector, abi.encode(realtor, mode, maxLeases, windowSeconds));
        emit RealtorLeaseRateLimitSet(realtor, mode, maxLeases, windowSeconds);
    }

    function _emitTronReaderSet(address reader) internal {
        _appendEventChain(TronReaderSet.selector, abi.encode(reader));
        emit TronReaderSet(reader);
    }

    function _emitTronUsdtSet(address tronUsdt) internal {
        _appendEventChain(TronUsdtSet.selector, abi.encode(tronUsdt));
        emit TronUsdtSet(tronUsdt);
    }

    function _emitSwapRateSet(address targetToken, uint256 ratePpm) internal {
        _appendEventChain(SwapRateSet.selector, abi.encode(targetToken, ratePpm));
        emit SwapRateSet(targetToken, ratePpm);
    }

    function _emitBridgerSet(address targetToken, uint256 targetChainId, address bridger) internal {
        _appendEventChain(BridgerSet.selector, abi.encode(targetToken, targetChainId, bridger));
        emit BridgerSet(targetToken, targetChainId, bridger);
    }

    function _emitLeaseCreated(
        uint256 leaseId,
        bytes32 receiverSalt,
        address realtor,
        address lessee,
        uint64 startTime,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee
    ) internal {
        _appendEventChain(
            LeaseCreated.selector,
            abi.encode(leaseId, receiverSalt, realtor, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee)
        );
        emit LeaseCreated(leaseId, receiverSalt, realtor, lessee, startTime, nukeableAfter, leaseFeePpm, flatFee);
    }

    function _emitPayoutConfigUpdated(uint256 leaseId, uint256 targetChainId, address targetToken, address beneficiary)
        internal
    {
        _appendEventChain(PayoutConfigUpdated.selector, abi.encode(leaseId, targetChainId, targetToken, beneficiary));
        emit PayoutConfigUpdated(leaseId, targetChainId, targetToken, beneficiary);
    }

    function _emitControllerEventChainTipUpdated(
        bytes32 previousTip,
        uint256 blockNumber,
        uint256 blockTimestamp,
        bytes32 eventSignature,
        bytes memory abiEncodedEventData
    ) internal {
        _appendEventChain(
            ControllerEventChainTipUpdated.selector,
            abi.encode(previousTip, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData)
        );
        emit ControllerEventChainTipUpdated(
            previousTip, blockNumber, blockTimestamp, eventSignature, abiEncodedEventData
        );
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _emitClaimCreated(uint256 claimIndex, uint256 leaseId, uint256 amountUSDT) internal {
        _appendEventChain(ClaimCreated.selector, abi.encode(claimIndex, leaseId, amountUSDT));
        emit ClaimCreated(claimIndex, leaseId, amountUSDT);
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function _emitClaimFilled(uint256 claimIndex, uint256 leaseId, uint256 amountUSDT) internal {
        _appendEventChain(ClaimFilled.selector, abi.encode(claimIndex, leaseId, amountUSDT));
        emit ClaimFilled(claimIndex, leaseId, amountUSDT);
    }

    function _emitDepositPreEntitled(bytes32 txId, uint256 leaseId, uint256 rawAmount, uint256 netOut) internal {
        _appendEventChain(DepositPreEntitled.selector, abi.encode(txId, leaseId, rawAmount, netOut));
        emit DepositPreEntitled(txId, leaseId, rawAmount, netOut);
    }

    function _emitLpDeposited(address lp, uint256 amount) internal {
        _appendEventChain(LpDeposited.selector, abi.encode(lp, amount));
        emit LpDeposited(lp, amount);
    }

    function _emitLpWithdrawn(address lp, uint256 amount) internal {
        _appendEventChain(LpWithdrawn.selector, abi.encode(lp, amount));
        emit LpWithdrawn(lp, amount);
    }

    function _emitProtocolPnlUpdated(int256 pnl, int256 delta, PnlReason reason) internal {
        _appendEventChain(ProtocolPnlUpdated.selector, abi.encode(pnl, delta, reason));
        emit ProtocolPnlUpdated(pnl, delta, reason);
    }

    function _emitLeaseNonceUpdated(uint256 leaseId, uint256 nonce) internal {
        _appendEventChain(LeaseNonceUpdated.selector, abi.encode(leaseId, nonce));
        emit LeaseNonceUpdated(leaseId, nonce);
    }

    function _emitTokensRescued(address token, uint256 amount) internal {
        _appendEventChain(TokensRescued.selector, abi.encode(token, amount));
        emit TokensRescued(token, amount);
    }

    function _emitOwnershipTransferred(address oldOwner, address newOwner) internal {
        _appendEventChain(OwnershipTransferred.selector, abi.encode(oldOwner, newOwner));
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}

/// @notice Fork of Solady's Ownable to work with Untron V3's event chain index
/// @author Ultrasound Labs
/// @author Forked from Solady (https://github.com/vectorized/solady/blob/main/src/auth/Ownable.sol)
///
/// @dev Note:
/// This implementation does NOT auto-initialize the owner to `msg.sender`.
/// You MUST call the `_initializeOwner` in the constructor / initializer.
abstract contract UntronV3IndexedOwnable is UntronV3Index {
    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                       CUSTOM ERRORS                        */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The caller is not authorized to call the function.
    error Unauthorized();

    /// @dev The `newOwner` cannot be the zero address.
    error NewOwnerIsZeroAddress();

    /// @dev Cannot double-initialize.
    error AlreadyInitialized();

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          STORAGE                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev The owner slot is given by:
    /// `bytes32(~uint256(uint32(bytes4(keccak256("_OWNER_SLOT_NOT")))))`.
    /// It is intentionally chosen to be a high value
    /// to avoid collision with lower slots.
    /// The choice of manual storage layout is to enable compatibility
    /// with both regular and upgradeable contracts.
    bytes32 internal constant _OWNER_SLOT = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffff74873927;

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                     INTERNAL FUNCTIONS                     */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Override to return true to make `_initializeOwner` prevent double-initialization.
    function _guardInitializeOwner() internal pure virtual returns (bool guard) {}

    /// @dev Initializes the owner directly without authorization guard.
    /// This function must be called upon initialization,
    /// regardless of whether the contract is upgradeable or not.
    /// This is to enable generalization to both regular and upgradeable contracts,
    /// and to save gas in case the initial owner is not the caller.
    /// For performance reasons, this function will not check if there
    /// is an existing owner.
    function _initializeOwner(address newOwner) internal virtual {
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                if sload(ownerSlot) {
                    mstore(0x00, 0x0dc149f0) // `AlreadyInitialized()`.
                    revert(0x1c, 0x04)
                }
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(_OWNER_SLOT, newOwner)
            }
        }
        _emitOwnershipTransferred(address(0), newOwner);
    }

    /// @dev Sets the owner directly without authorization guard.
    function _setOwner(address newOwner) internal virtual {
        address oldOwner;
        /// @solidity memory-safe-assembly
        assembly {
            oldOwner := sload(_OWNER_SLOT)
        }
        if (_guardInitializeOwner()) {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(ownerSlot, or(newOwner, shl(255, iszero(newOwner))))
            }
        } else {
            /// @solidity memory-safe-assembly
            assembly {
                let ownerSlot := _OWNER_SLOT
                // Clean the upper 96 bits.
                newOwner := shr(96, shl(96, newOwner))
                // Store the new value.
                sstore(ownerSlot, newOwner)
            }
        }
        _emitOwnershipTransferred(oldOwner, newOwner);
    }

    /// @dev Throws if the sender is not the owner.
    function _checkOwner() internal view virtual {
        /// @solidity memory-safe-assembly
        assembly {
            // If the caller is not the stored owner, revert.
            if iszero(eq(caller(), sload(_OWNER_SLOT))) {
                mstore(0x00, 0x82b42900) // `Unauthorized()`.
                revert(0x1c, 0x04)
            }
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                  PUBLIC UPDATE FUNCTIONS                   */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Allows the owner to transfer the ownership to `newOwner`.
    function transferOwnership(address newOwner) public payable virtual onlyOwner {
        /// @solidity memory-safe-assembly
        assembly {
            if iszero(shl(96, newOwner)) {
                mstore(0x00, 0x7448fbae) // `NewOwnerIsZeroAddress()`.
                revert(0x1c, 0x04)
            }
        }
        _setOwner(newOwner);
    }

    /// @dev Allows the owner to renounce their ownership.
    function renounceOwnership() public payable virtual onlyOwner {
        _setOwner(address(0));
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                   PUBLIC READ FUNCTIONS                    */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Returns the owner of the contract.
    function owner() public view virtual returns (address result) {
        /// @solidity memory-safe-assembly
        assembly {
            result := sload(_OWNER_SLOT)
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                         MODIFIERS                          */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    /// @dev Marks a function as only callable by the owner.
    modifier onlyOwner() virtual {
        _checkOwner();
        _;
    }
}
