import { Effect, Runtime } from "effect";
import { createSmartAccountClient } from "permissionless";
import { http, type Address, type Hash, type PublicClient } from "viem";
import { entryPoint06Abi } from "viem/account-abstraction";

import type { MainnetUserOperationCall, SendMainnetUserOperationResult } from "../types";
import type { ResolvedPaymaster } from "./paymasters";
import { makeUserOperationInclusionChecker, type SentUserOperation } from "./inclusion";
import { makePaymasterCallbacks } from "./promisePaymaster";
import { isProbablyBundler429, safeUrlForLogs } from "./utils";

type SafeAccountLike = Readonly<{
  address: Address;
  entryPoint: Readonly<{ address: Address }>;
}>;

const summarizeError = (error: unknown): string =>
  error instanceof Error ? `${error.name}: ${error.message}` : String(error);

export const sendUserOperationViaBundlers = (args: {
  runtime: Runtime.Runtime<never>;
  publicClient: PublicClient;
  account: SafeAccountLike;
  calls: readonly MainnetUserOperationCall[];
  bundlerUrls: readonly string[];
  paymasters: readonly ResolvedPaymaster[];
  timeoutBlocks: bigint;
  pollIntervalMs: number;
}): Effect.Effect<SendMainnetUserOperationResult, Error> =>
  Effect.gen(function* () {
    const entryPointAddress = args.account.entryPoint.address;
    const entryPointAbi = entryPoint06Abi;

    const normalizedCalls = args.calls.map((call) => ({
      to: call.to,
      value: call.value ?? 0n,
      data: call.data ?? "0x",
    }));

    const sent: Array<SentUserOperation> = [];
    const checker = makeUserOperationInclusionChecker({
      publicClient: args.publicClient,
      entryPointAddress,
      entryPointAbi,
      fromBlock: yield* Effect.tryPromise({
        try: () => args.publicClient.getBlockNumber(),
        catch: (error) => (error instanceof Error ? error : new Error(String(error))),
      }),
    });

    const checkInclusionUpTo = (toBlock: bigint) =>
      checker
        .checkUpTo(sent, toBlock)
        .pipe(
          Effect.tap((included) => {
            if (!included) return Effect.void;
            if (!included.success) {
              return Effect.logWarning("[mainnet] UserOperation included but failed").pipe(
                Effect.annotateLogs({
                  bundlerUrl: safeUrlForLogs(included.bundlerUrl),
                  userOpHash: included.userOpHash,
                  transactionHash: included.transactionHash,
                  blockNumber: included.blockNumber.toString(),
                })
              );
            }
            return Effect.logInfo("[mainnet] UserOperation included").pipe(
              Effect.annotateLogs({
                bundlerUrl: safeUrlForLogs(included.bundlerUrl),
                userOpHash: included.userOpHash,
                transactionHash: included.transactionHash,
                blockNumber: included.blockNumber.toString(),
                success: String(included.success),
              })
            );
          })
        )
        .pipe(
          Effect.flatMap((included) => {
            if (!included) return Effect.succeed(null);
            if (included.success) return Effect.succeed(included);
            return Effect.fail(
              new Error(
                `UserOperation was included but failed (userOpHash=${included.userOpHash}, tx=${included.transactionHash})`
              )
            );
          })
        );

    const errors: string[] = [];

    for (const [bundlerIndex, bundlerUrl] of args.bundlerUrls.entries()) {
      const bundlerUrlForLogs = safeUrlForLogs(bundlerUrl);
      const bundlerLabel = `bundler#${bundlerIndex + 1}`;

      const includedBeforeSend = yield* checkInclusionUpTo(
        yield* Effect.tryPromise({
          try: () => args.publicClient.getBlockNumber(),
          catch: (error) => (error instanceof Error ? error : new Error(String(error))),
        })
      );
      if (includedBeforeSend) return includedBeforeSend;

      try {
        const paymasterCallbacks = makePaymasterCallbacks({
          runtime: args.runtime,
          bundlerUrlForLogs,
          paymasters: args.paymasters,
        });

        const smartAccountClient = createSmartAccountClient({
          account: args.account as any,
          bundlerTransport: http(bundlerUrl),
          client: args.publicClient,
          paymaster: paymasterCallbacks,
        });

        const userOpHash = (yield* Effect.tryPromise({
          try: () =>
            smartAccountClient.sendUserOperation({
              account: args.account as any,
              calls: normalizedCalls,
            }),
          catch: (error) => (error instanceof Error ? error : new Error(String(error))),
        })) as Hash;

        yield* Effect.logInfo("[mainnet] UserOperation sent").pipe(
          Effect.annotateLogs({ bundlerUrl: bundlerUrlForLogs, userOpHash })
        );

        sent.push({ bundlerUrl, bundlerUrlForLogs, userOpHash });

        const startWaitBlock = yield* Effect.tryPromise({
          try: () => args.publicClient.getBlockNumber(),
          catch: (error) => (error instanceof Error ? error : new Error(String(error))),
        });
        const deadlineBlock = startWaitBlock + args.timeoutBlocks;

        while (true) {
          const head = yield* Effect.tryPromise({
            try: () => args.publicClient.getBlockNumber(),
            catch: (error) => (error instanceof Error ? error : new Error(String(error))),
          });

          const included = yield* checkInclusionUpTo(head);
          if (included) return included;

          if (head >= deadlineBlock) break;
          yield* Effect.sleep(args.pollIntervalMs);
        }

        const includedAfterTimeout = yield* checkInclusionUpTo(
          yield* Effect.tryPromise({
            try: () => args.publicClient.getBlockNumber(),
            catch: (error) => (error instanceof Error ? error : new Error(String(error))),
          })
        );
        if (includedAfterTimeout) return includedAfterTimeout;
      } catch (error) {
        const includedAfterError = yield* checkInclusionUpTo(
          yield* Effect.tryPromise({
            try: () => args.publicClient.getBlockNumber(),
            catch: (error) => (error instanceof Error ? error : new Error(String(error))),
          })
        );
        if (includedAfterError) return includedAfterError;

        const errorMessage = summarizeError(error);
        errors.push(`${bundlerLabel}: ${errorMessage}`);

        const log = isProbablyBundler429(error)
          ? Effect.logWarning("[mainnet] bundler rate-limited (429), trying next bundler")
          : Effect.logWarning("[mainnet] bundler failed, trying next bundler");
        yield* log.pipe(
          Effect.annotateLogs({ bundlerUrl: bundlerUrlForLogs, error: errorMessage })
        );
      }
    }

    const sentHashes = sent.map((s) => s.userOpHash).join(", ");
    const errorsJoined = errors.length > 0 ? ` Errors: ${errors.join(" | ")}` : "";
    return yield* Effect.fail(
      new Error(
        `UserOperation not included after trying ${args.bundlerUrls.length} bundler(s). Sent userOpHash(es): ${sentHashes}.${errorsJoined}`
      )
    );
  });
