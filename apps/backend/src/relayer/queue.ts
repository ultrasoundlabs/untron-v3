import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";
import { sql } from "ponder";

import { relayJob } from "ponder:schema";

import type { RelayJobKind, RelayJobRow, RelayJobStatus } from "./types";

import { getRows } from "./sqlRows";

function normalizePayloadJson(value: unknown, jobId: string): unknown {
  if (typeof value !== "string") return value;
  try {
    return JSON.parse(value) as unknown;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`Failed to parse relay_job.payload_json for job ${jobId}: ${message}`);
  }
}

function pick(row: Record<string, unknown>, camel: string, snake: string): unknown {
  if (camel in row) return row[camel];
  if (snake in row) return row[snake];
  return undefined;
}

function coerceBigint(value: unknown, label: string): bigint | null {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      return null;
    }
  }
  if (value == null) return null;
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

function coerceNumber(value: unknown, label: string): number | null {
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (typeof value === "string" && value.length > 0) {
    const n = Number(value);
    if (Number.isFinite(n)) return n;
    return null;
  }
  if (value == null) return null;
  throw new Error(`Invalid ${label} (expected number)`);
}

function normalizeRelayJobRow(row: Record<string, unknown>): RelayJobRow {
  const id = String(pick(row, "id", "id") ?? "unknown");
  const payloadRaw = pick(row, "payloadJson", "payload_json");
  const payloadJson = normalizePayloadJson(payloadRaw, id);

  const chainId = coerceNumber(pick(row, "chainId", "chain_id"), "relay_job.chain_id");
  if (chainId === null) throw new Error(`Invalid relay_job.chain_id for job ${id}`);

  const createdAtBlockNumber = coerceBigint(
    pick(row, "createdAtBlockNumber", "created_at_block_number"),
    "relay_job.created_at_block_number"
  );
  const createdAtBlockTimestamp = coerceBigint(
    pick(row, "createdAtBlockTimestamp", "created_at_block_timestamp"),
    "relay_job.created_at_block_timestamp"
  );
  const updatedAtBlockNumber = coerceBigint(
    pick(row, "updatedAtBlockNumber", "updated_at_block_number"),
    "relay_job.updated_at_block_number"
  );
  const updatedAtBlockTimestamp = coerceBigint(
    pick(row, "updatedAtBlockTimestamp", "updated_at_block_timestamp"),
    "relay_job.updated_at_block_timestamp"
  );

  if (createdAtBlockNumber === null || createdAtBlockTimestamp === null) {
    throw new Error(`Invalid createdAt block fields for job ${id}`);
  }
  if (updatedAtBlockNumber === null || updatedAtBlockTimestamp === null) {
    throw new Error(`Invalid updatedAt block fields for job ${id}`);
  }

  return {
    id,
    chainId,
    createdAtBlockNumber,
    createdAtBlockTimestamp,
    kind: String(pick(row, "kind", "kind") ?? "") as RelayJobRow["kind"],
    status: String(pick(row, "status", "status") ?? "") as RelayJobRow["status"],
    attempts: coerceNumber(pick(row, "attempts", "attempts"), "relay_job.attempts") ?? 0,
    lockedAtBlockNumber: coerceBigint(
      pick(row, "lockedAtBlockNumber", "locked_at_block_number"),
      "relay_job.locked_at_block_number"
    ),
    lockedAtBlockTimestamp: coerceBigint(
      pick(row, "lockedAtBlockTimestamp", "locked_at_block_timestamp"),
      "relay_job.locked_at_block_timestamp"
    ),
    lockedBy: (pick(row, "lockedBy", "locked_by") as string | null | undefined) ?? null,
    updatedAtBlockNumber,
    updatedAtBlockTimestamp,
    lastError: (pick(row, "lastError", "last_error") as string | null | undefined) ?? null,
    nextRetryBlockNumber: coerceBigint(
      pick(row, "nextRetryBlockNumber", "next_retry_block_number"),
      "relay_job.next_retry_block_number"
    ),
    payloadJson,
  } satisfies RelayJobRow;
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
      const staleLockBlocks = 50n;
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
            AND (
              (
                "status" = 'pending'
                AND created_at_block_number <= ${eligibleBlock}
                AND (next_retry_block_number IS NULL OR next_retry_block_number <= ${args.headBlockNumber})
              )
              OR (
                "status" = 'processing'
                AND locked_by IS NOT NULL
                AND locked_by <> ${args.workerId}
                AND locked_at_block_number IS NOT NULL
                AND locked_at_block_number <= ${args.headBlockNumber} - ${staleLockBlocks}
              )
            )
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

      const rows = getRows(result) as Array<Record<string, unknown>>;
      return rows.map(normalizeRelayJobRow);
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
        lockedAtBlockNumber: null,
        lockedAtBlockTimestamp: null,
        lockedBy: null,
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
          lockedAtBlockNumber: null,
          lockedAtBlockTimestamp: null,
          lockedBy: null,
          updatedAtBlockNumber: args.headBlockNumber,
          updatedAtBlockTimestamp: args.headBlockTimestamp,
          nextRetryBlockNumber: isTerminal ? null : args.headBlockNumber + args.retryDelayBlocks,
        };
      }),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const markRelayJobRetryLater = (args: {
  context: PonderContext;
  id: string;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  errorMessage: string;
  retryDelayBlocks: bigint;
}) =>
  Effect.tryPromise({
    try: () =>
      args.context.db.update(relayJob, { id: args.id }).set({
        status: "pending",
        lastError: args.errorMessage,
        lockedAtBlockNumber: null,
        lockedAtBlockTimestamp: null,
        lockedBy: null,
        updatedAtBlockNumber: args.headBlockNumber,
        updatedAtBlockTimestamp: args.headBlockTimestamp,
        nextRetryBlockNumber: args.headBlockNumber + args.retryDelayBlocks,
      }),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });
