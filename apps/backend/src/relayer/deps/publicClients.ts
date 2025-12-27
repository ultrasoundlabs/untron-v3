import { Config, ConfigError, Effect, Layer } from "effect";
import { createPublicClient, defineChain, http, type PublicClient } from "viem";

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
          const chainId = yield* Config.number("UNTRON_V3_CHAIN_ID");
          const rpcUrl = yield* Config.nonEmptyString("UNTRON_V3_CHAIN_RPC_URL");
          const chain = defineChain({
            id: chainId,
            name: "mainnet",
            nativeCurrency: { name: "Native", symbol: "NATIVE", decimals: 18 },
            rpcUrls: { default: { http: [rpcUrl] } },
          });
          return createPublicClient({ chain, transport: http(rpcUrl) });
        })
      );

      const tronClient = yield* Effect.cached(
        Effect.gen(function* () {
          const rpcUrl = yield* Config.nonEmptyString("TRON_JSON_RPC_URL");
          const chain = defineChain({
            id: 728126428,
            name: "tron",
            nativeCurrency: { name: "TRX", symbol: "TRX", decimals: 6 },
            rpcUrls: { default: { http: [rpcUrl] } },
          });
          return createPublicClient({ chain, transport: http(rpcUrl) });
        })
      );

      return {
        get: (chain) => (chain === "mainnet" ? mainnetClient : tronClient),
      };
    })
  );
}
