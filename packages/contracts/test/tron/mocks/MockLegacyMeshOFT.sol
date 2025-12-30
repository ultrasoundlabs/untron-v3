// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {SendParam, OFTReceipt} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingReceipt, MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

contract MockLegacyMeshOFT {
    IERC20 public immutable TOKEN;

    uint16 public feeBps;
    uint16 public constant BPS_DENOMINATOR = 10_000;

    uint256 public quoteNativeFee;
    uint256 public quoteLzTokenFee;

    uint64 public lastNonce;
    uint32 public lastDstEid;
    bytes32 public lastTo;
    uint256 public lastAmountLD;
    uint256 public lastMinAmountLD;
    address public lastRefundAddress;
    uint256 public lastMsgValue;

    constructor(IERC20 token_) {
        TOKEN = token_;
    }

    function setFeeBps(uint16 feeBps_) external {
        feeBps = feeBps_;
    }

    function setQuoteSendFee(uint256 nativeFee, uint256 lzTokenFee) external {
        quoteNativeFee = nativeFee;
        quoteLzTokenFee = lzTokenFee;
    }

    function quoteSend(SendParam calldata, bool) external view returns (MessagingFee memory fee) {
        fee = MessagingFee({nativeFee: quoteNativeFee, lzTokenFee: quoteLzTokenFee});
    }

    function send(SendParam calldata sp, MessagingFee calldata fee, address refundAddress)
        external
        payable
        returns (MessagingReceipt memory receipt, OFTReceipt memory oftReceipt)
    {
        require(msg.value >= fee.nativeFee, "INSUFFICIENT_NATIVE_FEE");

        (bool success) = TOKEN.transferFrom(msg.sender, address(this), sp.amountLD);
        success;

        lastNonce++;
        bytes32 guid = keccak256(abi.encodePacked(address(this), msg.sender, lastNonce, sp.dstEid, sp.to, sp.amountLD));

        lastDstEid = sp.dstEid;
        lastTo = sp.to;
        lastAmountLD = sp.amountLD;
        lastMinAmountLD = sp.minAmountLD;
        lastRefundAddress = refundAddress;
        lastMsgValue = msg.value;

        receipt = MessagingReceipt({guid: guid, nonce: lastNonce, fee: fee});
        oftReceipt = OFTReceipt({amountSentLD: sp.amountLD, amountReceivedLD: sp.minAmountLD});

        unchecked {
            uint256 refund = msg.value - fee.nativeFee;
            if (refund != 0) {
                (bool ok,) = payable(refundAddress).call{value: refund}("");
                require(ok, "REFUND_FAILED");
            }
        }
    }
}
