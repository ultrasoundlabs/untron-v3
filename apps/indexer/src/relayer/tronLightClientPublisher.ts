import { Effect } from "effect";
import { encodeFunctionData, type Address, type Hex } from "viem";

import type { BlockExtention } from "@untron/tron-protocol/api";

import { tronLightClientAbi } from "@untron/v3-contracts";

import {
  encodeStoreOffsets16,
  encodeTronLightClientMetadataAndSignatures,
  parseTronBlockForLightClient,
} from "./tronProofs";

const UINT256_MAX = (1n << 256n) - 1n;

const FINALITY_DISTINCT_SR_THRESHOLD = 19;

const TRON_BLOCK_FETCH_CONCURRENCY = (() => {
  const raw = process.env.TRON_BLOCK_FETCH_CONCURRENCY;
  if (!raw) return 1;
  const parsed = Number.parseInt(raw, 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 10;
})();

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
  storeOffsets: readonly number[];
  storedRequestedTronBlockNumbers: readonly bigint[];
  maxFinalizableOffset: number;
  distinctWitnessesAtLastStoredOffset: number;
}>;

export const buildTronLightClientProveBlocksCall = (args: {
  tronLightClientAddress: Address;
  startingBlockId: Hex;
  rangeStart: bigint;
  rangeEnd: bigint;
  requestedOffsets: readonly number[];
  witnessIndexByTronOwnerAddressHex: ReadonlyMap<string, number>;
  fetchTronBlockByNum: (blockNumber: bigint) => Effect.Effect<BlockExtention, Error>;
}): Effect.Effect<TronLightClientProveBlocksPlan | null, Error> =>
  Effect.gen(function* () {
    if (args.rangeEnd < args.rangeStart) return null;

    const numBlocksBig = args.rangeEnd - args.rangeStart + 1n;
    const numBlocks = Number(numBlocksBig);
    if (!Number.isSafeInteger(numBlocks) || numBlocks <= 0) return null;

    const nums: bigint[] = [];
    for (let n = args.rangeStart; n <= args.rangeEnd; n++) nums.push(n);

    const blocks = yield* Effect.forEach(
      nums,
      (blockNumber) =>
        args.fetchTronBlockByNum(blockNumber).pipe(Effect.map(parseTronBlockForLightClient)),
      { concurrency: TRON_BLOCK_FETCH_CONCURRENCY }
    );

    const { compressedTronBlockMetadata, compressedSignatures, witnessIndices } =
      encodeTronLightClientMetadataAndSignatures({
        blocks,
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
      const eligible = requestedOffsetsBounded
        .filter((off) => off <= maxFinalizableOffset)
        .slice(0, 16);
      if (eligible.length > 0) return eligible;
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
        UINT256_MAX,
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
      storeOffsets,
      storedRequestedTronBlockNumbers,
      maxFinalizableOffset,
      distinctWitnessesAtLastStoredOffset,
    };
  });
