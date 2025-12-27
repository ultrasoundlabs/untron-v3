import { Effect, Runtime } from "effect";
import type { Hex } from "viem";

import type { ResolvedPaymaster } from "./paymasters";
import { canonicalJsonStringify, toPaymasterAndDataV06 } from "./utils";

type PaymasterResult = { paymasterAndData: Hex };

const runLog = (runtime: Runtime.Runtime<never>, effect: Effect.Effect<unknown, never, never>) =>
  Runtime.runPromise(runtime)(effect).catch(() => {});

const summarizeError = (error: unknown): string =>
  error instanceof Error ? `${error.name}: ${error.message}` : String(error);

export const makePaymasterCallbacks = (args: {
  runtime: Runtime.Runtime<never>;
  bundlerUrlForLogs: string;
  paymasters: readonly ResolvedPaymaster[];
}) => {
  const paymasterDataCache = new Map<string, Promise<PaymasterResult>>();
  const paymasterStubCache = new Map<string, Promise<PaymasterResult>>();

  const callPaymasters = async (
    mode: "data" | "stub",
    parameters: unknown
  ): Promise<PaymasterResult> => {
    const params = parameters as Record<string, unknown>;
    const errors: string[] = [];

    for (const pm of args.paymasters) {
      const start = Date.now();
      await runLog(
        args.runtime,
        Effect.logInfo(`[mainnet] paymaster ${mode} attempt`).pipe(
          Effect.annotateLogs({
            bundlerUrl: args.bundlerUrlForLogs,
            paymaster: pm.name,
            paymasterUrl: pm.urlForLogs,
          })
        )
      );

      try {
        const method = mode === "stub" ? "getPaymasterStubData" : "getPaymasterData";
        const result = await pm.client[method]({
          ...(params as any),
          context: pm.context,
        });

        const normalized = toPaymasterAndDataV06(result);

        await runLog(
          args.runtime,
          Effect.logInfo(`[mainnet] paymaster ${mode} success`).pipe(
            Effect.annotateLogs({
              bundlerUrl: args.bundlerUrlForLogs,
              paymaster: pm.name,
              paymasterUrl: pm.urlForLogs,
              latencyMs: Date.now() - start,
            })
          )
        );

        return normalized;
      } catch (error) {
        const message = summarizeError(error);
        errors.push(`${pm.name}: ${message}`);
        await runLog(
          args.runtime,
          Effect.logWarning(`[mainnet] paymaster ${mode} failure`).pipe(
            Effect.annotateLogs({
              bundlerUrl: args.bundlerUrlForLogs,
              paymaster: pm.name,
              paymasterUrl: pm.urlForLogs,
              latencyMs: Date.now() - start,
              error: message,
            })
          )
        );
      }
    }

    if (mode === "stub") return callPaymasters("data", parameters);
    throw new Error(`All paymasters failed: ${errors.join(" | ")}`);
  };

  const withCache = (cache: Map<string, Promise<PaymasterResult>>, mode: "data" | "stub") => {
    return async (parameters: unknown): Promise<PaymasterResult> => {
      const key = canonicalJsonStringify(parameters);
      const cached = cache.get(key);
      if (cached) return cached;

      const promise = callPaymasters(mode, parameters);
      cache.set(key, promise);
      try {
        return await promise;
      } catch (error) {
        cache.delete(key);
        throw error;
      }
    };
  };

  return {
    getPaymasterData: withCache(paymasterDataCache, "data"),
    getPaymasterStubData: withCache(paymasterStubCache, "stub"),
  };
};
