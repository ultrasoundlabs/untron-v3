import { Effect } from "effect";
import { tronPullFromReceiversSent } from "ponder:schema";
import type { Address } from "viem";
import { encodeAbiParameters, keccak256 } from "viem";

import { TronRelayer } from "../../../deps/tron";
import type { RelayJobHandlerContext } from "../../types";
import { tryPromise } from "../../../../effect/tryPromise";

const TRX_TOKEN_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;
const RATE_SCALE = 1_000_000_000_000_000_000n; // 1e18

type SweepCandidate = {
  readonly receiverSalt: `0x${string}`;
  readonly usdtCost: bigint;
};

export const tronSweepTrxFromReceivers = (ctx: RelayJobHandlerContext) =>
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
