import { ConfigError, Effect } from "effect";
import { decodeFunctionResult, encodeFunctionData, type Hex } from "viem";

import type { BytesMessage, TransactionExtention } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { SmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { TransactionInfo } from "@untron/tron-protocol/tron";

import type { TronGrpcClients } from "../types";
import { isGrpcNotFoundError, isGrpcUnimplementedError, type UnaryCall } from "./grpcClient";

export type TronReadContract = <T>(args: {
  addressBytes21: Buffer;
  abi: readonly unknown[];
  functionName: string;
  args?: readonly unknown[];
}) => Effect.Effect<T, ConfigError.ConfigError | Error>;

export function makeTronReadContract(args: {
  tronGrpcGet: () => Effect.Effect<TronGrpcClients, ConfigError.ConfigError | Error>;
  grpcUnary: <Req, Res>(
    call: UnaryCall<Req, Res>,
    req: Req
  ) => Effect.Effect<Res, ConfigError.ConfigError | Error>;
  ownerAddressBytes21: () => Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
}): TronReadContract {
  return <T>({
    addressBytes21,
    abi,
    functionName,
    args: fnArgs,
  }: {
    addressBytes21: Buffer;
    abi: readonly unknown[];
    functionName: string;
    args?: readonly unknown[];
  }): Effect.Effect<T, ConfigError.ConfigError | Error> =>
    Effect.gen(function* () {
      const { wallet } = yield* args.tronGrpcGet();
      const ownerAddress = yield* args.ownerAddressBytes21();

      const data = encodeFunctionData({
        abi: abi as any,
        functionName: functionName as never,
        args: (fnArgs ?? []) as never,
      });

      const request = TriggerSmartContract.fromPartial({
        ownerAddress,
        contractAddress: addressBytes21,
        callValue: 0,
        data: Buffer.from(data.slice(2), "hex"),
      });

      const res = yield* args.grpcUnary(
        wallet.triggerConstantContract.bind(wallet) as unknown as UnaryCall<
          TriggerSmartContract,
          TransactionExtention
        >,
        request
      );

      if (!res.result?.result) {
        const msg = res.result?.message?.length ? res.result.message.toString("utf8") : "unknown";
        return yield* Effect.fail(new Error(`Tron triggerConstantContract failed: ${msg}`));
      }

      const buf = res.constantResult?.[0];
      if (!buf || buf.length === 0) {
        return yield* Effect.fail(
          new Error("Tron triggerConstantContract returned empty constantResult")
        );
      }

      return decodeFunctionResult({
        abi: abi as any,
        functionName: functionName as never,
        data: `0x${buf.toString("hex")}` as Hex,
      }) as T;
    });
}

export function isTronContractDeployed(args: {
  tronGrpcGet: () => Effect.Effect<TronGrpcClients, ConfigError.ConfigError | Error>;
  grpcUnary: <Req, Res>(
    call: UnaryCall<Req, Res>,
    req: Req
  ) => Effect.Effect<Res, ConfigError.ConfigError | Error>;
  addressBytes21: Buffer;
}): Effect.Effect<boolean, ConfigError.ConfigError | Error> {
  return Effect.gen(function* () {
    const { wallet } = yield* args.tronGrpcGet();
    const req: BytesMessage = { value: args.addressBytes21 };
    try {
      const res = yield* args.grpcUnary(
        wallet.getContract.bind(wallet) as unknown as UnaryCall<BytesMessage, SmartContract>,
        req
      );
      return (res.bytecode?.length ?? 0) > 0 || (res.codeHash?.length ?? 0) > 0;
    } catch (error) {
      if (isGrpcNotFoundError(error)) return false;
      return yield* Effect.fail(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

export function getTransactionInfoById(args: {
  tronGrpcGet: () => Effect.Effect<TronGrpcClients, ConfigError.ConfigError | Error>;
  grpcUnary: <Req, Res>(
    call: UnaryCall<Req, Res>,
    req: Req
  ) => Effect.Effect<Res, ConfigError.ConfigError | Error>;
  txidHex: string;
}): Effect.Effect<TransactionInfo | null, ConfigError.ConfigError | Error> {
  return Effect.gen(function* () {
    const { wallet } = yield* args.tronGrpcGet();
    const req: BytesMessage = { value: Buffer.from(args.txidHex, "hex") };

    const fromWallet = yield* args
      .grpcUnary(
        wallet.getTransactionInfoById.bind(wallet) as unknown as UnaryCall<
          BytesMessage,
          TransactionInfo
        >,
        req
      )
      .pipe(
        Effect.catchAll((error) =>
          isGrpcNotFoundError(error)
            ? Effect.succeed(null)
            : isGrpcUnimplementedError(error)
              ? Effect.fail(
                  new Error(
                    "Tron gRPC endpoint does not implement Wallet/GetTransactionInfoById (check TRON_GRPC_HOST)"
                  )
                )
              : Effect.fail(error)
        )
      );

    if (fromWallet && fromWallet.id?.length) return fromWallet;
    return null;
  });
}
