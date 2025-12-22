import { Effect } from "effect";
import type { Context as PonderContext, IndexingFunctionArgs } from "ponder:registry";

import { untronV3ClaimQueue, untronV3LeasePayoutConfig, untronV3SwapRate } from "ponder:schema";

import { IndexerRuntime } from "./effect/runtime";

type PonderRegistry = (typeof import("ponder:registry"))["ponder"];

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

function expectString(value: unknown, label: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`Invalid ${label}`);
  return value;
}

function expectHexAddress(value: unknown, label: string): `0x${string}` {
  const raw = expectString(value, label).toLowerCase();
  if (!/^0x[0-9a-f]{40}$/.test(raw))
    throw new Error(`Invalid ${label} (expected 0x + 40 hex chars)`);
  return raw as `0x${string}`;
}

function expectBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      // fall through
    }
  }
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

function getArgValue(args: unknown, index: number, name: string): unknown {
  if (args && typeof args === "object" && !Array.isArray(args) && name in args) {
    return (args as Record<string, unknown>)[name];
  }
  if (Array.isArray(args)) return args[index];
  return undefined;
}

function makeLeaseConfigId(args: {
  chainId: number;
  contractAddress: string;
  leaseId: bigint;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.leaseId.toString()}`;
}

function makeTokenStateId(args: {
  chainId: number;
  contractAddress: string;
  targetToken: string;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.targetToken.toLowerCase()}`;
}

export function registerUntronV3DerivedIndexer({ ponder }: { ponder: PonderRegistry }) {
  ponder.on(
    "UntronV3:PayoutConfigUpdated",
    ({ event, context }: IndexingFunctionArgs<"UntronV3:PayoutConfigUpdated">) =>
      IndexerRuntime.runPromise(
        Effect.gen(function* () {
          const args = expectRecord(event.args, "event.args");
          const chainId = context.chain.id;
          const contractAddress = expectHexAddress(event.log.address, "event.log.address");

          const leaseId = expectBigint(
            getArgValue(args, 0, "leaseId"),
            "PayoutConfigUpdated.leaseId"
          );
          const targetChainId = expectBigint(
            getArgValue(args, 1, "targetChainId"),
            "PayoutConfigUpdated.targetChainId"
          );
          const targetToken = expectHexAddress(
            getArgValue(args, 2, "targetToken"),
            "PayoutConfigUpdated.targetToken"
          );
          const beneficiary = expectHexAddress(
            getArgValue(args, 3, "beneficiary"),
            "PayoutConfigUpdated.beneficiary"
          );

          const id = makeLeaseConfigId({ chainId, contractAddress, leaseId });

          yield* Effect.tryPromise(() =>
            (context as PonderContext).db
              .insert(untronV3LeasePayoutConfig)
              .values({
                id,
                chainId,
                contractAddress,
                leaseId,
                targetChainId,
                targetToken,
                beneficiary,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
              .onConflictDoUpdate({
                targetChainId,
                targetToken,
                beneficiary,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
          );
        })
      )
  );

  ponder.on(
    "UntronV3:SwapRateSet",
    ({ event, context }: IndexingFunctionArgs<"UntronV3:SwapRateSet">) =>
      IndexerRuntime.runPromise(
        Effect.gen(function* () {
          const args = expectRecord(event.args, "event.args");
          const chainId = context.chain.id;
          const contractAddress = expectHexAddress(event.log.address, "event.log.address");

          const targetToken = expectHexAddress(
            getArgValue(args, 0, "targetToken"),
            "SwapRateSet.targetToken"
          );
          const ratePpm = expectBigint(getArgValue(args, 1, "ratePpm"), "SwapRateSet.ratePpm");

          const id = makeTokenStateId({ chainId, contractAddress, targetToken });

          yield* Effect.tryPromise(() =>
            (context as PonderContext).db
              .insert(untronV3SwapRate)
              .values({
                id,
                chainId,
                contractAddress,
                targetToken,
                ratePpm,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
              .onConflictDoUpdate({
                ratePpm,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
          );
        })
      )
  );

  ponder.on(
    "UntronV3:ClaimCreated",
    ({ event, context }: IndexingFunctionArgs<"UntronV3:ClaimCreated">) =>
      IndexerRuntime.runPromise(
        Effect.gen(function* () {
          const args = expectRecord(event.args, "event.args");
          const chainId = context.chain.id;
          const contractAddress = expectHexAddress(event.log.address, "event.log.address");

          const claimIndex = expectBigint(
            getArgValue(args, 0, "claimIndex"),
            "ClaimCreated.claimIndex"
          );
          const leaseId = expectBigint(getArgValue(args, 1, "leaseId"), "ClaimCreated.leaseId");
          expectBigint(getArgValue(args, 2, "amountUsdt"), "ClaimCreated.amountUsdt");

          const leaseConfigId = makeLeaseConfigId({ chainId, contractAddress, leaseId });
          const leaseConfig = yield* Effect.tryPromise(() =>
            (context as PonderContext).db.find(untronV3LeasePayoutConfig, { id: leaseConfigId })
          );
          if (!leaseConfig) {
            return yield* Effect.fail(
              new Error(`Missing lease payout config for leaseId=${leaseId.toString()}`)
            );
          }

          const targetToken = expectHexAddress(leaseConfig.targetToken, "leaseConfig.targetToken");
          const queueId = makeTokenStateId({ chainId, contractAddress, targetToken });

          const existing = yield* Effect.tryPromise(() =>
            (context as PonderContext).db.find(untronV3ClaimQueue, { id: queueId })
          );

          const nextMaxClaimIndex =
            existing && existing.maxClaimIndex > claimIndex ? existing.maxClaimIndex : claimIndex;
          const queueLength = nextMaxClaimIndex + 1n;

          yield* Effect.tryPromise(() =>
            (context as PonderContext).db
              .insert(untronV3ClaimQueue)
              .values({
                id: queueId,
                chainId,
                contractAddress,
                targetToken,
                maxClaimIndex: nextMaxClaimIndex,
                queueLength,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
              .onConflictDoUpdate({
                maxClaimIndex: nextMaxClaimIndex,
                queueLength,
                updatedAtBlockNumber: event.block.number,
                updatedAtBlockTimestamp: event.block.timestamp,
                updatedAtTransactionHash: event.transaction.hash,
                updatedAtLogIndex: event.log.logIndex,
              })
          );
        })
      )
  );
}
