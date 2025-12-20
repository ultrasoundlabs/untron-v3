import type { Context as PonderContext } from "ponder:registry";

import type { RelayerDeps } from "./deps";
import { handleMainnetHeartbeat } from "./jobs/mainnetHeartbeat";
import { handleTronHeartbeat } from "./jobs/tronHeartbeat";
import { handleTrc20Transfer } from "./jobs/trc20Transfer";
import type { RelayJobHandlerContext } from "./jobs/types";
import type { RelayJobKind, RelayJobRow } from "./types";
import { claimRelayJobs, markRelayJobFailed, markRelayJobSent } from "./queue";

export async function handleRelayJob({
  job,
  ctx,
}: {
  job: RelayJobRow;
  ctx: RelayJobHandlerContext;
}) {
  switch (job.kind) {
    case "mainnet_heartbeat":
      await handleMainnetHeartbeat({
        job: job as RelayJobRow & { kind: "mainnet_heartbeat" },
        ctx,
      });
      return;
    case "tron_heartbeat":
      await handleTronHeartbeat({ job: job as RelayJobRow & { kind: "tron_heartbeat" }, ctx });
      return;
    case "trc20_transfer":
      await handleTrc20Transfer({ job: job as RelayJobRow & { kind: "trc20_transfer" }, ctx });
      return;
    default: {
      const exhaustive: never = job.kind;
      return exhaustive;
    }
  }
}

export async function processRelayJobs({
  context,
  chainId,
  kind,
  headBlockNumber,
  headBlockTimestamp,
  minConfirmations,
  workerId,
  limit,
  dryRun,
  maxAttempts,
  retryDelayBlocks,
  deps,
}: {
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
  deps: RelayerDeps;
}) {
  const jobs = await claimRelayJobs({
    context,
    chainId,
    kind,
    headBlockNumber,
    headBlockTimestamp,
    minConfirmations,
    limit,
    workerId,
  });

  const ctx: RelayJobHandlerContext = {
    ponderContext: context,
    deps,
    headBlockNumber,
    headBlockTimestamp,
    dryRun,
  };

  for (const job of jobs) {
    try {
      await handleRelayJob({ job, ctx });
      await markRelayJobSent({ context, id: job.id, headBlockNumber, headBlockTimestamp });
    } catch (error) {
      const errorMessage =
        error instanceof Error ? `${error.name}: ${error.message}` : String(error);
      await markRelayJobFailed({
        context,
        id: job.id,
        headBlockNumber,
        headBlockTimestamp,
        errorMessage,
        maxAttempts,
        retryDelayBlocks,
      });
    }
  }
}
