// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IOFT, SendParam} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";
import {TokenUtils} from "../utils/TokenUtils.sol";

/// @title USDT0Forwarder
/// @notice Helper contract for forwarding USDT0 tokens across chains using LayerZero's OFT protocol.
/// @dev We utilize this contract to perform double hops from Tron to UntronV3's deployment chain.
///      The Legacy Mesh performs all swaps through Arbitrum using double hops orchestrated through LZ.
///      However, their double hops are complex to orchestrate in UntronController's static rebalancing setup.
///      So, this contract is supposed to be the beneficiary of rebalancing from UntronController, so that
///      from The Legacy Mesh's perspective the swaps are single-hop, and we do the second hop on our own.
/// @author Ultrasound Labs
contract USDT0Forwarder {
    /// @notice Address of the underlying USDT0 token on this chain.
    address public immutable TOKEN;

    /// @notice LayerZero OFT contract used to bridge `TOKEN`.
    IOFT public immutable OFT;

    /// @notice Destination LayerZero endpoint id (EID).
    uint32 public immutable DST_EID;

    /// @notice Remote-chain beneficiary address (bytes32 format required by OFT).
    bytes32 public immutable BENEFICIARY;

    error InsufficientMsgValue(uint256 required, uint256 received);
    error RefundFailed();
    error OFTSendFailed();

    /// @notice Creates a new forwarder for USDT0 double-hop bridging.
    /// @param _token Address of the underlying USDT0 token on this chain.
    /// @param _oft LayerZero OFT contract used to bridge `_token`.
    /// @param _dstEid Destination LayerZero endpoint id (EID).
    /// @param _beneficiary Beneficiary address on the destination chain (bytes32 format).
    constructor(address _token, IOFT _oft, uint32 _dstEid, bytes32 _beneficiary) {
        TOKEN = _token;
        OFT = _oft;
        DST_EID = _dstEid;
        BENEFICIARY = _beneficiary;
        TokenUtils.approve(_token, address(_oft), type(uint256).max);
    }

    /// @notice Forwards USDT0 held by this contract across chains to `BENEFICIARY`.
    /// @dev `amountLD` is debited from this contract by `OFT`, so tokens must already be held here.
    /// @param amountLD Amount to bridge, in local decimals.
    function forward(uint256 amountLD) external payable {
        SendParam memory sp = SendParam({
            dstEid: DST_EID,
            to: BENEFICIARY,
            amountLD: amountLD,
            minAmountLD: amountLD,
            extraOptions: "",
            composeMsg: "",
            oftCmd: ""
        });

        MessagingFee memory fee = OFT.quoteSend(sp, false);
        if (msg.value < fee.nativeFee) revert InsufficientMsgValue(fee.nativeFee, msg.value);

        // solhint-disable-next-line check-send-result
        bytes memory callData = abi.encodeCall(IOFT.send, (sp, fee, msg.sender));
        // solhint-disable-next-line check-send-result
        (bool ok,) = address(OFT).call{value: fee.nativeFee}(callData);
        if (!ok) revert OFTSendFailed();

        // refund any excess msg.value
        if (msg.value > fee.nativeFee) {
            // solhint-disable-next-line check-send-result
            (ok,) = msg.sender.call{value: msg.value - fee.nativeFee}("");
            if (!ok) revert RefundFailed();
        }
    }
}
