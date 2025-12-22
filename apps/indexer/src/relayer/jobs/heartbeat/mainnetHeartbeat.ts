import { Effect } from "effect";
import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";

import type { HeartbeatHandler } from "./types";
import { fillClaimsFromUntronBalance } from "./handlers/fillClaimsFromUntronBalance";
import { sweepTronReceiversIfPendingClaims } from "./handlers/sweepTronReceiversIfPendingClaims";
import { runHeartbeatHandlers } from "./runHeartbeatHandlers";

export const handleMainnetHeartbeat = (_args: {
  job: RelayJobRow & { kind: "mainnet_heartbeat" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    const { ctx } = _args;

    if (ctx.dryRun) return;

    const handlers: ReadonlyArray<HeartbeatHandler> = [
      {
        name: "fill_claims_from_untron_balance",
        effect: fillClaimsFromUntronBalance(ctx),
      },
      {
        name: "sweep_tron_receivers_if_pending_claims",
        effect: sweepTronReceiversIfPendingClaims(ctx),
      },
    ];

    yield* runHeartbeatHandlers({ jobName: "mainnet heartbeat", handlers });
  });
