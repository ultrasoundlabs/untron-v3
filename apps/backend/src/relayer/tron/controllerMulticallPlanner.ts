import { Effect } from "effect";
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

import { untronControllerAbi } from "@untron/v3-contracts";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { tronEvmAddressToBytes21 } from "../deps/tron/protocol";

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

export type PlannedIndexedEvent = Readonly<{
  eventSignature: Hex;
  encodedEventData: Hex;
}>;

export type TronReadContract<E> = <T>(args: {
  addressBytes21: Buffer;
  abi: readonly unknown[];
  functionName: string;
  args?: readonly unknown[];
}) => Effect.Effect<T, E>;

export function planIndexedEventsForControllerCalls<E>(args: {
  controllerBytes21: Buffer;
  calls: readonly Hex[];
  tronReadContract: TronReadContract<E>;
  isTronContractDeployed: (addressBytes21: Buffer) => Effect.Effect<boolean, E>;
}): Effect.Effect<readonly PlannedIndexedEvent[], E | Error> {
  return Effect.gen(function* () {
    const usdtAddress = yield* args.tronReadContract<Address>({
      addressBytes21: args.controllerBytes21,
      abi: untronControllerAbi,
      functionName: "usdt",
    });

    const receiverAddressCache = new Map<string, Address>();
    const receiverDeployedCache = new Map<string, boolean>();
    const tokenRateCache = new Map<string, bigint>();
    const balanceCache = new Map<string, bigint>();

    const predictReceiverAddresses = (salts: readonly Hex[]) =>
      Effect.gen(function* () {
        const pendingSalts = salts.filter((salt) => !receiverAddressCache.has(salt.toLowerCase()));

        if (pendingSalts.length) {
          const multicallCalls = pendingSalts.map((salt) =>
            encodeFunctionData({
              abi: untronControllerAbi,
              functionName: "predictReceiverAddress",
              args: [salt],
            })
          );

          const results = yield* args.tronReadContract<readonly Hex[]>({
            addressBytes21: args.controllerBytes21,
            abi: untronControllerAbi,
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
              abi: untronControllerAbi,
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
      });

    const getLpExchangeRateFor = (token: Address) =>
      Effect.gen(function* () {
        const key = token.toLowerCase();
        const cached = tokenRateCache.get(key);
        if (cached !== undefined) return cached;

        const rate = yield* args.tronReadContract<bigint>({
          addressBytes21: args.controllerBytes21,
          abi: untronControllerAbi,
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

        const balance = yield* args.tronReadContract<bigint>({
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

        const deployed = yield* args.isTronContractDeployed(tronEvmAddressToBytes21(receiver));
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
        const payload = yield* args.tronReadContract<Hex>({
          addressBytes21: args.controllerBytes21,
          abi: untronControllerAbi,
          functionName: "payloadFor",
          args: [rebalancer],
        });
        if (!payload || payload === "0x") {
          return yield* Effect.fail(new Error("Tron rebalance route not set (payloadFor empty)"));
        }

        const decoded = decodeAbiParameters(
          [{ type: "address" }, { type: "uint32" }, { type: "bytes32" }],
          payload
        );
        const oft = decoded[0];
        if (typeof oft !== "string" || !isAddress(oft)) {
          return yield* Effect.fail(
            new Error("Unsupported rebalance payload (expected (address oft, uint32, bytes32))")
          );
        }

        const feeBps = yield* args.tronReadContract<bigint>({
          addressBytes21: tronEvmAddressToBytes21(oft),
          abi: LEGACY_MESH_OFT_ABI,
          functionName: "feeBps",
        });
        const denom = yield* args.tronReadContract<bigint>({
          addressBytes21: tronEvmAddressToBytes21(oft),
          abi: LEGACY_MESH_OFT_ABI,
          functionName: "BPS_DENOMINATOR",
        });

        if (denom === 0n) {
          return yield* Effect.fail(new Error("Legacy Mesh OFT BPS_DENOMINATOR returned 0"));
        }
        const fee = (inAmount * feeBps) / denom;
        return inAmount - fee;
      });

    const plannedEvents: PlannedIndexedEvent[] = [];

    for (const callData of args.calls) {
      const decoded = decodeFunctionData({ abi: untronControllerAbi, data: callData });

      if (decoded.functionName === "pullFromReceivers") {
        const token = decoded.args?.[0] as Address | undefined;
        const salts = decoded.args?.[1] as readonly Hex[] | undefined;
        if (!token || !isAddress(token)) {
          return yield* Effect.fail(new Error("pullFromReceivers: invalid token arg"));
        }
        if (!salts || salts.length === 0) {
          return yield* Effect.fail(new Error("pullFromReceivers: missing receiverSalts"));
        }

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
        if (!rebalancer || !isAddress(rebalancer)) {
          return yield* Effect.fail(new Error("rebalanceUsdt: invalid rebalancer"));
        }
        if (typeof inAmount !== "bigint") {
          return yield* Effect.fail(new Error("rebalanceUsdt: invalid inAmount"));
        }

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
}
