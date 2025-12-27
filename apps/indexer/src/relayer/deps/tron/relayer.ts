import { ConfigError, Effect, Layer, Option, Redacted } from "effect";
import { type Address, type Hex } from "viem";

import { AccountBalanceRequest } from "@untron/tron-protocol/core/contract/balance_contract";
import type { AccountBalanceResponse } from "@untron/tron-protocol/core/contract/balance_contract";

import { ERC20Abi } from "../../../../abis/ERC20Abi";
import { AppConfig, type TronNetworkConfig } from "../../../effect/config";

import type { SendTronTransactionResult, TronReceiverMapEntry } from "../types";
import { TronGrpc, makeGrpcUnary, type UnaryCall } from "./grpcClient";
import {
  tronEvmAddressToBytes21,
  tronPrivateKeyToAddressBase58,
  tronPrivateKeyToAddressBytes21,
} from "./protocol";
import { makeTronReadContract, type TronReadContract } from "./contractCalls";
import { makeUntronControllerOperations } from "./untronController";
import { untronControllerAbi } from "@untron/v3-contracts";

const requireSome = <A>(opt: Option.Option<A>, message: string): Effect.Effect<A, Error> =>
  Option.match(opt, {
    onNone: () => Effect.fail(new Error(message)),
    onSome: Effect.succeed,
  });

type TronIdentity = Readonly<{
  controllerAddressBytes21: () => Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
  controllerEvmAddress: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
  receiverSalts: () => Effect.Effect<readonly Hex[], ConfigError.ConfigError | Error>;
  privateKey: () => Effect.Effect<string, ConfigError.ConfigError | Error>;
  relayerAddressCached: Effect.Effect<string, ConfigError.ConfigError | Error>;
  relayerAddressBytes21Cached: Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
}>;

const makeTronIdentity = (args: {
  tronConfigCached: Effect.Effect<TronNetworkConfig, ConfigError.ConfigError>;
}): Effect.Effect<TronIdentity, never> =>
  Effect.gen(function* () {
    const tronConfigCached = args.tronConfigCached;

    const controllerAddressBytes21 = () =>
      tronConfigCached.pipe(
        Effect.flatMap((config) =>
          requireSome(config.controllerAddressBytes21, "Missing env var UNTRON_CONTROLLER_ADDRESS")
        )
      );

    const controllerEvmAddress = () =>
      tronConfigCached.pipe(
        Effect.flatMap((config) =>
          requireSome(config.controllerEvmAddress, "Missing env var UNTRON_CONTROLLER_ADDRESS")
        )
      );

    const receiverSalts = () =>
      tronConfigCached.pipe(
        Effect.flatMap((config) =>
          requireSome(config.preknownReceiverSalts, "Missing env var PREKNOWN_RECEIVER_SALTS")
        )
      );

    const privateKey = () =>
      tronConfigCached.pipe(
        Effect.flatMap((config) =>
          requireSome(config.privateKey, "Missing env var RELAYER_TRON_PRIVATE_KEY")
        ),
        Effect.map(Redacted.value)
      );

    const relayerAddressCached = yield* Effect.cached(
      privateKey().pipe(Effect.map(tronPrivateKeyToAddressBase58))
    );

    const relayerAddressBytes21Cached = yield* Effect.cached(
      privateKey().pipe(Effect.map(tronPrivateKeyToAddressBytes21))
    );

    return {
      controllerAddressBytes21,
      controllerEvmAddress,
      receiverSalts,
      privateKey,
      relayerAddressCached,
      relayerAddressBytes21Cached,
    };
  });

const makeReceiverMapCached = (args: {
  receiverSalts: () => Effect.Effect<readonly Hex[], ConfigError.ConfigError | Error>;
  controllerAddressBytes21: () => Effect.Effect<Buffer, ConfigError.ConfigError | Error>;
  tronReadContract: TronReadContract;
}): Effect.Effect<
  Effect.Effect<ReadonlyMap<string, TronReceiverMapEntry>, ConfigError.ConfigError | Error>,
  never
> =>
  Effect.cached(
    Effect.gen(function* () {
      const salts = yield* args.receiverSalts();
      const controllerBytes21 = yield* args.controllerAddressBytes21();

      const entries = yield* Effect.forEach(salts, (receiverSalt) =>
        args
          .tronReadContract<Address>({
            addressBytes21: controllerBytes21,
            abi: untronControllerAbi,
            functionName: "predictReceiverAddress",
            args: [receiverSalt],
          })
          .pipe(
            Effect.map(
              (receiverAddress) =>
                ({ receiverAddress, receiverSalt }) satisfies TronReceiverMapEntry
            )
          )
      );

      const map = new Map<string, TronReceiverMapEntry>();
      for (const entry of entries) map.set(entry.receiverAddress.toLowerCase(), entry);
      return map as ReadonlyMap<string, TronReceiverMapEntry>;
    })
  );

export class TronRelayer extends Effect.Tag("TronRelayer")<
  TronRelayer,
  {
    readonly getRelayerAddress: () => Effect.Effect<string, ConfigError.ConfigError | Error>;
    readonly getControllerEvmAddress: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
    readonly getReceiverMap: () => Effect.Effect<
      ReadonlyMap<string, TronReceiverMapEntry>,
      ConfigError.ConfigError | Error
    >;
    readonly getControllerUsdt: () => Effect.Effect<Address, ConfigError.ConfigError | Error>;
    readonly getControllerEventChainTip: () => Effect.Effect<Hex, ConfigError.ConfigError | Error>;
    readonly getControllerPulledUsdt: () => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getControllerLpExchangeRateFor: (args: {
      tokenAddress: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getErc20BalanceOf: (args: {
      tokenAddress: Address;
      account: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly getTrxBalanceOf: (args: {
      account: Address;
    }) => Effect.Effect<bigint, ConfigError.ConfigError | Error>;
    readonly sendTronControllerPullFromReceivers: (args: {
      tokenAddress: Address;
      receiverSalts: readonly Hex[];
    }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
    readonly sendTronControllerIsEventChainTip: () => Effect.Effect<
      SendTronTransactionResult,
      ConfigError.ConfigError | Error
    >;
    readonly sendTronControllerRebalanceUsdt: (args: {
      rebalancer: Address;
      inAmount: bigint;
    }) => Effect.Effect<SendTronTransactionResult, ConfigError.ConfigError | Error>;
  }
>() {
  static readonly Live = Layer.effect(
    this,
    Effect.gen(function* () {
      const appConfig = yield* AppConfig;
      const tronGrpc = yield* TronGrpc;

      const tronConfigCached = yield* Effect.cached(appConfig.tronNetwork());

      const grpcUnary = makeGrpcUnary(() => tronGrpc.get());

      const identity = yield* makeTronIdentity({ tronConfigCached });

      const tronReadContract = makeTronReadContract({
        tronGrpcGet: () => tronGrpc.get(),
        grpcUnary,
        ownerAddressBytes21: identity.controllerAddressBytes21,
      });

      const receiverMapCached = yield* makeReceiverMapCached({
        receiverSalts: identity.receiverSalts,
        controllerAddressBytes21: identity.controllerAddressBytes21,
        tronReadContract,
      });

      const controller = makeUntronControllerOperations({
        tronConfigCached,
        tronGrpcGet: () => tronGrpc.get(),
        grpcUnary,
        controllerAddressBytes21: identity.controllerAddressBytes21,
        relayerAddressBytes21Cached: identity.relayerAddressBytes21Cached,
        privateKey: identity.privateKey,
        tronReadContract,
      });

      const getErc20BalanceOf = ({
        tokenAddress,
        account,
      }: {
        tokenAddress: Address;
        account: Address;
      }) =>
        tronReadContract<bigint>({
          addressBytes21: tronEvmAddressToBytes21(tokenAddress),
          abi: ERC20Abi,
          functionName: "balanceOf",
          args: [account],
        });

      const getTrxBalanceOf = ({ account }: { account: Address }) =>
        Effect.gen(function* () {
          const { wallet } = yield* tronGrpc.get();

          const request = AccountBalanceRequest.fromPartial({
            accountIdentifier: {
              address: tronEvmAddressToBytes21(account),
            },
          });

          const res = yield* grpcUnary(
            wallet.getAccountBalance.bind(wallet) as unknown as UnaryCall<
              AccountBalanceRequest,
              AccountBalanceResponse
            >,
            request
          );

          return BigInt(res.balance.toString());
        });

      return {
        getRelayerAddress: () => identity.relayerAddressCached,
        getControllerEvmAddress: identity.controllerEvmAddress,
        getReceiverMap: () => receiverMapCached,
        getControllerUsdt: controller.getControllerUsdt,
        getControllerEventChainTip: controller.getControllerEventChainTip,
        getControllerPulledUsdt: controller.getControllerPulledUsdt,
        getControllerLpExchangeRateFor: controller.getControllerLpExchangeRateFor,
        getErc20BalanceOf,
        getTrxBalanceOf,
        sendTronControllerPullFromReceivers: controller.sendTronControllerPullFromReceivers,
        sendTronControllerIsEventChainTip: controller.sendTronControllerIsEventChainTip,
        sendTronControllerRebalanceUsdt: controller.sendTronControllerRebalanceUsdt,
      };
    })
  );
}
