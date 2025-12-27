import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";

import { relayerStatus } from "ponder:schema";

export const getRpcHeadBlockNumber = (
  context: PonderContext
): Effect.Effect<bigint | null, Error> =>
  Effect.tryPromise({
    try: async () => {
      const hex = (await context.client.request({
        method: "eth_blockNumber",
      } as any)) as unknown;

      if (typeof hex !== "string") return null;
      return BigInt(hex);
    },
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const isProbablyLiveEvent = (args: {
  context: PonderContext;
  eventBlockNumber: bigint;
  maxLagBlocks: bigint;
}): Effect.Effect<boolean, Error> =>
  Effect.gen(function* () {
    const status = yield* Effect.tryPromise({
      try: () => args.context.db.find(relayerStatus, { chainId: args.context.chain.id }),
      catch: (error) => (error instanceof Error ? error : new Error(String(error))),
    });

    const head =
      status?.isLive === true && typeof status.headBlockNumber === "bigint"
        ? status.headBlockNumber
        : yield* getRpcHeadBlockNumber(args.context);

    if (head === null) return false;
    if (head < args.eventBlockNumber) return false;
    return head - args.eventBlockNumber <= args.maxLagBlocks;
  });
