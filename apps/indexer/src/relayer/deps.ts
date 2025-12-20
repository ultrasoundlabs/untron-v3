import { createTronClients } from "@untron/tron-protocol";
import { createPublicClient, http, type PublicClient } from "viem";

export type TronGrpcClients = ReturnType<typeof createTronClients>;
export type EvmChainName = "mainnet" | "tron";

export type RelayerDeps = {
  getPublicClient: (chain: EvmChainName) => PublicClient;
  getTronGrpcClients: () => TronGrpcClients;
};

export function createRelayerDeps(): RelayerDeps {
  let mainnetPublicClient: PublicClient | null = null;
  let tronPublicClient: PublicClient | null = null;
  let tronGrpcClients: TronGrpcClients | null = null;

  return {
    getPublicClient: (chain) => {
      if (chain === "mainnet") {
        if (!mainnetPublicClient) {
          const rpcUrl = process.env.UNTRON_V3_CHAIN_RPC_URL;
          if (!rpcUrl) throw new Error("Missing env var UNTRON_V3_CHAIN_RPC_URL");
          mainnetPublicClient = createPublicClient({ transport: http(rpcUrl) });
        }
        return mainnetPublicClient;
      }

      if (!tronPublicClient) {
        const rpcUrl = process.env.TRON_JSON_RPC_URL;
        if (!rpcUrl) throw new Error("Missing env var TRON_JSON_RPC_URL");
        tronPublicClient = createPublicClient({ transport: http(rpcUrl) });
      }
      return tronPublicClient;
    },

    getTronGrpcClients: () => {
      if (tronGrpcClients) return tronGrpcClients;

      const host = process.env.TRON_GRPC_HOST;
      if (!host) throw new Error("Missing env var TRON_GRPC_HOST");

      const apiKey = process.env.TRON_API_KEY;
      const insecure = process.env.TRON_GRPC_INSECURE === "true";

      tronGrpcClients = createTronClients(host, apiKey, { insecure });
      return tronGrpcClients;
    },
  };
}
