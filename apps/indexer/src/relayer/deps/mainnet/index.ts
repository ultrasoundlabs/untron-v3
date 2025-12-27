import { ConfigError, Effect, Layer, Option, Runtime } from "effect";
import { type Address } from "viem";

import { AppConfig } from "../../../effect/config";
import { PublicClients } from "../publicClients";
import type { MainnetUserOperationCall, SendMainnetUserOperationResult } from "../types";

import { sendUserOperationViaBundlers } from "./bundlers";
import { resolvePaymasters } from "./paymasters";
import { makeSafeAccount } from "./safeAccount";

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
      const runtime = (yield* Effect.runtime()) as Runtime.Runtime<never>;

      const safeAccountCached = yield* Effect.cached(
        Effect.gen(function* () {
          const config = yield* appConfig.mainnetRelayer();
          const publicClient = yield* publicClients.get("mainnet");
          return yield* makeSafeAccount({ config, publicClient });
        })
      );

      const paymastersCached = yield* Effect.cached(
        Effect.gen(function* () {
          const config = yield* appConfig.mainnetRelayer();
          const json = yield* requireSome(
            config.paymastersJson,
            "Missing env var RELAYER_MAINNET_PAYMASTERS_JSON"
          );
          return yield* resolvePaymasters(json);
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

          const paymasters = yield* paymastersCached;

          yield* Effect.logInfo("[mainnet] send UserOperation").pipe(
            Effect.annotateLogs({
              callCount: args.calls.length,
              bundlerCount: resolvedBundlerUrls.length,
              paymasterCount: paymasters.length,
              timeoutBlocks: resolvedTimeoutBlocks.toString(),
              pollIntervalMs: resolvedPollIntervalMs,
            })
          );

          const account = yield* safeAccountCached;
          const publicClient = yield* publicClients.get("mainnet");

          return yield* sendUserOperationViaBundlers({
            runtime,
            publicClient,
            account,
            calls: args.calls,
            bundlerUrls: resolvedBundlerUrls,
            paymasters,
            timeoutBlocks: resolvedTimeoutBlocks,
            pollIntervalMs: resolvedPollIntervalMs,
          });
        }).pipe(Effect.withLogSpan("mainnet.userOperation"));

      return {
        getAddress,
        sendUserOperation,
      };
    })
  );
}
