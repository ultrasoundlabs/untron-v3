import { Effect } from "effect";

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
  getEligibleRequestBlockNumbersInRange,
  getLatestCheckpoint,
  loadWitnessIndexByTronOwnerAddressHex,
  markPublishRequestsSentInRange,
} from "./repo";

const MAX_TRON_BLOCKS_PER_PROVE_CALL = 500n;
const MAX_REQUESTS_PER_RANGE_QUERY = 256;

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

    const startingBlockNumber = latestCheckpoint.tronBlockNumber;
    const startingBlockId = latestCheckpoint.tronBlockId;

    if (startingBlockNumber >= publishTargetBlockNumber) return;
    if (startingBlockNumber >= tronHeadBlockNumber) return;

    const rangeStart = startingBlockNumber + 1n;
    let rangeEnd = rangeStart + (MAX_TRON_BLOCKS_PER_PROVE_CALL - 1n);
    if (rangeEnd > publishTargetBlockNumber) rangeEnd = publishTargetBlockNumber;
    if (rangeEnd < rangeStart) return;

    const eligibleLastSent =
      tronHeadBlockNumber > publisherConfig.requestCooldownBlocks
        ? tronHeadBlockNumber - publisherConfig.requestCooldownBlocks
        : 0n;

    const requestedBlockNumbers = yield* getEligibleRequestBlockNumbersInRange({
      context: ctx.ponderContext,
      tronLightClientAddress,
      rangeStart,
      rangeEnd,
      eligibleLastSent,
      limit: MAX_REQUESTS_PER_RANGE_QUERY,
    });
    if (requestedBlockNumbers.length === 0) return;
    const nextRequestedTronBlockNumber = requestedBlockNumbers[0]!;

    const tronGrpc = yield* TronGrpc;
    const { wallet, callOpts } = yield* tronGrpc.get();

    const witnessIndexByTronOwnerAddressHex = yield* loadWitnessIndexByTronOwnerAddressHex({
      context: ctx.ponderContext,
      tronLightClientAddress,
    });

    const blocks = yield* fetchTronBlocksForLightClient({
      wallet,
      metadata: callOpts.metadata,
      rangeStart,
      rangeEnd,
      concurrency: publisherConfig.blockFetchConcurrency,
    });

    const plan = planTronLightClientProveBlocksCall({
      tronLightClientAddress,
      startingBlockId,
      rangeStart,
      rangeEnd,
      blocks,
      requestedOffsets: safeOffsetsFromBlockNumbers({ rangeStart, requestedBlockNumbers }),
      witnessIndexByTronOwnerAddressHex,
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

    yield* markPublishRequestsSentInRange({
      context: ctx.ponderContext,
      tronLightClientAddress,
      rangeStart,
      rangeEnd,
      eligibleLastSent,
      headBlockNumber: tronHeadBlockNumber,
      headBlockTimestamp: ctx.headBlockTimestamp,
    });

    yield* Effect.logInfo("[tron_light_client] publish proveBlocks done").pipe(
      Effect.annotateLogs({
        chainId: MAINNET_CHAIN_ID,
        tronLightClientAddress,
        rangeStart: rangeStart.toString(),
        rangeEnd: rangeEnd.toString(),
      })
    );
  });
