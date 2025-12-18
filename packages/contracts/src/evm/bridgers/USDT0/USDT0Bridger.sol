// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IBridger} from "../interfaces/IBridger.sol";
import {TokenUtils} from "../../../utils/TokenUtils.sol";

import {Ownable} from "solady/auth/Ownable.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {IOFT, SendParam, OFTReceipt} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

/// @title USDT0Bridger
/// @notice Bridger for the USDT0 (USD₮0) protocol core mesh using LayerZero V2 OFT.
/// @dev UntronV3 transfers `amount` of USDT0 to this contract, then calls `bridge` to send via LayerZero.
/// @author Ultrasound Labs
contract USDT0Bridger is IBridger, Ownable {
    // --- Errors ---
    error NotUntron();
    error UnsupportedToken(address token);
    error UnsupportedChainId(uint256 chainId);
    error ZeroBeneficiary();
    error AmountZero();
    error ApproveFailed();
    error FeeTooHigh(uint256 fee, uint256 maxFee);
    error InsufficientNativeBalance(uint256 have, uint256 need);
    error ZeroAddress();

    // --- Immutables ---
    /// @notice The UntronV3 contract allowed to call `bridge`.
    address public immutable UNTRON;
    /// @notice The USDT0 token address on the current chain.
    IERC20 public immutable USDT0;
    /// @notice The USDT0 OFT module on this chain (LayerZero V2).
    IOFT public immutable OFT; // the USDT0 OFT module on this chain (OAdapterUpgradeable / OUpgradeable)

    /// @notice EVM chainId -> LayerZero endpoint ID (eid) for USDT0 core mesh destinations.
    mapping(uint256 => uint32) public eidByChainId;

    /// @notice Creates a new USDT0 bridger instance.
    /// @param untron The UntronV3 contract allowed to call `bridge`.
    /// @param usdt0 USDT0 token address on the current chain.
    /// @param oft The LayerZero OFT contract/module used to send USDT0 on the current chain.
    constructor(address untron, address usdt0, address oft) {
        if (untron == address(0) || usdt0 == address(0) || oft == address(0)) revert ZeroAddress();

        UNTRON = untron;
        USDT0 = IERC20(usdt0);
        OFT = IOFT(oft);

        _initializeOwner(msg.sender);

        // TODO: make this deployer-specified

        eidByChainId[1] = 30101; // Ethereum
        // eidByChainId[42161] = 30110; // Arbitrum One
        eidByChainId[137] = 30109; // Polygon PoS
        eidByChainId[10] = 30111; // Optimism
        eidByChainId[80094] = 30362; // Berachain
        eidByChainId[57073] = 30339; // Ink
        eidByChainId[130] = 30320; // Unichain
        eidByChainId[21000000] = 30331; // Corn
        eidByChainId[1329] = 30280; // Sei (EVM)
        eidByChainId[14] = 30295; // Flare
        eidByChainId[999] = 30367; // HyperEVM
        eidByChainId[30] = 30333; // Rootstock
        eidByChainId[196] = 30274; // XLayer
        eidByChainId[9745] = 30383; // Plasma
        eidByChainId[1030] = 30212; // Conflux eSpace
        eidByChainId[5000] = 30181; // Mantle
        eidByChainId[143] = 30390; // Monad
        eidByChainId[988] = 30396; // Stable
    }

    // --- IBridger ---
    /// @notice Sends `amount` of USDT0 to `beneficiary` on `targetChainId` using the USDT0 core mesh.
    /// @dev This contract must already custody `amount` USDT0. The call quotes OFT dust/decimals conversion
    ///      and LayerZero messaging fees, then executes `OFT.send`.
    /// @param token ERC-20 token being bridged (must be USDT0).
    /// @param amount Amount of `token` to bridge (in token's smallest units).
    /// @param targetChainId EVM chain id of the destination chain.
    /// @param beneficiary Recipient address on the destination chain.
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external {
        if (msg.sender != UNTRON) revert NotUntron();
        if (beneficiary == address(0)) revert ZeroBeneficiary();
        if (amount == 0) revert AmountZero();
        if (token != address(USDT0)) revert UnsupportedToken(token);

        uint32 dstEid = eidByChainId[targetChainId];
        if (dstEid == 0) revert UnsupportedChainId(targetChainId);

        // Build send params (follow USDT0 dev guide pattern: quoteOFT then set minAmountLD = amountReceivedLD)
        SendParam memory sp = SendParam({
            dstEid: dstEid,
            to: bytes32(uint256(uint160(beneficiary))),
            amountLD: amount,
            minAmountLD: amount,
            extraOptions: "",
            composeMsg: "",
            oftCmd: ""
        });

        // Quote OFT receipt to account for dust/decimals conversion; use it as minAmountLD.
        (,, OFTReceipt memory oftReceipt) = OFT.quoteOFT(sp);
        sp.minAmountLD = oftReceipt.amountReceivedLD;

        IERC20(token).approve(address(OFT), amount);

        // Quote LayerZero messaging fee (native gas by default).
        MessagingFee memory msgFee = OFT.quoteSend(sp, false);

        uint256 bal = address(this).balance;
        if (bal < msgFee.nativeFee) revert InsufficientNativeBalance(bal, msgFee.nativeFee);

        /* solhint-disable check-send-result */

        // Execute send. Refund any excess (if any) back to this contract.
        OFT.send{value: msgFee.nativeFee}(sp, msgFee, address(this));

        /* solhint-enable check-send-result */
    }

    /// @notice Withdraws tokens accidentally left in this contract.
    /// @dev Owner-only escape hatch; also used to recover bridged tokens if a send is not executed.
    /// @param token The ERC-20 token to withdraw.
    /// @param amount Amount of `token` to withdraw.
    function withdraw(address token, uint256 amount) external onlyOwner {
        TokenUtils.transfer(token, payable(msg.sender), amount);
    }

    /// @notice Accepts native gas token needed to pay LayerZero messaging fees.
    receive() external payable {}
}
