import { ponder } from "ponder:registry";
import { Effect } from "effect";

import { tronLightClientAbi, untronControllerAbi, untronV3Abi } from "@untron/v3-contracts";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";
import {
  tronLightClientCheckpoint,
  tronLightClientConfig,
  tronLightClientPublishRequest,
  untronControllerIsEventChainTipCalled,
} from "ponder:schema";

import { registerEventChainIndexer } from "./eventChainIndexer";
import { getTronLightClientAddress } from "./contracts";
import { AppConfig } from "./effect/config";
import { tryPromise } from "./effect/tryPromise";
import { MAINNET_CHAIN_ID } from "./env";
import { expectBigint, expectHex, expectRecord, getArgValue } from "./parse";
import { registerRelayer } from "./relayer/register";
import { enqueueRelayJob } from "./relayer/queue";
import { handleUntronV3DerivedEvent } from "./untronV3DerivedIndexer";

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;

type TronLightClientDerivedEventName = "TronBlockStored" | "TronLightClientConfigured";

registerEventChainIndexer({
  ponder,
  contractName: "UntronV3",
  indexName: "UntronV3Index",
  abi: untronV3Abi,
  afterEvent: ({ eventName, event, context }) =>
    handleUntronV3DerivedEvent({ eventName, event, context }),
});

registerEventChainIndexer({
  ponder,
  contractName: "TronLightClient",
  indexName: "TronLightClientIndex",
  abi: tronLightClientAbi,
  afterEvent: ({ eventName, event, context }) =>
    eventName === "TronBlockStored" || eventName === "TronLightClientConfigured"
      ? handleTronLightClientDerivedEvent({
          eventName: eventName as any,
          event,
          context,
        })
      : Effect.void,
});

registerEventChainIndexer({
  ponder,
  contractName: "UntronController",
  indexName: "UntronControllerIndex",
  abi: untronControllerAbi,
  onchainTipValidation: "head",
  skipTipUpdateEvents: ["IsEventChainTipCalled" as any],
  afterEvent: ({ eventName, event, context }) =>
    eventName === ("IsEventChainTipCalled" as any)
      ? handleUntronControllerIsEventChainTipCalled({ event, context })
      : Effect.void,
});

registerRelayer({ ponder });

const handleTronLightClientDerivedEvent = (args: {
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

const handleUntronControllerIsEventChainTipCalled = (args: {
  event: PonderLogEvent;
  context: PonderContext;
}) =>
  Effect.gen(function* () {
    const relayerRuntime = yield* AppConfig.relayerRuntime();

    const chainId = args.context.chain.id;
    const contractAddress = (
      args.event.log.address as `0x${string}`
    ).toLowerCase() as `0x${string}`;
    const transactionHash = (
      args.event.transaction.hash as `0x${string}`
    ).toLowerCase() as `0x${string}`;
    const logIndex = args.event.log.logIndex as number;

    const caller = (args.event.args as any)?.caller as `0x${string}` | undefined;
    const eventChainTip = (args.event.args as any)?.eventChainTip as `0x${string}` | undefined;
    if (!caller || !eventChainTip) return;

    yield* tryPromise(() =>
      args.context.db
        .insert(untronControllerIsEventChainTipCalled)
        .values({
          id: `${chainId}:${contractAddress}:${transactionHash}:${logIndex}`,
          chainId,
          contractAddress: contractAddress as `0x${string}`,
          caller,
          eventChainTip,
          blockNumber: args.event.block.number,
          blockTimestamp: args.event.block.timestamp,
          transactionHash,
          logIndex,
        })
        .onConflictDoNothing()
    );

    if (!relayerRuntime.enabled) return;

    const tronLightClientAddress = getTronLightClientAddress() as `0x${string}`;
    yield* tryPromise(() =>
      args.context.db
        .insert(tronLightClientPublishRequest)
        .values({
          id: `${MAINNET_CHAIN_ID}:${tronLightClientAddress}:${args.event.block.number.toString()}`,
          chainId: MAINNET_CHAIN_ID,
          tronLightClientAddress,
          tronBlockNumber: args.event.block.number,
          requestedAtTronBlockTimestamp: args.event.block.timestamp,
          source: "relay_controller_event_chain",
        })
        .onConflictDoNothing()
    );

    yield* enqueueRelayJob({
      context: args.context,
      id: `${chainId}:relay_controller_event_chain:${transactionHash}:${logIndex}`,
      chainId,
      createdAtBlockNumber: args.event.block.number,
      createdAtBlockTimestamp: args.event.block.timestamp,
      kind: "relay_controller_event_chain",
      payloadJson: {
        controllerAddress: contractAddress,
        tronBlockNumber: args.event.block.number.toString(),
        transactionHash,
        logIndex,
        eventChainTip,
      },
    });
  });
