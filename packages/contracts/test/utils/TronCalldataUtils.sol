// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronCalldataUtils} from "../../src/utils/TronCalldataUtils.sol";

contract TronCalldataUtilsHarness {
    function evmToTron(address a) external pure returns (bytes21) {
        return TronCalldataUtils.evmToTronAddress(a);
    }

    function decodeTrc20(bytes calldata data, bytes21 senderTron)
        external
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        bytes memory m = data;
        return TronCalldataUtils.decodeTrc20FromCalldata(m, senderTron);
    }

    function decodeIsTip(bytes calldata data) external pure returns (bytes32 tip) {
        bytes memory m = data;
        return TronCalldataUtils.decodeIsEventChainTip(m);
    }

    function decodeMulticallTip(bytes calldata data, bytes4 selectorIsTip) external pure returns (bytes32 tip) {
        bytes memory m = data;
        return TronCalldataUtils.decodeMulticallEventChainTip(m, selectorIsTip);
    }
}

contract TronCalldataUtilsTest is Test {
    TronCalldataUtilsHarness internal _h;

    function setUp() public {
        _h = new TronCalldataUtilsHarness();
    }

    function test_evmToTronAddress_prefixAndBody() public view {
        address a = address(0x1234567890123456789012345678901234567890);
        bytes21 got = _h.evmToTron(a);
        bytes21 expected = bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
        assertEq(bytes32(got), bytes32(expected));
    }

    function test_decodeTrc20_transfer() public view {
        bytes21 senderTron = _h.evmToTron(address(0xAAAA));
        address toAddr = address(0xBEEF);
        uint256 amount = 100;

        bytes memory data = abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), toAddr, amount);

        (bytes21 fromTron, bytes21 toTron, uint256 gotAmount) = _h.decodeTrc20(data, senderTron);
        assertEq(bytes32(fromTron), bytes32(senderTron));
        assertEq(bytes32(toTron), bytes32(_h.evmToTron(toAddr)));
        assertEq(gotAmount, amount);
    }

    function test_decodeTrc20_transferFrom() public view {
        address fromAddr = address(0x1111);
        address toAddr = address(0x2222);
        uint256 amount = 777;

        bytes memory data = abi.encodeWithSelector(
            bytes4(keccak256("transferFrom(address,address,uint256)")), fromAddr, toAddr, amount
        );

        (bytes21 fromTron, bytes21 toTron, uint256 gotAmount) = _h.decodeTrc20(data, _h.evmToTron(address(0)));
        assertEq(bytes32(fromTron), bytes32(_h.evmToTron(fromAddr)));
        assertEq(bytes32(toTron), bytes32(_h.evmToTron(toAddr)));
        assertEq(gotAmount, amount);
    }

    function test_decodeTrc20_reverts_on_short_data() public {
        bytes21 senderTron = _h.evmToTron(address(0));
        vm.expectRevert(TronCalldataUtils.TronInvalidCalldataLength.selector);
        _h.decodeTrc20(hex"01", senderTron);
    }

    function test_decodeTrc20_reverts_on_bad_selector() public {
        bytes21 senderTron = _h.evmToTron(address(0));
        bytes memory data =
            abi.encodeWithSelector(bytes4(keccak256("approve(address,uint256)")), address(0x1), uint256(1));
        vm.expectRevert(TronCalldataUtils.NotATrc20Transfer.selector);
        _h.decodeTrc20(data, senderTron);
    }

    function test_decodeTrc20_reverts_on_bad_length_transfer() public {
        bytes21 senderTron = _h.evmToTron(address(0));
        bytes memory data = abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), address(0x1));
        vm.expectRevert(TronCalldataUtils.TronInvalidTrc20DataLength.selector);
        _h.decodeTrc20(data, senderTron);
    }

    function test_decodeIsEventChainTip_ok() public view {
        bytes32 tip = keccak256("tip");
        bytes memory data = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), tip);
        assertEq(_h.decodeIsTip(data), tip);
    }

    function test_decodeIsEventChainTip_reverts_on_bad_length() public {
        bytes32 tip = keccak256("tip");
        bytes memory data = abi.encodeWithSelector(bytes4(keccak256("isEventChainTip(bytes32)")), tip, uint256(1));
        vm.expectRevert(TronCalldataUtils.TronInvalidCalldataLength.selector);
        _h.decodeIsTip(data);
    }

    function test_decodeMulticallEventChainTip_standard_abi_finds_inner() public view {
        bytes32 tip = keccak256("tip2");
        bytes4 multicallSel = bytes4(keccak256("multicall(bytes[])"));
        bytes4 isTipSel = bytes4(keccak256("isEventChainTip(bytes32)"));

        bytes[] memory calls = new bytes[](2);
        calls[0] = abi.encodeWithSelector(bytes4(keccak256("foo(uint256)")), uint256(1));
        calls[1] = abi.encodeWithSelector(isTipSel, tip);

        bytes memory data = abi.encodeWithSelector(multicallSel, calls);

        assertEq(_h.decodeMulticallTip(data, isTipSel), tip);
    }

    function test_decodeMulticallEventChainTip_reverts_if_missing_standard() public {
        bytes4 multicallSel = bytes4(keccak256("multicall(bytes[])"));
        bytes4 isTipSel = bytes4(keccak256("isEventChainTip(bytes32)"));

        bytes[] memory calls = new bytes[](1);
        calls[0] = abi.encodeWithSelector(bytes4(keccak256("foo(uint256)")), uint256(1));

        bytes memory data = abi.encodeWithSelector(multicallSel, calls);

        vm.expectRevert(TronCalldataUtils.NoEventChainTipInMulticall.selector);
        _h.decodeMulticallTip(data, isTipSel);
    }
}
