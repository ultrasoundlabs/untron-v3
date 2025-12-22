import { ConfigProvider, Layer, ManagedRuntime } from "effect";

import { AppConfig } from "./config";
import { MainnetRelayer } from "../relayer/deps/mainnet";
import { PublicClients } from "../relayer/deps/publicClients";
import { TronRelayer } from "../relayer/deps/tron";
import { TronGrpc } from "../relayer/deps/tronGrpc";

const BaseLayer = Layer.mergeAll(
  Layer.setConfigProvider(ConfigProvider.fromEnv()),
  AppConfig.Live,
  PublicClients.Live
);

const TronGrpcLayer = TronGrpc.Live.pipe(Layer.provide(BaseLayer));
const BaseWithTronGrpcLayer = Layer.mergeAll(BaseLayer, TronGrpcLayer);

const MainnetRelayerLayer = MainnetRelayer.Live.pipe(Layer.provide(BaseWithTronGrpcLayer));
const TronRelayerLayer = TronRelayer.Live.pipe(Layer.provide(BaseWithTronGrpcLayer));

const RuntimeLayer = Layer.mergeAll(BaseWithTronGrpcLayer, MainnetRelayerLayer, TronRelayerLayer);

export const IndexerRuntime = ManagedRuntime.make(RuntimeLayer);
