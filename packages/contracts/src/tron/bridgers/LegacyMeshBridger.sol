// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IBridger} from "./interfaces/IBridger.sol";
import {IOFT, SendParam} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";
import {MessagingFee} from "@layerzerolabs/oapp-evm/contracts/oapp/OAppSender.sol";
import {TokenUtils} from "../../utils/TokenUtils.sol";

/// @dev Minimal OFT interface with Legacy Mesh-specific functions.
interface ILegacyMeshOFT is IOFT {
    function feeBps() external view returns (uint16);
    function BPS_DENOMINATOR() external view returns (uint16);
}

/// @title LegacyMeshBridger
/// @notice Bridger implementation for OFT-based Legacy Mesh bridge.
/// @author Ultrasound Labs
contract LegacyMeshBridger is IBridger {
    /// @notice Quote the native fee required for bridging via LayerZero OFT.
    /// @dev Payload must be ABI-encoded: (address oft, uint32 dstEid, bytes32 to)
    /// @param payload ABI-encoded (address oft, uint32 dstEid, bytes32 to).
    /// @return nativeFee Native token fee required.
    function quoteFee(
        address,
        /* token */
        uint256 amount,
        bytes calldata payload
    )
        external
        view
        returns (uint256 nativeFee)
    {
        (ILegacyMeshOFT oft, SendParam memory sp) = _parsePayload(payload, amount);
        MessagingFee memory msgFee = oft.quoteSend(sp, false);
        return msgFee.nativeFee;
    }

    /// @notice Bridge tokens via LayerZero OFT.
    /// @dev Payload must be ABI-encoded: (address oft, uint32 dstEid, bytes32 to)
    ///      Runs via DELEGATECALL in the controller context; controller holds funds.
    /// @param token Token address to bridge.
    /// @param amount Amount to bridge.
    /// @param payload ABI-encoded (address oft, uint32 dstEid, bytes32 to).
    /// @return bridgerReceipt Empty bytes (could return OFTReceipt in future).
    function bridge(address token, uint256 amount, bytes calldata payload)
        external
        returns (bytes memory bridgerReceipt)
    {
        (ILegacyMeshOFT oft, SendParam memory sp) = _parsePayload(payload, amount);

        TokenUtils.approve(token, address(oft), amount);

        // Quote fee
        MessagingFee memory msgFee = oft.quoteSend(sp, false);

        // Execute the bridge send from controller context; refund to controller
        oft.send{value: msgFee.nativeFee}(sp, msgFee, address(this));

        // Return empty receipt
        return "";
    }

    /*//////////////////////////////////////////////////////////////
                                HELPERS
    //////////////////////////////////////////////////////////////*/

    function _computeMinAmount(ILegacyMeshOFT oft, uint256 amount) internal view returns (uint256) {
        uint256 feeBps = oft.feeBps();
        uint256 denom = oft.BPS_DENOMINATOR();
        uint256 bpsFee = amount * feeBps / denom;
        return amount - bpsFee;
    }

    function _parsePayload(bytes calldata payload, uint256 amount)
        internal
        view
        returns (ILegacyMeshOFT, SendParam memory)
    {
        (ILegacyMeshOFT oft, uint32 dstEid, bytes32 to) = abi.decode(payload, (ILegacyMeshOFT, uint32, bytes32));
        // forge-lint: disable-next-line(mixed-case-variable)
        uint256 minAmountLD = _computeMinAmount(ILegacyMeshOFT(oft), amount);
        return (
            oft,
            SendParam({
                dstEid: dstEid,
                to: to,
                amountLD: amount,
                minAmountLD: minAmountLD,
                extraOptions: "",
                composeMsg: "",
                oftCmd: ""
            })
        );
    }
}

