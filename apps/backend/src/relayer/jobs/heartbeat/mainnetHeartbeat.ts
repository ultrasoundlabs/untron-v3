import { Effect } from "effect";
import { sql } from "ponder";
import { tronPullFromReceiversSent } from "ponder:schema";
import { encodeAbiParameters, keccak256 } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";

import { tryPromise } from "../../../effect/tryPromise";
import { buildMainnetFillCalls } from "../../claimFiller/buildMainnetFillCalls";
import { MainnetRelayer } from "../../deps/mainnet";
import { TronRelayer } from "../../deps/tron";
import { getRows } from "../../sqlRows";
import type { RelayJobRow } from "../../types";
import type { RelayJobHandlerContext } from "../types";
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

    const nonEmptyQueues = getRows(result) as Array<{
      targetToken: `0x${string}`;
      queueLength: bigint;
    }>;
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
      ctx.ponderContext.contracts.UntronController.address as `0x${string}`
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
