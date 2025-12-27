import { Effect } from "effect";

import type { TronGrpcClients } from "../types";

export type UnaryCall<Req, Res> = (
  request: Req,
  metadata: unknown,
  callback: (error: unknown, response?: Res) => void
) => unknown;

export const isGrpcNotFoundError = (error: unknown): boolean => {
  if (!error || typeof error !== "object") return false;
  const maybeCode = (error as { readonly code?: unknown }).code;
  return maybeCode === 5;
};

export const isGrpcUnimplementedError = (error: unknown): boolean => {
  if (!error || typeof error !== "object") return false;
  const maybeCode = (error as { readonly code?: unknown }).code;
  return maybeCode === 12;
};

export const makeGrpcUnary =
  <E>(getClients: () => Effect.Effect<TronGrpcClients, E>) =>
  <Req, Res>(call: UnaryCall<Req, Res>, req: Req): Effect.Effect<Res, E | Error> =>
    getClients().pipe(
      Effect.flatMap(({ callOpts }) =>
        Effect.tryPromise({
          try: () =>
            new Promise<Res>((resolve, reject) => {
              try {
                call(req, callOpts.metadata, (err, res) => {
                  if (err) return reject(err);
                  if (res === undefined) return reject(new Error("Empty gRPC response"));
                  resolve(res);
                });
              } catch (err) {
                reject(err);
              }
            }),
          catch: (error) => (error instanceof Error ? error : new Error(String(error))),
        })
      )
    );
