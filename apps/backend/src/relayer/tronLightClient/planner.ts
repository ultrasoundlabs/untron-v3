import type { TronBlockForLightClient } from "../tronProofs";
import { encodeStoreOffsets16, encodeTronLightClientMetadataAndSignatures } from "../tronProofs";
import { encodeFunctionData, type Address, type Hex } from "viem";

import { tronLightClientAbi } from "@untron/v3-contracts";
const FINALITY_DISTINCT_SR_THRESHOLD = 19;
const MAX_STORE_OFFSETS = 16;

function popcount32(x: number): number {
  let v = x >>> 0;
  let c = 0;
  while (v !== 0) {
    v = (v & (v - 1)) >>> 0;
    c++;
  }
  return c;
}

function suffixDistinctCounts(witnessIndices: readonly number[]): number[] {
  const counts = new Array<number>(witnessIndices.length);
  let mask = 0;
  for (let i = witnessIndices.length - 1; i >= 0; i--) {
    mask |= 1 << witnessIndices[i]!;
    counts[i] = popcount32(mask);
  }
  return counts;
}

export type TronLightClientProveBlocksPlan = Readonly<{
  call: { to: Address; data: Hex };
  rangeStart: bigint;
  rangeEnd: bigint;
  intersectionOffset: bigint;
  storeOffsets: readonly number[];
  storedRequestedTronBlockNumbers: readonly bigint[];
  maxFinalizableOffset: number;
  distinctWitnessesAtLastStoredOffset: number;
}>;

export function planTronLightClientProveBlocksCall(args: {
  tronLightClientAddress: Address;
  startingBlockId: Hex;
  rangeStart: bigint;
  rangeEnd: bigint;
  blocks: readonly TronBlockForLightClient[];
  intersectionOffset: bigint;
  requestedOffsets: readonly number[];
  progressOffset?: "start" | "end";
  witnessIndexByTronOwnerAddressHex: ReadonlyMap<string, number>;
}): TronLightClientProveBlocksPlan | null {
  if (args.rangeEnd < args.rangeStart) return null;

  const numBlocksBig = args.rangeEnd - args.rangeStart + 1n;
  const numBlocks = Number(numBlocksBig);
  if (!Number.isSafeInteger(numBlocks) || numBlocks <= 0) return null;
  if (args.blocks.length !== numBlocks) {
    throw new Error(
      `Invalid blocks length (expected ${numBlocks}, got ${args.blocks.length.toString()})`
    );
  }

  const { compressedTronBlockMetadata, compressedSignatures, witnessIndices } =
    encodeTronLightClientMetadataAndSignatures({
      blocks: args.blocks,
      witnessIndexByTronOwnerAddressHex: args.witnessIndexByTronOwnerAddressHex,
    });

  const distinctCounts = suffixDistinctCounts(witnessIndices);

  let maxFinalizableOffset = -1;
  for (let i = distinctCounts.length - 1; i >= 0; i--) {
    const distinct = distinctCounts[i]!;
    if (distinct >= FINALITY_DISTINCT_SR_THRESHOLD) {
      maxFinalizableOffset = i;
      break;
    }
  }
  if (maxFinalizableOffset < 0) return null;

  const requestedOffsetsSorted = [...args.requestedOffsets].sort((a, b) => a - b);
  const requestedOffsetsBounded = requestedOffsetsSorted.filter(
    (off) => Number.isInteger(off) && off >= 0 && off < numBlocks
  );

  const storeOffsets = (() => {
    const progressOffset = args.progressOffset ?? "end";
    const eligible = requestedOffsetsBounded
      .filter((off) => off <= maxFinalizableOffset)
      .slice(0, MAX_STORE_OFFSETS);
    if (eligible.length > 0) return eligible;
    if (progressOffset === "start") return [0];
    return [maxFinalizableOffset];
  })();

  const storeOffsets16 = encodeStoreOffsets16(storeOffsets);
  const distinctWitnessesAtLastStoredOffset = distinctCounts[storeOffsets.at(-1)!]!;

  const data = encodeFunctionData({
    abi: tronLightClientAbi,
    functionName: "proveBlocks",
    args: [
      args.startingBlockId,
      compressedTronBlockMetadata,
      compressedSignatures,
      args.intersectionOffset,
      storeOffsets16,
    ],
  });

  const storedRequestedTronBlockNumbers = storeOffsets
    .filter((off) => requestedOffsetsBounded.includes(off))
    .map((off) => args.rangeStart + BigInt(off));

  return {
    call: { to: args.tronLightClientAddress, data },
    rangeStart: args.rangeStart,
    rangeEnd: args.rangeEnd,
    intersectionOffset: args.intersectionOffset,
    storeOffsets,
    storedRequestedTronBlockNumbers,
    maxFinalizableOffset,
    distinctWitnessesAtLastStoredOffset,
  };
}
