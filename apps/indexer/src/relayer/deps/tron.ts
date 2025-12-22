import { ConfigError, Effect, Layer, Option, Redacted } from "effect";
import {
  decodeAbiParameters,
  decodeFunctionData,
  decodeFunctionResult,
  encodeAbiParameters,
  encodeFunctionData,
  isAddress,
  keccak256,
  stringToHex,
  type Address,
  type Hex,
} from "viem";

import type { BytesMessage, Return, TransactionExtention } from "@untron/tron-protocol/api";
import { AccountBalanceRequest } from "@untron/tron-protocol/core/contract/balance_contract";
import type { AccountBalanceResponse } from "@untron/tron-protocol/core/contract/balance_contract";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { SmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import {
  TransactionInfo_code,
  Transaction_raw,
  type TransactionInfo,
  type Transaction,
} from "@untron/tron-protocol/tron";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { UntronControllerAbi } from "../../../abis/tron/UntronControllerAbi";
import { AppConfig } from "../../effect/config";
import { computeNextEventChainTip } from "../../eventChain/tip";

import type { SendTronTransactionResult, TronReceiverMapEntry } from "./types";
import { TronGrpc } from "./tronGrpc";
import {
  signTronTransaction,
  tronEvmAddressToBytes21,
  tronPrivateKeyToAddressBase58,
  tronPrivateKeyToAddressBytes21,
} from "./tronProtocol";

const LEGACY_MESH_OFT_ABI = [
  {
    type: "function",
    name: "feeBps",
    inputs: [],
    outputs: [{ name: "", type: "uint16", internalType: "uint16" }],
    stateMutability: "view",
  },
  {
    type: "function",
    name: "BPS_DENOMINATOR",
    inputs: [],
    outputs: [{ name: "", type: "uint16", internalType: "uint16" }],
    stateMutability: "view",
  },
] as const;

const RATE_SCALE = 1_000_000_000_000_000_000n; // 1e18

const EVENT_SIG_RECEIVER_DEPLOYED = keccak256(
  stringToHex("ReceiverDeployed(address,bytes32)")
) as Hex;
const EVENT_SIG_PULLED_FROM_RECEIVER = keccak256(
  stringToHex("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)")
) as Hex;
const EVENT_SIG_USDT_REBALANCED = keccak256(
  stringToHex("UsdtRebalanced(uint256,uint256,address)")
) as Hex;

type UnaryCall<Req, Res> = (
  request: Req,
  metadata: unknown,
  callback: (error: unknown, response?: Res) => void
) => unknown;

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

      const grpcUnary = <Req, Res>(call: UnaryCall<Req, Res>, req: Req) =>
        tronGrpc.get().pipe(
          Effect.flatMap(({ callOpts }) =>
            Effect.tryPromise({
              try: () =>
                new Promise<Res>((resolve, reject) => {
                  try {
                    call(req, callOpts.metadata, (err, res) => {
                      if (err) return reject(err);
                      if (res === undefined) return reject(new Error("Empty gRPC response"));
                      resolve(res);
                    });
                  } catch (err) {
                    reject(err);
                  }
                }),
              catch: (error) => (error instanceof Error ? error : new Error(String(error))),
            })
          )
        );

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
              abi: UntronControllerAbi,
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

      const isGrpcNotFoundError = (error: unknown): boolean => {
        if (!error || typeof error !== "object") return false;
        const maybeCode = (error as { readonly code?: unknown }).code;
        return maybeCode === 5;
      };

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

      const getTxRefBlockNumber = (tx: Transaction): bigint => {
        const rawData = tx.rawData;
        if (!rawData) throw new Error("Tron tx missing rawData");
        return BigInt(rawData.refBlockNum.toString());
      };

      const getTxTimestamp = (tx: Transaction): bigint => {
        const rawData = tx.rawData;
        if (!rawData) throw new Error("Tron tx missing rawData");
        return BigInt(rawData.timestamp.toString());
      };

      const setTxTriggerSmartContractData = (tx: Transaction, data: Hex) => {
        const rawData = tx.rawData;
        if (!rawData) throw new Error("Tron tx missing rawData");

        const contract = rawData.contract?.[0];
        if (!contract?.parameter?.value?.length) {
          throw new Error("Tron tx missing contract parameter value");
        }

        const trigger = TriggerSmartContract.decode(contract.parameter.value);
        trigger.data = Buffer.from(data.slice(2), "hex");
        contract.parameter.value = Buffer.from(TriggerSmartContract.encode(trigger).finish());
      };

      const getTransactionInfoById = (txidHex: string) =>
        Effect.gen(function* () {
          const { wallet, solidity } = yield* tronGrpc.get();
          const req: BytesMessage = { value: Buffer.from(txidHex, "hex") };

          const fetch = (call: UnaryCall<BytesMessage, TransactionInfo>) =>
            grpcUnary(call, req).pipe(
              Effect.catchAll((error) =>
                isGrpcNotFoundError(error) ? Effect.succeed(null) : Effect.fail(error)
              )
            );

          const fromWallet = yield* fetch(
            wallet.getTransactionInfoById.bind(wallet) as unknown as UnaryCall<
              BytesMessage,
              TransactionInfo
            >
          );
          if (fromWallet && fromWallet.id?.length) return fromWallet;

          const fromSolidity = yield* fetch(
            solidity.getTransactionInfoById.bind(solidity) as unknown as UnaryCall<
              BytesMessage,
              TransactionInfo
            >
          );
          if (fromSolidity && fromSolidity.id?.length) return fromSolidity;
          return null;
        });

      const waitForTronTransaction = ({
        txid,
        pollTimes,
        pollIntervalMs,
      }: {
        txid: string;
        pollTimes: number;
        pollIntervalMs: number;
      }) =>
        Effect.gen(function* () {
          for (let i = 0; i < pollTimes; i++) {
            const info = yield* getTransactionInfoById(txid);
            if (!info) {
              yield* Effect.sleep(pollIntervalMs);
              continue;
            }

            if (info.result === TransactionInfo_code.FAILED) {
              const message = info.resMessage?.length
                ? info.resMessage.toString("utf8")
                : "Tron transaction execution failed";
              return yield* Effect.fail(new Error(message));
            }

            return;
          }

          return yield* Effect.fail(new Error(`Timed out waiting for Tron tx receipt: ${txid}`));
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
            abi: UntronControllerAbi,
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

      const broadcastTronTx = (tx: Transaction) =>
        Effect.gen(function* () {
          const config = yield* tronConfigCached;
          const { wallet } = yield* tronGrpc.get();

          const privateKeyRaw = yield* privateKey();

          if (!tx.rawData) return yield* Effect.fail(new Error("Tron tx missing rawData"));
          tx.rawData = Transaction_raw.fromPartial({ ...tx.rawData, feeLimit: config.feeLimit });

          const { txidHex, signed } = signTronTransaction(tx, privateKeyRaw);

          const broadcast = yield* grpcUnary(
            wallet.broadcastTransaction.bind(wallet) as unknown as UnaryCall<Transaction, Return>,
            signed
          );

          if (!broadcast.result) {
            const msg = broadcast.message?.length ? broadcast.message.toString("utf8") : "unknown";
            return yield* Effect.fail(new Error(`Tron broadcast rejected: ${msg}`));
          }

          yield* waitForTronTransaction({
            txid: txidHex,
            pollTimes: config.pollTimes,
            pollIntervalMs: config.pollIntervalMs,
          });

          return txidHex;
        });

      type PlannedIndexedEvent = {
        eventSignature: Hex;
        encodedEventData: Hex;
      };

      const planIndexedEventsForCalls = (calls: readonly Hex[]) =>
        Effect.gen(function* () {
          const controllerBytes21 = yield* controllerAddressBytes21();
          const usdtAddress = yield* tronReadContract<Address>({
            addressBytes21: controllerBytes21,
            abi: UntronControllerAbi,
            functionName: "usdt",
          });

          const receiverAddressCache = new Map<string, Address>();
          const receiverDeployedCache = new Map<string, boolean>();
          const tokenRateCache = new Map<string, bigint>();
          const balanceCache = new Map<string, bigint>();

          const predictReceiverAddresses = (salts: readonly Hex[]) =>
            Effect.gen(function* () {
              const pendingSalts = salts.filter(
                (salt) => !receiverAddressCache.has(salt.toLowerCase())
              );

              if (pendingSalts.length) {
                const multicallCalls = pendingSalts.map((salt) =>
                  encodeFunctionData({
                    abi: UntronControllerAbi,
                    functionName: "predictReceiverAddress",
                    args: [salt],
                  })
                );

                const results = yield* tronReadContract<readonly Hex[]>({
                  addressBytes21: controllerBytes21,
                  abi: UntronControllerAbi,
                  functionName: "multicall",
                  args: [multicallCalls],
                });

                if (results.length !== pendingSalts.length) {
                  return yield* Effect.fail(
                    new Error(
                      `Tron controller multicall returned unexpected results length (expected ${pendingSalts.length}, got ${results.length})`
                    )
                  );
                }

                for (let i = 0; i < pendingSalts.length; i++) {
                  const salt = pendingSalts[i]!;
                  const data = results[i]!;
                  const decoded = decodeFunctionResult({
                    abi: UntronControllerAbi,
                    functionName: "predictReceiverAddress",
                    data,
                  }) as Address;
                  receiverAddressCache.set(salt.toLowerCase(), decoded);
                }
              }

              return salts.map((salt) => {
                const address = receiverAddressCache.get(salt.toLowerCase());
                if (!address)
                  throw new Error(`Missing predicted receiver address for salt ${salt}`);
                return address;
              });
            });

          const getLpExchangeRateFor = (token: Address) =>
            Effect.gen(function* () {
              const key = token.toLowerCase();
              const cached = tokenRateCache.get(key);
              if (cached !== undefined) return cached;

              const rate = yield* tronReadContract<bigint>({
                addressBytes21: controllerBytes21,
                abi: UntronControllerAbi,
                functionName: "lpExchangeRateFor",
                args: [token],
              });

              tokenRateCache.set(key, rate);
              return rate;
            });

          const getTokenBalanceOf = (token: Address, account: Address) =>
            Effect.gen(function* () {
              const key = `${token.toLowerCase()}:${account.toLowerCase()}`;
              const cached = balanceCache.get(key);
              if (cached !== undefined) return cached;

              const balance = yield* tronReadContract<bigint>({
                addressBytes21: tronEvmAddressToBytes21(token),
                abi: ERC20Abi,
                functionName: "balanceOf",
                args: [account],
              });

              balanceCache.set(key, balance);
              return balance;
            });

          const getReceiverDeployed = (receiver: Address) =>
            Effect.gen(function* () {
              const key = receiver.toLowerCase();
              const cached = receiverDeployedCache.get(key);
              if (cached !== undefined) return cached;

              const deployed = yield* isTronContractDeployed(tronEvmAddressToBytes21(receiver));
              receiverDeployedCache.set(key, deployed);
              return deployed;
            });

          const computeLegacyMeshOutAmount = ({
            rebalancer,
            inAmount,
          }: {
            rebalancer: Address;
            inAmount: bigint;
          }) =>
            Effect.gen(function* () {
              const payload = yield* tronReadContract<Hex>({
                addressBytes21: controllerBytes21,
                abi: UntronControllerAbi,
                functionName: "payloadFor",
                args: [rebalancer],
              });
              if (!payload || payload === "0x") {
                return yield* Effect.fail(
                  new Error("Tron rebalance route not set (payloadFor empty)")
                );
              }

              const decoded = decodeAbiParameters(
                [{ type: "address" }, { type: "uint32" }, { type: "bytes32" }],
                payload
              );
              const oft = decoded[0];
              if (typeof oft !== "string" || !isAddress(oft)) {
                return yield* Effect.fail(
                  new Error(
                    "Unsupported rebalance payload (expected (address oft, uint32, bytes32))"
                  )
                );
              }

              const feeBps = yield* tronReadContract<bigint>({
                addressBytes21: tronEvmAddressToBytes21(oft),
                abi: LEGACY_MESH_OFT_ABI,
                functionName: "feeBps",
              });
              const denom = yield* tronReadContract<bigint>({
                addressBytes21: tronEvmAddressToBytes21(oft),
                abi: LEGACY_MESH_OFT_ABI,
                functionName: "BPS_DENOMINATOR",
              });

              if (denom === 0n)
                return yield* Effect.fail(new Error("Legacy Mesh OFT BPS_DENOMINATOR returned 0"));
              const fee = (inAmount * feeBps) / denom;
              return inAmount - fee;
            });

          const plannedEvents: PlannedIndexedEvent[] = [];

          for (const callData of calls) {
            const decoded = decodeFunctionData({ abi: UntronControllerAbi, data: callData });

            if (decoded.functionName === "pullFromReceivers") {
              const token = decoded.args?.[0] as Address | undefined;
              const salts = decoded.args?.[1] as readonly Hex[] | undefined;
              if (!token || !isAddress(token))
                return yield* Effect.fail(new Error("pullFromReceivers: invalid token arg"));
              if (!salts || salts.length === 0)
                return yield* Effect.fail(new Error("pullFromReceivers: missing receiverSalts"));

              const isUsdt = token.toLowerCase() === usdtAddress.toLowerCase();
              const rateUsed = isUsdt ? RATE_SCALE : yield* getLpExchangeRateFor(token);
              if (!isUsdt && rateUsed === 0n) {
                return yield* Effect.fail(
                  new Error("pullFromReceivers: LP exchange rate not set (lpExchangeRateFor == 0)")
                );
              }

              const receiverAddresses = yield* predictReceiverAddresses(salts);

              for (let i = 0; i < salts.length; i++) {
                const receiverSalt = salts[i]!;
                const receiverAddress = receiverAddresses[i]!;

                const balance = yield* getTokenBalanceOf(token, receiverAddress);
                const sweepAmount = balance > 0n ? balance - 1n : 0n;
                if (sweepAmount === 0n) continue;

                const deployed = yield* getReceiverDeployed(receiverAddress);
                if (!deployed) {
                  plannedEvents.push({
                    eventSignature: EVENT_SIG_RECEIVER_DEPLOYED,
                    encodedEventData: encodeAbiParameters(
                      [{ type: "address" }, { type: "bytes32" }],
                      [receiverAddress, receiverSalt]
                    ),
                  });
                  receiverDeployedCache.set(receiverAddress.toLowerCase(), true);
                }

                const usdtAmount = isUsdt ? sweepAmount : (sweepAmount * rateUsed) / RATE_SCALE;
                plannedEvents.push({
                  eventSignature: EVENT_SIG_PULLED_FROM_RECEIVER,
                  encodedEventData: encodeAbiParameters(
                    [
                      { type: "bytes32" },
                      { type: "address" },
                      { type: "uint256" },
                      { type: "uint256" },
                      { type: "uint256" },
                    ],
                    [receiverSalt, token, sweepAmount, rateUsed, usdtAmount]
                  ),
                });

                balanceCache.set(`${token.toLowerCase()}:${receiverAddress.toLowerCase()}`, 1n);
              }

              continue;
            }

            if (decoded.functionName === "rebalanceUsdt") {
              const rebalancer = decoded.args?.[0] as Address | undefined;
              const inAmount = decoded.args?.[1] as bigint | undefined;
              if (!rebalancer || !isAddress(rebalancer))
                return yield* Effect.fail(new Error("rebalanceUsdt: invalid rebalancer"));
              if (typeof inAmount !== "bigint")
                return yield* Effect.fail(new Error("rebalanceUsdt: invalid inAmount"));

              const outAmount = yield* computeLegacyMeshOutAmount({ rebalancer, inAmount });

              plannedEvents.push({
                eventSignature: EVENT_SIG_USDT_REBALANCED,
                encodedEventData: encodeAbiParameters(
                  [{ type: "uint256" }, { type: "uint256" }, { type: "address" }],
                  [inAmount, outAmount, rebalancer]
                ),
              });
              continue;
            }

            if (decoded.functionName === "isEventChainTip") continue;

            return yield* Effect.fail(
              new Error(`sendTronControllerMulticall: unsupported call "${decoded.functionName}"`)
            );
          }

          return plannedEvents;
        });

      const sendTronControllerMulticall = ({ calls }: { calls: readonly Hex[] }) =>
        Effect.gen(function* () {
          if (calls.length === 0) {
            return yield* Effect.fail(
              new Error("sendTronControllerMulticall: expected at least 1 call")
            );
          }

          const controllerBytes21 = yield* controllerAddressBytes21();

          for (let attempt = 0; attempt < 3; attempt++) {
            const preTip = yield* tronReadContract<Hex>({
              addressBytes21: controllerBytes21,
              abi: UntronControllerAbi,
              functionName: "eventChainTip",
            });

            const plannedEvents = yield* planIndexedEventsForCalls(calls);

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
              abi: UntronControllerAbi,
              functionName: "isEventChainTip",
              args: [expectedTip],
            });

            const finalData = encodeFunctionData({
              abi: UntronControllerAbi,
              functionName: "multicall",
              args: [[...calls, checkpointCall]],
            });

            setTxTriggerSmartContractData(tx, finalData);

            try {
              return yield* broadcastTronTx(tx);
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
        });

      const getControllerUsdt = () =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<Address>({
              addressBytes21: controllerBytes21,
              abi: UntronControllerAbi,
              functionName: "usdt",
            })
          )
        );

      const getControllerPulledUsdt = () =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<bigint>({
              addressBytes21: controllerBytes21,
              abi: UntronControllerAbi,
              functionName: "pulledUsdt",
            })
          )
        );

      const getControllerLpExchangeRateFor = ({ tokenAddress }: { tokenAddress: Address }) =>
        controllerAddressBytes21().pipe(
          Effect.flatMap((controllerBytes21) =>
            tronReadContract<bigint>({
              addressBytes21: controllerBytes21,
              abi: UntronControllerAbi,
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
            abi: UntronControllerAbi,
            functionName: "pullFromReceivers",
            args: [tokenAddress, receiverSalts],
          });

          const txid = yield* sendTronControllerMulticall({ calls: [call] });
          return { txid };
        });

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
            abi: UntronControllerAbi,
            functionName: "rebalanceUsdt",
            args: [rebalancer, inAmount],
          });

          const txid = yield* sendTronControllerMulticall({ calls: [call] });
          return { txid };
        });

      return {
        getRelayerAddress: () => relayerAddressCached,
        getControllerEvmAddress: controllerEvmAddress,
        getReceiverMap: () => getReceiverMapCached,
        getControllerUsdt,
        getControllerPulledUsdt,
        getControllerLpExchangeRateFor,
        getErc20BalanceOf,
        getTrxBalanceOf,
        sendTronControllerPullFromReceivers,
        sendTronControllerRebalanceUsdt,
      };
    })
  );
}
