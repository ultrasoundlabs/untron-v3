import { Effect } from "effect";
import type { Address } from "viem";

import { TronRelayer } from "../deps/tron";
import type { RelayJobRow } from "../types";
import type { RelayJobHandlerContext } from "./types";

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;

export const handleTronHeartbeat = ({
  ctx,
}: {
  job: RelayJobRow & { kind: "tron_heartbeat" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const receiverMap = yield* TronRelayer.getReceiverMap();
    if (receiverMap.size === 0) return;

    const usdtAddress = yield* TronRelayer.getControllerUsdt();
    if (usdtAddress.toLowerCase() === ZERO_ADDRESS) return;

    const entries = Array.from(receiverMap.values());
    const receiverSalts = yield* Effect.forEach(entries, (entry) =>
      TronRelayer.getErc20BalanceOf({
        tokenAddress: usdtAddress,
        account: entry.receiverAddress,
      }).pipe(Effect.map((balance) => (balance > 0n ? entry.receiverSalt : null)))
    ).pipe(
      Effect.map((salts) => salts.filter((salt): salt is NonNullable<typeof salt> => salt !== null))
    );

    if (receiverSalts.length === 0) return;

    yield* TronRelayer.sendTronControllerPullFromReceivers({
      tokenAddress: usdtAddress,
      receiverSalts,
    });
  });
