import type { Hash } from "viem";

import type { SendMainnetUserOperationResult } from "../relayer/deps/types";

export type ApiUserOperation = Readonly<{
  userOpHash: Hash;
  transactionHash: Hash;
  blockNumber: bigint;
  success: boolean;
}>;

export const toApiUserOperation = (sent: SendMainnetUserOperationResult): ApiUserOperation => ({
  userOpHash: sent.userOpHash,
  transactionHash: sent.transactionHash,
  blockNumber: sent.blockNumber,
  success: sent.success,
});
