import type { Address, Hex } from "viem";

import type { MainnetUserOperationCall } from "../deps/types";

export type Claim = Readonly<{
  claimIndex: bigint;
  leaseId: bigint;
  amountUsdt: bigint;
  targetChainId: bigint;
  beneficiary: Address;
}>;

export type ClaimQueue = Readonly<{
  targetToken: Address;
  queueLength: bigint;
  nextIndex: bigint;
  pendingCount: bigint;
  headClaimAmountUsdt: bigint;
}>;

export type SwapExecutorCall = Readonly<{
  to: Address;
  value: bigint;
  data: Hex;
}>;

export type PlannedQueueFill = Readonly<{
  targetToken: Address;
  maxClaims: bigint;
  totalUsdt: bigint;
  swapExecutorCalls: readonly SwapExecutorCall[];
  safePreCalls: readonly MainnetUserOperationCall[];
}>;
