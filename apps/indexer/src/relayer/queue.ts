import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";
import { sql } from "ponder";

import { relayJob } from "ponder:schema";

import type { RelayJobKind, RelayJobRow, RelayJobStatus } from "./types";

import { getRows } from "./sqlRows";

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
          SELECT "id" AS id
          FROM "relay_job"
          WHERE chain_id = ${args.chainId}
            AND "kind" = ${args.kind}
            AND "status" = 'pending'
            AND created_at_block_number <= ${eligibleBlock}
            AND (next_retry_block_number IS NULL OR next_retry_block_number <= ${args.headBlockNumber})
          ORDER BY created_at_block_number ASC, "id" ASC
          LIMIT ${args.limit}
          FOR UPDATE SKIP LOCKED
        )
        UPDATE "relay_job"
        SET "status" = 'processing',
            locked_at_block_number = ${args.headBlockNumber},
            locked_at_block_timestamp = ${args.headBlockTimestamp},
            locked_by = ${args.workerId},
            updated_at_block_number = ${args.headBlockNumber},
            updated_at_block_timestamp = ${args.headBlockTimestamp},
            last_error = NULL
        WHERE "id" IN (SELECT id FROM candidates)
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
