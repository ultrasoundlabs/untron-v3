import { describe, expect, test } from "vitest";
import { decodeFunctionData, type Hex } from "viem";

import { tronLightClientAbi } from "@untron/v3-contracts";
import { planTronLightClientProveBlocksCall } from "../src/relayer/tronLightClient/planner";

function hexByte(value: number): string {
  return value.toString(16).padStart(2, "0");
}

function makeBytes32(byte: number): Buffer {
  return Buffer.from(Array.from({ length: 32 }, () => byte));
}

function makeWitnessAddress(index: number): Buffer {
  const bytes = Buffer.alloc(21);
  bytes[0] = 0x41;
  bytes[20] = index;
  return bytes;
}

function makeWitnessIndexMap(): ReadonlyMap<string, number> {
  const entries: Array<[string, number]> = [];
  for (let i = 0; i < 27; i++) {
    const addrHex = `41${"00".repeat(19)}${hexByte(i)}`;
    entries.push([addrHex, i]);
  }
  return new Map(entries);
}

function makeBlocks(numBlocks: number) {
  return Array.from({ length: numBlocks }, (_v, i) => ({
    parentHash: makeBytes32(1),
    txTrieRoot: makeBytes32(2),
    timestampSec: 1_700_000_000 + i,
    witnessAddress: makeWitnessAddress(i),
    witnessSignature: Buffer.alloc(65, 3),
  }));
}

describe("planTronLightClientProveBlocksCall", () => {
  test("encodes intersectionOffset into proveBlocks calldata", () => {
    const rangeStart = 100n;
    const rangeEnd = 119n;

    const plan = planTronLightClientProveBlocksCall({
      tronLightClientAddress: "0x0000000000000000000000000000000000000001",
      startingBlockId: `0x${"11".repeat(32)}` as Hex,
      rangeStart,
      rangeEnd,
      blocks: makeBlocks(Number(rangeEnd - rangeStart + 1n)),
      intersectionOffset: 5n,
      requestedOffsets: [],
      progressOffset: "start",
      witnessIndexByTronOwnerAddressHex: makeWitnessIndexMap(),
    });

    expect(plan).not.toBeNull();
    const decoded = decodeFunctionData({
      abi: tronLightClientAbi,
      data: plan!.call.data,
    });
    expect(decoded.functionName).toBe("proveBlocks");
    expect(decoded.args?.[3]).toBe(5n);
  });

  test("stores offset 0 for backfill progress when requestedOffsets is empty", () => {
    const rangeStart = 100n;
    const rangeEnd = 119n;

    const plan = planTronLightClientProveBlocksCall({
      tronLightClientAddress: "0x0000000000000000000000000000000000000001",
      startingBlockId: `0x${"11".repeat(32)}` as Hex,
      rangeStart,
      rangeEnd,
      blocks: makeBlocks(Number(rangeEnd - rangeStart + 1n)),
      intersectionOffset: 5n,
      requestedOffsets: [],
      progressOffset: "start",
      witnessIndexByTronOwnerAddressHex: makeWitnessIndexMap(),
    });

    expect(plan).not.toBeNull();
    expect(plan!.storeOffsets).toEqual([0]);
  });
});
