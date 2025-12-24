import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import type { PublicClient } from "viem";
import { encodeFunctionData, type Address, type Hex } from "viem";

import type { BlockExtention } from "@untron/tron-protocol/api";

import { TronLightClientAbi } from "../../abis/evm/TronLightClientAbi";

import { getRows } from "./sqlRows";
import {
  encodeStoreOffsets16,
  encodeTronLightClientMetadataAndSignatures,
  parseTronBlockForLightClient,
} from "./tronProofs";

const UINT256_MAX = (1n << 256n) - 1n;

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
        abi: TronLightClientAbi,
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

  const rows = getRows(result) as Array<{ chainId: number }>;
  const chainId = rows[0]?.chainId;
  if (typeof chainId !== "number")
    throw new Error("Failed to resolve mainnet chainId for TronLightClient");
  return chainId;
}

type TronLightClientCheckpointRow = {
  tronBlockNumber: bigint;
  tronBlockId: Hex;
};

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

  const prevRows = getRows(prevResult) as Array<TronLightClientCheckpointRow>;
  const nextRows = getRows(nextResult) as Array<TronLightClientCheckpointRow>;
  return { prev: prevRows[0] ?? null, next: nextRows[0] ?? null };
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
    const rangeEnd = preferForward ? args.tronBlockNumber : nearest.next!.tronBlockNumber;

    const blockNumbers: bigint[] = [];
    for (let n = rangeStart; n <= rangeEnd; n++) blockNumbers.push(n);

    const blocks = yield* Effect.forEach(blockNumbers, (blockNumber) =>
      args.fetchTronBlockByNum(blockNumber)
    );

    const witnessIndexByTronOwnerHex = (yield* Effect.tryPromise({
      try: () =>
        loadTronLightClientSrs({
          mainnetClient: args.mainnetClient,
          tronLightClientAddress: args.tronLightClientAddress,
        }),
      catch: (e) => (e instanceof Error ? e : new Error(String(e))),
    })).witnessIndexByTronOwnerHex;

    const parsedBlocks = blocks.map(parseTronBlockForLightClient);
    const { compressedTronBlockMetadata, compressedSignatures } =
      encodeTronLightClientMetadataAndSignatures({
        blocks: parsedBlocks,
        witnessIndexByTronOwnerAddressHex: witnessIndexByTronOwnerHex,
      });

    const startingBlockId = preferForward
      ? nearest.prev!.tronBlockId
      : (`0x${Buffer.from(parsedBlocks[0]!.parentHash).toString("hex")}` as Hex);

    const numBlocks = BigInt(parsedBlocks.length);
    const storeOffsets16 = encodeStoreOffsets16([preferForward ? Number(numBlocks - 1n) : 0]);

    const intersectionOffset = preferForward
      ? UINT256_MAX
      : BigInt(nearest.next!.tronBlockNumber - args.tronBlockNumber);

    const data = encodeFunctionData({
      abi: TronLightClientAbi,
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
  });
