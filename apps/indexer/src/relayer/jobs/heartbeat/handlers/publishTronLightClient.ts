import { Effect } from "effect";
import { sql } from "ponder";
import { encodeFunctionData, type Address, type Hex } from "viem";

import type { BlockExtention } from "@untron/tron-protocol/api";

import { tronLightClientAbi } from "@untron/v3-contracts";

import { tryPromise } from "../../../../effect/tryPromise";
import { MAINNET_CHAIN_ID } from "../../../../env";
import { MainnetRelayer } from "../../../deps/mainnet";
import { TronGrpc, fetchTronBlockByNum } from "../../../deps/tronGrpc";
import { getRows } from "../../../sqlRows";
import {
  encodeStoreOffsets16,
  encodeTronLightClientMetadataAndSignatures,
  parseTronBlockForLightClient,
} from "../../../tronProofs";
import type { RelayJobHandlerContext } from "../../types";
import { tronLightClientConfig } from "ponder:schema";

const DEFAULT_PUBLISH_LAG_BLOCKS = 0n;
const MAX_TRON_BLOCKS_PER_PROVE_CALL = 500n;
const MAX_REQUESTS_PER_RANGE_QUERY = 256;
const DEFAULT_REQUEST_COOLDOWN_BLOCKS = 5n;

const UINT256_MAX = (1n << 256n) - 1n;

const FINALITY_DISTINCT_SR_THRESHOLD = 19;

const TRON_BLOCK_FETCH_CONCURRENCY = (() => {
  const raw = process.env.TRON_BLOCK_FETCH_CONCURRENCY;
  if (!raw) return 1;
  const parsed = Number.parseInt(raw, 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 10;
})();

function parseNonNegativeBigintEnv(name: string, fallback: bigint): bigint {
  const raw = process.env[name];
  if (!raw) return fallback;
  try {
    const value = BigInt(raw);
    return value >= 0n ? value : fallback;
  } catch {
    return fallback;
  }
}

function coerceBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) return BigInt(value);
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

type WitnessIndexMap = ReadonlyMap<string, number>;

const witnessIndexCache = new Map<string, Promise<WitnessIndexMap>>();

function normalizeBytes20ToOwnerHex(bytes20Hex: string, label: string): string {
  const raw = bytes20Hex.toLowerCase();
  if (!/^0x[0-9a-f]{40}$/.test(raw)) throw new Error(`Invalid ${label} (expected bytes20 hex)`);
  return `41${raw.slice(2)}`;
}

function parseJsonArrayOfBytes20Hex(value: string, label: string): string[] {
  let parsed: unknown;
  try {
    parsed = JSON.parse(value) as unknown;
  } catch {
    throw new Error(`Invalid ${label} (expected JSON array)`);
  }
  if (!Array.isArray(parsed) || parsed.length !== 27) {
    throw new Error(`Invalid ${label} (expected JSON array length 27)`);
  }
  for (let i = 0; i < parsed.length; i++) {
    const v = parsed[i];
    if (typeof v !== "string" || !/^0x[0-9a-f]{40}$/i.test(v)) {
      throw new Error(`Invalid ${label}[${i}] (expected bytes20 hex)`);
    }
  }
  return parsed as string[];
}

async function loadWitnessIndexByTronOwnerAddressHex(args: {
  context: RelayJobHandlerContext["ponderContext"];
  tronLightClientAddress: Address;
}): Promise<WitnessIndexMap> {
  const key = args.tronLightClientAddress.toLowerCase();
  const cached = witnessIndexCache.get(key);
  if (cached) return cached;

  const promise = (async () => {
    const config = await args.context.db.find(tronLightClientConfig, {
      id: `${MAINNET_CHAIN_ID}:${key}`,
    });
    if (!config) {
      throw new Error(
        "Missing tron_light_client_config row in DB; ensure TronLightClientConfigured is indexed"
      );
    }

    const srs = parseJsonArrayOfBytes20Hex(config.srsJson, "tron_light_client_config.srsJson");

    const entries: Array<[string, number]> = [];
    for (let i = 0; i < 27; i++) {
      entries.push([normalizeBytes20ToOwnerHex(srs[i]!, `srs[${i}]`), i]);
    }

    return new Map(entries) as WitnessIndexMap;
  })();

  witnessIndexCache.set(key, promise);
  try {
    return await promise;
  } catch (error) {
    witnessIndexCache.delete(key);
    throw error;
  }
}

export const publishTronLightClient = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const tronHeadBlockNumber = ctx.headBlockNumber;

    const publishLagBlocks = parseNonNegativeBigintEnv(
      "TRON_LIGHT_CLIENT_PUBLISH_LAG_BLOCKS",
      DEFAULT_PUBLISH_LAG_BLOCKS
    );

    const publishTargetBlockNumber =
      tronHeadBlockNumber > publishLagBlocks ? tronHeadBlockNumber - publishLagBlocks : 0n;

    const tronLightClientAddress = (
      ctx.ponderContext.contracts.TronLightClient.address as Address
    ).toLowerCase() as Address;

    const tronGrpc = yield* TronGrpc;

    yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        DELETE FROM "tron_light_client_publish_request" r
        USING "tron_light_client_checkpoint" c
        WHERE r.chain_id = ${MAINNET_CHAIN_ID}
          AND r.tron_light_client_address = ${tronLightClientAddress}
          AND c.chain_id = r.chain_id
          AND c.contract_address = r.tron_light_client_address
          AND c.tron_block_number = r.tron_block_number;
      `)
    );

    const latestCheckpointResult = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          tron_block_number AS "tronBlockNumber",
          tron_block_id AS "tronBlockId"
        FROM "tron_light_client_checkpoint"
        WHERE chain_id = ${MAINNET_CHAIN_ID}
          AND contract_address = ${tronLightClientAddress}
        ORDER BY tron_block_number DESC
        LIMIT 1;
      `)
    );
    const latestCheckpointRows = getRows(latestCheckpointResult) as Array<{
      tronBlockNumber: unknown;
      tronBlockId: unknown;
    }>;
    if (latestCheckpointRows.length === 0) {
      yield* Effect.logError(
        "[tron_light_client] missing checkpoints in DB; cannot publish without initial TronBlockStored"
      ).pipe(Effect.annotateLogs({ tronLightClientAddress }));
      return;
    }

    const startingBlockNumber = coerceBigint(
      latestCheckpointRows[0]!.tronBlockNumber,
      "checkpoint.tronBlockNumber"
    );
    const startingBlockId = String(latestCheckpointRows[0]!.tronBlockId).toLowerCase() as Hex;

    if (!/^0x[0-9a-f]{64}$/.test(startingBlockId)) {
      throw new Error("Invalid checkpoint.tronBlockId (expected bytes32 hex)");
    }

    if (startingBlockNumber >= publishTargetBlockNumber) return;
    if (startingBlockNumber >= tronHeadBlockNumber) return;

    const rangeStart = startingBlockNumber + 1n;
    let rangeEnd = rangeStart + (MAX_TRON_BLOCKS_PER_PROVE_CALL - 1n);
    if (rangeEnd > publishTargetBlockNumber) rangeEnd = publishTargetBlockNumber;
    if (rangeEnd < rangeStart) return;

    const requestCooldownBlocks = parseNonNegativeBigintEnv(
      "TRON_LIGHT_CLIENT_PUBLISH_REQUEST_COOLDOWN_BLOCKS",
      DEFAULT_REQUEST_COOLDOWN_BLOCKS
    );
    const eligibleLastSent =
      tronHeadBlockNumber > requestCooldownBlocks
        ? tronHeadBlockNumber - requestCooldownBlocks
        : 0n;

    const nextRequestResult = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          tron_block_number AS "tronBlockNumber"
        FROM "tron_light_client_publish_request"
        WHERE chain_id = ${MAINNET_CHAIN_ID}
          AND tron_light_client_address = ${tronLightClientAddress}
          AND tron_block_number >= ${rangeStart}
          AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${eligibleLastSent})
        ORDER BY tron_block_number ASC, "id" ASC
        LIMIT 1;
      `)
    );
    const nextRequestRows = getRows(nextRequestResult) as Array<{ tronBlockNumber: unknown }>;
    if (nextRequestRows.length === 0) return;
    const nextRequestedTronBlockNumber = coerceBigint(
      nextRequestRows[0]!.tronBlockNumber,
      "publish_request.tronBlockNumber"
    );

    const requestRangeResult = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          tron_block_number AS "tronBlockNumber"
        FROM "tron_light_client_publish_request"
        WHERE chain_id = ${MAINNET_CHAIN_ID}
          AND tron_light_client_address = ${tronLightClientAddress}
          AND tron_block_number >= ${rangeStart}
          AND tron_block_number <= ${rangeEnd}
          AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${eligibleLastSent})
        ORDER BY tron_block_number ASC, "id" ASC
        LIMIT ${MAX_REQUESTS_PER_RANGE_QUERY};
      `)
    );
    const requestRangeRows = getRows(requestRangeResult) as Array<{ tronBlockNumber: unknown }>;
    const requestedOffsets = requestRangeRows.map((r) =>
      Number(coerceBigint(r.tronBlockNumber, "publish_request.tronBlockNumber") - rangeStart)
    );

    const { wallet, callOpts } = yield* tronGrpc.get();

    const witnessIndexByTronOwnerAddressHex = yield* tryPromise(() =>
      loadWitnessIndexByTronOwnerAddressHex({
        context: ctx.ponderContext,
        tronLightClientAddress,
      })
    );

    const plan = yield* buildTronLightClientProveBlocksCall({
      tronLightClientAddress,
      startingBlockId,
      rangeStart,
      rangeEnd,
      requestedOffsets,
      witnessIndexByTronOwnerAddressHex: witnessIndexByTronOwnerAddressHex,
      fetchTronBlockByNum: (blockNumber) =>
        tryPromise(() =>
          fetchTronBlockByNum({
            wallet,
            metadata: callOpts.metadata,
            blockNumber,
            timeoutMs: 60_000,
            retries: 2,
          })
        ),
    });

    if (!plan) return;

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks").pipe(
      Effect.annotateLogs({
        tronHeadBlockNumber: tronHeadBlockNumber.toString(),
        publishTargetBlockNumber: publishTargetBlockNumber.toString(),
        latestCheckpointBlockNumber: startingBlockNumber.toString(),
        nextRequestedTronBlockNumber: nextRequestedTronBlockNumber.toString(),
        rangeStart: plan.rangeStart.toString(),
        rangeEnd: plan.rangeEnd.toString(),
        storeOffsets: plan.storeOffsets.join(","),
        storedRequestedTronBlockNumbers: plan.storedRequestedTronBlockNumbers
          .map((n) => n.toString())
          .join(","),
        maxFinalizableOffset: String(plan.maxFinalizableOffset),
        distinctWitnessesAtLastStoredOffset: String(plan.distinctWitnessesAtLastStoredOffset),
      })
    );

    const included = yield* MainnetRelayer.sendUserOperation({ calls: [plan.call] });

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks included").pipe(
      Effect.annotateLogs({
        transactionHash: included.transactionHash,
        blockNumber: included.blockNumber.toString(),
      })
    );

    yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        UPDATE "tron_light_client_publish_request"
        SET last_sent_at_tron_block_number = ${tronHeadBlockNumber},
            last_sent_at_tron_block_timestamp = ${ctx.headBlockTimestamp}
        WHERE chain_id = ${MAINNET_CHAIN_ID}
          AND tron_light_client_address = ${tronLightClientAddress}
          AND tron_block_number >= ${rangeStart}
          AND tron_block_number <= ${rangeEnd}
          AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${eligibleLastSent});
      `)
    );
  });

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

type TronLightClientProveBlocksPlan = Readonly<{
  call: { to: Address; data: Hex };
  rangeStart: bigint;
  rangeEnd: bigint;
  storeOffsets: readonly number[];
  storedRequestedTronBlockNumbers: readonly bigint[];
  maxFinalizableOffset: number;
  distinctWitnessesAtLastStoredOffset: number;
}>;

const buildTronLightClientProveBlocksCall = (args: {
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
