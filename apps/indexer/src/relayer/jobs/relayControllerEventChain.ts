import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import {
  decodeFunctionData,
  encodeFunctionData,
  isAddress,
  type Address,
  type Hex,
  type PublicClient,
} from "viem";

import { NumberMessage, type BlockExtention } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { Transaction } from "@untron/tron-protocol/tron";

import { TronLightClientAbi } from "../../../abis/evm/TronLightClientAbi";
import { UntronV3Abi } from "../../../abis/evm/UntronV3Abi";
import { UntronControllerAbi } from "../../../abis/tron/UntronControllerAbi";
import { computeNextEventChainTip } from "../../eventChain/tip";
import { tryPromise } from "../../effect/tryPromise";
import type { RelayJobRow } from "../types";
import type { RelayJobHandlerContext } from "./types";
import { eventChainEvent, eventChainState, tronLightClientCheckpoint } from "ponder:schema";
import { getRows } from "../sqlRows";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc } from "../deps/tronGrpc";
import {
  computeTronTxMerkleProof,
  encodeStoreOffsets16,
  encodeTronLightClientMetadataAndSignatures,
  parseTronBlockForLightClient,
} from "../tronProofs";

const UINT256_MAX = (1n << 256n) - 1n;

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

function expectString(value: unknown, label: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`Invalid ${label}`);
  return value;
}

function expectHex(value: unknown, label: string): Hex {
  const raw = expectString(value, label).toLowerCase();
  if (!/^0x[0-9a-f]+$/.test(raw)) throw new Error(`Invalid ${label} (expected 0x-hex)`);
  return raw as Hex;
}

function expectAddress(value: unknown, label: string): Address {
  const raw = expectString(value, label);
  if (!isAddress(raw)) throw new Error(`Invalid ${label} (expected EVM address)`);
  return raw.toLowerCase() as Address;
}

function expectBigint(value: unknown, label: string): bigint {
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

function evmAddressToTronOwnerHex(evm: Address): string {
  return `41${evm.slice(2).toLowerCase()}`;
}

type TronSrsCache = {
  witnessIndexByTronOwnerHex: ReadonlyMap<string, number>;
};

const tronLightClientSrsCache = new Map<string, Promise<TronSrsCache>>();

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

function hasIsEventChainTipCallInMulticall(calls: readonly Hex[], selectorIsEventChainTip: Hex) {
  for (const call of calls) {
    if (call.length !== 2 + (4 + 32) * 2) continue; // selector + one bytes32 arg
    if (call.slice(0, 10).toLowerCase() === selectorIsEventChainTip.toLowerCase()) return true;
  }
  return false;
}

async function fetchTronBlockByNum(args: {
  wallet: any;
  metadata: unknown;
  blockNumber: bigint;
}): Promise<BlockExtention> {
  const req = NumberMessage.fromPartial({ num: args.blockNumber.toString() });
  return await new Promise((resolve, reject) => {
    args.wallet.getBlockByNum2(req, args.metadata, (err: unknown, res: BlockExtention | null) =>
      err || !res ? reject(err ?? new Error("Empty response from getBlockByNum2")) : resolve(res)
    );
  });
}

function findTransactionInBlock(args: { block: BlockExtention; txid: Hex }): Transaction {
  const txidHex = args.txid.replace(/^0x/i, "").toLowerCase();
  const txs = args.block.transactions ?? [];

  for (const txExt of txs) {
    const got = Buffer.from(txExt?.txid ?? Buffer.alloc(0))
      .toString("hex")
      .toLowerCase();
    if (got !== txidHex) continue;
    const tx = txExt?.transaction;
    if (!tx) throw new Error("Transaction extension missing transaction");
    return tx;
  }

  throw new Error("Transaction not found in block");
}

function parseTriggerSmartContract(tx: Transaction): { toEvm: Address; data: Hex } {
  const rawData = tx.rawData;
  if (!rawData) throw new Error("Tron tx missing rawData");
  const contract = rawData.contract?.[0];
  if (!contract?.parameter?.value?.length) throw new Error("Tron tx missing contract parameter");

  const trigger = TriggerSmartContract.decode(contract.parameter.value);
  const contractAddressBytes21 = trigger.contractAddress as Buffer | undefined;
  if (
    !contractAddressBytes21 ||
    contractAddressBytes21.length !== 21 ||
    contractAddressBytes21[0] !== 0x41
  ) {
    throw new Error("Tron tx TriggerSmartContract.contractAddress is not a canonical Tron address");
  }

  const dataBuf = trigger.data as Buffer | undefined;
  const data = dataBuf?.length ? (`0x${dataBuf.toString("hex")}` as Hex) : ("0x" as Hex);

  return {
    toEvm: (
      `0x${contractAddressBytes21.subarray(1).toString("hex")}` as Address
    ).toLowerCase() as Address,
    data,
  };
}

async function lookupMainnetChainIdForTronLightClient(args: {
  context: PonderContext;
  tronLightClientAddress: Address;
}): Promise<number> {
  const result = await args.context.db.sql.execute(sql`
    SELECT ${eventChainState.chainId} AS "chainId"
    FROM ${eventChainState}
    WHERE ${eventChainState.contractName} = 'TronLightClient'
      AND ${eventChainState.contractAddress} = ${args.tronLightClientAddress}
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
      ${tronLightClientCheckpoint.tronBlockNumber} AS "tronBlockNumber",
      ${tronLightClientCheckpoint.tronBlockId} AS "tronBlockId"
    FROM ${tronLightClientCheckpoint}
    WHERE ${tronLightClientCheckpoint.chainId} = ${args.mainnetChainId}
      AND ${tronLightClientCheckpoint.contractAddress} = ${args.tronLightClientAddress}
      AND ${tronLightClientCheckpoint.tronBlockNumber} <= ${args.tronBlockNumber}
    ORDER BY ${tronLightClientCheckpoint.tronBlockNumber} DESC
    LIMIT 1;
  `);

  const nextResult = await args.context.db.sql.execute(sql`
    SELECT
      ${tronLightClientCheckpoint.tronBlockNumber} AS "tronBlockNumber",
      ${tronLightClientCheckpoint.tronBlockId} AS "tronBlockId"
    FROM ${tronLightClientCheckpoint}
    WHERE ${tronLightClientCheckpoint.chainId} = ${args.mainnetChainId}
      AND ${tronLightClientCheckpoint.contractAddress} = ${args.tronLightClientAddress}
      AND ${tronLightClientCheckpoint.tronBlockNumber} >= ${args.tronBlockNumber}
    ORDER BY ${tronLightClientCheckpoint.tronBlockNumber} ASC
    LIMIT 1;
  `);

  const prevRows = getRows(prevResult) as Array<TronLightClientCheckpointRow>;
  const nextRows = getRows(nextResult) as Array<TronLightClientCheckpointRow>;
  return { prev: prevRows[0] ?? null, next: nextRows[0] ?? null };
}

async function loadControllerEventsToRelay(args: {
  context: PonderContext;
  tronChainId: number;
  controllerAddress: Address;
  startTip: Hex;
  endTip: Hex;
}): Promise<
  Array<{
    sig: Hex;
    data: Hex;
    blockNumber: bigint;
    blockTimestamp: bigint;
    tip: Hex;
    previousTip: Hex;
  }>
> {
  const endTipLower = args.endTip.toLowerCase() as Hex;
  const startTipLower = args.startTip.toLowerCase() as Hex;

  const endRowResult = await args.context.db.sql.execute(sql`
    SELECT
      ${eventChainEvent.sequence} AS "sequence"
    FROM ${eventChainEvent}
    WHERE ${eventChainEvent.chainId} = ${args.tronChainId}
      AND ${eventChainEvent.contractName} = 'UntronController'
      AND ${eventChainEvent.contractAddress} = ${args.controllerAddress}
      AND ${eventChainEvent.tip} = ${endTipLower}
    LIMIT 1;
  `);
  const endRows = getRows(endRowResult) as Array<{ sequence: bigint }>;
  const endSequence = endRows[0]?.sequence;
  if (typeof endSequence !== "bigint") {
    throw new Error(`Missing UntronController event chain row for end tip ${endTipLower}`);
  }

  let startSequence = 0n;
  if (startTipLower !== endTipLower) {
    const startRowResult = await args.context.db.sql.execute(sql`
      SELECT
        ${eventChainEvent.sequence} AS "sequence"
      FROM ${eventChainEvent}
      WHERE ${eventChainEvent.chainId} = ${args.tronChainId}
        AND ${eventChainEvent.contractName} = 'UntronController'
        AND ${eventChainEvent.contractAddress} = ${args.controllerAddress}
        AND ${eventChainEvent.tip} = ${startTipLower}
      LIMIT 1;
    `);
    const startRows = getRows(startRowResult) as Array<{ sequence: bigint }>;
    startSequence = startRows[0]?.sequence ?? 0n;
  }

  const eventsResult = await args.context.db.sql.execute(sql`
    SELECT
      ${eventChainEvent.tip} AS "tip",
      ${eventChainEvent.previousTip} AS "previousTip",
      ${eventChainEvent.blockNumber} AS "blockNumber",
      ${eventChainEvent.blockTimestamp} AS "blockTimestamp",
      ${eventChainEvent.eventSignature} AS "eventSignature",
      ${eventChainEvent.encodedEventData} AS "encodedEventData",
      ${eventChainEvent.sequence} AS "sequence"
    FROM ${eventChainEvent}
    WHERE ${eventChainEvent.chainId} = ${args.tronChainId}
      AND ${eventChainEvent.contractName} = 'UntronController'
      AND ${eventChainEvent.contractAddress} = ${args.controllerAddress}
      AND ${eventChainEvent.sequence} > ${startSequence}
      AND ${eventChainEvent.sequence} <= ${endSequence}
    ORDER BY ${eventChainEvent.sequence} ASC;
  `);

  const rows = getRows(eventsResult) as Array<{
    tip: Hex;
    previousTip: Hex;
    blockNumber: bigint;
    blockTimestamp: bigint;
    eventSignature: Hex;
    encodedEventData: Hex;
    sequence: bigint;
  }>;

  return rows.map((row) => ({
    sig: row.eventSignature,
    data: row.encodedEventData,
    blockNumber: row.blockNumber,
    blockTimestamp: row.blockTimestamp,
    tip: row.tip,
    previousTip: row.previousTip,
  }));
}

export const handleRelayControllerEventChain = ({
  ctx,
  job,
}: {
  job: RelayJobRow & { kind: "relay_controller_event_chain" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const payload = expectRecord(job.payloadJson, "payloadJson");
    const controllerAddress = expectAddress(payload.controllerAddress, "payload.controllerAddress");
    const tronBlockNumber = expectBigint(payload.tronBlockNumber, "payload.tronBlockNumber");
    const transactionHash = expectHex(payload.transactionHash, "payload.transactionHash");
    const eventChainTip = expectHex(payload.eventChainTip, "payload.eventChainTip");

    const publicClients = yield* PublicClients;
    const tronGrpc = yield* TronGrpc;

    const mainnetClient = yield* publicClients.get("mainnet");

    const tronLightClientAddress = (
      ctx.ponderContext.contracts.TronLightClient.address as `0x${string}`
    ).toLowerCase() as Address;
    const untronV3Address = (
      ctx.ponderContext.contracts.UntronV3.address as `0x${string}`
    ).toLowerCase() as Address;

    const lastControllerEventTip = (yield* tryPromise(() =>
      mainnetClient.readContract({
        address: untronV3Address,
        abi: UntronV3Abi,
        functionName: "lastControllerEventTip",
      })
    )) as Hex;

    if (lastControllerEventTip.toLowerCase() === eventChainTip.toLowerCase()) return;

    const { wallet, callOpts } = yield* tronGrpc.get();

    const block = yield* tryPromise(() =>
      fetchTronBlockByNum({ wallet, metadata: callOpts.metadata, blockNumber: tronBlockNumber })
    );
    const tx = findTransactionInBlock({ block, txid: transactionHash });

    const trigger = parseTriggerSmartContract(tx);
    if (trigger.toEvm.toLowerCase() !== controllerAddress.toLowerCase()) return;

    const selectorIsEventChainTip = encodeFunctionData({
      abi: UntronControllerAbi,
      functionName: "isEventChainTip",
      args: [eventChainTip],
    }).slice(0, 10) as Hex;

    const selectorMulticall = encodeFunctionData({
      abi: UntronControllerAbi,
      functionName: "multicall",
      args: [[] as Hex[]],
    }).slice(0, 10) as Hex;

    const topSelector = trigger.data.slice(0, 10).toLowerCase();
    if (topSelector === selectorIsEventChainTip.toLowerCase()) {
      // ok
    } else if (topSelector === selectorMulticall.toLowerCase()) {
      const decoded = decodeFunctionData({
        abi: UntronControllerAbi,
        data: trigger.data,
      });
      if (decoded.functionName !== "multicall") return;
      const calls = (decoded.args?.[0] ?? []) as readonly Hex[];
      if (!hasIsEventChainTipCallInMulticall(calls, selectorIsEventChainTip)) return;
    } else {
      return;
    }

    const proof = computeTronTxMerkleProof({ block, txidHex: transactionHash });

    const mainnetChainId = yield* tryPromise(() =>
      lookupMainnetChainIdForTronLightClient({
        context: ctx.ponderContext,
        tronLightClientAddress,
      })
    );

    const tronBlockPublished = yield* tryPromise(() =>
      mainnetClient.readContract({
        address: tronLightClientAddress,
        abi: TronLightClientAbi,
        functionName: "getTxTrieRoot",
        args: [tronBlockNumber],
      })
    ).pipe(
      Effect.as(true as const),
      Effect.catchAll(() => Effect.succeed(false as const))
    );

    const plannedCalls: Array<{ to: Address; data: Hex }> = [];

    if (!tronBlockPublished) {
      const nearest = yield* tryPromise(() =>
        getNearestTronLightClientCheckpoints({
          context: ctx.ponderContext,
          mainnetChainId,
          tronLightClientAddress,
          tronBlockNumber,
        })
      );

      if (nearest.prev?.tronBlockNumber === tronBlockNumber) {
        // block is already checkpointed according to DB (indexer lagged the onchain check)
      } else if (nearest.next?.tronBlockNumber === tronBlockNumber) {
        // block is already checkpointed according to DB (indexer lagged the onchain check)
      } else {
        const forwardLen =
          nearest.prev && nearest.prev.tronBlockNumber < tronBlockNumber
            ? tronBlockNumber - nearest.prev.tronBlockNumber
            : null;
        const backfillLen =
          nearest.next && nearest.next.tronBlockNumber > tronBlockNumber
            ? nearest.next.tronBlockNumber - tronBlockNumber + 1n
            : null;

        const preferForward =
          forwardLen !== null && (backfillLen === null || forwardLen <= backfillLen);

        if (!nearest.prev && !nearest.next) {
          return yield* Effect.fail(new Error("No TronLightClient checkpoints found in DB"));
        }

        const rangeStart = preferForward ? nearest.prev!.tronBlockNumber + 1n : tronBlockNumber;
        const rangeEnd = preferForward ? tronBlockNumber : nearest.next!.tronBlockNumber;

        const blockNumbers: bigint[] = [];
        for (let n = rangeStart; n <= rangeEnd; n++) blockNumbers.push(n);

        const blocks = yield* Effect.forEach(blockNumbers, (blockNumber) =>
          tryPromise(() =>
            fetchTronBlockByNum({ wallet, metadata: callOpts.metadata, blockNumber })
          )
        );

        const witnessIndexByTronOwnerHex = (yield* tryPromise(() =>
          loadTronLightClientSrs({ mainnetClient, tronLightClientAddress })
        )).witnessIndexByTronOwnerHex;

        const parsedBlocks = blocks.map(parseTronBlockForLightClient);
        const { compressedTronBlockMetadata, compressedSignatures } =
          encodeTronLightClientMetadataAndSignatures({
            blocks: parsedBlocks,
            witnessIndexByTronOwnerAddressHex: witnessIndexByTronOwnerHex,
          });

        const proveStartingBlock = preferForward
          ? nearest.prev!.tronBlockId
          : (`0x${Buffer.from(parsedBlocks[0]!.parentHash).toString("hex")}` as Hex);

        const numBlocks = BigInt(parsedBlocks.length);
        const storeOffsets16 = encodeStoreOffsets16([preferForward ? Number(numBlocks - 1n) : 0]);

        const intersectionOffset = preferForward
          ? UINT256_MAX
          : BigInt(nearest.next!.tronBlockNumber - tronBlockNumber);

        const proveCall = encodeFunctionData({
          abi: TronLightClientAbi,
          functionName: "proveBlocks",
          args: [
            proveStartingBlock,
            compressedTronBlockMetadata,
            compressedSignatures,
            intersectionOffset,
            storeOffsets16,
          ],
        });

        plannedCalls.push({ to: tronLightClientAddress, data: proveCall });
      }
    }

    const controllerEvents = yield* tryPromise(() =>
      loadControllerEventsToRelay({
        context: ctx.ponderContext,
        tronChainId: job.chainId,
        controllerAddress,
        startTip: lastControllerEventTip,
        endTip: eventChainTip,
      })
    );

    let computed = lastControllerEventTip as Hex;
    for (const ev of controllerEvents) {
      computed = computeNextEventChainTip({
        previousTip: computed,
        blockNumber: ev.blockNumber,
        blockTimestamp: ev.blockTimestamp,
        eventSignature: ev.sig,
        encodedEventData: ev.data,
      });
    }

    if (computed.toLowerCase() !== eventChainTip.toLowerCase()) {
      return yield* Effect.fail(
        new Error(
          `Controller event chain mismatch (computed=${computed}, expected=${eventChainTip}). Indexer may be behind.`
        )
      );
    }

    const relayCall = encodeFunctionData({
      abi: UntronV3Abi,
      functionName: "relayControllerEventChain",
      args: [
        tronBlockNumber,
        proof.encodedTx,
        [...proof.proof] as readonly Hex[],
        proof.index,
        controllerEvents.map((ev) => ({
          sig: ev.sig,
          data: ev.data,
          blockNumber: ev.blockNumber,
          blockTimestamp: ev.blockTimestamp,
        })),
      ],
    });

    plannedCalls.push({ to: untronV3Address, data: relayCall });

    yield* MainnetRelayer.sendUserOperation({
      calls: plannedCalls,
    });
  });
