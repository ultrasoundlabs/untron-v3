import { ConfigError, Effect, Layer, Option } from "effect";

import { createTronClients } from "@untron/tron-protocol";

import { AppConfig } from "../../effect/config";
import type { TronGrpcClients } from "./types";

const requireSome = <A>(value: Option.Option<A>, message: string): Effect.Effect<A, Error> =>
  Option.match(value, {
    onNone: () => Effect.fail(new Error(message)),
    onSome: (a) => Effect.succeed(a),
  });

export class TronGrpc extends Effect.Tag("TronGrpc")<
  TronGrpc,
  {
    readonly get: () => Effect.Effect<TronGrpcClients, ConfigError.ConfigError | Error>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const appConfig = yield* AppConfig;

      const cachedClients = yield* Effect.cached(
        Effect.gen(function* () {
          const config = yield* appConfig.tronNetwork();
          const host = yield* requireSome(config.grpcHost, "Missing env var TRON_GRPC_HOST");

          return createTronClients(host, Option.getOrUndefined(config.apiKey), {
            insecure: config.grpcInsecure,
          });
        })
      );

      return {
        get: () => cachedClients,
      };
    })
  );
}
