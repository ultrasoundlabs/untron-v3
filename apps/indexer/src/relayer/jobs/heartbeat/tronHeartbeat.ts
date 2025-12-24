import { Effect } from "effect";

import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";

import type { HeartbeatHandler } from "./types";
import { ensureIsEventChainTipCalled } from "./handlers/ensureIsEventChainTipCalled";
import { rebalancePulledUsdtIfOverThreshold } from "./handlers/rebalancePulledUsdt";
import { tronSweepTrxFromReceivers } from "./handlers/trxSweep";
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
      { name: "rebalance_pulled_usdt", effect: rebalancePulledUsdtIfOverThreshold() },
      { name: "ensure_is_event_chain_tip_called", effect: ensureIsEventChainTipCalled(ctx) },
    ];

    yield* runHeartbeatHandlers({ jobName: "tron heartbeat", handlers });
  });
