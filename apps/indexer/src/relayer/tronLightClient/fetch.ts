import { Effect } from "effect";
import type { TronBlockForLightClient } from "../tronProofs";

import { tryPromise } from "../../effect/tryPromise";
import { fetchTronBlockByNum } from "../deps/tron";
import { parseTronBlockForLightClient } from "../tronProofs";

export const fetchTronBlocksForLightClient = (args: {
  wallet: any;
  metadata: unknown;
  rangeStart: bigint;
  rangeEnd: bigint;
  concurrency: number;
}): Effect.Effect<readonly TronBlockForLightClient[], Error> =>
  Effect.gen(function* () {
    if (args.rangeEnd < args.rangeStart) {
      return yield* Effect.fail(
        new Error("fetchTronBlocksForLightClient: expected rangeEnd >= rangeStart")
      );
    }

    const nums: bigint[] = [];
    for (let n = args.rangeStart; n <= args.rangeEnd; n++) nums.push(n);

    return yield* Effect.forEach(
      nums,
      (blockNumber) =>
        tryPromise(() =>
          fetchTronBlockByNum({
            wallet: args.wallet,
            metadata: args.metadata,
            blockNumber,
            timeoutMs: 60_000,
            retries: 2,
          })
        ).pipe(Effect.map(parseTronBlockForLightClient)),
      { concurrency: args.concurrency }
    );
  });
