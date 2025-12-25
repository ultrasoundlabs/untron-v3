import { ConfigError, Duration, Effect, Layer, Option, Redacted, Schedule } from "effect";
import { createSmartAccountClient } from "permissionless";
import { type SafeVersion, toSafeSmartAccount } from "permissionless/accounts";
import {
  entryPoint06Abi,
  entryPoint06Address,
  entryPoint07Abi,
  entryPoint07Address,
} from "viem/account-abstraction";
import { privateKeyToAccount } from "viem/accounts";
import { http, type Address, type Hash, type Hex } from "viem";

import { AppConfig } from "../../effect/config";
import { tryPromise } from "../../effect/tryPromise";

import { PublicClients } from "./publicClients";
import type {
  EntryPointVersion,
  MainnetUserOperationCall,
  SendMainnetUserOperationResult,
} from "./types";

const shouldRetryBundler429 = (error: unknown): boolean => {
  const seen = new Set<unknown>();
  const queue: unknown[] = [error];

  while (queue.length > 0) {
    const current = queue.shift();
    if (!current || seen.has(current)) continue;
    seen.add(current);

    if (current instanceof Error) {
      const message = current.message.toLowerCase();
      if (
        message.includes("too many requests") ||
        message.includes("rate limit") ||
        message.includes("ratelimit") ||
        message.includes("http 429") ||
        message.includes("status code 429") ||
        message.includes("429")
      ) {
        return true;
      }

      const cause = (current as { cause?: unknown }).cause;
      if (cause) queue.push(cause);
    } else if (typeof current === "object") {
      const maybeMessage = (current as { message?: unknown }).message;
      if (typeof maybeMessage === "string") {
        const message = maybeMessage.toLowerCase();
        if (
          message.includes("too many requests") ||
          message.includes("rate limit") ||
          message.includes("ratelimit") ||
          message.includes("http 429") ||
          message.includes("status code 429") ||
          message.includes("429")
        ) {
          return true;
        }
      }

      const cause = (current as { cause?: unknown }).cause;
      if (cause) queue.push(cause);
    }
  }

  return false;
};

const requireSome = <A>(opt: Option.Option<A>, message: string): Effect.Effect<A, Error> =>
  Option.match(opt, {
    onNone: () => Effect.fail(new Error(message)),
    onSome: Effect.succeed,
  });

const requireNonEmptyArray = <A>(
  opt: Option.Option<readonly A[]>,
  message: string
): Effect.Effect<readonly A[], Error> =>
  requireSome(opt, message).pipe(
    Effect.filterOrFail(
      (values) => values.length > 0,
      () => new Error(message)
    )
  );

const parseMainnetOwnerPrivateKey = (value: Redacted.Redacted<string>): Hex => {
  const raw = Redacted.value(value).trim();
  if (!/^0x[0-9a-fA-F]{64}$/.test(raw)) {
    throw new Error('Invalid RELAYER_MAINNET_OWNER_PRIVATE_KEY (expected "0x" + 64 hex chars)');
  }
  return raw as Hex;
};

export class MainnetRelayer extends Effect.Tag("MainnetRelayer")<
  MainnetRelayer,
  {
    readonly getAddress: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
    readonly sendUserOperation: (args: {
      calls: readonly MainnetUserOperationCall[];
      bundlerUrls?: readonly string[];
      timeoutBlocks?: bigint;
      pollIntervalMs?: number;
    }) => Effect.Effect<SendMainnetUserOperationResult, ConfigError.ConfigError | Error>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const appConfig = yield* AppConfig;
      const publicClients = yield* PublicClients;

      const makeSafeAccount = Effect.gen(function* () {
        const config = yield* appConfig.mainnetRelayer();

        const ownerPrivateKey = yield* requireSome(
          config.ownerPrivateKey,
          "Missing env var RELAYER_MAINNET_OWNER_PRIVATE_KEY"
        );

        const safeVersion = config.safeVersion as SafeVersion;
        const entryPointVersion = config.entryPointVersion as EntryPointVersion;

        const entryPointAddress =
          Option.getOrUndefined(config.entryPointAddress) ??
          (entryPointVersion === "0.6" ? entryPoint06Address : entryPoint07Address);

        const safeAddress = Option.getOrUndefined(config.safeAddress);
        const publicClient = yield* publicClients.get("mainnet");

        return yield* tryPromise(() =>
          toSafeSmartAccount({
            client: publicClient,
            owners: [privateKeyToAccount(parseMainnetOwnerPrivateKey(ownerPrivateKey))],
            version: safeVersion,
            entryPoint: { address: entryPointAddress, version: entryPointVersion },
            address: safeAddress,
            saltNonce: config.saltNonce,
          })
        );
      });

      const safeAccountCached = yield* Effect.cached(makeSafeAccount);

      const getAddress = () => safeAccountCached.pipe(Effect.map((account) => account.address));

      const sendUserOperation = (args: {
        calls: readonly MainnetUserOperationCall[];
        bundlerUrls?: readonly string[];
        timeoutBlocks?: bigint;
        pollIntervalMs?: number;
      }) =>
        Effect.gen(function* () {
          if (args.calls.length === 0) {
            return yield* Effect.fail(
              new Error("sendUserOperation: expected at least 1 call in args.calls")
            );
          }

          const config = yield* appConfig.mainnetRelayer();
          const resolvedBundlerUrls =
            args.bundlerUrls && args.bundlerUrls.length > 0
              ? args.bundlerUrls
              : yield* requireNonEmptyArray(
                  config.bundlerUrls,
                  "Missing env var RELAYER_MAINNET_BUNDLER_URLS"
                );

          const resolvedTimeoutBlocks = args.timeoutBlocks ?? config.bundlerTimeoutBlocks;
          const resolvedPollIntervalMs = args.pollIntervalMs ?? config.bundlerPollIntervalMs;
          const sponsoredBundler = config.bundlerSponsored;
          const bundler429MaxRetries = Math.max(0, Math.floor(config.bundler429MaxRetries));
          const bundler429BaseDelayMs = Math.max(0, Math.floor(config.bundler429BaseDelayMs));

          yield* Effect.logInfo("[mainnet] send UserOperation").pipe(
            Effect.annotateLogs({
              callCount: args.calls.length,
              bundlerCount: resolvedBundlerUrls.length,
              sponsoredBundler,
              bundler429MaxRetries,
              bundler429BaseDelayMs,
              timeoutBlocks: resolvedTimeoutBlocks.toString(),
              pollIntervalMs: resolvedPollIntervalMs,
            })
          );

          const normalizedCalls = args.calls.map((call) => ({
            to: call.to,
            value: call.value ?? 0n,
            data: call.data ?? "0x",
          }));

          const account = yield* safeAccountCached;
          const publicClient = yield* publicClients.get("mainnet");

          const entryPointAddress = account.entryPoint.address;
          const entryPointAbi =
            account.entryPoint.version === "0.6" ? entryPoint06Abi : entryPoint07Abi;

          const sent: Array<{ bundlerUrl: string; userOpHash: Hash }> = [];
          let nextFromBlock = yield* tryPromise(() => publicClient.getBlockNumber());

          const checkInclusionUpTo = (toBlock: bigint) =>
            Effect.gen(function* () {
              if (sent.length === 0) return null;
              if (toBlock < nextFromBlock) return null;

              const fromBlock = nextFromBlock;

              for (const attempt of sent) {
                const logs = yield* tryPromise(() =>
                  publicClient.getContractEvents({
                    address: entryPointAddress,
                    abi: entryPointAbi,
                    eventName: "UserOperationEvent",
                    args: { userOpHash: attempt.userOpHash },
                    fromBlock,
                    toBlock,
                  })
                );

                const log = logs[0];
                if (!log) continue;

                const result = {
                  bundlerUrl: attempt.bundlerUrl,
                  userOpHash: attempt.userOpHash,
                  transactionHash: log.transactionHash,
                  blockNumber: log.blockNumber,
                } satisfies SendMainnetUserOperationResult;

                yield* Effect.logInfo("[mainnet] UserOperation included").pipe(
                  Effect.annotateLogs({
                    bundlerUrl: result.bundlerUrl,
                    userOpHash: result.userOpHash,
                    transactionHash: result.transactionHash,
                    blockNumber: result.blockNumber.toString(),
                  })
                );

                return result;
              }

              nextFromBlock = toBlock + 1n;
              return null;
            });

          const errors: string[] = [];

          for (const bundlerUrl of resolvedBundlerUrls) {
            const includedBeforeSend = yield* checkInclusionUpTo(
              yield* tryPromise(() => publicClient.getBlockNumber())
            );
            if (includedBeforeSend) return includedBeforeSend;

            try {
              const smartAccountClient = createSmartAccountClient({
                account,
                bundlerTransport: http(bundlerUrl),
                client: publicClient,
                userOperation: sponsoredBundler
                  ? {
                      estimateFeesPerGas: async () => ({
                        maxFeePerGas: 0n,
                        maxPriorityFeePerGas: 0n,
                      }),
                    }
                  : undefined,
              });

              const sendUserOperationOnce = tryPromise(() =>
                smartAccountClient.sendUserOperation({
                  account,
                  calls: normalizedCalls,
                })
              );

              const retrySchedule = Schedule.intersect(
                Schedule.mapInput(
                  Schedule.jittered(Schedule.exponential(bundler429BaseDelayMs)),
                  (_: Error) => _
                ),
                Schedule.mapInput(Schedule.count, (_: Error) => _)
              ).pipe(
                Schedule.tapOutput(([delay, attempt]) =>
                  Effect.logWarning("[mainnet] bundler rate-limited (429), retrying").pipe(
                    Effect.annotateLogs({
                      bundlerUrl,
                      attempt: attempt + 1,
                      maxRetries: bundler429MaxRetries,
                      delayMs: Duration.toMillis(delay),
                    })
                  )
                )
              );

              const userOpHash = yield* sendUserOperationOnce.pipe(
                Effect.retry({
                  times: bundler429MaxRetries,
                  while: (error) => shouldRetryBundler429(error),
                  schedule: retrySchedule,
                })
              );

              yield* Effect.logInfo("[mainnet] UserOperation sent").pipe(
                Effect.annotateLogs({ bundlerUrl, userOpHash })
              );

              sent.push({ bundlerUrl, userOpHash });

              const startWaitBlock = yield* tryPromise(() => publicClient.getBlockNumber());
              const deadlineBlock = startWaitBlock + resolvedTimeoutBlocks;

              while (true) {
                const head = yield* tryPromise(() => publicClient.getBlockNumber());

                const included = yield* checkInclusionUpTo(head);
                if (included) return included;

                if (head >= deadlineBlock) break;
                yield* Effect.sleep(resolvedPollIntervalMs);
              }

              const includedAfterTimeout = yield* checkInclusionUpTo(
                yield* tryPromise(() => publicClient.getBlockNumber())
              );
              if (includedAfterTimeout) return includedAfterTimeout;
            } catch (error) {
              const includedAfterError = yield* checkInclusionUpTo(
                yield* tryPromise(() => publicClient.getBlockNumber())
              );
              if (includedAfterError) return includedAfterError;

              const errorMessage =
                error instanceof Error ? `${error.name}: ${error.message}` : String(error);
              errors.push(`${bundlerUrl}: ${errorMessage}`);
            }
          }

          const sentHashes = sent.map((s) => `${s.bundlerUrl} => ${s.userOpHash}`).join(", ");
          const errorsJoined = errors.length > 0 ? ` Errors: ${errors.join(" | ")}` : "";
          return yield* Effect.fail(
            new Error(
              `UserOperation not included after trying ${resolvedBundlerUrls.length} bundler(s). Sent: ${sentHashes}.${errorsJoined}`
            )
          );
        }).pipe(Effect.withLogSpan("mainnet.userOperation"));

      return {
        getAddress,
        sendUserOperation,
      };
    })
  );
}
