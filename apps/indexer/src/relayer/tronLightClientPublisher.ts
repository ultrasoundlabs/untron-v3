import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import type { PublicClient } from "viem";
import { decodeEventLog, encodeFunctionData, type Address, type Hex } from "viem";

import type { BlockExtention } from "@untron/tron-protocol/api";

import { tronLightClientAbi } from "@untron/v3-contracts";
import { tronLightClientCheckpoint } from "ponder:schema";

import { getRows } from "./sqlRows";
import {
  encodeStoreOffsets16,
  encodeTronLightClientMetadataAndSignatures,
  parseTronBlockForLightClient,
} from "./tronProofs";

const UINT256_MAX = (1n << 256n) - 1n;

const MAX_TRON_BLOCKS_PER_PROVE_CALL = 500n;
const FINALITY_DISTINCT_SR_THRESHOLD = 19;
const FINALITY_LOOKAHEAD_INITIAL_BLOCKS = 60n;
const FINALITY_LOOKAHEAD_STEP_BLOCKS = 60n;
const TRON_BLOCK_FETCH_CONCURRENCY = (() => {
  const raw = process.env.TRON_BLOCK_FETCH_CONCURRENCY;
  if (!raw) return 1;
  const parsed = Number.parseInt(raw, 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 10;
})();
const TRON_BLOCK_FETCH_PROGRESS_INTERVAL_MS = 5_000;

function coerceBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      // fall through
    }
  }
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

function coerceInt(value: unknown, label: string): number {
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (typeof value === "string" && value.length > 0) {
    const parsed = Number.parseInt(value, 10);
    if (Number.isFinite(parsed)) return parsed;
  }
  throw new Error(`Invalid ${label} (expected integer)`);
}

function coerceHex(value: unknown, label: string): Hex {
  if (typeof value !== "string" || !value.startsWith("0x")) throw new Error(`Invalid ${label}`);
  return value.toLowerCase() as Hex;
}

function popcount32(x: number): number {
  let v = x >>> 0;
  let c = 0;
  while (v !== 0) {
    v = (v & (v - 1)) >>> 0;
    c++;
  }
  return c;
}

function distinctWitnessesFromOffset(witnessIndices: readonly number[], offset: number): number {
  let mask = 0;
  for (let i = witnessIndices.length - 1; i >= offset; i--) {
    const wi = witnessIndices[i]!;
    mask |= 1 << wi;
  }
  return popcount32(mask);
}

type TronSrsCache = {
  witnessIndexByTronOwnerHex: ReadonlyMap<string, number>;
};

const tronLightClientSrsCache = new Map<string, Promise<TronSrsCache>>();

function evmAddressToTronOwnerHex(evm: Address): string {
  return `41${evm.slice(2).toLowerCase()}`;
}

async function loadTronLightClientSrs(args: {
  mainnetClient: PublicClient;
  tronLightClientAddress: Address;
}): Promise<TronSrsCache> {
  const key = args.tronLightClientAddress.toLowerCase();
  const existing = tronLightClientSrsCache.get(key);
  if (existing) return existing;

  const promise = (async () => {
    const entries: Array<[string, number]> = [];
    for (let i = 0; i < 27; i++) {
      const sr = (await args.mainnetClient.readContract({
        address: args.tronLightClientAddress,
        abi: tronLightClientAbi,
        functionName: "srs",
        args: [BigInt(i)],
      })) as `0x${string}`;
      const evm = sr.toLowerCase() as Address;
      entries.push([evmAddressToTronOwnerHex(evm), i]);
    }
    return { witnessIndexByTronOwnerHex: new Map(entries) } satisfies TronSrsCache;
  })();

  tronLightClientSrsCache.set(key, promise);
  return promise;
}

async function lookupMainnetChainIdForTronLightClient(args: {
  context: PonderContext;
  tronLightClientAddress: Address;
}): Promise<number> {
  const result = await args.context.db.sql.execute(sql`
    SELECT chain_id AS "chainId"
    FROM "event_chain_state"
    WHERE contract_name = 'TronLightClient'
      AND contract_address = ${args.tronLightClientAddress}
    LIMIT 1;
  `);

  const rows = getRows(result) as Array<Record<string, unknown>>;
  const chainIdRaw = rows[0]?.chainId;
  if (chainIdRaw != null) {
    try {
      return coerceInt(chainIdRaw, "event_chain_state.chain_id");
    } catch {
      // fall through to env fallback
    }
  }

  const env = process.env.UNTRON_V3_CHAIN_ID;
  if (env) {
    const parsed = Number.parseInt(env, 10);
    if (Number.isFinite(parsed)) return parsed;
  }

  throw new Error("Failed to resolve mainnet chainId for TronLightClient");
}

type TronLightClientCheckpointRow = {
  tronBlockNumber: bigint;
  tronBlockId: Hex;
};

function normalizeCheckpointRow(row: Record<string, unknown>): TronLightClientCheckpointRow {
  const tronBlockNumber = coerceBigint(
    row.tronBlockNumber,
    "tron_light_client_checkpoint.tron_block_number"
  );
  const tronBlockId = coerceHex(row.tronBlockId, "tron_light_client_checkpoint.tron_block_id");
  return { tronBlockNumber, tronBlockId };
}

async function getNearestTronLightClientCheckpoints(args: {
  context: PonderContext;
  mainnetChainId: number;
  tronLightClientAddress: Address;
  tronBlockNumber: bigint;
}): Promise<{
  prev: TronLightClientCheckpointRow | null;
  next: TronLightClientCheckpointRow | null;
}> {
  const prevResult = await args.context.db.sql.execute(sql`
    SELECT
      tron_block_number AS "tronBlockNumber",
      tron_block_id AS "tronBlockId"
    FROM "tron_light_client_checkpoint"
    WHERE chain_id = ${args.mainnetChainId}
      AND contract_address = ${args.tronLightClientAddress}
      AND tron_block_number <= ${args.tronBlockNumber}
    ORDER BY tron_block_number DESC
    LIMIT 1;
  `);

  const nextResult = await args.context.db.sql.execute(sql`
    SELECT
      tron_block_number AS "tronBlockNumber",
      tron_block_id AS "tronBlockId"
    FROM "tron_light_client_checkpoint"
    WHERE chain_id = ${args.mainnetChainId}
      AND contract_address = ${args.tronLightClientAddress}
      AND tron_block_number >= ${args.tronBlockNumber}
    ORDER BY tron_block_number ASC
    LIMIT 1;
  `);

  const prevRows = getRows(prevResult) as Array<Record<string, unknown>>;
  const nextRows = getRows(nextResult) as Array<Record<string, unknown>>;
  return {
    prev: prevRows[0] ? normalizeCheckpointRow(prevRows[0]) : null,
    next: nextRows[0] ? normalizeCheckpointRow(nextRows[0]) : null,
  };
}

export type BuildTronLightClientProveBlocksCallArgs = {
  context: PonderContext;
  mainnetClient: PublicClient;
  tronLightClientAddress: Address;
  tronBlockNumber: bigint;
  fetchTronBlockByNum: (blockNumber: bigint) => Effect.Effect<BlockExtention, Error>;
};

export type TronLightClientProveBlocksPlan = {
  startingBlockId: Hex;
  intersectionOffset: bigint;
  storeOffsets16: bigint;
  compressedTronBlockMetadata: Hex;
  compressedSignatures: Hex;
};

export const upsertTronLightClientCheckpointsFromTransaction = (args: {
  context: PonderContext;
  mainnetClient: PublicClient;
  tronLightClientAddress: Address;
  transactionHash: Hex;
}): Effect.Effect<{ storedCount: number; maxStoredTronBlockNumber: bigint | null }, Error> =>
  Effect.gen(function* () {
    const receipt = yield* Effect.tryPromise({
      try: () => args.mainnetClient.getTransactionReceipt({ hash: args.transactionHash }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    });

    const block = yield* Effect.tryPromise({
      try: () => args.mainnetClient.getBlock({ blockNumber: receipt.blockNumber }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    });

    const chainId = yield* Effect.tryPromise({
      try: async () => Number(await args.mainnetClient.getChainId()),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    });

    const tronLightClientAddressLower = args.tronLightClientAddress.toLowerCase() as Address;

    const decodedLogs: Array<{
      tronBlockNumber: bigint;
      tronBlockId: Hex;
      tronTxTrieRoot: Hex;
      tronBlockTimestamp: bigint;
      logIndex: number;
    }> = [];

    for (const log of receipt.logs) {
      if (log.address.toLowerCase() !== tronLightClientAddressLower) continue;

      try {
        const decoded = decodeEventLog({
          abi: tronLightClientAbi,
          data: log.data,
          topics: log.topics,
        });
        if (decoded.eventName !== "TronBlockStored") continue;

        const { blockNumber, blockId, txTrieRoot, timestamp } = decoded.args as {
          blockNumber: bigint;
          blockId: Hex;
          txTrieRoot: Hex;
          timestamp: number | bigint;
        };

        decodedLogs.push({
          tronBlockNumber: blockNumber,
          tronBlockId: blockId,
          tronTxTrieRoot: txTrieRoot,
          tronBlockTimestamp: typeof timestamp === "bigint" ? timestamp : BigInt(timestamp),
          logIndex: log.logIndex,
        });
      } catch {
        // ignore non-matching logs
      }
    }

    if (decodedLogs.length === 0) {
      return { storedCount: 0, maxStoredTronBlockNumber: null };
    }

    let maxStoredTronBlockNumber: bigint | null = null;

    for (const stored of decodedLogs) {
      const id = `${chainId}:${tronLightClientAddressLower}:${stored.tronBlockNumber.toString()}`;
      yield* Effect.tryPromise({
        try: () =>
          args.context.db
            .insert(tronLightClientCheckpoint)
            .values({
              id,
              chainId,
              contractAddress: tronLightClientAddressLower,
              tronBlockNumber: stored.tronBlockNumber,
              tronBlockId: stored.tronBlockId,
              tronTxTrieRoot: stored.tronTxTrieRoot,
              tronBlockTimestamp: stored.tronBlockTimestamp,
              storedAtBlockNumber: receipt.blockNumber,
              storedAtBlockTimestamp: block.timestamp,
              storedAtTransactionHash: receipt.transactionHash,
              storedAtLogIndex: stored.logIndex,
            })
            .onConflictDoUpdate({
              tronBlockId: stored.tronBlockId,
              tronTxTrieRoot: stored.tronTxTrieRoot,
              tronBlockTimestamp: stored.tronBlockTimestamp,
              storedAtBlockNumber: receipt.blockNumber,
              storedAtBlockTimestamp: block.timestamp,
              storedAtTransactionHash: receipt.transactionHash,
              storedAtLogIndex: stored.logIndex,
            }),
        catch: (e) => (e instanceof Error ? e : new Error(String(e))),
      });

      if (maxStoredTronBlockNumber === null || stored.tronBlockNumber > maxStoredTronBlockNumber) {
        maxStoredTronBlockNumber = stored.tronBlockNumber;
      }
    }

    return { storedCount: decodedLogs.length, maxStoredTronBlockNumber };
  });

export const buildTronLightClientProveBlocksCallToCheckpointBlock = (
  args: BuildTronLightClientProveBlocksCallArgs
): Effect.Effect<{ to: Address; data: Hex } | null, Error> =>
  Effect.gen(function* () {
    const mainnetChainId = yield* Effect.tryPromise({
      try: () =>
        lookupMainnetChainIdForTronLightClient({
          context: args.context,
          tronLightClientAddress: args.tronLightClientAddress,
        }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    });

    const nearest = yield* Effect.tryPromise({
      try: () =>
        getNearestTronLightClientCheckpoints({
          context: args.context,
          mainnetChainId,
          tronLightClientAddress: args.tronLightClientAddress,
          tronBlockNumber: args.tronBlockNumber,
        }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    });

    if (!nearest.prev && !nearest.next) {
      return yield* Effect.fail(new Error("No TronLightClient checkpoints found in DB"));
    }

    if (
      nearest.prev?.tronBlockNumber === args.tronBlockNumber ||
      nearest.next?.tronBlockNumber === args.tronBlockNumber
    ) {
      return null;
    }

    const forwardLen =
      nearest.prev && nearest.prev.tronBlockNumber < args.tronBlockNumber
        ? args.tronBlockNumber - nearest.prev.tronBlockNumber
        : null;
    const backfillLen =
      nearest.next && nearest.next.tronBlockNumber > args.tronBlockNumber
        ? nearest.next.tronBlockNumber - args.tronBlockNumber + 1n
        : null;

    const preferForward =
      forwardLen !== null && (backfillLen === null || forwardLen <= backfillLen);

    const rangeStart = preferForward ? nearest.prev!.tronBlockNumber + 1n : args.tronBlockNumber;
    const candidateOffsetBigint = args.tronBlockNumber - rangeStart;
    if (candidateOffsetBigint < 0n) {
      return yield* Effect.fail(new Error("proveBlocks planning error: negative candidate offset"));
    }

    const maxRangeEnd = rangeStart + (MAX_TRON_BLOCKS_PER_PROVE_CALL - 1n);
    const baseRangeEnd = preferForward ? args.tronBlockNumber : nearest.next!.tronBlockNumber;
    let rangeEnd =
      baseRangeEnd + FINALITY_LOOKAHEAD_INITIAL_BLOCKS > maxRangeEnd
        ? maxRangeEnd
        : baseRangeEnd + FINALITY_LOOKAHEAD_INITIAL_BLOCKS;

    yield* Effect.logInfo("[tron_light_client] proveBlocks plan").pipe(
      Effect.annotateLogs({
        tronBlockNumber: args.tronBlockNumber.toString(),
        rangeStart: rangeStart.toString(),
        rangeEnd: rangeEnd.toString(),
        candidateOffset: candidateOffsetBigint.toString(),
        direction: preferForward ? "forward" : "backfill",
        maxRangeEnd: maxRangeEnd.toString(),
      })
    );

    const witnessIndexByTronOwnerHex = (yield* Effect.tryPromise({
      try: () =>
        loadTronLightClientSrs({
          mainnetClient: args.mainnetClient,
          tronLightClientAddress: args.tronLightClientAddress,
        }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    })).witnessIndexByTronOwnerHex;

    const parsedBlocks: Array<ReturnType<typeof parseTronBlockForLightClient>> = [];
    const blockNumbers: bigint[] = [];

    const downloadStartedAtMs = Date.now();
    let downloaded = 0;
    let lastProgressLogAtMs = downloadStartedAtMs;

    const downloadSegment = (segmentStart: bigint, segmentEnd: bigint) =>
      Effect.forEach(
        (() => {
          const nums: bigint[] = [];
          for (let n = segmentStart; n <= segmentEnd; n++) nums.push(n);
          return nums;
        })(),
        (blockNumber) =>
          args.fetchTronBlockByNum(blockNumber).pipe(
            Effect.tap(() =>
              Effect.sync(() => {
                downloaded += 1;
                const now = Date.now();
                if (now - lastProgressLogAtMs < TRON_BLOCK_FETCH_PROGRESS_INTERVAL_MS) return null;
                lastProgressLogAtMs = now;
                return { downloaded, elapsedMs: now - downloadStartedAtMs };
              }).pipe(
                Effect.flatMap((progress) =>
                  progress
                    ? Effect.logInfo("[tron_light_client] proveBlocks downloading").pipe(
                        Effect.annotateLogs({
                          downloaded: progress.downloaded,
                          concurrency: TRON_BLOCK_FETCH_CONCURRENCY,
                          elapsedMs: progress.elapsedMs,
                        })
                      )
                    : Effect.void
                )
              )
            ),
            Effect.map(parseTronBlockForLightClient)
          ),
        { concurrency: TRON_BLOCK_FETCH_CONCURRENCY }
      );

    let segmentStart = rangeStart;

    while (true) {
      const newlyParsed = yield* downloadSegment(segmentStart, rangeEnd);
      parsedBlocks.push(...newlyParsed);
      for (let n = segmentStart; n <= rangeEnd; n++) blockNumbers.push(n);

      const { compressedTronBlockMetadata, compressedSignatures, witnessIndices } =
        encodeTronLightClientMetadataAndSignatures({
          blocks: parsedBlocks,
          witnessIndexByTronOwnerAddressHex: witnessIndexByTronOwnerHex,
        });

      const candidateOffset = Number(candidateOffsetBigint);
      const distinct = distinctWitnessesFromOffset(witnessIndices, candidateOffset);

      if (distinct >= FINALITY_DISTINCT_SR_THRESHOLD) {
        const downloadElapsedMs = Date.now() - downloadStartedAtMs;
        const blocksPerSecond = downloadElapsedMs > 0 ? (downloaded / downloadElapsedMs) * 1000 : 0;
        yield* Effect.logInfo("[tron_light_client] proveBlocks downloaded").pipe(
          Effect.annotateLogs({
            downloaded,
            total: blockNumbers.length,
            rangeEnd: rangeEnd.toString(),
            distinct,
            elapsedMs: downloadElapsedMs,
            blocksPerSecond: blocksPerSecond.toFixed(2),
          })
        );

        const startingBlockId = preferForward
          ? nearest.prev!.tronBlockId
          : (`0x${Buffer.from(parsedBlocks[0]!.parentHash).toString("hex")}` as Hex);

        const storeOffsets16 = encodeStoreOffsets16([candidateOffset]);

        const intersectionOffset = preferForward
          ? UINT256_MAX
          : BigInt(nearest.next!.tronBlockNumber - args.tronBlockNumber);

        const data = encodeFunctionData({
          abi: tronLightClientAbi,
          functionName: "proveBlocks",
          args: [
            startingBlockId,
            compressedTronBlockMetadata,
            compressedSignatures,
            intersectionOffset,
            storeOffsets16,
          ],
        });

        return { to: args.tronLightClientAddress, data };
      }

      if (rangeEnd >= maxRangeEnd) {
        return yield* Effect.fail(
          new Error(
            `proveBlocks planning error: checkpoint not finalized within max range (distinct=${distinct}, need>=${FINALITY_DISTINCT_SR_THRESHOLD})`
          )
        );
      }

      const nextEndCandidate = rangeEnd + FINALITY_LOOKAHEAD_STEP_BLOCKS;
      const nextEnd = nextEndCandidate > maxRangeEnd ? maxRangeEnd : nextEndCandidate;
      yield* Effect.logInfo("[tron_light_client] proveBlocks extending for finality").pipe(
        Effect.annotateLogs({
          previousRangeEnd: rangeEnd.toString(),
          nextRangeEnd: nextEnd.toString(),
          distinct,
          threshold: FINALITY_DISTINCT_SR_THRESHOLD,
        })
      );

      segmentStart = rangeEnd + 1n;
      rangeEnd = nextEnd;
    }

    // unreachable
  });
