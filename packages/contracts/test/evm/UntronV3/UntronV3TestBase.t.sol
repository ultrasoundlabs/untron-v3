// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {MockERC20} from "../../tron/mocks/MockERC20.sol";
import {MockSwapRouter, MockTronTxReader, UntronV3Harness} from "./UntronV3TestUtils.sol";
import {UntronV3AdminFacet} from "../../../src/evm/hub/UntronV3AdminFacet.sol";
import {UntronV3LeaseFacet} from "../../../src/evm/hub/UntronV3LeaseFacet.sol";
import {UntronV3EntitleFacet} from "../../../src/evm/hub/UntronV3EntitleFacet.sol";
import {UntronV3ControllerFacet} from "../../../src/evm/hub/UntronV3ControllerFacet.sol";
import {UntronV3LpFacet} from "../../../src/evm/hub/UntronV3LpFacet.sol";
import {UntronV3FillFacet} from "../../../src/evm/hub/UntronV3FillFacet.sol";

abstract contract UntronV3TestBase is Test {
    MockTronTxReader internal _reader;
    UntronV3Harness internal _untron;

    MockERC20 internal _usdt;
    MockERC20 internal _tokenX;
    MockSwapRouter internal _swapRouter;

    address internal constant _CONTROLLER = address(0xCAFE);
    address internal constant _RECEIVER_IMPL_OVERRIDE = address(0xBEEF);

    function setUp() public virtual {
        _reader = new MockTronTxReader();
        UntronV3AdminFacet adminFacet = new UntronV3AdminFacet();
        UntronV3LeaseFacet leaseFacet = new UntronV3LeaseFacet();
        UntronV3EntitleFacet entitleFacet = new UntronV3EntitleFacet();
        UntronV3ControllerFacet controllerFacet = new UntronV3ControllerFacet();
        UntronV3LpFacet lpFacet = new UntronV3LpFacet();
        UntronV3FillFacet fillFacet = new UntronV3FillFacet();

        _untron = new UntronV3Harness(
            _CONTROLLER,
            0xff,
            _RECEIVER_IMPL_OVERRIDE,
            address(adminFacet),
            address(leaseFacet),
            address(entitleFacet),
            address(controllerFacet),
            address(lpFacet),
            address(fillFacet)
        );
        _untron.setTronReader(address(_reader));

        _usdt = new MockERC20("USDT", "USDT", 6);
        _tokenX = new MockERC20("TokenX", "TKX", 18);
        _swapRouter = new MockSwapRouter();

        _untron.setUsdt(address(_usdt));
        _untron.setRealtor(address(this), true);
    }

    function _emptyBlocks() internal pure returns (bytes[20] memory blocks) {}

    function _createLease(
        bytes32 receiverSalt,
        address lessee,
        uint64 nukeableAfter,
        uint32 leaseFeePpm,
        uint64 flatFee,
        uint256 targetChainId,
        address targetToken,
        address beneficiary
    ) internal returns (uint256 leaseId, uint256 leaseNumber) {
        (leaseId, leaseNumber) = _untron.createLease(
            receiverSalt, lessee, nukeableAfter, leaseFeePpm, flatFee, targetChainId, targetToken, beneficiary
        );
    }

    function _predictedReceiver(bytes32 receiverSalt) internal view returns (address) {
        return _untron.predictReceiverAddress(_CONTROLLER, receiverSalt);
    }

    function _trc20TransferCalldata(address to, uint256 amount) internal pure returns (bytes memory) {
        return abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), to, amount);
    }

    function _mockTronTransferIntoReceiver(
        bytes32 txId,
        uint256 tronBlockNumber,
        uint32 tronBlockTimestamp,
        bytes21 senderTron,
        address receiver
    ) internal {
        bytes memory data = _trc20TransferCalldata(receiver, 100);
        _reader.setNextCallData(
            txId,
            tronBlockNumber,
            tronBlockTimestamp,
            senderTron,
            TronCalldataUtils.evmToTronAddress(_untron.tronUsdt()),
            data
        );
    }
}
