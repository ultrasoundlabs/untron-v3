// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {UntronV3} from "../../../src/evm/UntronV3.sol";
import {TronTxReader} from "../../../src/evm/TronTxReader.sol";
import {IBridger} from "../../../src/evm/bridgers/interfaces/IBridger.sol";
import {Call} from "../../../src/evm/SwapExecutor.sol";
import {TronCalldataUtils} from "../../../src/utils/TronCalldataUtils.sol";

import {MockERC20} from "../../tron/mocks/MockERC20.sol";

interface IMintableERC20 {
    function mint(address to, uint256 amount) external;
}

contract MockTronTxReader {
    TronTxReader.TriggerSmartContract internal _nextCallData;

    function setNextCallData(
        bytes32 txId,
        uint256 tronBlockNumber,
        uint32 tronBlockTimestamp,
        bytes21 senderTron,
        bytes21 toTron,
        bytes calldata data
    ) external {
        bytes memory data_ = data;
        _nextCallData = TronTxReader.TriggerSmartContract({
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
        TronTxReader.TriggerSmartContract storage s = _nextCallData;
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

contract MockSwapRouter {
    function mintToCaller(address token, uint256 amount) external {
        IMintableERC20(token).mint(msg.sender, amount);
    }

    function noop() external {}
}

contract MockBridger is IBridger {
    struct BridgeCall {
        address token;
        uint256 amount;
        uint256 targetChainId;
        address beneficiary;
    }

    BridgeCall[] internal _calls;

    function bridge(address token, uint256 amount, uint256 targetChainId, address beneficiary) external override {
        _calls.push(BridgeCall({token: token, amount: amount, targetChainId: targetChainId, beneficiary: beneficiary}));
    }

    function callCount() external view returns (uint256) {
        return _calls.length;
    }

    function callAt(uint256 idx)
        external
        view
        returns (address token, uint256 amount, uint256 targetChainId, address beneficiary)
    {
        BridgeCall storage c = _calls[idx];
        return (c.token, c.amount, c.targetChainId, c.beneficiary);
    }
}

contract ReentrantBridger is IBridger {
    error ClaimNotDeleted();
    error ReenterSucceeded();

    UntronV3 public immutable UNTRON;
    address public immutable TARGET_TOKEN;
    uint256 public immutable EXPECTED_CLAIM_IDX;

    bool public didCheckDeletion;
    bool public didReenter;

    constructor(UntronV3 untron, address targetToken, uint256 expectedClaimIdx) {
        UNTRON = untron;
        TARGET_TOKEN = targetToken;
        EXPECTED_CLAIM_IDX = expectedClaimIdx;
    }

    function bridge(address, uint256, uint256, address) external override {
        (uint256 amountUsdt,,,) = UNTRON.claimsByTargetToken(TARGET_TOKEN, EXPECTED_CLAIM_IDX);
        if (amountUsdt != 0) revert ClaimNotDeleted();
        didCheckDeletion = true;

        Call[] memory emptyCalls = new Call[](0);
        try UNTRON.fill(TARGET_TOKEN, 0, emptyCalls) {
            revert ReenterSucceeded();
        } catch {}
        didReenter = true;
    }
}

contract UntronV3Harness is UntronV3 {
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_)
        UntronV3(controllerAddress, create2Prefix, tronReader_)
    {}

    function pushControllerEvent(bytes32 sig, bytes calldata data, uint64 blockNumber, uint64 blockTimestamp) external {
        _controllerEvents.push(
            ControllerEvent({sig: sig, data: data, blockNumber: blockNumber, blockTimestamp: blockTimestamp})
        );
    }

    function controllerEventsLength() external view returns (uint256) {
        return _controllerEvents.length;
    }

    function controllerEventAt(uint256 idx)
        external
        view
        returns (bytes32 sig, bytes memory data, uint64 blockNumber, uint64 blockTimestamp)
    {
        ControllerEvent storage ev = _controllerEvents[idx];
        return (ev.sig, ev.data, ev.blockNumber, ev.blockTimestamp);
    }

    function leaseIdsByReceiver(bytes32 receiverSalt) external view returns (uint256[] memory ids) {
        ids = _leaseIdsByReceiver[receiverSalt];
    }

    function claimQueueLength(address targetToken) external view returns (uint256) {
        return claimsByTargetToken[targetToken].length;
    }

    function enqueueClaim(
        address targetToken,
        uint256 amountUsdt,
        uint256 leaseId,
        uint256 targetChainId,
        address beneficiary
    ) external returns (uint256 claimIndex) {
        claimIndex = _enqueueClaimForTargetToken(targetToken, amountUsdt, leaseId, targetChainId, beneficiary);
    }

    function exposedProcessReceiverPulled(bytes32 receiverSalt, uint256 usdtAmount, uint64 dumpTimestamp) external {
        _processReceiverPulled(receiverSalt, usdtAmount, dumpTimestamp);
    }

    function exposedDecodeEventChainTip(bytes memory data) external pure returns (bytes32 tip) {
        tip = _decodeEventChainTip(data);
    }

    function evmToTron(address a) external pure returns (bytes21 tron) {
        tron = TronCalldataUtils.evmToTronAddress(a);
    }
}
