import type { Address } from "viem";

import type { Claim } from "./types";

const RATE_DENOMINATOR = 1_000_000n;
const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;

export type PlanQueueFillResult = Readonly<{
  fillCount: bigint;
  totalUsdt: bigint;
  expectedOutTotal: bigint;
}>;

export const planQueueFill = (args: {
  chainId: number;
  usdt: Address;
  targetToken: Address;
  ratePpm: bigint | null;
  maxClaims: bigint;
  availableUsdt: bigint;
  claims: readonly Claim[];
  bridgerRoutes: ReadonlyMap<bigint, Address>;
}): PlanQueueFillResult => {
  let remainingUsdt = args.availableUsdt;
  let fillCount = 0n;
  let totalUsdt = 0n;
  let expectedOutTotal = 0n;

  const isUsdt = args.targetToken.toLowerCase() === args.usdt.toLowerCase();
  if (!isUsdt && (!args.ratePpm || args.ratePpm === 0n)) {
    return { fillCount: 0n, totalUsdt: 0n, expectedOutTotal: 0n };
  }

  const max = Number(
    args.maxClaims < BigInt(args.claims.length) ? args.maxClaims : BigInt(args.claims.length)
  );

  for (let i = 0; i < max; i++) {
    const claim = args.claims[i]!;
    const amountUsdt = claim.amountUsdt;
    if (remainingUsdt < amountUsdt) break;

    const needsBridge = claim.targetChainId !== BigInt(args.chainId);
    if (needsBridge) {
      const bridger = args.bridgerRoutes.get(claim.targetChainId);
      if (!bridger || bridger.toLowerCase() === ZERO_ADDRESS) break;
    }

    totalUsdt += amountUsdt;
    remainingUsdt -= amountUsdt;
    fillCount += 1n;

    if (!isUsdt) {
      expectedOutTotal += (amountUsdt * (args.ratePpm as bigint)) / RATE_DENOMINATOR;
    }
  }

  return { fillCount, totalUsdt, expectedOutTotal };
};
