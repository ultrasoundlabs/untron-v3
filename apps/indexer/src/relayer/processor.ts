import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";

import { handleMainnetHeartbeat } from "./jobs/heartbeat/mainnetHeartbeat";
import { handleTronHeartbeat } from "./jobs/heartbeat/tronHeartbeat";
import { handleRelayControllerEventChain } from "./jobs/relayControllerEventChain";
import { handleTrc20Transfer } from "./jobs/trc20Transfer";
import type { RelayJobHandlerContext } from "./jobs/types";
import type { RelayJobKind, RelayJobRow } from "./types";
import { claimRelayJobs, markRelayJobFailed, markRelayJobSent } from "./queue";

export const handleRelayJob = (args: { job: RelayJobRow; ctx: RelayJobHandlerContext }) => {
  switch (args.job.kind) {
    case "mainnet_heartbeat":
      return handleMainnetHeartbeat({
        job: args.job as RelayJobRow & { kind: "mainnet_heartbeat" },
        ctx: args.ctx,
      });
    case "tron_heartbeat":
      return handleTronHeartbeat({
        job: args.job as RelayJobRow & { kind: "tron_heartbeat" },
        ctx: args.ctx,
      });
    case "trc20_transfer":
      return handleTrc20Transfer({
        job: args.job as RelayJobRow & { kind: "trc20_transfer" },
        ctx: args.ctx,
      });
    case "relay_controller_event_chain":
      return handleRelayControllerEventChain({
        job: args.job as RelayJobRow & { kind: "relay_controller_event_chain" },
        ctx: args.ctx,
      });
    default: {
      const exhaustive: never = args.job.kind;
      return exhaustive;
    }
  }
};

export const processRelayJobs = (args: {
  context: PonderContext;
  chainId: number;
  kind: RelayJobKind;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  minConfirmations: bigint;
  workerId: string;
  limit: number;
  dryRun: boolean;
  maxAttempts: number;
  retryDelayBlocks: bigint;
}) =>
  Effect.gen(function* () {
    const jobs = yield* claimRelayJobs({
      context: args.context,
      chainId: args.chainId,
      kind: args.kind,
      headBlockNumber: args.headBlockNumber,
      headBlockTimestamp: args.headBlockTimestamp,
      minConfirmations: args.minConfirmations,
      limit: args.limit,
      workerId: args.workerId,
    });

    const ctx: RelayJobHandlerContext = {
      ponderContext: args.context,
      headBlockNumber: args.headBlockNumber,
      headBlockTimestamp: args.headBlockTimestamp,
      dryRun: args.dryRun,
    };

    yield* Effect.forEach(jobs, (job) =>
      handleRelayJob({ job, ctx }).pipe(
        Effect.andThen(
          markRelayJobSent({
            context: args.context,
            id: job.id,
            headBlockNumber: args.headBlockNumber,
            headBlockTimestamp: args.headBlockTimestamp,
          })
        ),
        Effect.catchAll((error) => {
          const errorMessage =
            error instanceof Error ? `${error.name}: ${error.message}` : String(error);
          return markRelayJobFailed({
            context: args.context,
            id: job.id,
            headBlockNumber: args.headBlockNumber,
            headBlockTimestamp: args.headBlockTimestamp,
            errorMessage,
            maxAttempts: args.maxAttempts,
            retryDelayBlocks: args.retryDelayBlocks,
          });
        })
      )
    );
  });
