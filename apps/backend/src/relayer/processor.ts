import { Cause, Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";
import { handleMainnetHeartbeat } from "./jobs/heartbeat/mainnetHeartbeat";
import { handleTronHeartbeat } from "./jobs/heartbeat/tronHeartbeat";
import { handleRelayControllerEventChain } from "./jobs/relayControllerEventChain";
import { handleTrc20Transfer } from "./jobs/trc20Transfer";
import type { RelayJobHandlerContext } from "./jobs/types";
import type { RelayJobKind, RelayJobRow } from "./types";
import { isRetryLaterError } from "./errors";
import {
  claimRelayJobs,
  markRelayJobFailed,
  markRelayJobRetryLater,
  markRelayJobSent,
} from "./queue";

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

    if (jobs.length > 0) {
      yield* Effect.logInfo("[relayer] claimed jobs").pipe(
        Effect.annotateLogs({
          chainId: args.chainId,
          kind: args.kind,
          count: jobs.length,
          headBlockNumber: args.headBlockNumber.toString(),
          minConfirmations: args.minConfirmations.toString(),
          workerId: args.workerId,
        })
      );
    }

    const ctx: RelayJobHandlerContext = {
      ponderContext: args.context,
      headBlockNumber: args.headBlockNumber,
      headBlockTimestamp: args.headBlockTimestamp,
      dryRun: args.dryRun,
    };

    yield* Effect.forEach(jobs, (job) =>
      Effect.logInfo("[relayer] job started").pipe(
        Effect.andThen(handleRelayJob({ job, ctx })),
        Effect.andThen(
          markRelayJobSent({
            context: args.context,
            id: job.id,
            headBlockNumber: args.headBlockNumber,
            headBlockTimestamp: args.headBlockTimestamp,
          })
        ),
        Effect.andThen(Effect.logInfo("[relayer] job sent")),
        Effect.catchAllCause((cause) => {
          const squashed = Cause.squash(cause);
          const errorMessage =
            squashed instanceof Error ? `${squashed.name}: ${squashed.message}` : String(squashed);

          if (isRetryLaterError(squashed)) {
            return Effect.logInfo("[relayer] job retry later").pipe(
              Effect.annotateLogs({
                id: job.id,
                chainId: args.chainId,
                kind: job.kind,
                headBlockNumber: args.headBlockNumber.toString(),
                error: errorMessage,
                attempts: job.attempts ?? 0,
                cause: Cause.pretty(cause),
              }),
              Effect.andThen(
                markRelayJobRetryLater({
                  context: args.context,
                  id: job.id,
                  headBlockNumber: args.headBlockNumber,
                  headBlockTimestamp: args.headBlockTimestamp,
                  errorMessage,
                  retryDelayBlocks: args.retryDelayBlocks,
                })
              )
            );
          }

          return Effect.logWarning("[relayer] job failed").pipe(
            Effect.annotateLogs({
              id: job.id,
              chainId: args.chainId,
              kind: job.kind,
              headBlockNumber: args.headBlockNumber.toString(),
              error: errorMessage,
              attempts: job.attempts ?? 0,
              cause: Cause.pretty(cause),
            }),
            Effect.andThen(
              markRelayJobFailed({
                context: args.context,
                id: job.id,
                headBlockNumber: args.headBlockNumber,
                headBlockTimestamp: args.headBlockTimestamp,
                errorMessage,
                maxAttempts: args.maxAttempts,
                retryDelayBlocks: args.retryDelayBlocks,
              })
            )
          );
        }),
        Effect.withLogSpan("relayer.job"),
        Effect.annotateLogs({
          jobId: job.id,
          jobKind: job.kind,
          chainId: args.chainId,
          headBlockNumber: args.headBlockNumber.toString(),
          attempts: job.attempts ?? 0,
          dryRun: args.dryRun,
        })
      )
    );
  });
