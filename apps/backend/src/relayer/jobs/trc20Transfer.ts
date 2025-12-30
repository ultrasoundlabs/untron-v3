import { ConfigError, Effect } from "effect";
import { sql } from "ponder";
import { encodeFunctionData, type Address, type Hex } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";
import { getTronLightClientAddress, getUntronV3Address } from "../../contracts";
import { tryPromise } from "../../effect/tryPromise";
import { MAINNET_CHAIN_ID } from "../../env";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc, TronRelayer, fetchTronBlockByNum } from "../deps/tron";
import { computeTronTxIdFromEncodedTx, computeTronTxMerkleProof } from "../tronProofs";
import { getRows } from "../sqlRows";

import type { TronReceiverMapEntry } from "../deps/types";
import { RetryLaterError } from "../errors";
import type { RelayJobRow } from "../types";
import {
  tronLightClientCheckpoint,
  trc20Transfer,
  untronV3DepositPreEntitled,
  untronV3LastReceiverPull,
  untronV3TronUsdt,
} from "ponder:schema";
import {
  expectAddress,
  expectBigint,
  expectHex,
  expectRecord,
  type RelayJobHandlerContext,
} from "./types";

function isIgnorablePreEntitleFailure(error: unknown): boolean {
  const msg = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
  return (
    msg.includes("DepositAlreadyProcessed") ||
    msg.includes("DepositNotAfterLastReceiverPull") ||
    msg.includes("NoActiveLease") ||
    msg.includes("InvalidReceiverForSalt") ||
    msg.includes("NotTronUsdt")
  );
}

const getKnownTronReceiver = (
  receiverAddress: Address
): Effect.Effect<TronReceiverMapEntry, ConfigError.ConfigError | Error, TronRelayer> =>
  TronRelayer.getReceiverMap().pipe(
    Effect.flatMap((receiverMap) => {
      const receiver = receiverMap.get(receiverAddress.toLowerCase());
      if (!receiver) {
        return Effect.fail(
          new Error(
            `Unknown receiver address (not in PREKNOWN_RECEIVER_SALTS mapping): ${receiverAddress}`
          )
        );
      }
      return Effect.succeed(receiver);
    })
  );

export const handleTrc20Transfer = ({
  ctx,
  job,
}: {
  job: RelayJobRow & { kind: "trc20_transfer" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const payload = expectRecord(job.payloadJson, "payloadJson");
    const transactionHash = expectHex(payload.transactionHash, "payload.transactionHash");
    const logIndex = Number(payload.logIndex);

    // Re-load canonical event data at execution time so reorgs / late indexing cannot leave us
    // with a permanently-stale `blockNumber` in the job payload.
    const transferId = `${job.chainId}:${transactionHash.toLowerCase()}:${logIndex}`;
    const transfer = yield* tryPromise(() =>
      ctx.ponderContext.db.find(trc20Transfer, { id: transferId })
    );
    if (!transfer) {
      return yield* Effect.fail(
        new RetryLaterError(`Missing trc20_transfer row for ${transferId} (indexer may be behind)`)
      );
    }

    const tokenAddress = expectAddress(transfer.tokenAddress, "trc20_transfer.token_address");
    const receiverAddress = expectAddress(transfer.to, "trc20_transfer.to");
    const tronBlockNumber = expectBigint(transfer.blockNumber, "trc20_transfer.block_number");
    const tronBlockTimestamp = expectBigint(
      transfer.blockTimestamp,
      "trc20_transfer.block_timestamp"
    );

    yield* Effect.logDebug("[trc20_transfer] handle").pipe(
      Effect.annotateLogs({
        tokenAddress,
        receiverAddress,
        tronBlockNumber: tronBlockNumber.toString(),
        tronBlockTimestamp: tronBlockTimestamp.toString(),
        transactionHash,
        logIndex: String(logIndex),
      })
    );

    const receiver = yield* getKnownTronReceiver(receiverAddress);

    const sweepReceiver = () =>
      Effect.gen(function* () {
        const balance = yield* TronRelayer.getErc20BalanceOf({
          tokenAddress,
          account: receiver.receiverAddress,
        });

        const sweepAmount = balance > 0n ? balance - 1n : 0n;
        if (sweepAmount === 0n) return;

        yield* TronRelayer.sendTronControllerPullFromReceivers({
          tokenAddress,
          receiverSalts: [receiver.receiverSalt],
        });
      });

    const controllerUsdt = (
      (yield* TronRelayer.getControllerUsdt()) as Address
    ).toLowerCase() as Address;
    const isControllerUsdt = tokenAddress.toLowerCase() === controllerUsdt;

    if (!isControllerUsdt) {
      yield* sweepReceiver();
      return;
    }

    const publicClients = yield* PublicClients;
    const tronGrpc = yield* TronGrpc;
    const mainnetClient = yield* publicClients.get("mainnet");

    const tronLightClientAddress = getTronLightClientAddress().toLowerCase() as Address;
    const untronV3Address = getUntronV3Address().toLowerCase() as Address;

    // Prefer event-derived state over onchain reads:
    // if we already indexed DepositPreEntitled(txId), there's nothing to do.
    const alreadyPreEntitled = yield* tryPromise(() =>
      ctx.ponderContext.db.find(untronV3DepositPreEntitled, {
        id: `${MAINNET_CHAIN_ID}:${untronV3Address}:${transactionHash.toLowerCase()}`,
      })
    );
    if (alreadyPreEntitled) {
      yield* Effect.logInfo("[trc20_transfer] skip (already pre-entitled)").pipe(
        Effect.annotateLogs({ transactionHash, logIndex: String(logIndex) })
      );
      return;
    }

    // Backward-compatible fallback for existing DBs before `untron_v3_deposit_preentitled` existed.
    const legacyPreEntitledResult = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT 1 AS ok
        FROM "untron_v3_event"
        WHERE chain_id = ${MAINNET_CHAIN_ID}
          AND lower(contract_address) = ${untronV3Address.toLowerCase()}
          AND event_name = 'DepositPreEntitled'
          AND lower(((args_json)::jsonb ->> 'txId')) = ${transactionHash.toLowerCase()}
        LIMIT 1;
      `)
    );
    if (getRows(legacyPreEntitledResult).length > 0) {
      yield* Effect.logInfo("[trc20_transfer] skip (already pre-entitled; legacy index)").pipe(
        Effect.annotateLogs({ transactionHash, logIndex: String(logIndex) })
      );
      return;
    }

    const tronUsdtRow = yield* tryPromise(() =>
      ctx.ponderContext.db.find(untronV3TronUsdt, {
        id: `${MAINNET_CHAIN_ID}:${untronV3Address}`,
      })
    );
    let tronUsdtFromEvents: string | null = tronUsdtRow?.tronUsdt ?? null;
    if (!tronUsdtFromEvents) {
      const legacyTronUsdtResult = yield* tryPromise(() =>
        ctx.ponderContext.db.sql.execute(sql`
          SELECT (args_json)::jsonb ->> 'tronUsdt' AS "tronUsdt"
          FROM "untron_v3_event"
          WHERE chain_id = ${MAINNET_CHAIN_ID}
            AND lower(contract_address) = ${untronV3Address.toLowerCase()}
            AND event_name = 'TronUsdtSet'
          ORDER BY block_number DESC, log_index DESC
          LIMIT 1;
        `)
      );
      const legacyRows = getRows(legacyTronUsdtResult) as Array<{ tronUsdt: unknown }>;
      const tronUsdt = legacyRows[0]?.tronUsdt;
      tronUsdtFromEvents = tronUsdt ? String(tronUsdt) : null;
    }

    if (tronUsdtFromEvents && tokenAddress.toLowerCase() !== tronUsdtFromEvents.toLowerCase()) {
      yield* Effect.logInfo("[trc20_transfer] skip (token != tronUsdt)").pipe(
        Effect.annotateLogs({
          tokenAddress,
          tronUsdt: tronUsdtFromEvents,
          transactionHash,
          logIndex: String(logIndex),
        })
      );
      yield* sweepReceiver();
      return;
    }

    // If the receiver has already been pulled at/after this transfer's Tron timestamp, preEntitle is
    // guaranteed to revert with DepositNotAfterLastReceiverPull. Skip early to avoid triggering
    // TronLightClient publish/backfill for an ineligible deposit.
    const lastPull = yield* tryPromise(() =>
      ctx.ponderContext.db.find(untronV3LastReceiverPull, {
        id: `${MAINNET_CHAIN_ID}:${untronV3Address}:${receiver.receiverSalt.toLowerCase()}:${tokenAddress.toLowerCase()}`,
      })
    );
    if (lastPull) {
      const lastPullTs = expectBigint(
        lastPull.lastPullTronBlockTimestamp,
        "lastPull.last_pull_tron_block_timestamp"
      );
      if (tronBlockTimestamp <= lastPullTs) {
        yield* Effect.logInfo("[trc20_transfer] skip (deposit at/before last receiver pull)").pipe(
          Effect.annotateLogs({
            transactionHash,
            logIndex: String(logIndex),
            receiverSalt: receiver.receiverSalt,
            transferTronBlockTimestamp: tronBlockTimestamp.toString(),
            lastPullTronBlockTimestamp: lastPullTs.toString(),
          })
        );
        return;
      }
    }

    {
      const { wallet, callOpts } = yield* tronGrpc.get();

      const block = yield* tryPromise(() =>
        fetchTronBlockByNum({
          wallet,
          metadata: callOpts.metadata,
          blockNumber: tronBlockNumber,
          timeoutMs: 15_000,
          retries: 2,
        })
      );

      const headerRaw = block.blockHeader?.rawData;
      if (!headerRaw) throw new Error("Tron block missing header/rawData");

      const proof = computeTronTxMerkleProof({ block, txidHex: transactionHash });
      const txId = computeTronTxIdFromEncodedTx(proof.encodedTx);

      const checkpoint = yield* tryPromise(() =>
        ctx.ponderContext.db.find(tronLightClientCheckpoint, {
          id: `${MAINNET_CHAIN_ID}:${tronLightClientAddress}:${tronBlockNumber.toString()}`,
        })
      );

      if (!checkpoint) {
        // Note: TronLightClient publishing is handled by the dedicated publisher (run on tron heartbeat).
        // We only retry once the target Tron block becomes available on mainnet.
        return yield* Effect.fail(
          new RetryLaterError(
            `Tron block ${tronBlockNumber.toString()} not yet published in TronLightClient`
          )
        );
      }

      const args = [
        receiver.receiverSalt,
        tronBlockNumber,
        proof.encodedTx,
        [...proof.proof] as readonly Hex[],
        proof.index,
      ] as const;

      const relayerAddress = yield* MainnetRelayer.getAddress();

      const simulation = yield* tryPromise(() =>
        mainnetClient.simulateContract({
          address: untronV3Address,
          abi: untronV3Abi,
          functionName: "preEntitle",
          args,
          account: relayerAddress,
        })
      ).pipe(
        Effect.map(() => ({ kind: "ok" as const })),
        Effect.catchAll((error) => {
          if (!isIgnorablePreEntitleFailure(error)) return Effect.fail(error);
          const reason = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
          return Effect.succeed({ kind: "skip" as const, reason });
        })
      );

      if (simulation.kind === "skip") {
        yield* Effect.logInfo("[trc20_transfer] skip (preEntitle not applicable)").pipe(
          Effect.annotateLogs({
            transactionHash,
            logIndex: String(logIndex),
            tronBlockNumber: tronBlockNumber.toString(),
            receiverSalt: receiver.receiverSalt,
            reason: simulation.reason,
          })
        );
        return;
      }

      const data = encodeFunctionData({
        abi: untronV3Abi,
        functionName: "preEntitle",
        args,
      });

      yield* MainnetRelayer.sendUserOperation({ calls: [{ to: untronV3Address, data }] }).pipe(
        Effect.annotateLogs({ jobKind: "trc20_transfer" })
      );
      return;
    }
  });
