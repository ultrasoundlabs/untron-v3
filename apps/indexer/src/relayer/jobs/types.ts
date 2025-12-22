import type { Context as PonderContext } from "ponder:registry";

export type RelayJobHandlerContext = {
  ponderContext: PonderContext;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  dryRun: boolean;
};
