import { Effect, Option } from "effect";
import { tronRebalanceUsdtSent } from "ponder:schema";

import { AppConfig } from "../../../../effect/config";
import { tryPromise } from "../../../../effect/tryPromise";
import { TronRelayer } from "../../../deps/tron";
import type { RelayJobHandlerContext } from "../../types";

const MIN_REBALANCE_AMOUNT = 2n;
const DUST_LEFT_BEHIND = 1n;

export const rebalancePulledUsdtIfOverThreshold = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const tronConfig = yield* AppConfig.tronNetwork();

    const threshold = Option.getOrUndefined(tronConfig.rebalancePulledUsdtThreshold);
    const rebalancer = Option.getOrUndefined(tronConfig.rebalanceRebalancerAddress);
    if (threshold == null || rebalancer == null) return;

    const chainId = ctx.ponderContext.chain.id;
    const controllerAddress = (
      ctx.ponderContext.contracts.UntronController.address as `0x${string}`
    ).toLowerCase() as `0x${string}`;

    const pulledUsdt = yield* TronRelayer.getControllerPulledUsdt();
    if (pulledUsdt <= threshold) return;
    if (pulledUsdt < MIN_REBALANCE_AMOUNT) return;

    const inAmount = pulledUsdt - DUST_LEFT_BEHIND;
    if (inAmount <= 0n) return;

    const lastSent = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronRebalanceUsdtSent, { id: `${chainId}:${controllerAddress}` })
    );
    if (lastSent && lastSent.pulledUsdt === pulledUsdt && lastSent.inAmount === inAmount) {
      return;
    }

    const { txid } = yield* TronRelayer.sendTronControllerRebalanceUsdt({
      rebalancer,
      inAmount,
    });

    yield* tryPromise(() =>
      ctx.ponderContext.db
        .insert(tronRebalanceUsdtSent)
        .values({
          id: `${chainId}:${controllerAddress}`,
          chainId,
          contractAddress: controllerAddress,
          pulledUsdt,
          inAmount,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
        .onConflictDoUpdate({
          pulledUsdt,
          inAmount,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
    );
  });
