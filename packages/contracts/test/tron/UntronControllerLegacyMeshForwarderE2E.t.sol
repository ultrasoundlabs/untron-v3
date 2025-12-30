// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Vm} from "forge-std/Vm.sol";

import {UntronController} from "../../src/tron/UntronController.sol";
import {LegacyMeshRebalancer, ILegacyMeshOFT} from "../../src/tron/rebalancers/LegacyMeshRebalancer.sol";
import {USDT0Forwarder} from "../../src/evm/USDT0Forwarder.sol";
import {IOFT} from "@layerzerolabs/oft-evm/contracts/interfaces/IOFT.sol";

import {MockERC20} from "./mocks/MockERC20.sol";
import {TronUsdtLikeERC20} from "./mocks/TronUsdtLikeERC20.sol";
import {MockLegacyMeshOFTAdapter} from "./mocks/MockLegacyMeshOFTAdapter.sol";
import {MockOFT} from "./mocks/MockOFT.sol";

contract UntronControllerLegacyMeshForwarderE2ETest is Test {
    bytes32 private constant _SIG_USDT_REBALANCED = keccak256("UsdtRebalanced(uint256,uint256,address)");

    uint32 internal constant _DST_EID_ARB = 110;
    uint32 internal constant _DST_EID_POLY = 109;

    address internal constant _RELAYER = address(0xB0B);
    address internal constant _POLYGON_BENEFICIARY = address(0x123456789);

    UntronController internal _controller;
    LegacyMeshRebalancer internal _rebalancer;
    TronUsdtLikeERC20 internal _usdtTron;
    MockERC20 internal _usdt0Arb;
    MockERC20 internal _usdt0Poly;
    MockOFT internal _arbOft;
    USDT0Forwarder internal _forwarder;
    MockLegacyMeshOFTAdapter internal _legacyOft;

    function setUp() public {
        _controller = new UntronController(0xff);
        _rebalancer = new LegacyMeshRebalancer();

        _usdtTron = new TronUsdtLikeERC20("USDT", "USDT", 6);
        _controller.setUsdt(address(_usdtTron));

        _usdt0Arb = new MockERC20("USDT0", "USDT0", 6);
        _usdt0Poly = new MockERC20("USDT0", "USDT0", 6);

        _arbOft = new MockOFT(_usdt0Arb, _usdt0Poly);
        _arbOft.setQuoteSendFee(0.2 ether, 0);

        _forwarder = new USDT0Forwarder(
            address(_usdt0Arb), IOFT(address(_arbOft)), _DST_EID_POLY, bytes32(uint256(uint160(_POLYGON_BENEFICIARY)))
        );

        _legacyOft = new MockLegacyMeshOFTAdapter(address(_usdtTron), _usdt0Arb);
        _legacyOft.setFeeBps(100); // 1% fee
        _legacyOft.setQuoteSendFee(0.3 ether, 0);

        _controller.setPayload(
            address(_rebalancer),
            abi.encode(
                ILegacyMeshOFT(address(_legacyOft)), _DST_EID_ARB, bytes32(uint256(uint160(address(_forwarder))))
            )
        );
    }

    function test_e2e_tronController_toLegacyMesh_toUSDT0Forwarder_toPolygonBeneficiary() public {
        uint256 pulled = 1_000_000; // 1 USDT with 6 decimals
        _doUsdtPull(_controller, _usdtTron, pulled);

        uint256 inAmount = 400_000;
        uint256 expectedOut = inAmount - (inAmount * 100 / 10_000);

        _controller.approveUsdt(address(_legacyOft), inAmount);

        vm.deal(address(this), 1 ether);
        uint256 controllerEthBefore = address(_controller).balance;

        vm.recordLogs();
        _controller.rebalanceUsdt{value: 1 ether}(address(_rebalancer), inAmount);
        Vm.Log[] memory logs = vm.getRecordedLogs();

        Vm.Log memory log = _findSingleLogFromEmitter(logs, address(_controller), _SIG_USDT_REBALANCED);
        (uint256 eventIn, uint256 eventOut) = abi.decode(log.data, (uint256, uint256));
        assertEq(eventIn, inAmount, "event inAmount mismatch");
        assertEq(eventOut, expectedOut, "event outAmount mismatch");

        // Accounting decreases by inAmount (even though bridged out is less due to fee).
        assertEq(_controller.pulledUsdt(), pulled - inAmount, "pulledUsdt should decrement by inAmount");

        // Underlying USDT should be debited from the controller.
        assertEq(_usdtTron.balanceOf(address(_controller)), pulled - inAmount, "controller USDT balance mismatch");
        assertEq(_usdtTron.balanceOf(address(_legacyOft)), inAmount, "legacyOft should hold debited USDT");

        // "Arbitrum" forwarder should receive USDT0 from the first hop.
        assertEq(_usdt0Arb.balanceOf(address(_forwarder)), expectedOut, "forwarder should receive USDT0");
        assertEq(_legacyOft.lastDstEid(), _DST_EID_ARB, "legacy dstEid mismatch");
        assertEq(_legacyOft.lastTo(), bytes32(uint256(uint160(address(_forwarder)))), "legacy to mismatch");
        assertEq(_legacyOft.lastAmountLD(), inAmount, "legacy amountLD mismatch");
        assertEq(_legacyOft.lastMinAmountLD(), expectedOut, "legacy minAmountLD mismatch");

        // Controller keeps unspent ETH from msg.value (0.3 was used as native fee by rebalancer).
        assertEq(address(_controller).balance, controllerEthBefore + 0.7 ether, "controller ETH leftover mismatch");

        // Second hop: relayer pushes USDT0 from forwarder to the beneficiary on "Polygon".
        vm.deal(_RELAYER, 1 ether);
        uint256 relayerEthBefore = _RELAYER.balance;

        vm.prank(_RELAYER);
        _forwarder.forward{value: 0.5 ether}(expectedOut);

        assertEq(_RELAYER.balance, relayerEthBefore - 0.2 ether, "relayer should pay exactly native fee");

        // Forwarder should be drained and beneficiary should receive on the destination token.
        assertEq(_usdt0Arb.balanceOf(address(_forwarder)), 0, "forwarder should be drained");
        assertEq(_usdt0Poly.balanceOf(_POLYGON_BENEFICIARY), expectedOut, "beneficiary should receive on dst chain");
        assertEq(_arbOft.lastDstEid(), _DST_EID_POLY, "dstEid mismatch");
        assertEq(_arbOft.lastTo(), bytes32(uint256(uint160(_POLYGON_BENEFICIARY))), "to mismatch");
        assertEq(_arbOft.lastAmountLD(), expectedOut, "amountLD mismatch");
        assertEq(_arbOft.lastMinAmountLD(), expectedOut, "minAmountLD mismatch");
    }

    function _doUsdtPull(UntronController controller, TronUsdtLikeERC20 usdt, uint256 sweepAmount) internal {
        bytes32 salt = keccak256(abi.encodePacked("usdt-pull", sweepAmount));
        address receiver = controller.predictReceiverAddress(salt);
        usdt.mint(receiver, sweepAmount + 1);
        controller.pullFromReceivers(address(usdt), _asArray(salt));
    }

    function _asArray(bytes32 salt) internal pure returns (bytes32[] memory arr) {
        arr = new bytes32[](1);
        arr[0] = salt;
    }

    function _findSingleLogFromEmitter(Vm.Log[] memory logs, address emitter, bytes32 sig)
        internal
        pure
        returns (Vm.Log memory found)
    {
        bool hasFound = false;
        for (uint256 i = 0; i < logs.length; i++) {
            if (logs[i].emitter != emitter) continue;
            if (logs[i].topics.length == 0 || logs[i].topics[0] != sig) continue;
            require(!hasFound, "MULTIPLE_LOGS");
            found = logs[i];
            hasFound = true;
        }
        require(hasFound, "LOG_NOT_FOUND");
    }
}
