import { ConfigError, Effect, Layer, Option } from "effect";

import { createTronClients } from "@untron/tron-protocol";
import { NumberMessage, type BlockExtention } from "@untron/tron-protocol/api";

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

export async function fetchTronBlockByNum(args: {
  wallet: any;
  metadata: unknown;
  blockNumber: bigint;
  timeoutMs?: number;
  retries?: number;
  retryDelayMs?: number;
}): Promise<BlockExtention> {
  const retries = args.retries ?? 2;
  const retryDelayMs = args.retryDelayMs ?? 500;
  const timeoutMs = args.timeoutMs ?? 15_000;

  let lastError: unknown = null;
  for (let attempt = 0; attempt <= retries; attempt++) {
    const req = NumberMessage.fromPartial({ num: args.blockNumber.toString() });
    try {
      return await new Promise((resolve, reject) => {
        const timeout = setTimeout(() => {
          reject(new Error(`Timeout in getBlockByNum2(${args.blockNumber.toString()})`));
        }, timeoutMs);

        args.wallet.getBlockByNum2(
          req,
          args.metadata,
          (err: unknown, res: BlockExtention | null) =>
            err || !res
              ? (clearTimeout(timeout),
                reject(err ?? new Error("Empty response from getBlockByNum2")))
              : (clearTimeout(timeout), resolve(res))
        );
      });
    } catch (error) {
      lastError = error;
      if (attempt >= retries) break;
      await new Promise((r) => setTimeout(r, retryDelayMs * (attempt + 1)));
    }
  }

  throw lastError instanceof Error ? lastError : new Error(String(lastError));
}
