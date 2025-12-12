// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IBridger} from "../interfaces/IBridger.sol";
import {TokenUtils} from "../../../utils/TokenUtils.sol";

import {Ownable} from "solady/auth/Ownable.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {
    IOFT,
    SendParam,
    OFTLimit,
    OFTReceipt,
    OFTFeeDetail
} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingFee, MessagingReceipt} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

/// @notice Bridger for the USDT0 (USD₮0) protocol core mesh using LayerZero V2 OFT.
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
    address public immutable untron;
    IERC20 public immutable usdt0;
    IOFT public immutable oft; // the USDT0 OFT module on this chain (OAdapterUpgradeable / OUpgradeable)

    /// @notice EVM chainId -> LayerZero endpoint ID (eid) for USDT0 core mesh destinations.
    mapping(uint256 => uint32) public eidByChainId;

    constructor(address _untron, address _usdt0, address _oft) {
        if (_untron == address(0) || _usdt0 == address(0) || _oft == address(0)) revert ZeroAddress();

        untron = _untron;
        usdt0 = IERC20(_usdt0);
        oft = IOFT(_oft);

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
    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external {
        if (msg.sender != untron) revert NotUntron();
        if (beneficiary == address(0)) revert ZeroBeneficiary();
        if (amount == 0) revert AmountZero();
        if (token != address(usdt0)) revert UnsupportedToken(token);

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
        (,, OFTReceipt memory oftReceipt) = oft.quoteOFT(sp);
        sp.minAmountLD = oftReceipt.amountReceivedLD;

        IERC20(token).approve(address(oft), amount);

        // Quote LayerZero messaging fee (native gas by default).
        MessagingFee memory msgFee = oft.quoteSend(sp, false);

        uint256 bal = address(this).balance;
        if (bal < msgFee.nativeFee) revert InsufficientNativeBalance(bal, msgFee.nativeFee);

        // Execute send. Refund any excess (if any) back to this contract.
        oft.send{value: msgFee.nativeFee}(sp, msgFee, address(this));
    }

    function withdraw(address token, uint256 amount) external onlyOwner {
        TokenUtils.transfer(token, payable(msg.sender), amount);
    }

    receive() external payable {}
}
