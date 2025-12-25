import { ConfigError, Duration, Effect, Layer, Option, Redacted, Runtime, Schedule } from "effect";
import { createSmartAccountClient } from "permissionless";
import { type SafeVersion, toSafeSmartAccount } from "permissionless/accounts";
import {
  createPaymasterClient,
  entryPoint06Abi,
  entryPoint06Address,
} from "viem/account-abstraction";
import { privateKeyToAccount } from "viem/accounts";
import { http, type Address, type Hash, type Hex } from "viem";

import { AppConfig } from "../../effect/config";
import { tryPromise } from "../../effect/tryPromise";

import { PublicClients } from "./publicClients";
import type { MainnetUserOperationCall, SendMainnetUserOperationResult } from "./types";

type PaymasterConfig = Readonly<{
  name: string;
  url: string;
  context?: unknown;
  timeoutMs?: number;
  maxRetries429?: number;
  baseDelayMs429?: number;
}>;

type ResolvedPaymaster = Readonly<{
  name: string;
  url: string;
  urlForLogs: string;
  context?: unknown;
  timeoutMs: number;
  maxRetries429: number;
  baseDelayMs429: number;
  client: ReturnType<typeof createPaymasterClient>;
}>;

const safeUrlForLogs = (rawUrl: string): string => {
  try {
    const url = new URL(rawUrl);
    const path = url.pathname === "/" ? "" : url.pathname;
    return `${url.protocol}//${url.host}${path}`;
  } catch {
    return "<invalid-url>";
  }
};

const parsePositiveInt = (value: unknown, label: string): number => {
  if (
    typeof value !== "number" ||
    !Number.isFinite(value) ||
    !Number.isInteger(value) ||
    value < 0
  ) {
    throw new Error(`Invalid ${label} (expected a non-negative integer)`);
  }
  return value;
};

const parsePositiveMs = (value: unknown, label: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
    throw new Error(`Invalid ${label} (expected a positive number)`);
  }
  return value;
};

const parsePaymastersJson = (raw: string): readonly PaymasterConfig[] => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected JSON array): ${message}`);
  }

  if (!Array.isArray(parsed)) {
    throw new Error("Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected JSON array)");
  }
  if (parsed.length === 0) {
    throw new Error("Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected non-empty array)");
  }

  return parsed.map((item, index): PaymasterConfig => {
    if (!item || typeof item !== "object" || Array.isArray(item)) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}] (expected object with { name, url, ... })`
      );
    }
    const record = item as Record<string, unknown>;

    const nameRaw = record.name;
    const urlRaw = record.url;
    if (typeof nameRaw !== "string" || nameRaw.trim().length === 0) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].name (expected non-empty string)`
      );
    }
    if (typeof urlRaw !== "string" || urlRaw.trim().length === 0) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected non-empty string)`
      );
    }

    const name = nameRaw.trim();
    const url = urlRaw.trim();
    if (/\s/.test(url)) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (must not contain whitespace)`
      );
    }

    let parsedUrl: URL;
    try {
      parsedUrl = new URL(url);
    } catch {
      throw new Error(`Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected valid URL)`);
    }
    if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected http(s) URL)`
      );
    }

    const timeoutMs =
      typeof record.timeoutMs === "undefined"
        ? undefined
        : parsePositiveMs(record.timeoutMs, `RELAYER_MAINNET_PAYMASTERS_JSON[${index}].timeoutMs`);
    const maxRetries429 =
      typeof record.maxRetries429 === "undefined"
        ? undefined
        : parsePositiveInt(
            record.maxRetries429,
            `RELAYER_MAINNET_PAYMASTERS_JSON[${index}].maxRetries429`
          );
    const baseDelayMs429 =
      typeof record.baseDelayMs429 === "undefined"
        ? undefined
        : parsePositiveMs(
            record.baseDelayMs429,
            `RELAYER_MAINNET_PAYMASTERS_JSON[${index}].baseDelayMs429`
          );

    return {
      name,
      url,
      context: record.context,
      timeoutMs,
      maxRetries429,
      baseDelayMs429,
    };
  });
};

const toPaymasterAndDataV06 = (value: unknown): { paymasterAndData: Hex } => {
  if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    const paymasterAndData = record.paymasterAndData;
    if (typeof paymasterAndData === "string" && paymasterAndData.startsWith("0x")) {
      return { paymasterAndData: paymasterAndData as Hex };
    }

    const paymaster = record.paymaster;
    const paymasterData = record.paymasterData;
    if (
      typeof paymaster === "string" &&
      paymaster.startsWith("0x") &&
      typeof paymasterData === "string" &&
      paymasterData.startsWith("0x")
    ) {
      return { paymasterAndData: `0x${paymaster.slice(2)}${paymasterData.slice(2)}` as Hex };
    }
  }
  throw new Error("Paymaster response missing paymasterAndData for EntryPoint v0.6");
};

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
      const runtime = yield* Effect.runtime();

      const makeSafeAccount = Effect.gen(function* () {
        const config = yield* appConfig.mainnetRelayer();

        const ownerPrivateKey = yield* requireSome(
          config.ownerPrivateKey,
          "Missing env var RELAYER_MAINNET_OWNER_PRIVATE_KEY"
        );

        const safeVersion = config.safeVersion as SafeVersion;

        const entryPointAddress =
          Option.getOrUndefined(config.entryPointAddress) ?? entryPoint06Address;

        const safeAddress = Option.getOrUndefined(config.safeAddress);
        const publicClient = yield* publicClients.get("mainnet");

        return yield* tryPromise(() =>
          toSafeSmartAccount({
            client: publicClient,
            owners: [privateKeyToAccount(parseMainnetOwnerPrivateKey(ownerPrivateKey))],
            version: safeVersion,
            entryPoint: { address: entryPointAddress, version: "0.6" },
            address: safeAddress,
            saltNonce: config.saltNonce,
          })
        );
      });

      const safeAccountCached = yield* Effect.cached(makeSafeAccount);
      const paymastersCached = yield* Effect.cached(
        Effect.gen(function* () {
          const config = yield* appConfig.mainnetRelayer();
          const json = yield* requireSome(
            config.paymastersJson,
            "Missing env var RELAYER_MAINNET_PAYMASTERS_JSON"
          );

          const parsed = yield* Effect.try({
            try: () => parsePaymastersJson(json),
            catch: (error) => (error instanceof Error ? error : new Error(String(error))),
          });

          return parsed.map(
            (pm): ResolvedPaymaster => ({
              name: pm.name,
              url: pm.url,
              urlForLogs: safeUrlForLogs(pm.url),
              context: pm.context,
              timeoutMs: pm.timeoutMs ?? 10_000,
              maxRetries429: pm.maxRetries429 ?? 0,
              baseDelayMs429: pm.baseDelayMs429 ?? 1_000,
              client: createPaymasterClient({
                transport: http(pm.url, { timeout: pm.timeoutMs ?? 10_000, retryCount: 0 }),
              }),
            })
          );
        })
      );

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
          const bundler429MaxRetries = Math.max(0, Math.floor(config.bundler429MaxRetries));
          const bundler429BaseDelayMs = Math.max(0, Math.floor(config.bundler429BaseDelayMs));
          const paymasters = yield* paymastersCached;

          yield* Effect.logInfo("[mainnet] send UserOperation").pipe(
            Effect.annotateLogs({
              callCount: args.calls.length,
              bundlerCount: resolvedBundlerUrls.length,
              bundler429MaxRetries,
              bundler429BaseDelayMs,
              paymasterCount: paymasters.length,
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
          const entryPointAbi = entryPoint06Abi;

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
              const retryScheduleFor = (pm: ResolvedPaymaster, message: string) =>
                Schedule.intersect(
                  Schedule.mapInput(
                    Schedule.jittered(Schedule.exponential(pm.baseDelayMs429)),
                    (_: Error) => _
                  ),
                  Schedule.mapInput(Schedule.count, (_: Error) => _)
                ).pipe(
                  Schedule.tapOutput(([delay, attempt]) =>
                    Effect.logWarning(message).pipe(
                      Effect.annotateLogs({
                        bundlerUrl,
                        paymaster: pm.name,
                        paymasterUrl: pm.urlForLogs,
                        attempt: attempt + 1,
                        maxRetries: pm.maxRetries429,
                        delayMs: Duration.toMillis(delay),
                      })
                    )
                  )
                );

              const getPaymasterData = async (
                parameters: unknown
              ): Promise<{ paymasterAndData: Hex }> => {
                const params = parameters as Record<string, unknown>;

                const errors: string[] = [];

                for (const pm of paymasters) {
                  const start = Date.now();
                  await Runtime.runPromise(runtime)(
                    Effect.logInfo("[mainnet] paymaster attempt").pipe(
                      Effect.annotateLogs({
                        bundlerUrl,
                        paymaster: pm.name,
                        paymasterUrl: pm.urlForLogs,
                      })
                    )
                  );

                  try {
                    const result = await Runtime.runPromise(runtime)(
                      tryPromise(() =>
                        pm.client.getPaymasterData({
                          ...(params as any),
                          context: pm.context,
                        })
                      ).pipe(
                        Effect.retry({
                          times: pm.maxRetries429,
                          while: (error) => shouldRetryBundler429(error),
                          schedule: retryScheduleFor(
                            pm,
                            "[mainnet] paymaster rate-limited (429), retrying"
                          ),
                        })
                      )
                    );

                    const normalized = toPaymasterAndDataV06(result);

                    await Runtime.runPromise(runtime)(
                      Effect.logInfo("[mainnet] paymaster success").pipe(
                        Effect.annotateLogs({
                          bundlerUrl,
                          paymaster: pm.name,
                          paymasterUrl: pm.urlForLogs,
                          latencyMs: Date.now() - start,
                        })
                      )
                    );

                    return normalized;
                  } catch (error) {
                    const message =
                      error instanceof Error ? `${error.name}: ${error.message}` : String(error);
                    errors.push(`${pm.name}: ${message}`);
                    await Runtime.runPromise(runtime)(
                      Effect.logWarning("[mainnet] paymaster failure").pipe(
                        Effect.annotateLogs({
                          bundlerUrl,
                          paymaster: pm.name,
                          paymasterUrl: pm.urlForLogs,
                          latencyMs: Date.now() - start,
                          error: message,
                        })
                      )
                    );
                  }
                }

                throw new Error(`All paymasters failed: ${errors.join(" | ")}`);
              };

              const getPaymasterStubData = async (
                parameters: unknown
              ): Promise<{ paymasterAndData: Hex }> => {
                const params = parameters as Record<string, unknown>;
                const errors: string[] = [];

                for (const pm of paymasters) {
                  const start = Date.now();
                  await Runtime.runPromise(runtime)(
                    Effect.logInfo("[mainnet] paymaster stub attempt").pipe(
                      Effect.annotateLogs({
                        bundlerUrl,
                        paymaster: pm.name,
                        paymasterUrl: pm.urlForLogs,
                      })
                    )
                  );

                  try {
                    const result = await Runtime.runPromise(runtime)(
                      tryPromise(() =>
                        pm.client.getPaymasterStubData({
                          ...(params as any),
                          context: pm.context,
                        })
                      ).pipe(
                        Effect.retry({
                          times: pm.maxRetries429,
                          while: (error) => shouldRetryBundler429(error),
                          schedule: retryScheduleFor(
                            pm,
                            "[mainnet] paymaster stub rate-limited (429), retrying"
                          ),
                        })
                      )
                    );

                    const normalized = toPaymasterAndDataV06(result);

                    await Runtime.runPromise(runtime)(
                      Effect.logInfo("[mainnet] paymaster stub success").pipe(
                        Effect.annotateLogs({
                          bundlerUrl,
                          paymaster: pm.name,
                          paymasterUrl: pm.urlForLogs,
                          latencyMs: Date.now() - start,
                        })
                      )
                    );

                    return normalized;
                  } catch (error) {
                    const message =
                      error instanceof Error ? `${error.name}: ${error.message}` : String(error);
                    errors.push(`${pm.name}: ${message}`);
                    await Runtime.runPromise(runtime)(
                      Effect.logWarning("[mainnet] paymaster stub failure").pipe(
                        Effect.annotateLogs({
                          bundlerUrl,
                          paymaster: pm.name,
                          paymasterUrl: pm.urlForLogs,
                          latencyMs: Date.now() - start,
                          error: message,
                        })
                      )
                    );
                  }
                }

                return getPaymasterData(parameters);
              };

              const smartAccountClient = createSmartAccountClient({
                account,
                bundlerTransport: http(bundlerUrl),
                client: publicClient,
                paymaster: {
                  getPaymasterData: (parameters) => getPaymasterData(parameters),
                  getPaymasterStubData: (parameters) => getPaymasterStubData(parameters),
                },
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
