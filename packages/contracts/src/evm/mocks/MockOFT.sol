// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {IERC20Metadata} from "openzeppelin-contracts/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {
    IOFT,
    SendParam,
    OFTLimit,
    OFTFeeDetail,
    OFTReceipt
} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingReceipt, MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";

/// @title MockOFT
/// @notice Minimal IOFT mock for local/anvil environments.
/// @dev Debits `TOKEN_SRC` from `msg.sender` on `send`. If `TOKEN_DST` is set, mints `sp.minAmountLD` to `sp.to`.
/// @author Ultrasound Labs
contract MockOFT is IOFT {
    /// @notice Underlying ERC20 token that is debited by this mock.
    IERC20 public immutable TOKEN_SRC;
    /// @notice Optional underlying ERC20 token that is minted to the recipient (simulates a destination chain mint).
    address public immutable TOKEN_DST;

    /// @notice Value returned as `nativeFee` by `quoteSend`.
    uint256 public quoteNativeFee;
    /// @notice Value returned as `lzTokenFee` by `quoteSend`.
    uint256 public quoteLzTokenFee;

    /// @notice Last nonce returned in a `MessagingReceipt`.
    uint64 public lastNonce;
    /// @notice Last `SendParam` passed to `send`.
    SendParam public lastSendParam;
    /// @notice Last `MessagingFee` passed to `send`.
    MessagingFee public lastFee;
    /// @notice Last refund address passed to `send`.
    address public lastRefundAddress;

    /// @notice Last destination endpoint id passed to `send`.
    uint32 public lastDstEid;
    /// @notice Last destination address passed to `send`.
    bytes32 public lastTo;
    /// @notice Last destination amount passed to `send`.
    uint256 public lastAmountLD;
    /// @notice Last minimum destination amount passed to `send`.
    uint256 public lastMinAmountLD;

    /// @notice Emitted when `send` debits tokens.
    /// @param from Token sender.
    /// @param to Destination recipient bytes32 (as used by LayerZero IOFT).
    /// @param amountLD Amount debited in local decimals.
    /// @param dstEid Destination endpoint id.
    event Sent(address indexed from, bytes32 indexed to, uint256 amountLD, uint32 dstEid);

    error MockOFT_InsufficientNativeFee();
    error MockOFT_TransferFromFailed();
    error MockOFT_MintFailed();
    error MockOFT_RefundFailed();

    /// @notice Creates the mock for a given underlying token.
    /// @param tokenSrc_ ERC20 token address that is debited on `send`.
    /// @param tokenDst_ Optional ERC20 token address that is minted to the recipient (set to address(0) to disable).
    constructor(address tokenSrc_, address tokenDst_) {
        TOKEN_SRC = IERC20(tokenSrc_);
        TOKEN_DST = tokenDst_;
    }

    /// @notice Returns the mocked IOFT interface id and version.
    /// @return interfaceId IOFT interface id.
    /// @return version IOFT version.
    function oftVersion() external pure returns (bytes4 interfaceId, uint64 version) {
        interfaceId = 0x02e49c2c;
        version = 1;
    }

    /// @notice Returns the underlying token address.
    /// @return Token address.
    function token() external view returns (address) {
        return address(TOKEN_SRC);
    }

    /// @notice Indicates whether approval is required before `send` can debit tokens.
    /// @return True for this mock.
    function approvalRequired() external pure returns (bool) {
        return true;
    }

    /// @notice Returns the shared decimals (best-effort reads `decimals()` from the token).
    /// @return Shared decimals value.
    function sharedDecimals() external view returns (uint8) {
        (bool ok, bytes memory data) = address(TOKEN_SRC).staticcall(abi.encodeCall(IERC20Metadata.decimals, ()));
        if (ok && data.length == 32) return abi.decode(data, (uint8));
        return 6;
    }

    /// @notice Sets the values returned by `quoteSend`.
    /// @param nativeFee Mocked native fee.
    /// @param lzTokenFee Mocked LayerZero token fee.
    function setQuoteSendFee(uint256 nativeFee, uint256 lzTokenFee) external {
        quoteNativeFee = nativeFee;
        quoteLzTokenFee = lzTokenFee;
    }

    /// @notice Returns a 1:1 receipt for the given send parameters.
    /// @param sp Send parameters.
    /// @return limit Limit struct.
    /// @return oftFeeDetails Empty fee details list.
    /// @return receipt 1:1 receipt.
    function quoteOFT(SendParam calldata sp)
        external
        pure
        returns (OFTLimit memory limit, OFTFeeDetail[] memory oftFeeDetails, OFTReceipt memory receipt)
    {
        limit = OFTLimit({minAmountLD: 0, maxAmountLD: type(uint256).max});
        oftFeeDetails = new OFTFeeDetail[](0);
        receipt = OFTReceipt({amountSentLD: sp.amountLD, amountReceivedLD: sp.amountLD});
    }

    /// @notice Returns the currently configured mocked fees.
    /// @param sp Unused.
    /// @param payInLzToken Unused.
    /// @return fee Messaging fee values.
    function quoteSend(SendParam calldata sp, bool payInLzToken) external view returns (MessagingFee memory fee) {
        sp;
        payInLzToken;
        fee = MessagingFee({nativeFee: quoteNativeFee, lzTokenFee: quoteLzTokenFee});
    }

    /// @notice Debits `TOKEN` from the caller and emits a receipt.
    /// @param sp Send parameters.
    /// @param fee Messaging fee.
    /// @param refundAddress Address that receives excess native fee.
    /// @return receipt Messaging receipt.
    /// @return oftReceipt OFT receipt.
    function send(SendParam calldata sp, MessagingFee calldata fee, address refundAddress)
        external
        payable
        returns (MessagingReceipt memory receipt, OFTReceipt memory oftReceipt)
    {
        if (msg.value < fee.nativeFee) revert MockOFT_InsufficientNativeFee();

        bool okTf = TOKEN_SRC.transferFrom(msg.sender, address(this), sp.amountLD);
        if (!okTf) revert MockOFT_TransferFromFailed();

        unchecked {
            ++lastNonce;
        }
        bytes32 guid = bytes32(uint256(lastNonce));

        lastSendParam = sp;
        lastFee = fee;
        lastRefundAddress = refundAddress;
        lastDstEid = sp.dstEid;
        lastTo = sp.to;
        lastAmountLD = sp.amountLD;
        lastMinAmountLD = sp.minAmountLD;

        receipt = MessagingReceipt({guid: guid, nonce: lastNonce, fee: fee});
        oftReceipt = OFTReceipt({amountSentLD: sp.amountLD, amountReceivedLD: sp.minAmountLD});

        emit Sent(msg.sender, sp.to, sp.amountLD, sp.dstEid);

        if (TOKEN_DST != address(0)) {
            address to = address(uint160(uint256(sp.to)));
            // solhint-disable-next-line avoid-low-level-calls
            (bool okMint,) = TOKEN_DST.call(abi.encodeWithSignature("mint(address,uint256)", to, sp.minAmountLD));
            if (!okMint) revert MockOFT_MintFailed();
        }

        unchecked {
            uint256 refund = msg.value - fee.nativeFee;
            if (refund != 0) {
                // solhint-disable-next-line avoid-low-level-calls
                (bool ok,) = payable(refundAddress).call{value: refund}("");
                if (!ok) revert MockOFT_RefundFailed();
            }
        }
    }
}
