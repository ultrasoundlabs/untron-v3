import { Effect, Option } from "effect";

import { AppConfig } from "../../../../effect/config";
import { TronRelayer } from "../../../deps/tron";

const MIN_REBALANCE_AMOUNT = 2n;
const DUST_LEFT_BEHIND = 1n;

export const rebalancePulledUsdtIfOverThreshold = () =>
  Effect.gen(function* () {
    const tronConfig = yield* AppConfig.tronNetwork();

    const threshold = Option.getOrUndefined(tronConfig.rebalancePulledUsdtThreshold);
    const rebalancer = Option.getOrUndefined(tronConfig.rebalanceRebalancerAddress);
    if (threshold == null || rebalancer == null) return;

    const pulledUsdt = yield* TronRelayer.getControllerPulledUsdt();
    if (pulledUsdt <= threshold) return;
    if (pulledUsdt < MIN_REBALANCE_AMOUNT) return;

    const inAmount = pulledUsdt - DUST_LEFT_BEHIND;
    if (inAmount <= 0n) return;

    yield* TronRelayer.sendTronControllerRebalanceUsdt({
      rebalancer,
      inAmount,
    });
  });
