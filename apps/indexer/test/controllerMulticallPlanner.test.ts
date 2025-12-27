import { Effect } from "effect";
import {
  encodeAbiParameters,
  encodeFunctionData,
  keccak256,
  stringToHex,
  type Address,
} from "viem";
import { untronControllerAbi } from "@untron/v3-contracts";

import { planIndexedEventsForControllerCalls } from "../src/relayer/tron/controllerMulticallPlanner";
import { tronEvmAddressToBytes21 } from "../src/relayer/deps/tron/protocol";
import { describe, expect, it } from "@effect/vitest";

const EVENT_SIG_RECEIVER_DEPLOYED = keccak256(
  stringToHex("ReceiverDeployed(address,bytes32)")
) as `0x${string}`;
const EVENT_SIG_PULLED_FROM_RECEIVER = keccak256(
  stringToHex("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)")
) as `0x${string}`;
const EVENT_SIG_USDT_REBALANCED = keccak256(
  stringToHex("UsdtRebalanced(uint256,uint256,address)")
) as `0x${string}`;

const asEvmAddress = (bytes21: Buffer): Address =>
  `0x${bytes21.subarray(1).toString("hex")}` as Address;

describe("controllerMulticallPlanner", () => {
  it.effect("pullFromReceivers plans deploy + pull events", () =>
    Effect.gen(function* () {
      const controllerBytes21 = tronEvmAddressToBytes21(
        "0x1000000000000000000000000000000000000000"
      );
      const controllerUsdt = "0x2000000000000000000000000000000000000000" as Address;

      const receiverSalt = "0x11".padEnd(66, "1") as `0x${string}`;
      const receiverAddress = "0x3000000000000000000000000000000000000000" as Address;

      const token = "0x4000000000000000000000000000000000000000" as Address;
      const rateUsed = 2_000_000_000_000_000_000n; // 2e18

      const call = encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "pullFromReceivers",
        args: [token, [receiverSalt]],
      });

      const balances = new Map<string, bigint>([
        [`${token.toLowerCase()}:${receiverAddress.toLowerCase()}`, 10n],
      ]);

      const deployed = new Set<string>();

      const tronReadContract = <T>(args: {
        addressBytes21: Buffer;
        abi: readonly unknown[];
        functionName: string;
        args?: readonly unknown[];
      }) => {
        if (args.functionName === "usdt") return Effect.succeed(controllerUsdt as unknown as T);

        if (args.functionName === "lpExchangeRateFor") {
          return Effect.succeed(rateUsed as unknown as T);
        }

        if (args.functionName === "multicall") {
          const calls = args.args?.[0] as readonly `0x${string}`[] | undefined;
          expect(Array.isArray(calls)).toBe(true);
          expect(calls?.length).toBe(1);
          const decoded = encodeAbiParameters([{ type: "address" }], [receiverAddress]);
          return Effect.succeed([decoded] as unknown as T);
        }

        if (args.functionName === "balanceOf") {
          const tokenAddress = asEvmAddress(args.addressBytes21);
          const account = args.args?.[0] as Address | undefined;
          expect(typeof account).toBe("string");
          expect(account?.startsWith("0x")).toBe(true);
          const key = `${tokenAddress.toLowerCase()}:${account!.toLowerCase()}`;
          return Effect.succeed((balances.get(key) ?? 0n) as unknown as T);
        }

        throw new Error(`unexpected tronReadContract call: ${args.functionName}`);
      };

      const isTronContractDeployed = (addressBytes21: Buffer) => {
        const evm = asEvmAddress(addressBytes21).toLowerCase();
        return Effect.succeed(deployed.has(evm));
      };

      const planned = yield* planIndexedEventsForControllerCalls({
        controllerBytes21,
        calls: [call],
        tronReadContract,
        isTronContractDeployed,
      });

      expect(planned).toHaveLength(2);

      expect(planned[0]!.eventSignature).toBe(EVENT_SIG_RECEIVER_DEPLOYED);
      expect(planned[0]!.encodedEventData).toBe(
        encodeAbiParameters(
          [{ type: "address" }, { type: "bytes32" }],
          [receiverAddress, receiverSalt]
        )
      );

      expect(planned[1]!.eventSignature).toBe(EVENT_SIG_PULLED_FROM_RECEIVER);
      expect(planned[1]!.encodedEventData).toBe(
        encodeAbiParameters(
          [
            { type: "bytes32" },
            { type: "address" },
            { type: "uint256" },
            { type: "uint256" },
            { type: "uint256" },
          ],
          [receiverSalt, token, 9n, rateUsed, 18n]
        )
      );
    })
  );

  it.effect("rebalanceUsdt plans UsdtRebalanced event", () =>
    Effect.gen(function* () {
      const controllerBytes21 = tronEvmAddressToBytes21(
        "0x1000000000000000000000000000000000000000"
      );
      const controllerUsdt = "0x2000000000000000000000000000000000000000" as Address;

      const rebalancer = "0x5000000000000000000000000000000000000000" as Address;
      const inAmount = 1_000_000n;

      const oft = "0x6000000000000000000000000000000000000000" as Address;
      const feeBps = 25n;
      const denom = 10_000n;
      const expectedOut = inAmount - (inAmount * feeBps) / denom;

      const payload = encodeAbiParameters(
        [{ type: "address" }, { type: "uint32" }, { type: "bytes32" }],
        [oft, 0, "0x00".padEnd(66, "0") as `0x${string}`]
      );

      const call = encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "rebalanceUsdt",
        args: [rebalancer, inAmount],
      });

      const tronReadContract = <T>(args: {
        addressBytes21: Buffer;
        abi: readonly unknown[];
        functionName: string;
        args?: readonly unknown[];
      }) => {
        if (args.functionName === "usdt") return Effect.succeed(controllerUsdt as unknown as T);
        if (args.functionName === "payloadFor") return Effect.succeed(payload as unknown as T);

        const tokenAddress = asEvmAddress(args.addressBytes21).toLowerCase();
        if (tokenAddress === oft.toLowerCase() && args.functionName === "feeBps") {
          return Effect.succeed(feeBps as unknown as T);
        }
        if (tokenAddress === oft.toLowerCase() && args.functionName === "BPS_DENOMINATOR") {
          return Effect.succeed(denom as unknown as T);
        }

        throw new Error(`unexpected tronReadContract call: ${args.functionName}`);
      };

      const planned = yield* planIndexedEventsForControllerCalls({
        controllerBytes21,
        calls: [call],
        tronReadContract,
        isTronContractDeployed: () => Effect.succeed(true),
      });

      expect(planned).toHaveLength(1);
      expect(planned[0]!.eventSignature).toBe(EVENT_SIG_USDT_REBALANCED);
      expect(planned[0]!.encodedEventData).toBe(
        encodeAbiParameters(
          [{ type: "uint256" }, { type: "uint256" }, { type: "address" }],
          [inAmount, expectedOut, rebalancer]
        )
      );
    })
  );
});
