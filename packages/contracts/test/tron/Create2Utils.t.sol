// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {UntronReceiver} from "../../src/tron/UntronReceiver.sol";

import {MockERC20} from "./mocks/MockERC20.sol";

contract Create2UtilsTest is Test {
    UntronController internal _controller;

    function setUp() public {
        _controller = new UntronController(0xff);
    }

    function test_predictReceiverAddress_matchesDeployReceiver() public {
        bytes32 salt = keccak256("salt");

        address predicted = _controller.predictReceiverAddress(salt);
        address deployed = _controller.deployReceiver(salt);

        assertEq(deployed, predicted, "deployed receiver != predicted");
        assertGt(deployed.code.length, 0, "receiver has no code");
    }

    function test_predictReceiverAddress_dependsOnPrefix() public {
        bytes32 salt = keccak256("same salt");

        UntronController controllerEvm = new UntronController(0xff);
        UntronController controllerTronPredict = new UntronController(0x41);

        address predictedEvm = controllerEvm.predictReceiverAddress(salt);
        address predictedTron = controllerTronPredict.predictReceiverAddress(salt);

        assertTrue(predictedEvm != predictedTron, "predicted addresses should differ");
    }

    function test_receiverBytecode_matchesExpectedEIP1167Stub() public view {
        bytes memory bytecode = _controller.receiverBytecode();
        assertEq(bytecode.length, 0x37, "unexpected receiver bytecode length");

        address embeddedImpl;
        // The implementation address is stored starting at offset 0x14 in the 55-byte initcode.
        assembly {
            embeddedImpl := shr(96, mload(add(add(bytecode, 0x20), 0x14)))
        }
        assertEq(embeddedImpl, _controller.RECEIVER_IMPL(), "impl address not embedded correctly");

        bytes memory expected = abi.encodePacked(
            hex"3d602d80600a3d3981f3363d3d373d3d3d363d73",
            _controller.RECEIVER_IMPL(),
            hex"5af43d82803e903d91602b57fd5bf3"
        );
        assertEq(bytecode, expected, "receiver bytecode mismatch");

        // Sanity: prediction formula agrees with manual CREATE2 computation for EVM prefix.
        bytes32 salt = keccak256("predict");
        bytes32 codeHash = keccak256(bytecode);
        address manualPredicted =
            address(uint160(uint256(keccak256(abi.encodePacked(bytes1(0xff), address(_controller), salt, codeHash)))));
        assertEq(_controller.predictReceiverAddress(salt), manualPredicted, "prediction mismatch");
    }

    function test_receiverImpl_pull_onlyController() public {
        MockERC20 token = new MockERC20("Mock", "MOCK", 18);
        address payable impl = payable(_controller.RECEIVER_IMPL());

        token.mint(impl, 123);

        vm.prank(address(0xBEEF));
        vm.expectRevert(UntronReceiver.NotController.selector);
        UntronReceiver(impl).pull(address(token), 1);

        vm.prank(address(_controller));
        UntronReceiver(impl).pull(address(token), 122);

        assertEq(token.balanceOf(address(_controller)), 122, "controller should receive tokens from impl");
        assertEq(token.balanceOf(impl), 1, "impl should keep 1 unit after pull");
    }
}
