import { Cause, Effect, Option } from "effect";
import { sql } from "ponder";
import {
  eventChainState,
  tronLightClientPublishRequest,
  tronIsEventChainTipSent,
  tronPullFromReceiversSent,
  tronRebalanceUsdtSent,
  untronV3LastReceiverPull,
} from "ponder:schema";
import type { Address } from "viem";
import { encodeAbiParameters, keccak256 } from "viem";

import { AppConfig } from "../../../effect/config";
import { tryPromise } from "../../../effect/tryPromise";
import { getTronLightClientAddress, getUntronV3Address } from "../../../contracts";
import { MAINNET_CHAIN_ID } from "../../../env";
import { TronRelayer } from "../../deps/tron";
import { getRows } from "../../sqlRows";
import type { RelayJobRow } from "../../types";
import { expectBigint, type RelayJobHandlerContext } from "../types";
import { publishTronLightClient } from "../../tronLightClient";
import { getCheckpointAtOrAbove } from "../../tronLightClient/repo";
import type { HeartbeatHandler } from "./types";
import { runHeartbeatHandlers } from "./runHeartbeatHandlers";
import { enqueueRelayJob } from "../../queue";

const TRX_TOKEN_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;
const RATE_SCALE = 1_000_000_000_000_000_000n; // 1e18

const MIN_REBALANCE_AMOUNT = 2n;
const DUST_LEFT_BEHIND = 1n;
const RESEND_IS_EVENT_CHAIN_TIP_IF_TLC_LAG_BLOCKS = 1000n;

function coerceBigint(value: unknown): bigint | null {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      return null;
    }
  }
  return null;
}

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
      { name: "enqueue_missing_trc20_transfers", effect: enqueueMissingTrc20TransferJobs(ctx) },
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

const enqueueMissingTrc20TransferJobs = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const runtime = yield* AppConfig.relayerRuntime();
    const limit = Math.max(0, Math.min(runtime.claimLimit, 25));
    if (limit === 0) return;

    const tronChainId = ctx.ponderContext.chain.id;
    const tronLightClientAddress = getTronLightClientAddress().toLowerCase() as `0x${string}`;
    const untronV3Address = getUntronV3Address().toLowerCase() as `0x${string}`;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        WITH candidates AS (
          SELECT
            t.token_address AS "tokenAddress",
            t."from" AS "from",
            t."to" AS "to",
            t.value AS "value",
            t.block_number AS "blockNumber",
            t.block_timestamp AS "blockTimestamp",
            lower(t.transaction_hash) AS "transactionHash",
            t.log_index AS "logIndex",
            (t.chain_id::text || ':trc20_transfer:' || lower(t.transaction_hash) || ':' || t.log_index::text) AS "jobId"
          FROM "trc20_transfer" t
          WHERE t.chain_id = ${tronChainId}
            AND NOT EXISTS (
              SELECT 1
              FROM "untron_v3_deposit_preentitled" d
              WHERE d.chain_id = ${MAINNET_CHAIN_ID}
                AND lower(d.contract_address) = ${untronV3Address}
                AND lower(d.tx_id) = lower(t.transaction_hash)
            )
            AND NOT EXISTS (
              SELECT 1
              FROM "untron_v3_event" u
              WHERE u.chain_id = ${MAINNET_CHAIN_ID}
                AND lower(u.contract_address) = ${untronV3Address}
                AND u.event_name = 'DepositPreEntitled'
                AND lower(((u.args_json)::jsonb ->> 'txId')) = lower(t.transaction_hash)
            )
            AND NOT EXISTS (
              SELECT 1
              FROM "relay_job" j
              WHERE j.id = (t.chain_id::text || ':trc20_transfer:' || lower(t.transaction_hash) || ':' || t.log_index::text)
            )
          ORDER BY t.block_number ASC, t.log_index ASC
          LIMIT ${limit}
        )
        SELECT * FROM candidates;
      `)
    );

    const rows = getRows(result) as Array<{
      jobId: unknown;
      tokenAddress: unknown;
      from: unknown;
      to: unknown;
      value: unknown;
      blockNumber: unknown;
      blockTimestamp: unknown;
      transactionHash: unknown;
      logIndex: unknown;
    }>;

    if (rows.length === 0) return;

    const receiverMap = yield* TronRelayer.getReceiverMap();
    const controllerUsdt = (yield* TronRelayer.getControllerUsdt()).toLowerCase() as `0x${string}`;

    let skippedTooOld = 0;
    let enqueued = 0;

    for (const row of rows) {
      const jobId = String(row.jobId);
      const tokenAddress = String(row.tokenAddress) as `0x${string}`;
      const from = String(row.from) as `0x${string}`;
      const to = String(row.to) as `0x${string}`;
      const value = expectBigint(row.value, "value");
      const blockNumber = expectBigint(row.blockNumber, "blockNumber");
      const blockTimestamp = expectBigint(row.blockTimestamp, "blockTimestamp");
      const transactionHash = String(row.transactionHash) as `0x${string}`;
      const logIndex = Number(row.logIndex);

      const receiver = receiverMap.get(to.toLowerCase());
      if (!receiver) continue;

      const isControllerUsdt = tokenAddress.toLowerCase() === controllerUsdt;
      if (isControllerUsdt) {
        const lastPull = yield* tryPromise(() =>
          ctx.ponderContext.db.find(untronV3LastReceiverPull, {
            id: `${MAINNET_CHAIN_ID}:${untronV3Address}:${receiver.receiverSalt.toLowerCase()}:${tokenAddress.toLowerCase()}`,
          })
        );
        const lastPullTs = lastPull ? coerceBigint(lastPull.lastPullTronBlockTimestamp) : null;
        if (lastPullTs !== null && blockTimestamp <= lastPullTs) {
          skippedTooOld++;
          // Mark as done so the backfill loop doesn't keep reconsidering it.
          yield* enqueueRelayJob({
            context: ctx.ponderContext,
            id: jobId,
            chainId: tronChainId,
            createdAtBlockNumber: blockNumber,
            createdAtBlockTimestamp: blockTimestamp,
            kind: "trc20_transfer",
            status: "sent",
            payloadJson: {
              tokenAddress,
              from,
              to,
              value: value.toString(),
              transactionHash,
              logIndex,
              blockNumber: blockNumber.toString(),
              receiverSalt: receiver.receiverSalt,
              skippedReason: "deposit_at_or_before_last_receiver_pull",
              lastPullTronBlockTimestamp: lastPullTs.toString(),
            },
          });
          continue;
        }

        yield* tryPromise(() =>
          ctx.ponderContext.db
            .insert(tronLightClientPublishRequest)
            .values({
              id: `${MAINNET_CHAIN_ID}:${tronLightClientAddress}:${blockNumber.toString()}`,
              chainId: MAINNET_CHAIN_ID,
              tronLightClientAddress,
              tronBlockNumber: blockNumber,
              requestedAtTronBlockTimestamp: blockTimestamp,
              source: "trc20_transfer_backfill",
            })
            .onConflictDoNothing()
        );
      }

      yield* enqueueRelayJob({
        context: ctx.ponderContext,
        id: jobId,
        chainId: tronChainId,
        createdAtBlockNumber: blockNumber,
        createdAtBlockTimestamp: blockTimestamp,
        kind: "trc20_transfer",
        payloadJson: {
          tokenAddress,
          from,
          to,
          value: value.toString(),
          transactionHash,
          logIndex,
          blockNumber: blockNumber.toString(),
        },
      });

      enqueued++;
    }

    if (enqueued === 0 && skippedTooOld === 0) return;

    yield* Effect.logInfo("[tron_heartbeat] processed missing TRC20 transfers").pipe(
      Effect.annotateLogs({
        chainId: tronChainId,
        enqueued: enqueued.toString(),
        skippedTooOld: skippedTooOld.toString(),
        oldestBlockNumber: String(rows[0]?.blockNumber ?? ""),
        newestBlockNumber: String(rows[rows.length - 1]?.blockNumber ?? ""),
      })
    );
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
    const tronLightClientAddress = getTronLightClientAddress();

    const state = yield* tryPromise(() =>
      ctx.ponderContext.db.find(eventChainState, {
        id: `${chainId}:UntronController:${controllerAddress}`,
      })
    );
    if (!state) return;

    const lastSent = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronIsEventChainTipSent, {
        id: `${chainId}:${controllerAddress}`,
      })
    );

    const lastCalledForTip = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          block_number AS "blockNumber"
        FROM "untron_controller_is_event_chain_tip_called"
        WHERE chain_id = ${chainId}
          AND contract_address = ${controllerAddress}
          AND event_chain_tip = ${state.eventChainTip}
        ORDER BY block_number DESC,
          log_index DESC
        LIMIT 1;
      `)
    );

    const lastCalledRows = getRows(lastCalledForTip) as Array<{ blockNumber: unknown }>;
    const lastCalledBlockNumber = coerceBigint(lastCalledRows[0]?.blockNumber);

    const lastSentBlockNumber =
      lastSent && lastSent.eventChainTip.toLowerCase() === state.eventChainTip.toLowerCase()
        ? coerceBigint(lastSent.confirmedAtBlockNumber)
        : null;

    const bestKnownCallBlockNumber =
      lastCalledBlockNumber === null
        ? lastSentBlockNumber
        : lastSentBlockNumber === null
          ? lastCalledBlockNumber
          : lastCalledBlockNumber > lastSentBlockNumber
            ? lastCalledBlockNumber
            : lastSentBlockNumber;

    if (bestKnownCallBlockNumber !== null) {
      const checkpoint = yield* getCheckpointAtOrAbove({
        context: ctx.ponderContext,
        tronLightClientAddress,
        tronBlockNumber: bestKnownCallBlockNumber,
      });

      if (checkpoint) {
        const lag = checkpoint.tronBlockNumber - bestKnownCallBlockNumber;
        if (lag <= RESEND_IS_EVENT_CHAIN_TIP_IF_TLC_LAG_BLOCKS) return;
      } else {
        return;
      }
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
