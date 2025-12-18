import { createConfig } from "ponder";

import { UntronV3Abi } from "./abis/UntronV3Abi";

export default createConfig({
  chains: {
    mainnet: {
      id: parseInt(process.env.UNTRON_V3_CHAIN_ID!),
      rpc: process.env.UNTRON_V3_CHAIN_RPC_URL!,
    },
  },
  contracts: {
    UntronV3: {
      chain: "mainnet",
      abi: UntronV3Abi,
      address: process.env.UNTRON_V3_ADDRESS! as `0x${string}`,
      startBlock: parseInt(process.env.UNTRON_V3_DEPLOYMENT_BLOCK!),
    },
  },
});
