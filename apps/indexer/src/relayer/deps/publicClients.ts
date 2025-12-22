import { Config, ConfigError, Effect, Layer } from "effect";
import { createPublicClient, http, type PublicClient } from "viem";

import type { EvmChainName } from "./types";

export class PublicClients extends Effect.Tag("PublicClients")<
  PublicClients,
  {
    readonly get: (chain: EvmChainName) => Effect.Effect<PublicClient, ConfigError.ConfigError>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const mainnetClient = yield* Effect.cached(
        Effect.gen(function* () {
          const rpcUrl = yield* Config.nonEmptyString("UNTRON_V3_CHAIN_RPC_URL");
          return createPublicClient({ transport: http(rpcUrl) });
        })
      );

      const tronClient = yield* Effect.cached(
        Effect.gen(function* () {
          const rpcUrl = yield* Config.nonEmptyString("TRON_JSON_RPC_URL");
          return createPublicClient({ transport: http(rpcUrl) });
        })
      );

      return {
        get: (chain) => (chain === "mainnet" ? mainnetClient : tronClient),
      };
    })
  );
}
