import { createConfig } from "ponder";

import { createBase58check } from "@scure/base";
import { sha256 } from "@noble/hashes/sha2.js";

import { UntronV3Abi } from "./abis/evm/UntronV3Abi";
import { TronLightClientAbi } from "./abis/evm/TronLightClientAbi";
import { TronTxReaderAbi } from "./abis/evm/TronTxReaderAbi";

import { UntronControllerAbi } from "./abis/tron/UntronControllerAbi";

const b58c = createBase58check(sha256);

export function tronToEVMAddress(str: string): string {
  const decoded = b58c.decode(str).slice(1);
  const hex = Array.from(decoded)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return `0x${hex}`;
}

export default createConfig({
  chains: {
    mainnet: {
      id: parseInt(process.env.UNTRON_V3_CHAIN_ID!),
      rpc: process.env.UNTRON_V3_CHAIN_RPC_URL!,
    },
    tron: {
      id: 728126428, // tron doesn't use chain ID but eth_chainId returns 728126428
      rpc: process.env.TRON_JSON_RPC_URL!,
    },
  },
  contracts: {
    UntronV3: {
      chain: "mainnet",
      abi: UntronV3Abi,
      address: process.env.UNTRON_V3_ADDRESS! as `0x${string}`,
      startBlock: parseInt(process.env.UNTRON_V3_DEPLOYMENT_BLOCK!),
    },
    TronLightClient: {
      chain: "mainnet",
      abi: TronLightClientAbi,
      address: process.env.TRON_LIGHT_CLIENT_ADDRESS! as `0x${string}`,
      startBlock: parseInt(process.env.TRON_LIGHT_CLIENT_DEPLOYMENT_BLOCK!),
    },
    TronTxReader: {
      chain: "mainnet",
      abi: TronTxReaderAbi,
      address: process.env.TRON_TX_READER_ADDRESS! as `0x${string}`,
      startBlock: parseInt(process.env.TRON_TX_READER_DEPLOYMENT_BLOCK!),
    },
    UntronController: {
      chain: "tron",
      abi: UntronControllerAbi,
      address: tronToEVMAddress(process.env.UNTRON_CONTROLLER_ADDRESS!) as `0x${string}`,
      startBlock: parseInt(process.env.UNTRON_CONTROLLER_DEPLOYMENT_BLOCK!),
    },
  },
});
