// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3Base} from "./UntronV3Base.sol";
import {Call} from "../SwapExecutor.sol";
import {TokenUtils} from "../../utils/TokenUtils.sol";

/// @title UntronV3 fill facet
/// @notice Claim settlement entrypoint (swap + local transfer/bridge).
/// @author Ultrasound Labs
contract UntronV3FillFacet is UntronV3Base {
    /// @notice Fill up to `maxClaims` claims for a target token, swapping once if needed then settling sequentially.
    /// @dev `calls` may be empty if no swap is needed (e.g. when `targetToken == usdt`).
    ///      Any swap output above `expectedOutTotal` is paid to the relayer (`msg.sender`).
    ///      This function is non-reentant because it calls executor that performs arbitrary onchain calls.
    ///
    /// Fill mechanics:
    /// - Claims are stored in a per-`targetToken` FIFO queue `claimsByTargetToken[targetToken]`.
    /// - For `targetToken != usdt`, this function:
    ///   - scans forward from the head to determine how many claims fit under current `usdtBalance()`,
    ///   - swaps USDT -> `targetToken` once for the batch,
    ///   - then fills those claims sequentially, either locally or via a configured bridger.
    ///
    /// @param targetToken The queue key: claims to be filled are `claimsByTargetToken[targetToken]`.
    /// @param maxClaims Maximum number of non-empty claims to fill in this call.
    /// @param calls Arbitrary swap calls executed by `SwapExecutor` if the plan requires swapping.
    function fill(address targetToken, uint256 maxClaims, Call[] calldata calls) external nonReentrant whenNotPaused {
        if (targetToken == address(0)) revert InvalidTargetToken();
        if (maxClaims == 0) return;

        Claim[] storage queue = claimsByTargetToken[targetToken];

        uint256 head = nextIndexByTargetToken[targetToken];

        uint256 ratePpm;
        if (targetToken != usdt) {
            ratePpm = swapRatePpm[targetToken];
            if (ratePpm == 0) revert RateNotSet();
        }

        (uint256 end, uint256 totalUsdt, uint256 expectedOutTotal) =
            _planFillBatch(targetToken, queue, head, maxClaims, ratePpm);

        uint256 surplusOut;
        if (targetToken != usdt) {
            surplusOut = _swapForBatch(targetToken, totalUsdt, expectedOutTotal, calls);
        }

        _settleClaimRange({targetToken: targetToken, ratePpm: ratePpm, queue: queue, start: head, end: end});

        nextIndexByTargetToken[targetToken] = end;

        if (surplusOut != 0) {
            TokenUtils.transfer(targetToken, payable(msg.sender), surplusOut);
        }
    }
}
