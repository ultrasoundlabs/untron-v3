// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {IRebalancer} from "../../../src/tron/rebalancers/interfaces/IRebalancer.sol";
import {TronTokenUtils as TokenUtils} from "../../../src/utils/TronTokenUtils.sol";

contract MockRebalancer is IRebalancer {
    error RebalanceReverted();
    error EthReceiveFailed();

    struct Config {
        uint256 outAmount;
        address sink;
        bool spendTokenInAmount;
        uint256 ethToSink;
        bool shouldRevert;
    }

    function rebalance(address token, uint256 inAmount, bytes calldata payload)
        external
        payable
        override
        returns (uint256 outAmount)
    {
        Config memory config = abi.decode(payload, (Config));

        if (config.shouldRevert) revert RebalanceReverted();

        if (config.spendTokenInAmount) {
            TokenUtils.transfer(token, payable(config.sink), inAmount);
        }

        if (config.ethToSink != 0) {
            TokenUtils.transfer(address(0), payable(config.sink), config.ethToSink);
        }

        return config.outAmount;
    }

    receive() external payable {
        // Useful for tests that want the rebalancer to be able to hold ETH.
        // solhint-disable-next-line gas-custom-errors
        if (msg.value == 0) revert EthReceiveFailed();
    }
}
