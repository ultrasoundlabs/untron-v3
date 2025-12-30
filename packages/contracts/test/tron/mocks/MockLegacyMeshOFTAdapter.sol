// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SendParam, OFTReceipt} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingReceipt, MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";
import {TronTokenUtils} from "../../../src/utils/TronTokenUtils.sol";

import {MockERC20} from "./MockERC20.sol";

/// @dev Minimal LegacyMesh-style OFT adapter mock:
///      - Debits `UNDERLYING` from `msg.sender` using TronTokenUtils (return-false-on-success OK).
///      - Mints `USDT0` to `sp.to` (recipient on "Arbitrum"), amount = `sp.minAmountLD`.
contract MockLegacyMeshOFTAdapter {
    address public immutable UNDERLYING;
    MockERC20 public immutable USDT0;

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

    event Bridged(address indexed from, address indexed to, uint256 inAmount, uint256 outAmount, uint32 dstEid);

    constructor(address underlying_, MockERC20 usdt0_) {
        UNDERLYING = underlying_;
        USDT0 = usdt0_;
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

        TronTokenUtils.transferFrom(UNDERLYING, msg.sender, payable(address(this)), sp.amountLD);

        address to = address(uint160(uint256(sp.to)));
        USDT0.mint(to, sp.minAmountLD);

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

        emit Bridged(msg.sender, to, sp.amountLD, sp.minAmountLD, sp.dstEid);

        unchecked {
            uint256 refund = msg.value - fee.nativeFee;
            if (refund != 0) {
                (bool ok,) = payable(refundAddress).call{value: refund}("");
                require(ok, "REFUND_FAILED");
            }
        }
    }
}

