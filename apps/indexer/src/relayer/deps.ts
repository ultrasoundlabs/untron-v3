import { createMainnetRelayer } from "./deps/mainnet";
import { createPublicClientGetter } from "./deps/publicClients";
import { createTronGrpcClientsGetter } from "./deps/tronGrpc";
import { createTronRelayer } from "./deps/tron";
import type { RelayerDeps } from "./deps/types";

export type { RelayerDeps } from "./deps/types";

export function createRelayerDeps(): RelayerDeps {
  const { getPublicClient } = createPublicClientGetter();
  const { getTronGrpcClients } = createTronGrpcClientsGetter();

  const base: Pick<RelayerDeps, "getPublicClient" | "getTronGrpcClients"> = {
    getPublicClient,
    getTronGrpcClients,
  };

  const mainnet: Pick<RelayerDeps, "getMainnetRelayerAddress" | "sendMainnetUserOperation"> =
    createMainnetRelayer({
      getPublicClient: (chain) => getPublicClient(chain),
    });

  const tron: Pick<
    RelayerDeps,
    | "getTronRelayerAddress"
    | "getTronControllerEvmAddress"
    | "getTronReceiverMap"
    | "sendTronControllerPullFromReceivers"
    | "sendTronControllerRebalanceUsdt"
  > = createTronRelayer({ getTronGrpcClients });

  return {
    ...base,
    ...mainnet,
    ...tron,
  } satisfies RelayerDeps;
}
