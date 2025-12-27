import { Cause, Effect } from "effect";

import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";

import type { HeartbeatHandler } from "./types";
import { ensureIsEventChainTipCalled } from "./handlers/ensureIsEventChainTipCalled";
import { rebalancePulledUsdtIfOverThreshold } from "./handlers/rebalancePulledUsdt";
import { tronSweepTrxFromReceivers } from "./handlers/trxSweep";
import { publishTronLightClient } from "./handlers/publishTronLightClient";
import { runHeartbeatHandlers } from "./runHeartbeatHandlers";

export const handleTronHeartbeat = ({
  ctx,
}: {
  job: RelayJobRow & { kind: "tron_heartbeat" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const handlers: ReadonlyArray<HeartbeatHandler> = [
      { name: "sweep_tron_receivers_trx", effect: tronSweepTrxFromReceivers(ctx) },
      { name: "rebalance_pulled_usdt", effect: rebalancePulledUsdtIfOverThreshold(ctx) },
      { name: "ensure_is_event_chain_tip_called", effect: ensureIsEventChainTipCalled(ctx) },
      {
        name: "publish_tron_light_client",
        effect: publishTronLightClient(ctx).pipe(
          Effect.catchAllCause((cause) =>
            Effect.logError("[tron_light_client] publish failed").pipe(
              Effect.annotateLogs({ cause: Cause.pretty(cause) })
            )
          )
        ),
      },
    ];

    yield* runHeartbeatHandlers({ jobName: "tron heartbeat", handlers });
  });
