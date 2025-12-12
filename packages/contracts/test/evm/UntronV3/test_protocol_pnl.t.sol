// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronTxReader} from "../../../src/evm/TronTxReader.sol";

contract MockTronTxReader {
    TronTxReader.TriggerSmartContract internal nextCallData;

    function setNextCallData(
        bytes32 txId,
        uint256 tronBlockNumber,
        uint32 tronBlockTimestamp,
        bytes21 senderTron,
        bytes21 toTron,
        bytes calldata data
    ) external {
        bytes memory data_ = data;
        nextCallData = TronTxReader.TriggerSmartContract({
            txId: txId,
            tronBlockNumber: tronBlockNumber,
            tronBlockTimestamp: tronBlockTimestamp,
            senderTron: senderTron,
            toTron: toTron,
            data: data_
        });
    }

    function readTriggerSmartContract(uint256, bytes calldata, bytes32[] calldata, uint256)
        external
        view
        returns (TronTxReader.TriggerSmartContract memory callData)
    {
        TronTxReader.TriggerSmartContract storage s = nextCallData;
        return TronTxReader.TriggerSmartContract({
            txId: s.txId,
            tronBlockNumber: s.tronBlockNumber,
            tronBlockTimestamp: s.tronBlockTimestamp,
            senderTron: s.senderTron,
            toTron: s.toTron,
            data: s.data
        });
    }
}

contract UntronV3Harness is UntronV3 {
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_)
        UntronV3(controllerAddress, create2Prefix, tronReader_)
    {}

    function pushControllerEvent(bytes32 sig, bytes calldata data, uint64 blockNumber, uint64 blockTimestamp) external {
        controllerEvents.push(
            ControllerEvent({sig: sig, data: data, blockNumber: blockNumber, blockTimestamp: blockTimestamp})
        );
    }

    function claimQueueLength(address bridgeToken) external view returns (uint256) {
        return claimsByBridgeToken[bridgeToken].length;
    }

    // forge-lint: disable-next-line(mixed-case-variable)
    function claimAt(address bridgeToken, uint256 idx) external view returns (uint256 amountUSDT, uint256 leaseId) {
        Claim storage c = claimsByBridgeToken[bridgeToken][idx];
        return (c.amountUSDT, c.leaseId);
    }

    function exposedProcessReceiverPulled(bytes32 receiverSalt, uint256 usdtAmount, uint64 dumpTimestamp) external {
        _processReceiverPulled(receiverSalt, usdtAmount, dumpTimestamp);
    }
}

contract UntronV3ProtocolPnlTest is Test {
    MockTronTxReader internal reader;
    UntronV3Harness internal untron;

    address internal constant DUMMY_USDT = address(0x1000);
    address internal constant CONTROLLER = address(0xCAFE);

    function setUp() public {
        reader = new MockTronTxReader();
        untron = new UntronV3Harness(CONTROLLER, 0xff, address(reader));
        untron.setUsdt(DUMMY_USDT);
        untron.setRealtor(address(this), true);
    }

    function testPreEntitleBooksFee() public {
        bytes32 salt = keccak256("salt1");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        uint256 leaseId = untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, DUMMY_USDT, address(0xB0B)
        );

        address predictedReceiver = untron.predictReceiverAddress(CONTROLLER, salt);
        bytes memory trc20Data =
            abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), predictedReceiver, uint256(100));

        reader.setNextCallData(
            keccak256("tx1"), 1, uint32(block.timestamp), _evmToTron(address(0x1111)), _evmToTron(address(0)), trc20Data
        );

        (uint256 claimIndex, uint256 gotLeaseId, uint256 netOut) =
            untron.preEntitle(salt, 1, hex"", new bytes32[](0), 0);

        assertEq(gotLeaseId, leaseId);
        assertEq(netOut, 99);
        assertEq(untron.protocolPnl(), 1);
        assertEq(untron.claimQueueLength(DUMMY_USDT), 1);
        (uint256 claimAmount, uint256 claimLeaseId) = untron.claimAt(DUMMY_USDT, claimIndex);
        assertEq(claimAmount, 99);
        assertEq(claimLeaseId, leaseId);
    }

    function testPreEntitleNetOutZeroStillBooksFeeNoClaim() public {
        bytes32 salt = keccak256("salt2");
        uint32 leaseFeePpm = 0;
        uint64 flatFee = 200;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, DUMMY_USDT, address(0xB0B)
        );

        address predictedReceiver = untron.predictReceiverAddress(CONTROLLER, salt);
        bytes memory trc20Data =
            abi.encodeWithSelector(bytes4(keccak256("transfer(address,uint256)")), predictedReceiver, uint256(100));

        reader.setNextCallData(
            keccak256("tx2"), 2, uint32(block.timestamp), _evmToTron(address(0x2222)), _evmToTron(address(0)), trc20Data
        );

        (,, uint256 netOut) = untron.preEntitle(salt, 2, hex"", new bytes32[](0), 0);

        assertEq(netOut, 0);
        assertEq(untron.protocolPnl(), 100);
        assertEq(untron.claimQueueLength(DUMMY_USDT), 0);
    }

    function testUsdtRebalancedBooksDrift() public {
        bytes32 sig = keccak256("UsdtRebalanced(uint256,uint256,address)");
        bytes memory data = abi.encode(uint256(1000), uint256(995), address(0x3333));
        untron.pushControllerEvent(sig, data, 1, uint64(block.timestamp));

        untron.processControllerEvents(1);

        assertEq(untron.protocolPnl(), -5);
    }

    function testReceiverPulledRemainingBooksFeeAndClaim() public {
        bytes32 salt = keccak256("salt3");
        uint32 leaseFeePpm = 10_000;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        uint256 leaseId = untron.createLease(
            salt, address(this), nukeableAfter, leaseFeePpm, flatFee, block.chainid, DUMMY_USDT, address(0xB0B)
        );

        untron.exposedProcessReceiverPulled(salt, 100, uint64(block.timestamp));

        assertEq(untron.protocolPnl(), 1);
        assertEq(untron.claimQueueLength(DUMMY_USDT), 1);
        (uint256 claimAmount, uint256 claimLeaseId) = untron.claimAt(DUMMY_USDT, 0);
        assertEq(claimAmount, 99);
        assertEq(claimLeaseId, leaseId);
    }

    function _evmToTron(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }
}
