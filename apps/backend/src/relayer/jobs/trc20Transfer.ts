import { ConfigError, Effect } from "effect";
import { encodeFunctionData, type Address, type Hex } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";
import { getTronLightClientAddress, getUntronV3Address } from "../../contracts";
import { tryPromise } from "../../effect/tryPromise";
import { MAINNET_CHAIN_ID } from "../../env";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc, TronRelayer, fetchTronBlockByNum } from "../deps/tron";
import { computeTronTxIdFromEncodedTx, computeTronTxMerkleProof } from "../tronProofs";
import { UntronV3Meta } from "../deps/untronV3Meta";

import type { TronReceiverMapEntry } from "../deps/types";
import { RetryLaterError } from "../errors";
import type { RelayJobRow } from "../types";
import { tronLightClientCheckpoint } from "ponder:schema";
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
    const tokenAddress = expectAddress(payload.tokenAddress, "payload.tokenAddress");
    const receiverAddress = expectAddress(payload.to, "payload.to");
    const tronBlockNumber = expectBigint(payload.blockNumber, "payload.blockNumber");
    const transactionHash = expectHex(payload.transactionHash, "payload.transactionHash");

    yield* Effect.logDebug("[trc20_transfer] handle").pipe(
      Effect.annotateLogs({
        tokenAddress,
        receiverAddress,
        tronBlockNumber: tronBlockNumber.toString(),
        transactionHash,
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

    const tronLightClientAddress = getTronLightClientAddress();
    const untronV3Address = getUntronV3Address();

    const tronUsdt = yield* UntronV3Meta.getTronUsdt({ untronV3Address });
    if (tokenAddress.toLowerCase() !== tronUsdt) {
      yield* sweepReceiver();
      return;
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

      const tronBlockTimestampSec = BigInt(headerRaw.timestamp.toString()) / 1000n;

      const lastPullTs = (yield* tryPromise(() =>
        mainnetClient.readContract({
          address: untronV3Address,
          abi: untronV3Abi,
          functionName: "lastReceiverPullTimestamp",
          args: [receiver.receiverSalt],
        })
      )) as bigint;

      if (lastPullTs !== 0n && tronBlockTimestampSec <= lastPullTs) return;

      const proof = computeTronTxMerkleProof({ block, txidHex: transactionHash });
      const txId = computeTronTxIdFromEncodedTx(proof.encodedTx);

      const alreadyProcessed = (yield* tryPromise(() =>
        mainnetClient.readContract({
          address: untronV3Address,
          abi: untronV3Abi,
          functionName: "depositProcessed",
          args: [txId],
        })
      )) as boolean;
      if (alreadyProcessed) return;

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
        Effect.map(() => "ok" as const),
        Effect.catchAll((error) =>
          isIgnorablePreEntitleFailure(error) ? Effect.succeed("skip" as const) : Effect.fail(error)
        )
      );

      if (simulation === "skip") return;

      const data = encodeFunctionData({
        abi: untronV3Abi,
        functionName: "preEntitle",
        args,
      });

      yield* MainnetRelayer.sendUserOperation({ calls: [{ to: untronV3Address, data }] });
      return;
    }
  });
