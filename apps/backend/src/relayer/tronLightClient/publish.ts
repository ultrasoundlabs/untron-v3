import { Effect } from "effect";
import type { Hex } from "viem";

import { AppConfig } from "../../effect/config";
import { getTronLightClientAddress } from "../../contracts";
import { MainnetRelayer } from "../deps/mainnet";
import { TronGrpc } from "../deps/tron";
import { MAINNET_CHAIN_ID } from "../../env";
import type { RelayJobHandlerContext } from "../jobs/types";

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

    const included = yield* MainnetRelayer.sendUserOperation({ calls: [plan.call] });

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks included").pipe(
      Effect.annotateLogs({
        transactionHash: included.transactionHash,
        blockNumber: included.blockNumber.toString(),
      })
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
