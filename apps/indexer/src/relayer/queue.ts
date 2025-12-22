import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";
import { sql } from "ponder";

import { relayJob } from "ponder:schema";

import type { RelayJobKind, RelayJobRow, RelayJobStatus } from "./types";

function getRows(result: unknown): unknown[] {
  if (Array.isArray(result)) return result;
  if (result && typeof result === "object" && "rows" in result) {
    const rows = (result as { readonly rows?: unknown }).rows;
    if (Array.isArray(rows)) return rows;
  }
  return [];
}

export const enqueueRelayJob = (args: {
  context: PonderContext;
  id: string;
  chainId: number;
  createdAtBlockNumber: bigint;
  createdAtBlockTimestamp: bigint;
  kind: RelayJobKind;
  status?: RelayJobStatus;
  payloadJson: Record<string, unknown>;
}) =>
  Effect.tryPromise({
    try: () =>
      args.context.db
        .insert(relayJob)
        .values({
          id: args.id,
          chainId: args.chainId,
          createdAtBlockNumber: args.createdAtBlockNumber,
          createdAtBlockTimestamp: args.createdAtBlockTimestamp,
          kind: args.kind,
          status: args.status ?? "pending",
          attempts: 0,
          updatedAtBlockNumber: args.createdAtBlockNumber,
          updatedAtBlockTimestamp: args.createdAtBlockTimestamp,
          payloadJson: args.payloadJson,
        })
        .onConflictDoNothing(),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const claimRelayJobs = (args: {
  context: PonderContext;
  chainId: number;
  kind: RelayJobKind;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  minConfirmations: bigint;
  limit: number;
  workerId: string;
}): Effect.Effect<RelayJobRow[], Error> =>
  Effect.tryPromise({
    try: async () => {
      const eligibleBlock =
        args.headBlockNumber > args.minConfirmations
          ? args.headBlockNumber - args.minConfirmations
          : 0n;

      const result = await args.context.db.sql.execute(sql`
        WITH candidates AS (
          SELECT ${relayJob.id} AS id
          FROM ${relayJob}
          WHERE ${relayJob.chainId} = ${args.chainId}
            AND ${relayJob.kind} = ${args.kind}
            AND ${relayJob.status} = 'pending'
            AND ${relayJob.createdAtBlockNumber} <= ${eligibleBlock}
            AND (${relayJob.nextRetryBlockNumber} IS NULL OR ${relayJob.nextRetryBlockNumber} <= ${args.headBlockNumber})
          ORDER BY ${relayJob.createdAtBlockNumber} ASC, ${relayJob.id} ASC
          LIMIT ${args.limit}
          FOR UPDATE SKIP LOCKED
        )
        UPDATE ${relayJob}
        SET ${relayJob.status} = 'processing',
            ${relayJob.lockedAtBlockNumber} = ${args.headBlockNumber},
            ${relayJob.lockedAtBlockTimestamp} = ${args.headBlockTimestamp},
            ${relayJob.lockedBy} = ${args.workerId},
            ${relayJob.updatedAtBlockNumber} = ${args.headBlockNumber},
            ${relayJob.updatedAtBlockTimestamp} = ${args.headBlockTimestamp},
            ${relayJob.lastError} = NULL
        WHERE ${relayJob.id} IN (SELECT id FROM candidates)
        RETURNING *;
      `);

      return getRows(result) as RelayJobRow[];
    },
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const markRelayJobSent = (args: {
  context: PonderContext;
  id: string;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
}) =>
  Effect.tryPromise({
    try: () =>
      args.context.db.update(relayJob, { id: args.id }).set({
        status: "sent",
        updatedAtBlockNumber: args.headBlockNumber,
        updatedAtBlockTimestamp: args.headBlockTimestamp,
      }),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const markRelayJobFailed = (args: {
  context: PonderContext;
  id: string;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  errorMessage: string;
  maxAttempts: number;
  retryDelayBlocks: bigint;
}) =>
  Effect.tryPromise({
    try: () =>
      args.context.db.update(relayJob, { id: args.id }).set((row: RelayJobRow) => {
        const nextAttempts = (row.attempts ?? 0) + 1;
        const isTerminal = nextAttempts >= args.maxAttempts;

        return {
          attempts: nextAttempts,
          status: isTerminal ? ("failed" as const) : ("pending" as const),
          lastError: args.errorMessage,
          updatedAtBlockNumber: args.headBlockNumber,
          updatedAtBlockTimestamp: args.headBlockTimestamp,
          nextRetryBlockNumber: isTerminal ? null : args.headBlockNumber + args.retryDelayBlocks,
        };
      }),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });
