import { Config, ConfigError, Effect, Layer, Option, Redacted } from "effect";
import { isAddress, type Address, type Hex } from "viem";

import {
  tronBase58ToBytes21,
  tronBase58ToEvmAddress,
  normalizeTronPrivateKey,
} from "../relayer/deps/tronProtocol";

export type RelayerRuntimeConfig = Readonly<{
  enabled: boolean;
  embeddedExecutorEnabled: boolean;
  dryRun: boolean;
  workerId: string;
  mainnetConfirmations: bigint;
  tronConfirmations: bigint;
  claimLimit: number;
  fillMaxClaimsPerQueue: number;
  maxAttempts: number;
  retryDelayBlocks: bigint;
}>;

export type TronNetworkConfig = Readonly<{
  grpcHost: Option.Option<string>;
  apiKey: Option.Option<string>;
  grpcInsecure: boolean;
  jsonRpcUrl: Option.Option<string>;
  controllerBase58: Option.Option<string>;
  controllerAddressBytes21: Option.Option<Buffer>;
  controllerEvmAddress: Option.Option<Address>;
  preknownReceiverSalts: Option.Option<readonly Hex[]>;
  privateKey: Option.Option<Redacted.Redacted<string>>;
  callValue: number;
  feeLimit: number;
  pollTimes: number;
  pollIntervalMs: number;
  rebalanceRebalancerAddress: Option.Option<Address>;
  rebalancePulledUsdtThreshold: Option.Option<bigint>;
}>;

export type MainnetRelayerConfig = Readonly<{
  bundlerUrls: Option.Option<readonly string[]>;
  ownerPrivateKey: Option.Option<Redacted.Redacted<string>>;
  safeVersion: "1.4.1" | "1.5.0";
  entryPointVersion: "0.6" | "0.7";
  entryPointAddress: Option.Option<Address>;
  safeAddress: Option.Option<Address>;
  saltNonce: bigint;
  bundlerTimeoutBlocks: bigint;
  bundlerPollIntervalMs: number;
}>;

const bigintFromString = (value: string, label: string) => {
  try {
    return BigInt(value);
  } catch {
    throw new Error(`Invalid ${label} (expected bigint-compatible string)`);
  }
};

const addressFromString = (value: string, label: string): Address => {
  if (!isAddress(value)) throw new Error(`Invalid ${label} (expected EVM address)`);
  return value as Address;
};

const nonEmptyTrimmedString = (name: string) =>
  Config.nonEmptyString(name).pipe(Config.map((s) => s.trim()));

const optionalNonEmptyTrimmedString = (name: string) => Config.option(nonEmptyTrimmedString(name));

const optionalAddress = (name: string) =>
  Config.option(nonEmptyTrimmedString(name)).pipe(
    Config.map((opt) => Option.map(opt, (value) => addressFromString(value, name)))
  );

const optionalTronBase58OrEvmAddress = (name: string) =>
  Config.option(nonEmptyTrimmedString(name)).pipe(
    Config.map((opt) =>
      Option.map(opt, (value) => {
        const trimmed = value.trim();
        if (trimmed.startsWith("T")) return tronBase58ToEvmAddress(trimmed);
        return addressFromString(trimmed, name);
      })
    )
  );

const optionalRedactedString = (name: string) =>
  Config.option(nonEmptyTrimmedString(name)).pipe(
    Config.map((opt) => Option.map(opt, Redacted.make))
  );

const requiredBigint = (name: string) =>
  nonEmptyTrimmedString(name).pipe(Config.mapAttempt((value) => bigintFromString(value, name)));

const requiredBooleanWithDefault = (name: string, fallback: boolean) =>
  Config.boolean(name).pipe(Config.withDefault(fallback));

const requiredNumberWithDefault = (name: string, fallback: number) =>
  Config.number(name).pipe(Config.withDefault(fallback));

const requiredLiteralWithDefault = <A extends string>(
  name: string,
  allowed: readonly [A, ...A[]],
  fallback: A
) => Config.literal(...allowed)(name).pipe(Config.withDefault(fallback));

export class AppConfig extends Effect.Tag("AppConfig")<
  AppConfig,
  {
    readonly relayerRuntime: () => Effect.Effect<RelayerRuntimeConfig, ConfigError.ConfigError>;
    readonly tronNetwork: () => Effect.Effect<TronNetworkConfig, ConfigError.ConfigError>;
    readonly mainnetRelayer: () => Effect.Effect<MainnetRelayerConfig, ConfigError.ConfigError>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const relayerRuntime = yield* Effect.cached(
        Effect.gen(function* () {
          const enabled = yield* requiredBooleanWithDefault("RELAYER_ENABLED", false);
          const embeddedExecutorEnabled = yield* requiredBooleanWithDefault(
            "RELAYER_EMBEDDED_EXECUTOR_ENABLED",
            false
          );

          const dryRun = yield* requiredBooleanWithDefault("RELAYER_DRY_RUN", true);
          const workerId = yield* nonEmptyTrimmedString("RELAYER_WORKER_ID").pipe(
            Config.withDefault(`embedded:${process.pid}`)
          );

          const mainnetConfirmations = yield* requiredBigint("RELAYER_MAINNET_CONFIRMATIONS").pipe(
            Config.withDefault(0n)
          );
          const tronConfirmations = yield* requiredBigint("RELAYER_TRON_CONFIRMATIONS").pipe(
            Config.withDefault(0n)
          );

          const claimLimit = yield* requiredNumberWithDefault("RELAYER_CLAIM_LIMIT", 10);
          const fillMaxClaimsPerQueue = yield* requiredNumberWithDefault(
            "RELAYER_FILL_MAX_CLAIMS_PER_QUEUE",
            500
          );
          const maxAttempts = yield* requiredNumberWithDefault("RELAYER_MAX_ATTEMPTS", 5);
          const retryDelayBlocks = yield* requiredBigint("RELAYER_RETRY_DELAY_BLOCKS").pipe(
            Config.withDefault(5n)
          );

          return {
            enabled,
            embeddedExecutorEnabled,
            dryRun,
            workerId,
            mainnetConfirmations,
            tronConfirmations,
            claimLimit,
            fillMaxClaimsPerQueue,
            maxAttempts,
            retryDelayBlocks,
          } satisfies RelayerRuntimeConfig;
        })
      );

      const tronNetwork = yield* Effect.cached(
        Effect.gen(function* () {
          const grpcHost = yield* optionalNonEmptyTrimmedString("TRON_GRPC_HOST");
          const apiKey = yield* optionalNonEmptyTrimmedString("TRON_API_KEY");
          const grpcInsecure = yield* requiredBooleanWithDefault("TRON_GRPC_INSECURE", false);
          const jsonRpcUrl = yield* optionalNonEmptyTrimmedString("TRON_JSON_RPC_URL");

          const controllerBase58 = yield* optionalNonEmptyTrimmedString(
            "UNTRON_CONTROLLER_ADDRESS"
          );
          const controllerAddressBytes21 = yield* Config.option(
            nonEmptyTrimmedString("UNTRON_CONTROLLER_ADDRESS").pipe(
              Config.mapAttempt(tronBase58ToBytes21)
            )
          );
          const controllerEvmAddress = yield* Config.option(
            nonEmptyTrimmedString("UNTRON_CONTROLLER_ADDRESS").pipe(
              Config.mapAttempt(tronBase58ToEvmAddress)
            )
          );

          const preknownReceiverSalts = yield* Config.option(
            Config.array(Config.nonEmptyString(), "PREKNOWN_RECEIVER_SALTS").pipe(
              Config.mapAttempt((salts) =>
                salts.map((salt): Hex => {
                  const normalized = salt.trim().startsWith("0x")
                    ? salt.trim()
                    : `0x${salt.trim()}`;
                  if (!/^0x[0-9a-f]{64}$/i.test(normalized)) {
                    throw new Error(`Invalid receiver salt "${salt}" (expected bytes32 hex)`);
                  }
                  return normalized as Hex;
                })
              )
            )
          );

          const privateKey = yield* Config.option(
            nonEmptyTrimmedString("RELAYER_TRON_PRIVATE_KEY").pipe(
              Config.mapAttempt(normalizeTronPrivateKey),
              Config.map(Redacted.make)
            )
          );

          const callValue = yield* requiredNumberWithDefault("RELAYER_TRON_CALL_VALUE", 0);
          const feeLimit = yield* requiredNumberWithDefault(
            "RELAYER_TRON_FEE_LIMIT",
            1_000_000_000
          );
          const pollTimes = yield* requiredNumberWithDefault("RELAYER_TRON_POLL_TIMES", 20);
          const pollIntervalMs = yield* requiredNumberWithDefault(
            "RELAYER_TRON_POLL_INTERVAL_MS",
            3_000
          );

          const rebalanceRebalancerAddress = yield* optionalTronBase58OrEvmAddress(
            "RELAYER_TRON_REBALANCER_ADDRESS"
          );
          const rebalancePulledUsdtThreshold = yield* Config.option(
            nonEmptyTrimmedString("RELAYER_TRON_REBALANCE_PULLED_USDT_THRESHOLD").pipe(
              Config.mapAttempt((value) =>
                bigintFromString(value, "RELAYER_TRON_REBALANCE_PULLED_USDT_THRESHOLD")
              )
            )
          );

          return {
            grpcHost,
            apiKey,
            grpcInsecure,
            jsonRpcUrl,
            controllerBase58,
            controllerAddressBytes21,
            controllerEvmAddress,
            preknownReceiverSalts,
            privateKey,
            callValue,
            feeLimit,
            pollTimes,
            pollIntervalMs,
            rebalanceRebalancerAddress,
            rebalancePulledUsdtThreshold,
          } satisfies TronNetworkConfig;
        })
      );

      const mainnetRelayer = yield* Effect.cached(
        Effect.gen(function* () {
          const bundlerUrls = yield* Config.option(
            Config.array(Config.nonEmptyString(), "RELAYER_MAINNET_BUNDLER_URLS").pipe(
              Config.map((urls) => urls.map((u) => u.trim()).filter(Boolean))
            )
          );

          const ownerPrivateKey = yield* optionalRedactedString(
            "RELAYER_MAINNET_OWNER_PRIVATE_KEY"
          );

          const safeVersion = yield* requiredLiteralWithDefault(
            "RELAYER_MAINNET_SAFE_VERSION",
            ["1.4.1", "1.5.0"],
            "1.4.1"
          );

          const entryPointVersion = yield* requiredLiteralWithDefault(
            "RELAYER_MAINNET_ENTRYPOINT_VERSION",
            ["0.6", "0.7"],
            "0.7"
          );

          const entryPointAddress = yield* optionalAddress("RELAYER_MAINNET_ENTRYPOINT_ADDRESS");
          const safeAddress = yield* optionalAddress("RELAYER_MAINNET_SAFE_ADDRESS");
          const saltNonce = yield* requiredBigint("RELAYER_MAINNET_SAFE_SALT_NONCE").pipe(
            Config.withDefault(0n)
          );

          const bundlerTimeoutBlocks = yield* requiredBigint(
            "RELAYER_MAINNET_BUNDLER_TIMEOUT_BLOCKS"
          ).pipe(Config.withDefault(20n));
          const bundlerPollIntervalMs = yield* requiredNumberWithDefault(
            "RELAYER_MAINNET_BUNDLER_POLL_INTERVAL_MS",
            3_000
          );

          return {
            bundlerUrls,
            ownerPrivateKey,
            safeVersion,
            entryPointVersion,
            entryPointAddress,
            safeAddress,
            saltNonce,
            bundlerTimeoutBlocks,
            bundlerPollIntervalMs,
          } satisfies MainnetRelayerConfig;
        })
      );

      return {
        relayerRuntime: () => relayerRuntime,
        tronNetwork: () => tronNetwork,
        mainnetRelayer: () => mainnetRelayer,
      };
    })
  );
}
