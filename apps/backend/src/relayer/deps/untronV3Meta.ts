import { Cache, Duration, Effect, Layer } from "effect";
import type { Address } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";

import { PublicClients } from "./publicClients";

export class UntronV3Meta extends Effect.Tag("UntronV3Meta")<
  UntronV3Meta,
  {
    readonly getTronUsdt: (args: { untronV3Address: Address }) => Effect.Effect<Address, Error>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const publicClients = yield* PublicClients;
      const mainnetClient = yield* publicClients.get("mainnet");

      const cache = yield* Cache.make({
        capacity: 64,
        timeToLive: Duration.infinity,
        lookup: (untronV3Address: Address) =>
          Effect.tryPromise({
            try: async () => {
              const tronUsdt = (await mainnetClient.readContract({
                address: untronV3Address,
                abi: untronV3Abi,
                functionName: "tronUsdt",
              })) as Address;
              return tronUsdt.toLowerCase() as Address;
            },
            catch: (error) => (error instanceof Error ? error : new Error(String(error))),
          }),
      });

      const getTronUsdt = ({ untronV3Address }: { untronV3Address: Address }) =>
        cache.get(untronV3Address.toLowerCase() as Address);

      return { getTronUsdt };
    })
  );
}
