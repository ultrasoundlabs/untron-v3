import { createPublicClient, http, type PublicClient } from "viem";

import type { EvmChainName } from "./types";

export function createPublicClientGetter() {
  let mainnetPublicClient: PublicClient | null = null;
  let tronPublicClient: PublicClient | null = null;

  const getPublicClient = (chain: EvmChainName): PublicClient => {
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
  };

  return { getPublicClient };
}
