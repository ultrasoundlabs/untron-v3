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
    /// @notice Bridge tokens via LayerZero OFT.
    /// @dev Payload must be ABI-encoded: (address oft, uint32 dstEid, bytes32 to)
    ///      Runs via DELEGATECALL in the controller context; controller holds funds.
    /// @param token Token address to bridge.
    /// @param inAmount Amount to bridge.
    /// @param outAmount Expected amount of tokens to be bridged.
    /// @param payload ABI-encoded (address oft, uint32 dstEid, bytes32 to).
    function bridge(address token, uint256 inAmount, uint256 outAmount, bytes calldata payload) external {
        (ILegacyMeshOFT oft, uint32 dstEid, bytes32 to) = abi.decode(payload, (ILegacyMeshOFT, uint32, bytes32));

        // Fetch the Legacy Mesh's fee in basis points
        uint256 feeBps = oft.feeBps();
        // Fetch the Legacy Mesh's BPS denominator
        // (it's always 10000 but still gotta have same logic as in their contract,
        // which is feeBps / BPS_DENOMINATOR)
        uint256 denom = oft.BPS_DENOMINATOR();
        // Calculate the fee that the Legacy Mesh will take
        uint256 fee = inAmount * feeBps / denom;
        // Calculate the minimum amount to receive
        // (amount - fee)
        // forge-lint: disable-next-line(mixed-case-variable)
        uint256 minAmountLD = inAmount - fee;

        require(minAmountLD == outAmount, "LegacyMeshBridger: minAmountLD != outAmount");

        // Construct OFT's SendParam for the Legacy Mesh
        SendParam memory sp = SendParam({
            // Destination endpoint ID in LayerZero
            dstEid: dstEid,
            // Recipient address on the destination chain
            to: to,
            // Amount to bridge
            amountLD: inAmount,
            // Minimum amount to receive
            minAmountLD: minAmountLD,
            // Extra options. When empty, Legacy Mesh uses defaults
            extraOptions: "",
            // LZ-specific stuff we don't use in our implementation
            composeMsg: "",
            oftCmd: ""
        });
        // Approve the token to be spent by the Legacy Mesh
        // TODO: it's incredibly expensive to max approve for every bridge,
        // need to figure out how to max approve in a secure way
        TokenUtils.approve(token, address(oft), inAmount);

        // Quote the fee for the bridge
        MessagingFee memory msgFee = oft.quoteSend(sp, false);

        // Execute the bridge send from controller context; refund to controller
        oft.send{value: msgFee.nativeFee}(sp, msgFee, address(this));
    }
}
