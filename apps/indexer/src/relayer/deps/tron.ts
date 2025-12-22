import {
  decodeAbiParameters,
  decodeFunctionData,
  decodeFunctionResult,
  encodeAbiParameters,
  encodeFunctionData,
  encodePacked,
  isAddress,
  keccak256,
  sha256,
  stringToHex,
  type Address,
  type Hex,
} from "viem";

import type { BytesMessage } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import {
  TransactionInfo_code,
  Transaction_raw,
  type Transaction,
} from "@untron/tron-protocol/tron";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { UntronControllerAbi } from "../../../abis/tron/UntronControllerAbi";
import { parseNumberEnv } from "../env";
import type { RelayerDeps, TronReceiverMapEntry } from "./types";
import {
  signTronTransaction,
  tronBase58ToBytes21,
  tronBase58ToEvmAddress,
  tronEvmAddressToBytes21,
  tronPrivateKeyToAddressBase58,
  tronPrivateKeyToAddressBytes21,
} from "./tronProtocol";

function sleep(ms: number) {
  return new Promise<void>((resolve) => setTimeout(resolve, ms));
}

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

function computeNextEventChainTip({
  previousTip,
  blockNumber,
  blockTimestamp,
  eventSignature,
  encodedEventData,
}: {
  previousTip: Hex;
  blockNumber: bigint;
  blockTimestamp: bigint;
  eventSignature: Hex;
  encodedEventData: Hex;
}): Hex {
  return sha256(
    encodePacked(
      ["bytes32", "uint256", "uint256", "bytes32", "bytes"],
      [previousTip, blockNumber, blockTimestamp, eventSignature, encodedEventData]
    )
  );
}

export function createTronRelayer({
  getTronGrpcClients,
}: {
  getTronGrpcClients: RelayerDeps["getTronGrpcClients"];
}): Pick<
  RelayerDeps,
  | "getTronRelayerAddress"
  | "getTronControllerEvmAddress"
  | "getTronReceiverMap"
  | "sendTronControllerPullFromReceivers"
  | "sendTronControllerRebalanceUsdt"
> {
  let tronRelayerAddress: string | null = null;
  let tronRelayerAddressBytes21: Buffer | null = null;
  let tronControllerEvmAddress: Address | null = null;
  let tronControllerAddressBytes21: Buffer | null = null;
  let tronReceiverMapPromise: Promise<ReadonlyMap<string, TronReceiverMapEntry>> | null = null;

  const grpcUnary = async (call: any, req: any) => {
    const { callOpts } = getTronGrpcClients();
    return await new Promise<any>((resolve, reject) => {
      call(req, callOpts.metadata, (err: any, res: any) => {
        if (err) return reject(err);
        if (!res) return reject(new Error("Empty gRPC response"));
        resolve(res);
      });
    });
  };

  const getTronControllerAddressBytes21 = (): Buffer => {
    if (tronControllerAddressBytes21) return tronControllerAddressBytes21;
    const controllerBase58 = process.env.UNTRON_CONTROLLER_ADDRESS;
    if (!controllerBase58) throw new Error("Missing env var UNTRON_CONTROLLER_ADDRESS");
    tronControllerAddressBytes21 = tronBase58ToBytes21(controllerBase58);
    return tronControllerAddressBytes21;
  };

  const getTronRelayerAddress = (): string => {
    if (tronRelayerAddress) return tronRelayerAddress;
    const privateKeyRaw = process.env.RELAYER_TRON_PRIVATE_KEY;
    if (!privateKeyRaw) throw new Error("Missing env var RELAYER_TRON_PRIVATE_KEY");
    tronRelayerAddress = tronPrivateKeyToAddressBase58(privateKeyRaw);
    return tronRelayerAddress;
  };

  const getTronRelayerAddressBytes21 = (): Buffer => {
    if (tronRelayerAddressBytes21) return tronRelayerAddressBytes21;
    const privateKeyRaw = process.env.RELAYER_TRON_PRIVATE_KEY;
    if (!privateKeyRaw) throw new Error("Missing env var RELAYER_TRON_PRIVATE_KEY");
    tronRelayerAddressBytes21 = tronPrivateKeyToAddressBytes21(privateKeyRaw);
    return tronRelayerAddressBytes21;
  };

  const getTronControllerEvmAddress = (): Address => {
    if (tronControllerEvmAddress) return tronControllerEvmAddress;
    const controllerBase58 = process.env.UNTRON_CONTROLLER_ADDRESS;
    if (!controllerBase58) throw new Error("Missing env var UNTRON_CONTROLLER_ADDRESS");
    tronControllerEvmAddress = tronBase58ToEvmAddress(controllerBase58);
    return tronControllerEvmAddress;
  };

  const tronReadContract = async <T>({
    addressBytes21,
    abi,
    functionName,
    args,
  }: {
    addressBytes21: Buffer;
    abi: any;
    functionName: string;
    args?: readonly unknown[];
  }): Promise<T> => {
    const { wallet } = getTronGrpcClients();

    const ownerAddress = getTronControllerAddressBytes21();
    const data = encodeFunctionData({
      abi,
      functionName: functionName as never,
      args: (args ?? []) as never,
    });

    const request = TriggerSmartContract.fromPartial({
      ownerAddress,
      contractAddress: addressBytes21,
      callValue: 0,
      data: Buffer.from(data.slice(2), "hex"),
    });

    const res = await grpcUnary(wallet.triggerConstantContract.bind(wallet), request);
    if (!res.result?.result) {
      const msg = res.result?.message?.length ? res.result.message.toString("utf8") : "unknown";
      throw new Error(`Tron triggerConstantContract failed: ${msg}`);
    }

    const buf = res.constantResult?.[0];
    if (!buf || buf.length === 0) {
      throw new Error("Tron triggerConstantContract returned empty constantResult");
    }

    return decodeFunctionResult({
      abi,
      functionName: functionName as never,
      data: `0x${buf.toString("hex")}` as Hex,
    }) as T;
  };

  const getTronReceiverMap = async (): Promise<ReadonlyMap<string, TronReceiverMapEntry>> => {
    if (tronReceiverMapPromise) return tronReceiverMapPromise;

    tronReceiverMapPromise = (async () => {
      const saltsRaw = process.env.PREKNOWN_RECEIVER_SALTS;
      if (!saltsRaw) throw new Error("Missing env var PREKNOWN_RECEIVER_SALTS");

      const salts = saltsRaw
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean)
        .map((salt) => (salt.startsWith("0x") ? (salt as Hex) : (`0x${salt}` as Hex)));

      if (salts.length === 0) throw new Error("Missing env var PREKNOWN_RECEIVER_SALTS");

      for (const salt of salts) {
        if (!/^0x[0-9a-f]{64}$/i.test(salt)) {
          throw new Error(`Invalid receiver salt "${salt}" (expected bytes32 hex)`);
        }
      }

      const controllerBytes21 = getTronControllerAddressBytes21();

      const entries = await Promise.all(
        salts.map(async (receiverSalt) => {
          const receiverAddress = await tronReadContract<Address>({
            addressBytes21: controllerBytes21,
            abi: UntronControllerAbi,
            functionName: "predictReceiverAddress",
            args: [receiverSalt],
          });

          return {
            receiverAddress,
            receiverSalt,
          } satisfies TronReceiverMapEntry;
        })
      );

      const map = new Map<string, TronReceiverMapEntry>();
      for (const entry of entries) {
        map.set(entry.receiverAddress.toLowerCase(), entry);
      }
      return map;
    })();

    return tronReceiverMapPromise;
  };

  const isGrpcNotFoundError = (error: unknown): boolean => {
    if (!error || typeof error !== "object") return false;
    const maybeCode = (error as any).code;
    return maybeCode === 5;
  };

  const isTronContractDeployed = async (addressBytes21: Buffer): Promise<boolean> => {
    const { wallet } = getTronGrpcClients();
    const req: BytesMessage = { value: addressBytes21 };
    try {
      const res = await grpcUnary(wallet.getContract.bind(wallet), req);
      return (res.bytecode?.length ?? 0) > 0 || (res.codeHash?.length ?? 0) > 0;
    } catch (error) {
      if (isGrpcNotFoundError(error)) return false;
      throw error;
    }
  };

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

  const getTransactionInfoById = async (txidHex: string) => {
    const { wallet, solidity } = getTronGrpcClients();
    const req: BytesMessage = { value: Buffer.from(txidHex, "hex") };

    const fetch = async (client: any) => {
      try {
        return await grpcUnary(client.getTransactionInfoById.bind(client), req);
      } catch (error) {
        if (isGrpcNotFoundError(error)) return null;
        throw error;
      }
    };

    const fromWallet = await fetch(wallet);
    if (fromWallet && fromWallet.id?.length) return fromWallet;
    const fromSolidity = await fetch(solidity);
    if (fromSolidity && fromSolidity.id?.length) return fromSolidity;
    return null;
  };

  const waitForTronTransaction = async ({
    txid,
    pollTimes,
    pollIntervalMs,
  }: {
    txid: string;
    pollTimes: number;
    pollIntervalMs: number;
  }) => {
    for (let i = 0; i < pollTimes; i++) {
      const info = await getTransactionInfoById(txid);
      if (!info) {
        await sleep(pollIntervalMs);
        continue;
      }

      if (info.result === TransactionInfo_code.FAILED) {
        const message = info.resMessage?.length
          ? info.resMessage.toString("utf8")
          : "Tron transaction execution failed";
        throw new Error(message);
      }

      return;
    }

    throw new Error(`Timed out waiting for Tron tx receipt: ${txid}`);
  };

  const getRelayerPrivateKey = (): string => {
    const privateKeyRaw = process.env.RELAYER_TRON_PRIVATE_KEY;
    if (!privateKeyRaw) throw new Error("Missing env var RELAYER_TRON_PRIVATE_KEY");
    return privateKeyRaw;
  };

  const buildControllerMulticallTx = async (calls: readonly Hex[]): Promise<Transaction> => {
    if (calls.length === 0) throw new Error("buildControllerMulticallTx: expected at least 1 call");

    const { wallet } = getTronGrpcClients();
    const controllerAddressBytes21 = getTronControllerAddressBytes21();
    const ownerAddressBytes21 = getTronRelayerAddressBytes21();

    const callValue = parseNumberEnv("RELAYER_TRON_CALL_VALUE", 0);

    if (callValue !== 0) {
      throw new Error(
        "RELAYER_TRON_CALL_VALUE must be 0 when using UntronController.multicall (Solady Multicallable disallows msg.value)"
      );
    }

    const data = encodeFunctionData({
      abi: UntronControllerAbi,
      functionName: "multicall",
      args: [calls],
    });

    const request = TriggerSmartContract.fromPartial({
      ownerAddress: ownerAddressBytes21,
      contractAddress: controllerAddressBytes21,
      callValue,
      data: Buffer.from(data.slice(2), "hex"),
    });

    const txExt = await grpcUnary(wallet.triggerContract.bind(wallet), request);
    if (!txExt.result?.result) {
      const msg = txExt.result?.message?.length ? txExt.result.message.toString("utf8") : "unknown";
      throw new Error(`Tron triggerContract failed: ${msg}`);
    }
    const tx = txExt.transaction;
    if (!tx || !tx.rawData) {
      throw new Error(`Tron triggerContract returned no transaction: ${JSON.stringify(txExt)}`);
    }

    return tx as Transaction;
  };

  const broadcastTronTx = async (tx: Transaction): Promise<string> => {
    const { wallet } = getTronGrpcClients();
    const privateKey = getRelayerPrivateKey();

    const feeLimit = parseNumberEnv("RELAYER_TRON_FEE_LIMIT", 100_000_000);
    const pollTimes = parseNumberEnv("RELAYER_TRON_POLL_TIMES", 20);
    const pollIntervalMs = parseNumberEnv("RELAYER_TRON_POLL_INTERVAL_MS", 3_000);

    if (!tx.rawData) throw new Error("Tron tx missing rawData");
    tx.rawData = Transaction_raw.fromPartial({ ...tx.rawData, feeLimit });

    const { txidHex, signed } = signTronTransaction(tx as Transaction, privateKey);

    const broadcast = await grpcUnary(wallet.broadcastTransaction.bind(wallet), signed);
    if (!broadcast.result) {
      const msg = broadcast.message?.length ? broadcast.message.toString("utf8") : "unknown";
      throw new Error(`Tron broadcast rejected: ${msg}`);
    }

    await waitForTronTransaction({ txid: txidHex, pollTimes, pollIntervalMs });
    return txidHex;
  };

  type PlannedIndexedEvent = {
    eventSignature: Hex;
    encodedEventData: Hex;
  };

  const planIndexedEventsForCalls = async (
    calls: readonly Hex[]
  ): Promise<PlannedIndexedEvent[]> => {
    const controllerBytes21 = getTronControllerAddressBytes21();
    const usdtAddress = await tronReadContract<Address>({
      addressBytes21: controllerBytes21,
      abi: UntronControllerAbi,
      functionName: "usdt",
    });

    const receiverAddressCache = new Map<string, Address>();
    const receiverDeployedCache = new Map<string, boolean>();
    const tokenRateCache = new Map<string, bigint>();
    const balanceCache = new Map<string, bigint>();

    const predictReceiverAddresses = async (salts: readonly Hex[]): Promise<readonly Address[]> => {
      const pendingSalts = salts.filter((salt) => !receiverAddressCache.has(salt.toLowerCase()));
      if (pendingSalts.length) {
        const multicallCalls = pendingSalts.map((salt) =>
          encodeFunctionData({
            abi: UntronControllerAbi,
            functionName: "predictReceiverAddress",
            args: [salt],
          })
        );

        const results = await tronReadContract<readonly Hex[]>({
          addressBytes21: controllerBytes21,
          abi: UntronControllerAbi,
          functionName: "multicall",
          args: [multicallCalls],
        });

        if (results.length !== pendingSalts.length) {
          throw new Error(
            `Tron controller multicall returned unexpected results length (expected ${pendingSalts.length}, got ${results.length})`
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
        if (!address) throw new Error(`Missing predicted receiver address for salt ${salt}`);
        return address;
      });
    };

    const getLpExchangeRateFor = async (token: Address): Promise<bigint> => {
      const key = token.toLowerCase();
      const cached = tokenRateCache.get(key);
      if (cached !== undefined) return cached;

      const rate = await tronReadContract<bigint>({
        addressBytes21: controllerBytes21,
        abi: UntronControllerAbi,
        functionName: "lpExchangeRateFor",
        args: [token],
      });

      tokenRateCache.set(key, rate);
      return rate;
    };

    const getTokenBalanceOf = async (token: Address, account: Address): Promise<bigint> => {
      const key = `${token.toLowerCase()}:${account.toLowerCase()}`;
      const cached = balanceCache.get(key);
      if (cached !== undefined) return cached;

      const balance = await tronReadContract<bigint>({
        addressBytes21: tronEvmAddressToBytes21(token),
        abi: ERC20Abi,
        functionName: "balanceOf",
        args: [account],
      });

      balanceCache.set(key, balance);
      return balance;
    };

    const getReceiverDeployed = async (receiver: Address): Promise<boolean> => {
      const key = receiver.toLowerCase();
      const cached = receiverDeployedCache.get(key);
      if (cached !== undefined) return cached;

      const deployed = await isTronContractDeployed(tronEvmAddressToBytes21(receiver));
      receiverDeployedCache.set(key, deployed);
      return deployed;
    };

    const computeLegacyMeshOutAmount = async ({
      rebalancer,
      inAmount,
    }: {
      rebalancer: Address;
      inAmount: bigint;
    }): Promise<bigint> => {
      const payload = await tronReadContract<Hex>({
        addressBytes21: controllerBytes21,
        abi: UntronControllerAbi,
        functionName: "payloadFor",
        args: [rebalancer],
      });
      if (!payload || payload === "0x")
        throw new Error("Tron rebalance route not set (payloadFor empty)");

      const decoded = decodeAbiParameters(
        [{ type: "address" }, { type: "uint32" }, { type: "bytes32" }],
        payload
      );
      const oft = decoded[0];
      if (typeof oft !== "string" || !isAddress(oft)) {
        throw new Error("Unsupported rebalance payload (expected (address oft, uint32, bytes32))");
      }

      const feeBps = await tronReadContract<bigint>({
        addressBytes21: tronEvmAddressToBytes21(oft),
        abi: LEGACY_MESH_OFT_ABI,
        functionName: "feeBps",
      });
      const denom = await tronReadContract<bigint>({
        addressBytes21: tronEvmAddressToBytes21(oft),
        abi: LEGACY_MESH_OFT_ABI,
        functionName: "BPS_DENOMINATOR",
      });

      if (denom === 0n) throw new Error("Legacy Mesh OFT BPS_DENOMINATOR returned 0");
      const fee = (inAmount * feeBps) / denom;
      return inAmount - fee;
    };

    const plannedEvents: PlannedIndexedEvent[] = [];

    for (const callData of calls) {
      const decoded = decodeFunctionData({ abi: UntronControllerAbi, data: callData });

      if (decoded.functionName === "pullFromReceivers") {
        const token = decoded.args?.[0] as Address | undefined;
        const receiverSalts = decoded.args?.[1] as readonly Hex[] | undefined;
        if (!token || !isAddress(token)) throw new Error("pullFromReceivers: invalid token arg");
        if (!receiverSalts || receiverSalts.length === 0)
          throw new Error("pullFromReceivers: missing receiverSalts");

        const isUsdt = token.toLowerCase() === usdtAddress.toLowerCase();
        const rateUsed = isUsdt ? RATE_SCALE : await getLpExchangeRateFor(token);
        if (!isUsdt && rateUsed === 0n) {
          throw new Error("pullFromReceivers: LP exchange rate not set (lpExchangeRateFor == 0)");
        }

        const receiverAddresses = await predictReceiverAddresses(receiverSalts);

        for (let i = 0; i < receiverSalts.length; i++) {
          const receiverSalt = receiverSalts[i]!;
          const receiverAddress = receiverAddresses[i]!;

          const balance = await getTokenBalanceOf(token, receiverAddress);
          const sweepAmount = balance > 0n ? balance - 1n : 0n;
          if (sweepAmount === 0n) continue;

          const deployed = await getReceiverDeployed(receiverAddress);
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
          throw new Error("rebalanceUsdt: invalid rebalancer");
        if (typeof inAmount !== "bigint") throw new Error("rebalanceUsdt: invalid inAmount");

        const outAmount = await computeLegacyMeshOutAmount({ rebalancer, inAmount });

        plannedEvents.push({
          eventSignature: EVENT_SIG_USDT_REBALANCED,
          encodedEventData: encodeAbiParameters(
            [{ type: "uint256" }, { type: "uint256" }, { type: "address" }],
            [inAmount, outAmount, rebalancer]
          ),
        });

        continue;
      }

      if (decoded.functionName === "isEventChainTip") {
        continue;
      }

      throw new Error(`sendTronControllerMulticall: unsupported call "${decoded.functionName}"`);
    }

    return plannedEvents;
  };

  const sendTronControllerMulticall = async ({
    calls,
  }: {
    calls: readonly Hex[];
  }): Promise<string> => {
    if (calls.length === 0)
      throw new Error("sendTronControllerMulticall: expected at least 1 call");

    const controllerBytes21 = getTronControllerAddressBytes21();

    for (let attempt = 0; attempt < 3; attempt++) {
      const preTip = await tronReadContract<Hex>({
        addressBytes21: controllerBytes21,
        abi: UntronControllerAbi,
        functionName: "eventChainTip",
      });

      const plannedEvents = await planIndexedEventsForCalls(calls);

      const tx = await buildControllerMulticallTx(calls);
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
        return await broadcastTronTx(tx);
      } catch (error) {
        const errorMessage =
          error instanceof Error ? `${error.name}: ${error.message}` : String(error);
        if (!errorMessage.toLowerCase().includes("no")) throw error;
      }
    }

    throw new Error(
      "Failed to send Tron multicall with in-tx eventChainTip checkpoint (tip kept changing or prediction mismatch)"
    );
  };

  const sendTronControllerPullFromReceivers: RelayerDeps["sendTronControllerPullFromReceivers"] =
    async ({ tokenAddress, receiverSalts }) => {
      if (receiverSalts.length === 0) {
        throw new Error("sendTronControllerPullFromReceivers: expected at least 1 receiver salt");
      }

      const call = encodeFunctionData({
        abi: UntronControllerAbi,
        functionName: "pullFromReceivers",
        args: [tokenAddress, receiverSalts],
      });

      const txid = await sendTronControllerMulticall({ calls: [call] });
      return { txid };
    };

  const sendTronControllerRebalanceUsdt: RelayerDeps["sendTronControllerRebalanceUsdt"] = async ({
    rebalancer,
    inAmount,
  }) => {
    if (!isAddress(rebalancer)) throw new Error("Invalid rebalancer address");

    const call = encodeFunctionData({
      abi: UntronControllerAbi,
      functionName: "rebalanceUsdt",
      args: [rebalancer, inAmount],
    });

    const txid = await sendTronControllerMulticall({ calls: [call] });
    return { txid };
  };

  return {
    getTronRelayerAddress,
    getTronControllerEvmAddress,
    getTronReceiverMap,
    sendTronControllerPullFromReceivers,
    sendTronControllerRebalanceUsdt,
  };
}
