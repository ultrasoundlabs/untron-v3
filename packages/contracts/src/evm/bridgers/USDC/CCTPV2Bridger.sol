// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IBridger} from "../interfaces/IBridger.sol";
import {TokenUtils} from "../../../utils/TokenUtils.sol";

import {Ownable} from "solady/auth/Ownable.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

/// @title ITokenMessengerV2
/// @notice Minimal interface for Circle CCTP V2 `TokenMessengerV2`.
/// @dev Signature matches Circle's `TokenMessengerV2.depositForBurn`.
/// @author Ultrasound Labs
interface ITokenMessengerV2 {
    /// @notice Burns `burnToken` on the source domain and emits a message that can be relayed to mint on the destination domain.
    /// @param amount Amount of `burnToken` to burn on the source domain (in token's smallest units).
    /// @param destinationDomain Circle domain id of the destination chain (NOT an EVM chainId).
    /// @param mintRecipient Recipient of minted tokens on the destination domain as a `bytes32` (EVM addresses are left-padded).
    /// @param burnToken ERC-20 token address to burn on the source domain (e.g., USDC).
    /// @param destinationCaller If nonzero, restricts who can call `receiveMessage` on the destination; `bytes32(0)` allows anyone.
    /// @param maxFee Maximum fee (in `burnToken` units) to pay for the transfer.
    /// @param minFinalityThreshold Minimum finality threshold required by Circle for the burn event (e.g., 1000 fast, 2000 standard).
    function depositForBurn(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 mintRecipient,
        address burnToken,
        bytes32 destinationCaller,
        uint256 maxFee,
        uint32 minFinalityThreshold
    ) external;
}

/// @title CCTPV2Bridger
/// @notice Simple, stateless CCTP V2 bridger (USDC-only).
/// @dev Uses Standard Transfer params: destinationCaller=0x0 (anyone can relay),
///      maxFee=0, minFinalityThreshold=2000 (standard finality).
/// @author Ultrasound Labs
contract CCTPV2Bridger is IBridger, Ownable {
    error NotUntron();
    error UnsupportedToken(address token);
    error UnsupportedChainId(uint256 chainId);
    error ZeroBeneficiary();
    error ApproveFailed();
    error InsufficientUsdcBalance(uint256 balance, uint256 required);
    error ZeroAddress();
    error ArrayLengthMismatch(uint256 a, uint256 b);
    error DuplicateChainId(uint256 chainId);

    /// @notice The only caller allowed to initiate a burn (expected to be UntronV3).
    address public immutable UNTRON;

    /// @notice Circle TokenMessengerV2 on this chain.
    ITokenMessengerV2 public immutable TOKEN_MESSENGER_V2;

    /// @notice The only supported token (CCTP burns/mints USDC).
    IERC20 public immutable USDC;

    /// @notice EVM chainId -> Circle CCTP domain id.
    /// @dev Circle domains are NOT EVM chainIds.
    mapping(uint256 => uint32) public circleDomainByChainId;

    /// @notice Whether an EVM chainId is supported by this bridger.
    /// @dev Needed because Circle domain id `0` (Ethereum) is valid.
    mapping(uint256 => bool) public isSupportedChainId;

    uint32 internal constant _FINALITY_STANDARD = 2000; // standard finality
    uint256 internal constant _MAX_FEE = 0;

    /// @notice Creates a new CCTP V2 bridger instance.
    /// @param untron The UntronV3 contract allowed to call `bridge`.
    /// @param tokenMessengerV2 Circle `TokenMessengerV2` address on the current chain.
    /// @param usdc USDC token address on the current chain.
    /// @param supportedChainIds Supported destination EVM chain ids.
    /// @param circleDomains Circle CCTP domain ids corresponding 1:1 with `supportedChainIds`.
    constructor(
        address untron,
        address tokenMessengerV2,
        address usdc,
        uint256[] memory supportedChainIds,
        uint32[] memory circleDomains
    ) {
        if (untron == address(0) || tokenMessengerV2 == address(0) || usdc == address(0)) revert ZeroAddress();
        if (supportedChainIds.length != circleDomains.length) {
            revert ArrayLengthMismatch(supportedChainIds.length, circleDomains.length);
        }

        UNTRON = untron;
        TOKEN_MESSENGER_V2 = ITokenMessengerV2(tokenMessengerV2);
        USDC = IERC20(usdc);
        _initializeOwner(msg.sender);

        for (uint256 i = 0; i < supportedChainIds.length; ++i) {
            uint256 chainId = supportedChainIds[i];
            if (isSupportedChainId[chainId]) revert DuplicateChainId(chainId);
            isSupportedChainId[chainId] = true;
            circleDomainByChainId[chainId] = circleDomains[i];
        }
    }

    /// @inheritdoc IBridger
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external {
        if (msg.sender != UNTRON) revert NotUntron();
        if (beneficiary == address(0)) revert ZeroBeneficiary();
        if (token != address(USDC)) revert UnsupportedToken(token);

        uint32 destinationDomain = _circleDomainForChainId(targetChainId);

        // `amount` is the desired mint amount on destination; pay no fees.
        uint256 maxFee = _MAX_FEE;
        uint256 burnAmount = amount;

        uint256 balance = USDC.balanceOf(address(this));
        if (balance < burnAmount) revert InsufficientUsdcBalance(balance, burnAmount);

        // Approve TokenMessengerV2 to pull `burnAmount` USDC from this contract to burn.
        if (!USDC.approve(address(TOKEN_MESSENGER_V2), burnAmount)) revert ApproveFailed();

        // Convert EVM address to bytes32 (left-padded) as required by CCTP.
        bytes32 mintRecipient = bytes32(uint256(uint160(beneficiary)));

        ITokenMessengerV2(TOKEN_MESSENGER_V2)
            .depositForBurn(
                burnAmount,
                destinationDomain,
                mintRecipient,
                token,
                bytes32(0), // destinationCaller = 0 => anyone can call receiveMessage on destination
                maxFee, // maxFee = 0
                _FINALITY_STANDARD
            );
    }

    /// @notice Withdraws tokens accidentally left in this contract.
    /// @dev Owner-only escape hatch. Bridging custody is expected to be ephemeral (UntronV3 transfers in and immediately calls `bridge`).
    /// @param token The ERC-20 token to withdraw.
    /// @param amount Amount of `token` to withdraw.
    function withdraw(address token, uint256 amount) external onlyOwner {
        TokenUtils.transfer(token, payable(msg.sender), amount);
    }

    /// @notice Maps an EVM `chainId` to a Circle CCTP domain id.
    /// @dev This mapping must match Circle's published domain table.
    /// @param chainId EVM chain id of the destination chain.
    /// @return Circle CCTP domain id for `chainId`.
    function _circleDomainForChainId(uint256 chainId) internal view returns (uint32) {
        if (!isSupportedChainId[chainId]) revert UnsupportedChainId(chainId);
        return circleDomainByChainId[chainId];
    }
}
