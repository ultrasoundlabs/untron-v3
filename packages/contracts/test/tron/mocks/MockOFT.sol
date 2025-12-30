// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SendParam, OFTReceipt} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingReceipt, MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

import {MockERC20} from "./MockERC20.sol";

/// @dev Minimal IOFT mock for the USDT0Forwarder second hop:
///      - Debits `TOKEN_SRC` from `msg.sender`.
///      - Mints `TOKEN_DST` to `sp.to` (recipient on "Polygon"), amount = `sp.minAmountLD`.
contract MockOFT {
    MockERC20 public immutable TOKEN_SRC;
    MockERC20 public immutable TOKEN_DST;

    uint256 public quoteNativeFee;
    uint256 public quoteLzTokenFee;

    uint64 public lastNonce;
    uint32 public lastDstEid;
    bytes32 public lastTo;
    uint256 public lastAmountLD;
    uint256 public lastMinAmountLD;
    address public lastRefundAddress;
    uint256 public lastMsgValue;

    event Sent(address indexed from, address indexed to, uint256 amount, uint32 dstEid);

    constructor(MockERC20 tokenSrc_, MockERC20 tokenDst_) {
        TOKEN_SRC = tokenSrc_;
        TOKEN_DST = tokenDst_;
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

        (bool success) = TOKEN_SRC.transferFrom(msg.sender, address(this), sp.amountLD);
        success;

        address to = address(uint160(uint256(sp.to)));
        TOKEN_DST.mint(to, sp.minAmountLD);

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

        emit Sent(msg.sender, to, sp.amountLD, sp.dstEid);

        unchecked {
            uint256 refund = msg.value - fee.nativeFee;
            if (refund != 0) {
                (bool ok,) = payable(refundAddress).call{value: refund}("");
                require(ok, "REFUND_FAILED");
            }
        }
    }
}
