// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {UntronReceiver} from "../src/tron/UntronReceiver.sol";

contract UntronReceiverTest is Test {
    UntronReceiver public untronReceiver;

    function setUp() public {
        untronReceiver = new UntronReceiver();
    }

    function test_test() public pure {
        assertEq(true, true);
    }
}
