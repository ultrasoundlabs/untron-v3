import { ConfigError, Effect, Layer, Option, Redacted } from "effect";
import { decodeFunctionResult, encodeFunctionData, isAddress, type Address, type Hex } from "viem";

import type { BytesMessage, TransactionExtention } from "@untron/tron-protocol/api";
import { AccountBalanceRequest } from "@untron/tron-protocol/core/contract/balance_contract";
import type { AccountBalanceResponse } from "@untron/tron-protocol/core/contract/balance_contract";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { SmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import { type TransactionInfo, type Transaction } from "@untron/tron-protocol/tron";

import { ERC20Abi } from "../../../../abis/ERC20Abi";
import { untronControllerAbi } from "@untron/v3-contracts";
import { AppConfig } from "../../../effect/config";
import { computeNextEventChainTip } from "../../../eventChain/tip";
import { planIndexedEventsForControllerCalls } from "../../tron/controllerMulticallPlanner";

import type { SendTronTransactionResult, TronReceiverMapEntry } from "../types";
import { TronGrpc } from "./grpc";
import {
  isGrpcNotFoundError,
  isGrpcUnimplementedError,
  makeGrpcUnary,
  type UnaryCall,
} from "./grpcHelpers";
import {
  broadcastTronTx,
  getTxRefBlockNumber,
  getTxTimestamp,
  setTxTriggerSmartContractData,
} from "./tx";
import {
  tronEvmAddressToBytes21,
  tronPrivateKeyToAddressBase58,
  tronPrivateKeyToAddressBytes21,
} from "./protocol";

const requireSome = <A>(opt: Option.Option<A>, message: string): Effect.Effect<A, Error> =>
  Option.match(opt, {
    onNone: () => Effect.fail(new Error(message)),
    onSome: Effect.succeed,
  });

export class TronRelayer extends Effect.Tag("TronRelayer")<
  TronRelayer,
  {
    readonly getRelayerAddress: () => Effect.Effect<string, ConfigError.ConfigError | Error>;
    readonly getControllerEvmAddress: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
    readonly getReceiverMap: () => Effect.Effect<
      ReadonlyMap<string, TronReceiverMapEntry>,
      ConfigError.ConfigError | Error
    >;
    readonly getControllerUsdt: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
    readonly getControllerEventChainTip: () => Effect.Effect<Hex, ConfigError.ConfigError | Error>;
    readonly getControllerPulledUsdt: () => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getControllerLpExchangeRateFor: (args: {
      tokenAddress: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getErc20BalanceOf: (args: {
      tokenAddress: Address;
      account: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getTrxBalanceOf: (args: {
      account: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly sendTronControllerPullFromReceivers: (args: {
      tokenAddress: Address;
      receiverSalts: readonly Hex[];
    }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
    readonly sendTronControllerIsEventChainTip: () => Effect.Effect<
      SendTronTransactionResult,
      ConfigError.ConfigError | Error
    >;
    readonly sendTronControllerRebalanceUsdt: (args: {
      rebalancer: Address;
      inAmount: bigint;
    }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const appConfig = yield* AppConfig;
      const tronGrpc = yield* TronGrpc;

      const tronConfigCached = yield* Effect.cached(appConfig.tronNetwork());

      const controllerAddressBytes21 = () =>
        tronConfigCached.pipe(
          Effect.flatMap((config) =>
            requireSome(
              config.controllerAddressBytes21,
              "Missing env var UNTRON_CONTROLLER_ADDRESS"
            )
          )
        );

      const controllerEvmAddress = () =>
        tronConfigCached.pipe(
          Effect.flatMap((config) =>
            requireSome(config.controllerEvmAddress, "Missing env var UNTRON_CONTROLLER_ADDRESS")
          )
        );

      const receiverSalts = () =>
        tronConfigCached.pipe(
          Effect.flatMap((config) =>
            requireSome(config.preknownReceiverSalts, "Missing env var PREKNOWN_RECEIVER_SALTS")
          )
        );

      const privateKey = () =>
        tronConfigCached.pipe(
          Effect.flatMap((config) =>
            requireSome(config.privateKey, "Missing env var RELAYER_TRON_PRIVATE_KEY")
          ),
          Effect.map(Redacted.value)
        );

      const relayerAddressCached = yield* Effect.cached(
        privateKey().pipe(Effect.map(tronPrivateKeyToAddressBase58))
      );

      const relayerAddressBytes21Cached = yield* Effect.cached(
        privateKey().pipe(Effect.map(tronPrivateKeyToAddressBytes21))
      );

      const grpcUnary = makeGrpcUnary(() => tronGrpc.get());

      const tronReadContract = <T>({
        addressBytes21,
        abi,
        functionName,
        args,
      }: {
        addressBytes21: Buffer;
        abi: readonly unknown[];
        functionName: string;
        args?: readonly unknown[];
      }): Effect.Effect<T, ConfigError.ConfigError | Error> =>
        Effect.gen(function* () {
          const { wallet } = yield* tronGrpc.get();
          const ownerAddress = yield* controllerAddressBytes21();

          const data = encodeFunctionData({
            abi: abi as any,
            functionName: functionName as never,
            args: (args ?? []) as never,
          });

          const request = TriggerSmartContract.fromPartial({
            ownerAddress,
            contractAddress: addressBytes21,
            callValue: 0,
            data: Buffer.from(data.slice(2), "hex"),
          });

          const res = yield* grpcUnary(
            wallet.triggerConstantContract.bind(wallet) as unknown as UnaryCall<
              TriggerSmartContract,
              TransactionExtention
            >,
            request
          );

          if (!res.result?.result) {
            const msg = res.result?.message?.length
              ? res.result.message.toString("utf8")
              : "unknown";
            return yield* Effect.fail(new Error(`Tron triggerConstantContract failed: ${msg}`));
          }

          const buf = res.constantResult?.[0];
          if (!buf || buf.length === 0) {
            return yield* Effect.fail(
              new Error("Tron triggerConstantContract returned empty constantResult")
            );
          }

          return decodeFunctionResult({
            abi: abi as any,
            functionName: functionName as never,
            data: `0x${buf.toString("hex")}` as Hex,
          }) as T;
        });

      const getReceiverMapCached = yield* Effect.cached(
        Effect.gen(function* () {
          const salts = yield* receiverSalts();
          const controllerBytes21 = yield* controllerAddressBytes21();

          const entries = yield* Effect.forEach(salts, (receiverSalt) =>
            tronReadContract<Address>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "predictReceiverAddress",
              args: [receiverSalt],
            }).pipe(
              Effect.map(
                (receiverAddress) =>
                  ({ receiverAddress, receiverSalt }) satisfies TronReceiverMapEntry
              )
            )
          );

          const map = new Map<string, TronReceiverMapEntry>();
          for (const entry of entries) map.set(entry.receiverAddress.toLowerCase(), entry);
          return map as ReadonlyMap<string, TronReceiverMapEntry>;
        })
      );

      const isTronContractDeployed = (
        addressBytes21: Buffer
      ): Effect.Effect<boolean, ConfigError.ConfigError | Error> =>
        Effect.gen(function* () {
          const { wallet } = yield* tronGrpc.get();
          const req: BytesMessage = { value: addressBytes21 };
          try {
            const res = yield* grpcUnary(
              wallet.getContract.bind(wallet) as unknown as UnaryCall<BytesMessage, SmartContract>,
              req
            );
            return (res.bytecode?.length ?? 0) > 0 || (res.codeHash?.length ?? 0) > 0;
          } catch (error) {
            if (isGrpcNotFoundError(error)) return false;
            return yield* Effect.fail(error instanceof Error ? error : new Error(String(error)));
          }
        });

      const getTransactionInfoById = (txidHex: string) =>
        Effect.gen(function* () {
          const { wallet } = yield* tronGrpc.get();
          const req: BytesMessage = { value: Buffer.from(txidHex, "hex") };

          const fromWallet = yield* grpcUnary(
            wallet.getTransactionInfoById.bind(wallet) as unknown as UnaryCall<
              BytesMessage,
              TransactionInfo
            >,
            req
          ).pipe(
            Effect.catchAll((error) =>
              isGrpcNotFoundError(error)
                ? Effect.succeed(null)
                : isGrpcUnimplementedError(error)
                  ? Effect.fail(
                      new Error(
                        "Tron gRPC endpoint does not implement Wallet/GetTransactionInfoById (check TRON_GRPC_HOST)"
                      )
                    )
                  : Effect.fail(error)
            )
          );
          if (fromWallet && fromWallet.id?.length) return fromWallet;
          return null;
        });

      const buildControllerMulticallTx = (calls: readonly Hex[]) =>
        Effect.gen(function* () {
          if (calls.length === 0) {
            return yield* Effect.fail(
              new Error("buildControllerMulticallTx: expected at least 1 call")
            );
          }

          const config = yield* tronConfigCached;
          if (config.callValue !== 0) {
            return yield* Effect.fail(
              new Error(
                "RELAYER_TRON_CALL_VALUE must be 0 when using UntronController.multicall (Solady Multicallable disallows msg.value)"
              )
            );
          }

          const { wallet } = yield* tronGrpc.get();
          const controllerBytes21 = yield* controllerAddressBytes21();
          const ownerAddressBytes21 = yield* relayerAddressBytes21Cached;

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

          const txExt = yield* grpcUnary(
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

          const controllerBytes21 = yield* controllerAddressBytes21();

          for (let attempt = 0; attempt < 3; attempt++) {
            const preTip = yield* tronReadContract<Hex>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "eventChainTip",
            });

            const plannedEvents = yield* planIndexedEventsForControllerCalls({
              controllerBytes21,
              calls,
              tronReadContract,
              isTronContractDeployed,
            });

            const tx = yield* buildControllerMulticallTx(calls);
            const blockNumber = getTxRefBlockNumber(tx);
            const blockTimestamp = getTxTimestamp(tx);

            let expectedTip = preTip;
            for (const event of plannedEvents) {
              expectedTip = computeNextEventChainTip({
                previousTip: expectedTip,
                blockNumber,
                blockTimestamp,
                eventSignature: event.eventSignature,
                encodedEventData: event.encodedEventData,
              });
            }

            const checkpointCall = encodeFunctionData({
              abi: untronControllerAbi,
              functionName: "isEventChainTip",
              args: [expectedTip],
            });

            const finalData = encodeFunctionData({
              abi: untronControllerAbi,
              functionName: "multicall",
              args: [[...calls, checkpointCall]],
            });

            setTxTriggerSmartContractData({ tx, data: finalData });

            try {
              const config = yield* tronConfigCached;
              const { wallet } = yield* tronGrpc.get();
              const privateKeyRaw = yield* privateKey();

              const txidHex = yield* broadcastTronTx({
                tx,
                feeLimit: config.feeLimit,
                privateKeyRaw,
                pollTimes: config.pollTimes,
                pollIntervalMs: config.pollIntervalMs,
                grpcUnary,
                wallet,
                getTransactionInfoById,
              });
              yield* Effect.logInfo("[tron] tx confirmed").pipe(
                Effect.annotateLogs({ txid: txidHex })
              );
              return txidHex;
            } catch (error) {
              const err = error instanceof Error ? error : new Error(String(error));
              const errorMessage = `${err.name}: ${err.message}`.toLowerCase();
              if (!errorMessage.includes("no")) return yield* Effect.fail(err);
            }
          }

          return yield* Effect.fail(
            new Error(
              "Failed to send Tron multicall with in-tx eventChainTip checkpoint (tip kept changing or prediction mismatch)"
            )
          );
        }).pipe(Effect.withLogSpan("tron.controllerMulticall"));

      const getControllerUsdt = () =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<Address>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "usdt",
            })
          )
        );

      const getControllerEventChainTip = () =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<Hex>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "eventChainTip",
            })
          )
        );

      const getControllerPulledUsdt = () =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<bigint>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "pulledUsdt",
            })
          )
        );

      const getControllerLpExchangeRateFor = ({ tokenAddress }: { tokenAddress: Address }) =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<bigint>({
              addressBytes21: controllerBytes21,
              abi: untronControllerAbi,
              functionName: "lpExchangeRateFor",
              args: [tokenAddress],
            })
          )
        );

      const getErc20BalanceOf = ({
        tokenAddress,
        account,
      }: {
        tokenAddress: Address;
        account: Address;
      }) =>
        tronReadContract<bigint>({
          addressBytes21: tronEvmAddressToBytes21(tokenAddress),
          abi: ERC20Abi,
          functionName: "balanceOf",
          args: [account],
        });

      const getTrxBalanceOf = ({ account }: { account: Address }) =>
        Effect.gen(function* () {
          const { wallet } = yield* tronGrpc.get();

          const request = AccountBalanceRequest.fromPartial({
            accountIdentifier: {
              address: tronEvmAddressToBytes21(account),
            },
          });

          const res = yield* grpcUnary(
            wallet.getAccountBalance.bind(wallet) as unknown as UnaryCall<
              AccountBalanceRequest,
              AccountBalanceResponse
            >,
            request
          );

          return BigInt(res.balance.toString());
        });

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
          const { wallet } = yield* tronGrpc.get();
          const config = yield* tronConfigCached;

          if (config.callValue !== 0) {
            return yield* Effect.fail(
              new Error("TRON_CALL_VALUE must be 0 when calling UntronController.isEventChainTip")
            );
          }

          const controllerBytes21 = yield* controllerAddressBytes21();
          const ownerAddressBytes21 = yield* relayerAddressBytes21Cached;

          for (let attempt = 0; attempt < 3; attempt++) {
            const tip = yield* tronReadContract<Hex>({
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

            const txExt = yield* grpcUnary(
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
              const config = yield* tronConfigCached;
              const privateKeyRaw = yield* privateKey();
              const txid = yield* broadcastTronTx({
                tx,
                feeLimit: config.feeLimit,
                privateKeyRaw,
                pollTimes: config.pollTimes,
                pollIntervalMs: config.pollIntervalMs,
                grpcUnary,
                wallet,
                getTransactionInfoById,
              });
              yield* Effect.logInfo("[tron] tx confirmed").pipe(Effect.annotateLogs({ txid }));
              return { txid };
            } catch (error) {
              const err = error instanceof Error ? error : new Error(String(error));
              const errorMessage = `${err.name}: ${err.message}`.toLowerCase();
              if (!errorMessage.includes("no")) return yield* Effect.fail(err);
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
        getRelayerAddress: () => relayerAddressCached,
        getControllerEvmAddress: controllerEvmAddress,
        getReceiverMap: () => getReceiverMapCached,
        getControllerUsdt,
        getControllerEventChainTip,
        getControllerPulledUsdt,
        getControllerLpExchangeRateFor,
        getErc20BalanceOf,
        getTrxBalanceOf,
        sendTronControllerPullFromReceivers,
        sendTronControllerIsEventChainTip,
        sendTronControllerRebalanceUsdt,
      };
    })
  );
}
