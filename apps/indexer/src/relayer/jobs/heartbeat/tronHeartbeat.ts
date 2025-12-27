import { Cause, Effect, Option } from "effect";
import { sql } from "ponder";
import {
  eventChainState,
  tronIsEventChainTipSent,
  tronPullFromReceiversSent,
  tronRebalanceUsdtSent,
} from "ponder:schema";
import type { Address } from "viem";
import { encodeAbiParameters, keccak256 } from "viem";

import { AppConfig } from "../../../effect/config";
import { tryPromise } from "../../../effect/tryPromise";
import { TronRelayer } from "../../deps/tron";
import { getRows } from "../../sqlRows";
import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";
import { publishTronLightClient } from "./handlers/publishTronLightClient";
import type { HeartbeatHandler } from "./types";
import { runHeartbeatHandlers } from "./runHeartbeatHandlers";

const TRX_TOKEN_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;
const RATE_SCALE = 1_000_000_000_000_000_000n; // 1e18

const MIN_REBALANCE_AMOUNT = 2n;
const DUST_LEFT_BEHIND = 1n;

type SweepCandidate = {
  readonly receiverSalt: `0x${string}`;
  readonly usdtCost: bigint;
};

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

const tronSweepTrxFromReceivers = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const receiverMap = yield* TronRelayer.getReceiverMap();
    if (receiverMap.size === 0) return;

    const usdtAddress = yield* TronRelayer.getControllerUsdt();
    if (usdtAddress.toLowerCase() === TRX_TOKEN_ADDRESS) return;

    const entries = Array.from(receiverMap.values());
    const controllerAddress = yield* TronRelayer.getControllerEvmAddress();
    const pulledUsdt = yield* TronRelayer.getControllerPulledUsdt();
    const controllerUsdtBalance = yield* TronRelayer.getErc20BalanceOf({
      tokenAddress: usdtAddress,
      account: controllerAddress,
    });
    if (controllerUsdtBalance < pulledUsdt) {
      return yield* Effect.fail(
        new Error(
          `Tron controller USDT balance (${controllerUsdtBalance}) is below pulledUsdt (${pulledUsdt})`
        )
      );
    }

    const lpFreeUsdt = controllerUsdtBalance - pulledUsdt;
    if (lpFreeUsdt === 0n) return;
    const trxToUsdtRate = yield* TronRelayer.getControllerLpExchangeRateFor({
      tokenAddress: TRX_TOKEN_ADDRESS,
    });
    if (trxToUsdtRate === 0n) return;

    const candidates = yield* Effect.forEach(
      entries,
      (entry) =>
        TronRelayer.getTrxBalanceOf({ account: entry.receiverAddress }).pipe(
          Effect.map((balance) => {
            const sweepAmount = balance > 0n ? balance - 1n : 0n;
            if (sweepAmount === 0n) return null;

            const usdtCost = (sweepAmount * trxToUsdtRate) / RATE_SCALE;
            return { receiverSalt: entry.receiverSalt, usdtCost };
          })
        ),
      { concurrency: 20 }
    ).pipe(Effect.map((items) => items.filter((item): item is SweepCandidate => item !== null)));

    if (candidates.length === 0) return;

    candidates.sort((a, b) => (a.usdtCost === b.usdtCost ? 0 : a.usdtCost < b.usdtCost ? -1 : 1));

    const receiverSalts: Array<`0x${string}`> = [];
    let remainingUsdt = lpFreeUsdt;
    for (const candidate of candidates) {
      if (candidate.usdtCost > remainingUsdt) continue;
      receiverSalts.push(candidate.receiverSalt);
      remainingUsdt -= candidate.usdtCost;
    }

    if (receiverSalts.length === 0) return;

    const chainId = ctx.ponderContext.chain.id;
    const controllerContractAddress = controllerAddress.toLowerCase() as `0x${string}`;
    const tokenAddress = TRX_TOKEN_ADDRESS.toLowerCase() as `0x${string}`;

    const receiverSaltsSorted = [...receiverSalts].sort((a, b) =>
      a.toLowerCase().localeCompare(b.toLowerCase())
    );
    const receiverSaltsHash = keccak256(
      encodeAbiParameters(
        [{ type: "bytes32[]" }],
        [receiverSaltsSorted as readonly `0x${string}`[]]
      )
    ) as `0x${string}`;

    const id = `${chainId}:${controllerContractAddress}:${tokenAddress}`;
    const lastSent = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronPullFromReceiversSent, {
        id,
      })
    );
    if (lastSent && lastSent.receiverSaltsHash.toLowerCase() === receiverSaltsHash.toLowerCase()) {
      return;
    }

    const { txid } = yield* TronRelayer.sendTronControllerPullFromReceivers({
      tokenAddress: TRX_TOKEN_ADDRESS,
      receiverSalts: receiverSaltsSorted,
    });

    yield* tryPromise(() =>
      ctx.ponderContext.db
        .insert(tronPullFromReceiversSent)
        .values({
          id,
          chainId,
          contractAddress: controllerContractAddress,
          tokenAddress,
          receiverSaltsHash,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
        .onConflictDoUpdate({
          receiverSaltsHash,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
    );
  });

const rebalancePulledUsdtIfOverThreshold = (ctx: RelayJobHandlerContext) =>
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

const ensureIsEventChainTipCalled = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const controllerAddress = (
      ctx.ponderContext.contracts.UntronController.address as `0x${string}`
    ).toLowerCase() as `0x${string}`;

    const state = yield* tryPromise(() =>
      ctx.ponderContext.db.find(eventChainState, {
        id: `${chainId}:UntronController:${controllerAddress}`,
      })
    );
    if (!state) return;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          event_chain_tip AS "eventChainTip"
        FROM "untron_controller_is_event_chain_tip_called"
        WHERE chain_id = ${chainId}
          AND contract_address = ${controllerAddress}
        ORDER BY block_number DESC,
          log_index DESC
        LIMIT 1;
      `)
    );

    const rows = getRows(result) as Array<{ eventChainTip: `0x${string}` }>;
    const lastCalledTip = rows[0]?.eventChainTip;
    if (lastCalledTip && lastCalledTip.toLowerCase() === state.eventChainTip.toLowerCase()) {
      return;
    }

    const lastSent = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronIsEventChainTipSent, {
        id: `${chainId}:${controllerAddress}`,
      })
    );
    if (lastSent && lastSent.eventChainTip.toLowerCase() === state.eventChainTip.toLowerCase()) {
      return;
    }

    const onchainTip = yield* TronRelayer.getControllerEventChainTip();
    if (onchainTip.toLowerCase() !== state.eventChainTip.toLowerCase()) return;

    const { txid } = yield* TronRelayer.sendTronControllerIsEventChainTip();

    yield* tryPromise(() =>
      ctx.ponderContext.db
        .insert(tronIsEventChainTipSent)
        .values({
          id: `${chainId}:${controllerAddress}`,
          chainId,
          contractAddress: controllerAddress,
          eventChainTip: onchainTip,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
        .onConflictDoUpdate({
          eventChainTip: onchainTip,
          txid: `0x${txid}` as `0x${string}`,
          confirmedAtBlockNumber: ctx.headBlockNumber,
          confirmedAtBlockTimestamp: ctx.headBlockTimestamp,
        })
    );
  });
