import { Effect } from "effect";
import { isAddress, type Address } from "viem";

import { TronRelayer } from "../deps/tron";
import { getKnownTronReceiver } from "../receivers";
import type { RelayJobRow } from "../types";
import type { RelayJobHandlerContext } from "./types";

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

function expectAddress(value: unknown, label: string): Address {
  if (typeof value !== "string" || !isAddress(value)) throw new Error(`Invalid ${label} address`);
  return value as Address;
}

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

    const receiver = yield* getKnownTronReceiver(receiverAddress);

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
