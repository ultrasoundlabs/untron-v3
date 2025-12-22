import { Effect } from "effect";
import { MainnetRelayer } from "../../../deps/mainnet";
import type { RelayJobHandlerContext } from "../../types";
import { buildMainnetFillCalls } from "../../../claimFiller/buildMainnetFillCalls";

export const fillClaimsFromUntronBalance = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const relayer = yield* MainnetRelayer;
    const calls = yield* buildMainnetFillCalls(ctx);
    if (calls.length === 0) return;
    yield* relayer.sendUserOperation({ calls });
  });
