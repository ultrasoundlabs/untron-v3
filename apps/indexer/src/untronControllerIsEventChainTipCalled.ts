import { Effect } from "effect";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";
import { untronControllerIsEventChainTipCalled } from "ponder:schema";

import { tryPromise } from "./effect/tryPromise";

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;

export const handleUntronControllerIsEventChainTipCalled = (args: {
  event: PonderLogEvent;
  context: PonderContext;
}) =>
  Effect.gen(function* () {
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
  });
