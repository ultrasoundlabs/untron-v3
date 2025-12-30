import { Effect } from "effect";
import { parseEventLogs, type Address, type Hash, type Hex } from "viem";

import { AppConfig } from "../../effect/config";
import { tryPromise } from "../../effect/tryPromise";
import { getTronLightClientAddress } from "../../contracts";
import { tronLightClientAbi } from "@untron/v3-contracts";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc } from "../deps/tron";
import { MAINNET_CHAIN_ID } from "../../env";
import type { RelayJobHandlerContext } from "../jobs/types";
import { tronLightClientCheckpoint } from "ponder:schema";

import { fetchTronBlocksForLightClient } from "./fetch";
import { planTronLightClientProveBlocksCall } from "./planner";
import {
  deleteFulfilledPublishRequests,
  getCheckpointAtOrAbove,
  getEligibleRequestBlockNumbersInRange,
  getLatestCheckpoint,
  getOldestEligibleRequestBlockNumber,
  loadWitnessIndexByTronOwnerAddressHex,
  markPublishRequestSent,
  markPublishRequestsSentInRange,
} from "./repo";

const MAX_TRON_BLOCKS_PER_PROVE_CALL = 500n;
const MAX_REQUESTS_PER_RANGE_QUERY = 256;
const UINT256_MAX = (1n << 256n) - 1n;

function bufferToHex32(value: Buffer, label: string): Hex {
  if (value.length !== 32) throw new Error(`Invalid ${label} (expected 32 bytes)`);
  return `0x${value.toString("hex")}` as Hex;
}

function safeOffsetsFromBlockNumbers(args: {
  rangeStart: bigint;
  requestedBlockNumbers: readonly bigint[];
}): number[] {
  const offsets: number[] = [];
  for (const n of args.requestedBlockNumbers) {
    const offBig = n - args.rangeStart;
    const off = Number(offBig);
    if (!Number.isSafeInteger(off) || off < 0) continue;
    offsets.push(off);
  }
  return offsets;
}

const fastIndexTronLightClientCheckpointsFromMainnetTx = (args: {
  ctx: RelayJobHandlerContext;
  tronLightClientAddress: Address;
  transactionHash: Hash;
}) =>
  Effect.gen(function* () {
    const publicClients = yield* PublicClients;
    const mainnetClient = yield* publicClients.get("mainnet");

    const receipt = yield* tryPromise(() =>
      mainnetClient.getTransactionReceipt({ hash: args.transactionHash })
    );

    const tronLightClientLogs = receipt.logs.filter(
      (log) => log.address.toLowerCase() === args.tronLightClientAddress.toLowerCase()
    );

    const parsed = parseEventLogs({
      abi: tronLightClientAbi,
      logs: tronLightClientLogs,
      eventName: "TronBlockStored",
      strict: false,
    });

    if (parsed.length === 0) return;

    const storedAtBlock = yield* tryPromise(() =>
      mainnetClient.getBlock({ blockNumber: receipt.blockNumber })
    );

    const contractAddress = args.tronLightClientAddress.toLowerCase() as Address;

    let inserted = 0;
    for (const log of parsed) {
      const parsedArgs = log.args as unknown as {
        blockNumber?: bigint;
        blockId?: Hex;
        txTrieRoot?: Hex;
        timestamp?: bigint | number;
      };

      const tronBlockNumber = parsedArgs.blockNumber;
      const tronBlockId = parsedArgs.blockId;
      const tronTxTrieRoot = parsedArgs.txTrieRoot;
      const tronBlockTimestampRaw = parsedArgs.timestamp;
      const storedAtLogIndex = log.logIndex;

      if (
        tronBlockNumber === undefined ||
        tronBlockId === undefined ||
        tronTxTrieRoot === undefined ||
        tronBlockTimestampRaw === undefined ||
        storedAtLogIndex === undefined
      ) {
        continue;
      }

      const tronBlockTimestamp =
        typeof tronBlockTimestampRaw === "bigint"
          ? tronBlockTimestampRaw
          : BigInt(tronBlockTimestampRaw);

      const id = `${MAINNET_CHAIN_ID}:${contractAddress}:${tronBlockNumber.toString()}`;

      yield* tryPromise(() =>
        args.ctx.ponderContext.db
          .insert(tronLightClientCheckpoint)
          .values({
            id,
            chainId: MAINNET_CHAIN_ID,
            contractAddress,
            tronBlockNumber,
            tronBlockId,
            tronTxTrieRoot,
            tronBlockTimestamp,
            storedAtBlockNumber: receipt.blockNumber,
            storedAtBlockTimestamp: storedAtBlock.timestamp,
            storedAtTransactionHash: receipt.transactionHash,
            storedAtLogIndex,
          })
          .onConflictDoUpdate({
            tronBlockId,
            tronTxTrieRoot,
            tronBlockTimestamp,
            storedAtBlockNumber: receipt.blockNumber,
            storedAtBlockTimestamp: storedAtBlock.timestamp,
            storedAtTransactionHash: receipt.transactionHash,
            storedAtLogIndex,
          })
      );

      inserted++;
    }

    yield* Effect.logInfo("[tron_light_client] fast-indexed TronBlockStored checkpoints").pipe(
      Effect.annotateLogs({
        tronLightClientAddress: contractAddress,
        transactionHash: receipt.transactionHash,
        checkpointCount: inserted.toString(),
      })
    );
  });

export const publishTronLightClient = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const tronHeadBlockNumber = ctx.headBlockNumber;

    const publisherConfig = yield* AppConfig.tronLightClientPublisher();

    const publishTargetBlockNumber =
      tronHeadBlockNumber > publisherConfig.publishLagBlocks
        ? tronHeadBlockNumber - publisherConfig.publishLagBlocks
        : 0n;

    const tronLightClientAddress = getTronLightClientAddress();

    yield* deleteFulfilledPublishRequests({
      context: ctx.ponderContext,
      tronLightClientAddress,
    });

    const eligibleLastSent =
      tronHeadBlockNumber > publisherConfig.requestCooldownBlocks
        ? tronHeadBlockNumber - publisherConfig.requestCooldownBlocks
        : 0n;

    const latestCheckpoint = yield* getLatestCheckpoint({
      context: ctx.ponderContext,
      tronLightClientAddress,
    });
    if (!latestCheckpoint) {
      yield* Effect.logError(
        "[tron_light_client] missing checkpoints in DB; cannot publish without initial TronBlockStored"
      ).pipe(Effect.annotateLogs({ tronLightClientAddress }));
      return;
    }

    const maxSpan = MAX_TRON_BLOCKS_PER_PROVE_CALL - 1n;

    const forwardRangeStart = latestCheckpoint.tronBlockNumber + 1n;
    const forwardRangeEnd = (() => {
      if (forwardRangeStart > publishTargetBlockNumber) return null;
      let end = forwardRangeStart + maxSpan;
      if (end > publishTargetBlockNumber) end = publishTargetBlockNumber;
      if (end < forwardRangeStart) return null;
      return end;
    })();

    const selected = yield* Effect.gen(function* () {
      if (forwardRangeEnd !== null) {
        const requestedBlockNumbers = yield* getEligibleRequestBlockNumbersInRange({
          context: ctx.ponderContext,
          tronLightClientAddress,
          rangeStart: forwardRangeStart,
          rangeEnd: forwardRangeEnd,
          eligibleLastSent,
          limit: MAX_REQUESTS_PER_RANGE_QUERY,
        });

        if (requestedBlockNumbers.length > 0) {
          return {
            kind: "forward" as const,
            rangeStart: forwardRangeStart,
            rangeEnd: forwardRangeEnd,
            intersectionOffset: UINT256_MAX,
            progressOffset: "end" as const,
            anchorCheckpoint: latestCheckpoint,
            oldestEligibleRequestBlockNumber: requestedBlockNumbers[0]!,
            requestedBlockNumbers,
          };
        }
      }

      const oldestEligibleRequestBlockNumber = yield* getOldestEligibleRequestBlockNumber({
        context: ctx.ponderContext,
        tronLightClientAddress,
        eligibleLastSent,
      });
      if (oldestEligibleRequestBlockNumber === null) return null;

      // If the oldest request is ahead of our latest stored checkpoint, we might not have a usable
      // "anchor checkpoint at-or-above" yet. In that case, keep publishing forward progress until
      // the requested block falls within a forward window.
      if (
        forwardRangeEnd !== null &&
        oldestEligibleRequestBlockNumber > latestCheckpoint.tronBlockNumber
      ) {
        return {
          kind: "forward_progress" as const,
          rangeStart: forwardRangeStart,
          rangeEnd: forwardRangeEnd,
          intersectionOffset: UINT256_MAX,
          progressOffset: "end" as const,
          anchorCheckpoint: latestCheckpoint,
          oldestEligibleRequestBlockNumber,
          requestedBlockNumbers: [] as readonly bigint[],
        };
      }

      const anchorCheckpoint = yield* getCheckpointAtOrAbove({
        context: ctx.ponderContext,
        tronLightClientAddress,
        tronBlockNumber: oldestEligibleRequestBlockNumber,
      });
      if (!anchorCheckpoint) {
        yield* Effect.logError(
          "[tron_light_client] missing anchor checkpoint for publish requests"
        ).pipe(Effect.annotateLogs({ tronLightClientAddress, oldestEligibleRequestBlockNumber }));
        return null;
      }

      const distance = anchorCheckpoint.tronBlockNumber - oldestEligibleRequestBlockNumber;

      if (distance > maxSpan) {
        const rangeEnd = anchorCheckpoint.tronBlockNumber;
        const rangeStart = rangeEnd > maxSpan ? rangeEnd - maxSpan : 0n;
        return {
          kind: "backfill_step" as const,
          rangeStart,
          rangeEnd,
          intersectionOffset: rangeEnd - rangeStart,
          progressOffset: "start" as const,
          anchorCheckpoint,
          oldestEligibleRequestBlockNumber,
          requestedBlockNumbers: [] as readonly bigint[],
        };
      }

      const rangeStart = oldestEligibleRequestBlockNumber;
      let rangeEnd = rangeStart + maxSpan;
      if (rangeEnd > tronHeadBlockNumber) rangeEnd = tronHeadBlockNumber;
      if (rangeEnd < anchorCheckpoint.tronBlockNumber) rangeEnd = anchorCheckpoint.tronBlockNumber;

      const requestedBlockNumbers = yield* getEligibleRequestBlockNumbersInRange({
        context: ctx.ponderContext,
        tronLightClientAddress,
        rangeStart,
        rangeEnd,
        eligibleLastSent,
        limit: MAX_REQUESTS_PER_RANGE_QUERY,
      });

      return {
        kind: "backfill" as const,
        rangeStart,
        rangeEnd,
        intersectionOffset: anchorCheckpoint.tronBlockNumber - rangeStart,
        progressOffset: "end" as const,
        anchorCheckpoint,
        oldestEligibleRequestBlockNumber,
        requestedBlockNumbers,
      };
    });

    if (!selected) return;

    const nextRequestedTronBlockNumber =
      selected.requestedBlockNumbers[0] ?? selected.oldestEligibleRequestBlockNumber;

    const tronGrpc = yield* TronGrpc;
    const { wallet, callOpts } = yield* tronGrpc.get();

    const witnessIndexByTronOwnerAddressHex = yield* loadWitnessIndexByTronOwnerAddressHex({
      context: ctx.ponderContext,
      tronLightClientAddress,
    });

    yield* Effect.logInfo("[tron_light_client] fetch tron blocks").pipe(
      Effect.annotateLogs({
        kind: selected.kind,
        rangeStart: selected.rangeStart.toString(),
        rangeEnd: selected.rangeEnd.toString(),
        blockCount: (selected.rangeEnd - selected.rangeStart + 1n).toString(),
        concurrency: publisherConfig.blockFetchConcurrency,
      })
    );

    const blocks = yield* fetchTronBlocksForLightClient({
      wallet,
      metadata: callOpts.metadata,
      rangeStart: selected.rangeStart,
      rangeEnd: selected.rangeEnd,
      concurrency: publisherConfig.blockFetchConcurrency,
    });

    const startingBlockId =
      selected.intersectionOffset === UINT256_MAX
        ? selected.anchorCheckpoint.tronBlockId
        : bufferToHex32(blocks[0]!.parentHash, "tronBlock.parentHash");

    const plan = planTronLightClientProveBlocksCall({
      tronLightClientAddress,
      startingBlockId,
      rangeStart: selected.rangeStart,
      rangeEnd: selected.rangeEnd,
      blocks,
      intersectionOffset: selected.intersectionOffset,
      requestedOffsets: safeOffsetsFromBlockNumbers({
        rangeStart: selected.rangeStart,
        requestedBlockNumbers: selected.requestedBlockNumbers,
      }),
      progressOffset: selected.progressOffset,
      witnessIndexByTronOwnerAddressHex,
    });
    if (!plan) return;

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks").pipe(
      Effect.annotateLogs({
        kind: selected.kind,
        tronHeadBlockNumber: tronHeadBlockNumber.toString(),
        publishTargetBlockNumber: publishTargetBlockNumber.toString(),
        anchorCheckpointBlockNumber: selected.anchorCheckpoint.tronBlockNumber.toString(),
        nextRequestedTronBlockNumber: nextRequestedTronBlockNumber.toString(),
        rangeStart: plan.rangeStart.toString(),
        rangeEnd: plan.rangeEnd.toString(),
        intersectionOffset: plan.intersectionOffset.toString(),
        storeOffsets: plan.storeOffsets.join(","),
        storedRequestedTronBlockNumbers: plan.storedRequestedTronBlockNumbers
          .map((n) => n.toString())
          .join(","),
        maxFinalizableOffset: String(plan.maxFinalizableOffset),
        distinctWitnessesAtLastStoredOffset: String(plan.distinctWitnessesAtLastStoredOffset),
      })
    );

    const included = yield* MainnetRelayer.sendUserOperation({ calls: [plan.call] }).pipe(
      Effect.annotateLogs({ jobKind: "tron_light_client_publish" })
    );

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks included").pipe(
      Effect.annotateLogs({
        transactionHash: included.transactionHash,
        blockNumber: included.blockNumber.toString(),
      })
    );

    yield* fastIndexTronLightClientCheckpointsFromMainnetTx({
      ctx,
      tronLightClientAddress,
      transactionHash: included.transactionHash,
    }).pipe(
      Effect.catchAll((error) =>
        Effect.logWarning("[tron_light_client] fast-index checkpoints failed").pipe(
          Effect.annotateLogs({
            tronLightClientAddress,
            transactionHash: included.transactionHash,
            error: error.message,
          })
        )
      )
    );

    if (selected.requestedBlockNumbers.length > 0) {
      yield* markPublishRequestsSentInRange({
        context: ctx.ponderContext,
        tronLightClientAddress,
        rangeStart: selected.rangeStart,
        rangeEnd: selected.rangeEnd,
        eligibleLastSent,
        headBlockNumber: tronHeadBlockNumber,
        headBlockTimestamp: ctx.headBlockTimestamp,
      });
    } else {
      yield* markPublishRequestSent({
        context: ctx.ponderContext,
        tronLightClientAddress,
        tronBlockNumber: selected.oldestEligibleRequestBlockNumber,
        headBlockNumber: tronHeadBlockNumber,
        headBlockTimestamp: ctx.headBlockTimestamp,
      });
    }

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks done").pipe(
      Effect.annotateLogs({
        chainId: MAINNET_CHAIN_ID,
        tronLightClientAddress,
        kind: selected.kind,
        rangeStart: selected.rangeStart.toString(),
        rangeEnd: selected.rangeEnd.toString(),
      })
    );
  });
