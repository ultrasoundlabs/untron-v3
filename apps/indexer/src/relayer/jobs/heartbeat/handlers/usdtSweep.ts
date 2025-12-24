import { Effect } from "effect";

import { TronRelayer } from "../../../deps/tron";

const MIN_RECEIVER_BALANCE = 2n;

export const tronSweepUsdtFromReceivers = () =>
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

    yield* TronRelayer.sendTronControllerPullFromReceivers({
      tokenAddress: usdtAddress,
      receiverSalts,
    });
  });
