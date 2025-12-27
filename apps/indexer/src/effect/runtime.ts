import { Config, ConfigProvider, Effect, Layer, LogLevel, Logger, ManagedRuntime } from "effect";

import { AppConfig } from "./config";
import { MainnetRelayer } from "../relayer/deps/mainnet";
import { PublicClients } from "../relayer/deps/publicClients";
import { TronGrpc, TronRelayer } from "../relayer/deps/tron";
import { UntronV3Meta } from "../relayer/deps/untronV3Meta";
import { SwapPlanner } from "../relayer/claimFiller/swapPlanner";

const MinimumLogLevel = Layer.unwrapEffect(
  Effect.gen(function* () {
    const level = yield* Config.logLevel("LOG_LEVEL").pipe(Config.withDefault(LogLevel.Info));
    return Logger.minimumLogLevel(level);
  })
);

const BaseLayer = Layer.mergeAll(
  Layer.setConfigProvider(ConfigProvider.fromEnv()),
  Logger.logFmt,
  MinimumLogLevel,
  AppConfig.Live,
  PublicClients.Live
);

const TronGrpcLayer = TronGrpc.Live.pipe(Layer.provide(BaseLayer));
const BaseWithTronGrpcLayer = Layer.mergeAll(BaseLayer, TronGrpcLayer);

const MainnetRelayerLayer = MainnetRelayer.Live.pipe(Layer.provide(BaseWithTronGrpcLayer));
const UntronV3MetaLayer = UntronV3Meta.Live.pipe(Layer.provide(BaseWithTronGrpcLayer));
const TronRelayerLayer = TronRelayer.Live.pipe(Layer.provide(BaseWithTronGrpcLayer));
const SwapPlannerLayer = SwapPlanner.Live().pipe(Layer.provide(BaseWithTronGrpcLayer));

const RuntimeLayer = Layer.mergeAll(
  BaseWithTronGrpcLayer,
  MainnetRelayerLayer,
  UntronV3MetaLayer,
  TronRelayerLayer,
  SwapPlannerLayer
);

export const IndexerRuntime = ManagedRuntime.make(RuntimeLayer);
