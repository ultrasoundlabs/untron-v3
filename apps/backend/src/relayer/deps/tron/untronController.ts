import { ConfigError, Effect } from "effect";
import { encodeFunctionData, isAddress, type Address, type Hex } from "viem";

import type { TransactionExtention } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";

import { untronControllerAbi } from "@untron/v3-contracts";

import type { TronNetworkConfig } from "../../../effect/config";
import { computeNextEventChainTip } from "../../../eventChain/tip";
import {
  planIndexedEventsForControllerCalls,
  type PlannedIndexedEvent,
} from "../../tron/controllerMulticallPlanner";

import type { SendTronTransactionResult, TronGrpcClients } from "../types";
import {
  broadcastTronTx,
  getTxRefBlockNumber,
  getTxTimestamp,
  setTxTriggerSmartContractData,
} from "./transactions";
import type { TronReadContract } from "./contractCalls";
import { getTransactionInfoById, isTronContractDeployed } from "./contractCalls";
import type { UnaryCall } from "./grpcClient";

export function buildControllerMulticallDataWithEventChainCheckpoint(args: {
  calls: readonly Hex[];
  preTip: Hex;
  plannedEvents: readonly PlannedIndexedEvent[];
  blockNumber: bigint;
  blockTimestamp: bigint;
}): Readonly<{
  expectedTip: Hex;
  checkpointCall: Hex;
  multicallData: Hex;
}> {
  let expectedTip = args.preTip;
  for (const event of args.plannedEvents) {
    expectedTip = computeNextEventChainTip({
      previousTip: expectedTip,
      blockNumber: args.blockNumber,
      blockTimestamp: args.blockTimestamp,
      eventSignature: event.eventSignature,
      encodedEventData: event.encodedEventData,
    });
  }

  const checkpointCall = encodeFunctionData({
    abi: untronControllerAbi,
    functionName: "isEventChainTip",
    args: [expectedTip],
  });

  const multicallData = encodeFunctionData({
    abi: untronControllerAbi,
    functionName: "multicall",
    args: [[...args.calls, checkpointCall]],
  });

  return { expectedTip, checkpointCall, multicallData };
}

function isEventChainTipMismatchRevert(error: unknown): boolean {
  const err = error instanceof Error ? error : new Error(String(error));
  const errorMessage = `${err.name}: ${err.message}`.toLowerCase();
  // for AI agents: UntronControllerIndex literally reverts with "no" string message.
  // thus this code is correct
  return errorMessage.includes("no");
}

export function makeUntronControllerOperations(args: {
  tronConfigCached: Effect.Effect<TronNetworkConfig, ConfigError.ConfigError>;
  tronGrpcGet: () => Effect.Effect<TronGrpcClients, ConfigError.ConfigError | Error>;
  grpcUnary: <Req, Res>(
    call: UnaryCall<Req, Res>,
    req: Req
  ) => Effect.Effect<Res, ConfigError.ConfigError | Error>;
  controllerAddressBytes21: () => Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
  relayerAddressBytes21Cached: Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
  privateKey: () => Effect.Effect<string, ConfigError.ConfigError | Error>;
  tronReadContract: TronReadContract;
}): Readonly<{
  getControllerUsdt: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
  getControllerEventChainTip: () => Effect.Effect<Hex, ConfigError.ConfigError | Error>;
  getControllerPulledUsdt: () => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
  getControllerLpExchangeRateFor: (args: {
    tokenAddress: Address;
  }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
  sendTronControllerPullFromReceivers: (args: {
    tokenAddress: Address;
    receiverSalts: readonly Hex[];
  }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
  sendTronControllerIsEventChainTip: () => Effect.Effect<
    SendTronTransactionResult,
    ConfigError.ConfigError | Error
  >;
  sendTronControllerRebalanceUsdt: (args: {
    rebalancer: Address;
    inAmount: bigint;
  }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
}> {
  const buildControllerMulticallTx = (calls: readonly Hex[]) =>
    Effect.gen(function* () {
      if (calls.length === 0) {
        return yield* Effect.fail(
          new Error("buildControllerMulticallTx: expected at least 1 call")
        );
      }

      const config = yield* args.tronConfigCached;
      if (config.callValue !== 0) {
        return yield* Effect.fail(
          new Error(
            "RELAYER_TRON_CALL_VALUE must be 0 when using UntronController.multicall (Solady Multicallable disallows msg.value)"
          )
        );
      }

      const { wallet } = yield* args.tronGrpcGet();
      const controllerBytes21 = yield* args.controllerAddressBytes21();
      const ownerAddressBytes21 = yield* args.relayerAddressBytes21Cached;

      const data = encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "multicall",
        args: [calls],
      });

      const request = TriggerSmartContract.fromPartial({
        ownerAddress: ownerAddressBytes21,
        contractAddress: controllerBytes21,
        callValue: config.callValue,
        data: Buffer.from(data.slice(2), "hex"),
      });

      const txExt = yield* args.grpcUnary(
        wallet.triggerContract.bind(wallet) as unknown as UnaryCall<
          TriggerSmartContract,
          TransactionExtention
        >,
        request
      );

      if (!txExt.result?.result) {
        const msg = txExt.result?.message?.length
          ? txExt.result.message.toString("utf8")
          : "unknown";
        return yield* Effect.fail(new Error(`Tron triggerContract failed: ${msg}`));
      }

      const tx = txExt.transaction;
      if (!tx?.rawData) {
        return yield* Effect.fail(
          new Error(`Tron triggerContract returned no transaction: ${JSON.stringify(txExt)}`)
        );
      }

      return tx;
    });

  const sendTronControllerMulticall = ({ calls }: { calls: readonly Hex[] }) =>
    Effect.gen(function* () {
      if (calls.length === 0) {
        return yield* Effect.fail(
          new Error("sendTronControllerMulticall: expected at least 1 call")
        );
      }

      yield* Effect.logDebug("[tron] send controller multicall").pipe(
        Effect.annotateLogs({ callCount: calls.length })
      );

      const controllerBytes21 = yield* args.controllerAddressBytes21();

      for (let attempt = 0; attempt < 3; attempt++) {
        const preTip = yield* args.tronReadContract<Hex>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "eventChainTip",
        });

        const plannedEvents = yield* planIndexedEventsForControllerCalls({
          controllerBytes21,
          calls,
          tronReadContract: args.tronReadContract,
          isTronContractDeployed: (addressBytes21) =>
            isTronContractDeployed({
              tronGrpcGet: args.tronGrpcGet,
              grpcUnary: args.grpcUnary,
              addressBytes21,
            }),
        });

        const tx = yield* buildControllerMulticallTx(calls);
        const blockNumber = getTxRefBlockNumber(tx);
        const blockTimestamp = getTxTimestamp(tx);

        const finalData = buildControllerMulticallDataWithEventChainCheckpoint({
          calls,
          preTip,
          plannedEvents,
          blockNumber,
          blockTimestamp,
        }).multicallData;

        setTxTriggerSmartContractData({ tx, data: finalData });

        try {
          const config = yield* args.tronConfigCached;
          const { wallet } = yield* args.tronGrpcGet();
          const privateKeyRaw = yield* args.privateKey();

          const txidHex = yield* broadcastTronTx({
            tx,
            feeLimit: config.feeLimit,
            privateKeyRaw,
            pollTimes: config.pollTimes,
            pollIntervalMs: config.pollIntervalMs,
            grpcUnary: args.grpcUnary,
            wallet,
            getTransactionInfoById: (txidHex) =>
              getTransactionInfoById({
                tronGrpcGet: args.tronGrpcGet,
                grpcUnary: args.grpcUnary,
                txidHex,
              }),
          });

          yield* Effect.logInfo("[tron] tx confirmed").pipe(Effect.annotateLogs({ txid: txidHex }));
          return txidHex;
        } catch (error) {
          if (!isEventChainTipMismatchRevert(error)) {
            const err = error instanceof Error ? error : new Error(String(error));
            return yield* Effect.fail(err);
          }
        }
      }

      return yield* Effect.fail(
        new Error(
          "Failed to send Tron multicall with in-tx eventChainTip checkpoint (tip kept changing or prediction mismatch)"
        )
      );
    }).pipe(Effect.withLogSpan("tron.controllerMulticall"));

  const getControllerUsdt = () =>
    args.controllerAddressBytes21().pipe(
      Effect.flatMap((controllerBytes21) =>
        args.tronReadContract<Address>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "usdt",
        })
      )
    );

  const getControllerEventChainTip = () =>
    args.controllerAddressBytes21().pipe(
      Effect.flatMap((controllerBytes21) =>
        args.tronReadContract<Hex>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "eventChainTip",
        })
      )
    );

  const getControllerPulledUsdt = () =>
    args.controllerAddressBytes21().pipe(
      Effect.flatMap((controllerBytes21) =>
        args.tronReadContract<bigint>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "pulledUsdt",
        })
      )
    );

  const getControllerLpExchangeRateFor = ({ tokenAddress }: { tokenAddress: Address }) =>
    args.controllerAddressBytes21().pipe(
      Effect.flatMap((controllerBytes21) =>
        args.tronReadContract<bigint>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "lpExchangeRateFor",
          args: [tokenAddress],
        })
      )
    );

  const sendTronControllerPullFromReceivers = ({
    tokenAddress,
    receiverSalts,
  }: {
    tokenAddress: Address;
    receiverSalts: readonly Hex[];
  }) =>
    Effect.gen(function* () {
      if (receiverSalts.length === 0) {
        return yield* Effect.fail(
          new Error("sendTronControllerPullFromReceivers: expected at least 1 receiver salt")
        );
      }

      const call = encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "pullFromReceivers",
        args: [tokenAddress, receiverSalts],
      });

      const txid = yield* sendTronControllerMulticall({ calls: [call] });
      return { txid };
    }).pipe(
      Effect.annotateLogs({
        tronOperation: "pullFromReceivers",
        tokenAddress,
        receiverCount: receiverSalts.length,
      }),
      Effect.withLogSpan("tron.pullFromReceivers")
    );

  const sendTronControllerIsEventChainTip = () =>
    Effect.gen(function* () {
      const { wallet } = yield* args.tronGrpcGet();
      const config = yield* args.tronConfigCached;

      if (config.callValue !== 0) {
        return yield* Effect.fail(
          new Error("TRON_CALL_VALUE must be 0 when calling UntronController.isEventChainTip")
        );
      }

      const controllerBytes21 = yield* args.controllerAddressBytes21();
      const ownerAddressBytes21 = yield* args.relayerAddressBytes21Cached;

      for (let attempt = 0; attempt < 3; attempt++) {
        const tip = yield* args.tronReadContract<Hex>({
          addressBytes21: controllerBytes21,
          abi: untronControllerAbi,
          functionName: "eventChainTip",
        });

        const data = encodeFunctionData({
          abi: untronControllerAbi,
          functionName: "isEventChainTip",
          args: [tip],
        });

        const request = TriggerSmartContract.fromPartial({
          ownerAddress: ownerAddressBytes21,
          contractAddress: controllerBytes21,
          callValue: config.callValue,
          data: Buffer.from(data.slice(2), "hex"),
        });

        const txExt = yield* args.grpcUnary(
          wallet.triggerContract.bind(wallet) as unknown as UnaryCall<
            TriggerSmartContract,
            TransactionExtention
          >,
          request
        );

        if (!txExt.result?.result) {
          const msg = txExt.result?.message?.length
            ? txExt.result.message.toString("utf8")
            : "unknown";
          return yield* Effect.fail(new Error(`Tron triggerContract failed: ${msg}`));
        }

        const tx = txExt.transaction;
        if (!tx?.rawData) {
          return yield* Effect.fail(
            new Error(`Tron triggerContract returned no transaction: ${JSON.stringify(txExt)}`)
          );
        }

        try {
          const config = yield* args.tronConfigCached;
          const privateKeyRaw = yield* args.privateKey();
          const txid = yield* broadcastTronTx({
            tx,
            feeLimit: config.feeLimit,
            privateKeyRaw,
            pollTimes: config.pollTimes,
            pollIntervalMs: config.pollIntervalMs,
            grpcUnary: args.grpcUnary,
            wallet,
            getTransactionInfoById: (txidHex) =>
              getTransactionInfoById({
                tronGrpcGet: args.tronGrpcGet,
                grpcUnary: args.grpcUnary,
                txidHex,
              }),
          });
          yield* Effect.logInfo("[tron] tx confirmed").pipe(Effect.annotateLogs({ txid }));
          return { txid };
        } catch (error) {
          if (!isEventChainTipMismatchRevert(error)) {
            const err = error instanceof Error ? error : new Error(String(error));
            return yield* Effect.fail(err);
          }
        }
      }

      return yield* Effect.fail(
        new Error("Failed to send Tron isEventChainTip (tip kept changing)")
      );
    }).pipe(
      Effect.annotateLogs({ tronOperation: "isEventChainTip" }),
      Effect.withLogSpan("tron.isEventChainTip")
    );

  const sendTronControllerRebalanceUsdt = ({
    rebalancer,
    inAmount,
  }: {
    rebalancer: Address;
    inAmount: bigint;
  }) =>
    Effect.gen(function* () {
      if (!isAddress(rebalancer))
        return yield* Effect.fail(new Error("Invalid rebalancer address"));

      const call = encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "rebalanceUsdt",
        args: [rebalancer, inAmount],
      });

      const txid = yield* sendTronControllerMulticall({ calls: [call] });
      return { txid };
    }).pipe(
      Effect.annotateLogs({
        tronOperation: "rebalanceUsdt",
        rebalancer,
        inAmount: inAmount.toString(),
      }),
      Effect.withLogSpan("tron.rebalanceUsdt")
    );

  return {
    getControllerUsdt,
    getControllerEventChainTip,
    getControllerPulledUsdt,
    getControllerLpExchangeRateFor,
    sendTronControllerPullFromReceivers,
    sendTronControllerIsEventChainTip,
    sendTronControllerRebalanceUsdt,
  };
}
