// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Create2Utils} from "../utils/Create2Utils.sol";
import {TronTxReader} from "./TronTxReader.sol";

contract UntronManager is Create2Utils, TronTxReader {
    /// @notice The address of the UntronController contract on Tron (source chain)
    bytes20 public immutable CONTROLLER_ADDRESS;

    constructor(bytes20 controllerAddress, bytes1 create2Prefix) Create2Utils(create2Prefix) {
        CONTROLLER_ADDRESS = controllerAddress;
        // TODO: implement
    }
}
