import { describe, expect, it } from "@effect/vitest";
import { decodeFunctionData, encodeFunctionData, type Hex } from "viem";

import { untronControllerAbi } from "@untron/v3-contracts";

import { computeNextEventChainTip } from "../src/eventChain/tip";
import { buildControllerMulticallDataWithEventChainCheckpoint } from "../src/relayer/deps/tron/untronController";

describe("buildControllerMulticallDataWithEventChainCheckpoint", () => {
  it("computes expectedTip and appends checkpoint call", () => {
    const calls: readonly Hex[] = [
      encodeFunctionData({
        abi: untronControllerAbi,
        functionName: "isEventChainTip",
        args: ["0x00".padEnd(66, "0") as Hex],
      }),
    ];

    const preTip = "0x11".padEnd(66, "1") as Hex;
    const blockNumber = 1234n;
    const blockTimestamp = 1_700_000_000n;

    const plannedEvents = [
      {
        eventSignature: "0xaa".padEnd(66, "a") as unknown as Hex,
        encodedEventData: "0xdeadbeef" as unknown as Hex,
      },
      {
        eventSignature: "0xbb".padEnd(66, "b") as unknown as Hex,
        encodedEventData: "0xcafebabe" as unknown as Hex,
      },
    ] as const;

    const expectedTip = plannedEvents.reduce(
      (tip, event) =>
        computeNextEventChainTip({
          previousTip: tip,
          blockNumber,
          blockTimestamp,
          eventSignature: event.eventSignature,
          encodedEventData: event.encodedEventData,
        }),
      preTip
    );

    const built = buildControllerMulticallDataWithEventChainCheckpoint({
      calls,
      preTip,
      plannedEvents,
      blockNumber,
      blockTimestamp,
    });

    expect(built.expectedTip).toBe(expectedTip);

    const decodedMulticall = decodeFunctionData({
      abi: untronControllerAbi,
      data: built.multicallData,
    });
    expect(decodedMulticall.functionName).toBe("multicall");

    const nestedCalls = decodedMulticall.args?.[0] as readonly Hex[] | undefined;
    expect(Array.isArray(nestedCalls)).toBe(true);
    expect(nestedCalls?.length).toBe(calls.length + 1);

    const lastCall = nestedCalls?.at(-1);
    expect(typeof lastCall).toBe("string");

    const decodedCheckpoint = decodeFunctionData({
      abi: untronControllerAbi,
      data: lastCall!,
    });
    expect(decodedCheckpoint.functionName).toBe("isEventChainTip");
    expect(decodedCheckpoint.args?.[0]).toBe(expectedTip);
  });
});
