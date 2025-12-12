// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IBridger} from "../interfaces/IBridger.sol";
import {TokenUtils} from "../../../utils/TokenUtils.sol";

import {Ownable} from "solady/auth/Ownable.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

/// @dev Minimal interface for Circle CCTP V2 TokenMessengerV2.
/// Signature matches Circle's TokenMessengerV2 depositForBurn.
interface ITokenMessengerV2 {
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

/// @notice Simple, stateless CCTP V2 bridger (USDC-only).
/// @dev Uses Standard Transfer params: destinationCaller=0x0 (anyone can relay),
///      maxFee=0, minFinalityThreshold=2000 (standard).
contract CCTPV2Bridger is IBridger, Ownable {
    error NotUntron();
    error UnsupportedToken(address token);
    error UnsupportedChainId(uint256 chainId);
    error ZeroBeneficiary();
    error ApproveFailed();

    /// @notice The only caller allowed to initiate a burn (expected to be UntronV3).
    address public immutable untron;

    /// @notice Circle TokenMessengerV2 on this chain.
    ITokenMessengerV2 public immutable tokenMessengerV2;

    /// @notice The only supported token (CCTP burns/mints USDC).
    IERC20 public immutable usdc;

    uint32 internal constant FINALITY_STANDARD = 2000;

    constructor(address _untron, address _tokenMessengerV2, address _usdc) {
        untron = _untron;
        tokenMessengerV2 = ITokenMessengerV2(_tokenMessengerV2);
        usdc = IERC20(_usdc);
        _initializeOwner(msg.sender);
    }

    /// @inheritdoc IBridger
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external {
        if (msg.sender != untron) revert NotUntron();
        if (beneficiary == address(0)) revert ZeroBeneficiary();
        if (token != address(usdc)) revert UnsupportedToken(token);

        uint32 destinationDomain = _circleDomainForChainId(targetChainId);

        // Approve TokenMessengerV2 to pull `amount` USDC from this contract to burn.
        if (!usdc.approve(address(tokenMessengerV2), amount)) revert ApproveFailed();

        // Convert EVM address to bytes32 (left-padded) as required by CCTP.
        bytes32 mintRecipient = bytes32(uint256(uint160(beneficiary)));

        ITokenMessengerV2(tokenMessengerV2)
            .depositForBurn(
                amount,
                destinationDomain,
                mintRecipient,
                token,
                bytes32(0), // destinationCaller = 0 => anyone can call receiveMessage on destination
                0, // maxFee = 0 for standard transfer
                FINALITY_STANDARD
            );
    }

    function withdraw(address token, uint256 amount) external onlyOwner {
        TokenUtils.transfer(token, payable(msg.sender), amount);
    }

    /// @dev Map EVM chainId -> Circle CCTP domain id. Circle domains are NOT chainIds.
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
