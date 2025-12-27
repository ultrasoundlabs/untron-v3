import { Effect } from "effect";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";

import { tronLightClientCheckpoint, tronLightClientConfig } from "ponder:schema";

import { tryPromise } from "./effect/tryPromise";

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;

type TronLightClientDerivedEventName = "TronBlockStored" | "TronLightClientConfigured";

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

function expectHex(value: unknown, label: string): `0x${string}` {
  if (typeof value !== "string" || !value.startsWith("0x")) throw new Error(`Invalid ${label}`);
  return value.toLowerCase() as `0x${string}`;
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

export const handleTronLightClientDerivedEvent = (args: {
  eventName: TronLightClientDerivedEventName;
  event: PonderLogEvent;
  context: PonderContext;
}) =>
  Effect.gen(function* () {
    const parsedArgs = expectRecord(args.event.args, "event.args");

    const chainId = args.context.chain.id;
    const contractAddress = expectHex(args.event.log.address, "event.log.address");

    if (args.eventName === "TronLightClientConfigured") {
      const srDataHash = expectHex(getArgValue(parsedArgs, 1, "srDataHash"), "srDataHash");
      const initialBlockId = expectHex(
        getArgValue(parsedArgs, 2, "initialBlockId"),
        "initialBlockId"
      );
      const srs = getArgValue(parsedArgs, 5, "srs");
      const witnessDelegatees = getArgValue(parsedArgs, 6, "witnessDelegatees");

      if (!Array.isArray(srs) || srs.length !== 27) {
        throw new Error("Invalid TronLightClientConfigured.srs (expected array length 27)");
      }
      if (!Array.isArray(witnessDelegatees) || witnessDelegatees.length !== 27) {
        throw new Error(
          "Invalid TronLightClientConfigured.witnessDelegatees (expected array length 27)"
        );
      }

      const srsNormalized = srs.map((v, i) => expectHex(v, `srs[${i}]`));
      const witnessDelegateesNormalized = witnessDelegatees.map((v, i) =>
        expectHex(v, `witnessDelegatees[${i}]`)
      );

      const id = `${chainId}:${contractAddress}`;

      yield* tryPromise(() =>
        args.context.db
          .insert(tronLightClientConfig)
          .values({
            id,
            chainId,
            contractAddress,
            srDataHash,
            initialBlockId,
            srsJson: JSON.stringify(srsNormalized),
            witnessDelegateesJson: JSON.stringify(witnessDelegateesNormalized),
            configuredAtBlockNumber: args.event.block.number,
            configuredAtBlockTimestamp: args.event.block.timestamp,
            configuredAtTransactionHash: args.event.transaction.hash,
            configuredAtLogIndex: args.event.log.logIndex,
          })
          .onConflictDoUpdate({
            srDataHash,
            initialBlockId,
            srsJson: JSON.stringify(srsNormalized),
            witnessDelegateesJson: JSON.stringify(witnessDelegateesNormalized),
            configuredAtBlockNumber: args.event.block.number,
            configuredAtBlockTimestamp: args.event.block.timestamp,
            configuredAtTransactionHash: args.event.transaction.hash,
            configuredAtLogIndex: args.event.log.logIndex,
          })
      );

      return;
    }

    if (args.eventName !== "TronBlockStored") return;

    const tronBlockNumber = expectBigint(
      getArgValue(parsedArgs, 0, "blockNumber"),
      "TronBlockStored.blockNumber"
    );
    const tronBlockId = expectHex(getArgValue(parsedArgs, 1, "blockId"), "TronBlockStored.blockId");
    const tronTxTrieRoot = expectHex(
      getArgValue(parsedArgs, 2, "txTrieRoot"),
      "TronBlockStored.txTrieRoot"
    );
    const tronBlockTimestamp = expectBigint(
      getArgValue(parsedArgs, 3, "timestamp"),
      "TronBlockStored.timestamp"
    );

    const id = `${chainId}:${contractAddress}:${tronBlockNumber.toString()}`;

    yield* tryPromise(() =>
      args.context.db
        .insert(tronLightClientCheckpoint)
        .values({
          id,
          chainId,
          contractAddress,
          tronBlockNumber,
          tronBlockId,
          tronTxTrieRoot,
          tronBlockTimestamp,
          storedAtBlockNumber: args.event.block.number,
          storedAtBlockTimestamp: args.event.block.timestamp,
          storedAtTransactionHash: args.event.transaction.hash,
          storedAtLogIndex: args.event.log.logIndex,
        })
        .onConflictDoUpdate({
          tronBlockId,
          tronTxTrieRoot,
          tronBlockTimestamp,
          storedAtBlockNumber: args.event.block.number,
          storedAtBlockTimestamp: args.event.block.timestamp,
          storedAtTransactionHash: args.event.transaction.hash,
          storedAtLogIndex: args.event.log.logIndex,
        })
    );
  });
