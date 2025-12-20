import type { Context as PonderContext } from "ponder:registry";
import { sql } from "ponder";

import { relayJob } from "ponder:schema";

import type { RelayJobKind, RelayJobRow, RelayJobStatus } from "./types";

function getRows(result: unknown): unknown[] {
  if (Array.isArray(result)) return result;
  if (result && typeof result === "object" && "rows" in result) {
    const rows = (result as any).rows;
    if (Array.isArray(rows)) return rows;
  }
  return [];
}

export async function enqueueRelayJob({
  context,
  id,
  chainId,
  createdAtBlockNumber,
  createdAtBlockTimestamp,
  kind,
  status = "pending",
  payloadJson,
}: {
  context: PonderContext;
  id: string;
  chainId: number;
  createdAtBlockNumber: bigint;
  createdAtBlockTimestamp: bigint;
  kind: RelayJobKind;
  status?: RelayJobStatus;
  payloadJson: Record<string, unknown>;
}) {
  await context.db
    .insert(relayJob)
    .values({
      id,
      chainId,
      createdAtBlockNumber,
      createdAtBlockTimestamp,
      kind,
      status,
      attempts: 0,
      updatedAtBlockNumber: createdAtBlockNumber,
      updatedAtBlockTimestamp: createdAtBlockTimestamp,
      payloadJson,
    })
    .onConflictDoNothing();
}

export async function claimRelayJobs({
  context,
  chainId,
  kind,
  headBlockNumber,
  headBlockTimestamp,
  minConfirmations,
  limit,
  workerId,
}: {
  context: PonderContext;
  chainId: number;
  kind: RelayJobKind;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  minConfirmations: bigint;
  limit: number;
  workerId: string;
}): Promise<RelayJobRow[]> {
  const eligibleBlock =
    headBlockNumber > minConfirmations ? headBlockNumber - minConfirmations : 0n;

  const result = await context.db.sql.execute(sql`
    WITH candidates AS (
      SELECT ${relayJob.id} AS id
      FROM ${relayJob}
      WHERE ${relayJob.chainId} = ${chainId}
        AND ${relayJob.kind} = ${kind}
        AND ${relayJob.status} = 'pending'
        AND ${relayJob.createdAtBlockNumber} <= ${eligibleBlock}
        AND (${relayJob.nextRetryBlockNumber} IS NULL OR ${relayJob.nextRetryBlockNumber} <= ${headBlockNumber})
      ORDER BY ${relayJob.createdAtBlockNumber} ASC, ${relayJob.id} ASC
      LIMIT ${limit}
      FOR UPDATE SKIP LOCKED
    )
    UPDATE ${relayJob}
    SET ${relayJob.status} = 'processing',
        ${relayJob.lockedAtBlockNumber} = ${headBlockNumber},
        ${relayJob.lockedAtBlockTimestamp} = ${headBlockTimestamp},
        ${relayJob.lockedBy} = ${workerId},
        ${relayJob.updatedAtBlockNumber} = ${headBlockNumber},
        ${relayJob.updatedAtBlockTimestamp} = ${headBlockTimestamp},
        ${relayJob.lastError} = NULL
    WHERE ${relayJob.id} IN (SELECT id FROM candidates)
    RETURNING *;
  `);

  return getRows(result) as RelayJobRow[];
}

export async function markRelayJobSent({
  context,
  id,
  headBlockNumber,
  headBlockTimestamp,
}: {
  context: PonderContext;
  id: string;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
}) {
  await context.db.update(relayJob, { id }).set({
    status: "sent",
    updatedAtBlockNumber: headBlockNumber,
    updatedAtBlockTimestamp: headBlockTimestamp,
  });
}

export async function markRelayJobFailed({
  context,
  id,
  headBlockNumber,
  headBlockTimestamp,
  errorMessage,
  maxAttempts,
  retryDelayBlocks,
}: {
  context: PonderContext;
  id: string;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  errorMessage: string;
  maxAttempts: number;
  retryDelayBlocks: bigint;
}) {
  await context.db.update(relayJob, { id }).set((row: RelayJobRow) => {
    const nextAttempts = (row.attempts ?? 0) + 1;
    const isTerminal = nextAttempts >= maxAttempts;

    return {
      attempts: nextAttempts,
      status: isTerminal ? ("failed" as const) : ("pending" as const),
      lastError: errorMessage,
      updatedAtBlockNumber: headBlockNumber,
      updatedAtBlockTimestamp: headBlockTimestamp,
      nextRetryBlockNumber: isTerminal ? null : headBlockNumber + retryDelayBlocks,
    };
  });
}
