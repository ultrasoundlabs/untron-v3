import type { Context as PonderContext } from "ponder:registry";
import { expectAddress, expectBigint, expectHex, expectRecord, expectString } from "../../parse";

export type RelayJobHandlerContext = {
  ponderContext: PonderContext;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
  dryRun: boolean;
};

export { expectAddress, expectBigint, expectHex, expectRecord, expectString };
