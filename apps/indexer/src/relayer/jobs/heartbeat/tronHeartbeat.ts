import { Effect } from "effect";

import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";

import type { HeartbeatHandler } from "./types";
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
    ];

    yield* runHeartbeatHandlers({ jobName: "tron heartbeat", handlers });
  });
