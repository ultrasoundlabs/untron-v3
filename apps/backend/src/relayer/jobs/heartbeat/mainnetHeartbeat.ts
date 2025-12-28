import { Effect } from "effect";
import { sql } from "ponder";
import {
  tronPullFromReceiversSent,
  untronV3ControllerEventQueue,
  untronV3ProcessControllerEventsSent,
} from "ponder:schema";
import { encodeAbiParameters, encodeFunctionData, keccak256, type Address } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";

import { AppConfig } from "../../../effect/config";
import { tryPromise } from "../../../effect/tryPromise";
import { buildMainnetFillCalls } from "../../claimFiller/buildMainnetFillCalls";
import { MainnetRelayer } from "../../deps/mainnet";
import { TronRelayer } from "../../deps/tron";
import { getRows } from "../../sqlRows";
import type { RelayJobRow } from "../../types";
import { expectBigint, type RelayJobHandlerContext } from "../types";
import type { HeartbeatHandler } from "./types";
import { runHeartbeatHandlers } from "./runHeartbeatHandlers";

const MIN_RECEIVER_BALANCE = 2n;

export const handleMainnetHeartbeat = (_args: {
  job: RelayJobRow & { kind: "mainnet_heartbeat" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    const { ctx } = _args;

    if (ctx.dryRun) return;

    const handlers: ReadonlyArray<HeartbeatHandler> = [
      {
        name: "process_controller_events",
        effect: processControllerEventsIfBacklog(ctx),
      },
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

const processControllerEventsIfBacklog = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = (
      ctx.ponderContext.contracts.UntronV3.address as Address
    ).toLowerCase() as Address;

    const runtime = yield* AppConfig.relayerRuntime();
    const maxToProcess = BigInt(runtime.processMaxControllerEvents);
    if (maxToProcess === 0n) return;

    const state = yield* tryPromise(() =>
      ctx.ponderContext.db.find(untronV3ControllerEventQueue, {
        id: `${chainId}:${untronV3Address}`,
      })
    );
    if (!state) return;

    const enqueuedCount = state.enqueuedCount;
    const processedCount = state.processedCount;
    if (enqueuedCount <= processedCount) return;

    const backlog = enqueuedCount - processedCount;
    const toProcess = backlog < maxToProcess ? backlog : maxToProcess;
    if (toProcess === 0n) return;

    const cooldownBlocks = runtime.processControllerEventsCooldownBlocks;
    const attemptId = `${chainId}:${untronV3Address}`;
    const attemptResult = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        INSERT INTO "untron_v3_process_controller_events_sent" (
          id,
          chain_id,
          contract_address,
          enqueued_count,
          processed_count,
          sent_at_block_number,
          sent_at_block_timestamp
        )
        VALUES (
          ${attemptId},
          ${chainId},
          ${untronV3Address},
          ${enqueuedCount},
          ${processedCount},
          ${ctx.headBlockNumber},
          ${ctx.headBlockTimestamp}
        )
        ON CONFLICT (id) DO UPDATE SET
          enqueued_count = EXCLUDED.enqueued_count,
          processed_count = EXCLUDED.processed_count,
          sent_at_block_number = EXCLUDED.sent_at_block_number,
          sent_at_block_timestamp = EXCLUDED.sent_at_block_timestamp
        WHERE
          "untron_v3_process_controller_events_sent".enqueued_count <> EXCLUDED.enqueued_count
          OR "untron_v3_process_controller_events_sent".processed_count <> EXCLUDED.processed_count
          OR "untron_v3_process_controller_events_sent".sent_at_block_number <= EXCLUDED.sent_at_block_number - ${cooldownBlocks}
        RETURNING id;
      `)
    );
    const attemptRows = getRows(attemptResult) as Array<{ id: unknown }>;
    if (attemptRows.length === 0) return;

    const relayer = yield* MainnetRelayer;
    yield* relayer.sendUserOperation({
      calls: [
        {
          to: untronV3Address,
          value: 0n,
          data: encodeFunctionData({
            abi: untronV3Abi,
            functionName: "processControllerEvents",
            args: [toProcess],
          }),
        },
      ],
    });
  });

const fillClaimsFromUntronBalance = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const relayer = yield* MainnetRelayer;
    const calls = yield* buildMainnetFillCalls(ctx);
    if (calls.length === 0) return;
    yield* relayer.sendUserOperation({ calls });
  });

const sweepTronReceiversIfPendingClaims = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = ctx.ponderContext.contracts.UntronV3.address as `0x${string}`;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          target_token AS targetToken,
          queue_length AS queueLength
        FROM "untron_v3_claim_queue"
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
          AND queue_length > 0;
      `)
    );

    const nonEmptyQueuesRaw = getRows(result) as Array<{
      targetToken: unknown;
      queueLength: unknown;
    }>;
    const nonEmptyQueues = nonEmptyQueuesRaw.map((row) => ({
      targetToken: String(row.targetToken) as `0x${string}`,
      queueLength: expectBigint(row.queueLength, "queueLength"),
    }));
    if (nonEmptyQueues.length === 0) return;

    let hasPendingClaims = false;
    for (const queue of nonEmptyQueues) {
      const nextIndex = (yield* tryPromise(() =>
        ctx.ponderContext.client.readContract({
          address: untronV3Address,
          abi: untronV3Abi,
          functionName: "nextIndexByTargetToken",
          args: [queue.targetToken],
        })
      )) as bigint;

      if (queue.queueLength > nextIndex) {
        hasPendingClaims = true;
        break;
      }
    }

    if (!hasPendingClaims) return;

    yield* tronSweepUsdtFromReceivers(ctx);
  });

const tronSweepUsdtFromReceivers = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const receiverMap = yield* TronRelayer.getReceiverMap();
    if (receiverMap.size === 0) return;

    const usdtAddress = yield* TronRelayer.getControllerUsdt();

    const entries = Array.from(receiverMap.values());

    const receiverSalts = yield* Effect.forEach(
      entries,
      (entry) =>
        TronRelayer.getErc20BalanceOf({
          tokenAddress: usdtAddress,
          account: entry.receiverAddress,
        }).pipe(
          Effect.map((balance) => (balance >= MIN_RECEIVER_BALANCE ? entry.receiverSalt : null))
        ),
      { concurrency: 20 }
    ).pipe(Effect.map((items) => items.filter((item): item is `0x${string}` => item !== null)));

    if (receiverSalts.length === 0) return;

    const chainId = ctx.ponderContext.chain.id;
    const controllerAddress = (
      (yield* TronRelayer.getControllerEvmAddress()) as `0x${string}`
    ).toLowerCase() as `0x${string}`;

    const receiverSaltsSorted = [...receiverSalts].sort((a, b) =>
      a.toLowerCase().localeCompare(b.toLowerCase())
    );
    const receiverSaltsHash = keccak256(
      encodeAbiParameters(
        [{ type: "bytes32[]" }],
        [receiverSaltsSorted as readonly `0x${string}`[]]
      )
    ) as `0x${string}`;

    const id = `${chainId}:${controllerAddress}:${usdtAddress.toLowerCase()}`;
    const lastSent = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronPullFromReceiversSent, { id })
    );
    if (lastSent && lastSent.receiverSaltsHash.toLowerCase() === receiverSaltsHash.toLowerCase()) {
      return;
    }

    const { txid } = yield* TronRelayer.sendTronControllerPullFromReceivers({
      tokenAddress: usdtAddress,
      receiverSalts: receiverSaltsSorted,
    });

    yield* tryPromise(() =>
      ctx.ponderContext.db
        .insert(tronPullFromReceiversSent)
        .values({
          id,
          chainId,
          contractAddress: controllerAddress,
          tokenAddress: usdtAddress.toLowerCase() as `0x${string}`,
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
