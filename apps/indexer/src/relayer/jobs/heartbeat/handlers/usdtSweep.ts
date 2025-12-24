import { Effect } from "effect";
import { tronPullFromReceiversSent } from "ponder:schema";
import { encodeAbiParameters, keccak256 } from "viem";

import { tryPromise } from "../../../../effect/tryPromise";
import { TronRelayer } from "../../../deps/tron";
import type { RelayJobHandlerContext } from "../../types";

const MIN_RECEIVER_BALANCE = 2n;

export const tronSweepUsdtFromReceivers = (ctx: RelayJobHandlerContext) =>
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
