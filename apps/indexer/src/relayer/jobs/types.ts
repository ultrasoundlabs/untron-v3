import type { Context as PonderContext } from "ponder:registry";

import type { RelayerDeps } from "../deps";
import type { RelayJobKind, RelayJobRow } from "../types";

export type RelayJobHandlerContext = {
  ponderContext: PonderContext;
  deps: RelayerDeps;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  dryRun: boolean;
};

export type RelayJobHandler<K extends RelayJobKind> = (args: {
  job: RelayJobRow & { kind: K };
  ctx: RelayJobHandlerContext;
}) => Promise<void>;
