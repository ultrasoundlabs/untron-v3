import { Effect } from "effect";
import type { Address, Hash, PublicClient } from "viem";
import type { Abi } from "viem";

import type { SendMainnetUserOperationResult } from "../types";

export type SentUserOperation = Readonly<{
  bundlerUrl: string;
  bundlerUrlForLogs: string;
  userOpHash: Hash;
}>;

export const makeUserOperationInclusionChecker = (args: {
  publicClient: PublicClient;
  entryPointAddress: Address;
  entryPointAbi: Abi;
  fromBlock: bigint;
}) => {
  let nextFromBlock = args.fromBlock;

  const checkUpTo = (sent: readonly SentUserOperation[], toBlock: bigint) =>
    Effect.gen(function* () {
      if (sent.length === 0) return null;
      if (toBlock < nextFromBlock) return null;

      const fromBlock = nextFromBlock;

      for (const attempt of sent) {
        const logs = yield* Effect.tryPromise({
          try: () =>
            args.publicClient.getContractEvents({
              address: args.entryPointAddress,
              abi: args.entryPointAbi,
              eventName: "UserOperationEvent",
              args: { userOpHash: attempt.userOpHash },
              fromBlock,
              toBlock,
            }),
          catch: (error) => (error instanceof Error ? error : new Error(String(error))),
        });

        const log = logs[0];
        if (!log) continue;

        return {
          bundlerUrl: attempt.bundlerUrl,
          userOpHash: attempt.userOpHash,
          transactionHash: log.transactionHash,
          blockNumber: log.blockNumber,
        } satisfies SendMainnetUserOperationResult;
      }

      nextFromBlock = toBlock + 1n;
      return null;
    });

  return { checkUpTo };
};
