// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

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
    /// @param minFinalityThreshold Minimum finality threshold required by Circle for the burn event (e.g., 1000 for fast finality).
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
///      maxFee=1 bps (rounded up), minFinalityThreshold=1000 (fast finality).
/// @author Ultrasound Labs
contract CCTPV2Bridger is IBridger, Ownable {
    error NotUntron();
    error UnsupportedToken(address token);
    error UnsupportedChainId(uint256 chainId);
    error ZeroBeneficiary();
    error ApproveFailed();
    error InsufficientUsdcBalance(uint256 balance, uint256 required);

    /// @notice The only caller allowed to initiate a burn (expected to be UntronV3).
    address public immutable UNTRON;

    /// @notice Circle TokenMessengerV2 on this chain.
    ITokenMessengerV2 public immutable TOKEN_MESSENGER_V2;

    /// @notice The only supported token (CCTP burns/mints USDC).
    IERC20 public immutable USDC;

    uint32 internal constant _FINALITY_STANDARD = 1000; // fast finality
    uint256 internal constant _ONE_BPS_DENOMINATOR = 10_000;

    /// @notice Creates a new CCTP V2 bridger instance.
    /// @param untron The UntronV3 contract allowed to call `bridge`.
    /// @param tokenMessengerV2 Circle `TokenMessengerV2` address on the current chain.
    /// @param usdc USDC token address on the current chain.
    constructor(address untron, address tokenMessengerV2, address usdc) {
        UNTRON = untron;
        TOKEN_MESSENGER_V2 = ITokenMessengerV2(tokenMessengerV2);
        USDC = IERC20(usdc);
        _initializeOwner(msg.sender);
    }

    /// @inheritdoc IBridger
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external {
        if (msg.sender != UNTRON) revert NotUntron();
        if (beneficiary == address(0)) revert ZeroBeneficiary();
        if (token != address(USDC)) revert UnsupportedToken(token);

        uint32 destinationDomain = _circleDomainForChainId(targetChainId);

        // `amount` is the desired mint amount on destination; provide the maxFee from this contract's balance.
        uint256 maxFee = amount / _ONE_BPS_DENOMINATOR;
        if (amount % _ONE_BPS_DENOMINATOR != 0) ++maxFee;
        uint256 burnAmount = amount + maxFee;

        uint256 balance = USDC.balanceOf(address(this));
        if (balance < burnAmount) revert InsufficientUsdcBalance(balance, burnAmount);

        // Approve TokenMessengerV2 to pull `burnAmount` USDC from this contract to burn (amount + fee).
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
                maxFee, // maxFee = 1 bps (rounded up)
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
    /// @dev Circle domains are NOT EVM chainIds; this mapping must match Circle's published domain table.
    /// @param chainId EVM chain id of the destination chain.
    /// @return Circle CCTP domain id for `chainId`.
    function _circleDomainForChainId(uint256 chainId) internal pure returns (uint32) {
        // TODO: switch to a constant mapping filled at constructor

        if (chainId == 1) return 0; // Ethereum -> domain 0
        if (chainId == 43114) return 1; // Avalanche -> domain 1
        if (chainId == 10) return 2; // OP Mainnet -> domain 2
        // if (chainId == 42161) return 3;     // Arbitrum One -> domain 3
        if (chainId == 8453) return 6; // Base -> domain 6
        if (chainId == 137) return 7; // Polygon PoS -> domain 7
        if (chainId == 130) return 10; // Unichain -> domain 10
        if (chainId == 59144) return 11; // Linea -> domain 11
        if (chainId == 146) return 13; // Sonic -> domain 13
        if (chainId == 480) return 14; // World Chain -> domain 14
        if (chainId == 999) return 19; // HyperEVM -> domain 19
        if (chainId == 57073) return 21; // Ink -> domain 21

        revert UnsupportedChainId(chainId);
    }
}
