import { createConfig } from "ponder";

import { createBase58check } from "@scure/base";
import { sha256 } from "@noble/hashes/sha2.js";
import { keccak_256 } from "@noble/hashes/sha3.js";
import { createPublicClient, http, type Hex } from "viem";

import { ERC20Abi } from "./abis/ERC20Abi";

import { UntronV3Abi } from "./abis/evm/UntronV3Abi";
import { TronLightClientAbi } from "./abis/evm/TronLightClientAbi";
import { TronTxReaderAbi } from "./abis/evm/TronTxReaderAbi";

import { UntronControllerAbi } from "./abis/tron/UntronControllerAbi";

const b58c = createBase58check(sha256);

function hexToBytes(hex: string): Uint8Array {
  const normalized = hex.startsWith("0x") ? hex.slice(2) : hex;
  const padded = normalized.length % 2 === 0 ? normalized : `0${normalized}`;
  const bytes = new Uint8Array(padded.length / 2);
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = parseInt(padded.slice(i * 2, i * 2 + 2), 16);
  }
  return bytes;
}

function bytesToHex(bytes: Uint8Array): Hex {
  return `0x${Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("")}` as Hex;
}

function parseBytes1(value: string): number {
  const trimmed = value.trim();
  const normalized = trimmed.startsWith("0x") ? trimmed.slice(2) : trimmed;
  if (normalized.length === 0 || normalized.length > 2) {
    throw new Error(`Invalid bytes1 value "${value}" (expected 0xNN)`);
  }
  const byte = Number.parseInt(normalized, 16);
  if (!Number.isFinite(byte) || byte < 0 || byte > 255) {
    throw new Error(`Invalid bytes1 value "${value}" (expected 0x00..0xff)`);
  }
  return byte;
}

export function tronToEVMAddress(str: string): string {
  const decoded = b58c.decode(str).slice(1);
  const hex = Array.from(decoded)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return `0x${hex}`;
}

type ReceiverSaltToEvmAddressConfig = {
  create2Prefix: number;
  deployerAddress: string;
  initCodeHash: Uint8Array;
};

export function receiverSaltToEvmAddress(
  receiverSalt: string,
  { create2Prefix, deployerAddress, initCodeHash }: ReceiverSaltToEvmAddressConfig
): string {
  const deployerBytes = hexToBytes(deployerAddress);
  if (deployerBytes.length !== 20) {
    throw new Error(`Invalid deployer address "${deployerAddress}" (expected 20 bytes).`);
  }

  // Salt (must be 32‑byte, left‑padded with zeros)
  const saltHex = receiverSalt.startsWith("0x") ? receiverSalt.slice(2) : receiverSalt;
  const saltPadded = saltHex.padStart(64, "0");
  const saltBytes = hexToBytes(saltPadded);
  if (saltBytes.length !== 32) {
    throw new Error(`Invalid receiver salt "${receiverSalt}" (expected 32 bytes).`);
  }

  if (initCodeHash.length !== 32) {
    throw new Error("Invalid initCodeHash (expected 32 bytes).");
  }

  // Build the CREATE2 pre‑image: CREATE2_PREFIX ++ deployer ++ salt ++ init_code_hash
  const data = new Uint8Array(1 + deployerBytes.length + saltBytes.length + initCodeHash.length);
  let offset = 0;
  data[offset++] = create2Prefix;
  data.set(deployerBytes, offset);
  offset += deployerBytes.length;
  data.set(saltBytes, offset);
  offset += saltBytes.length;
  data.set(initCodeHash, offset);

  // Compute the address: keccak256(data)[12:]
  const hash = keccak_256(data);
  return bytesToHex(hash.slice(-20));
}

async function getTronReceiverInitCodeHash(untronControllerAddress: Hex): Promise<Uint8Array> {
  const envHash = process.env.UNTRON_RECEIVER_INIT_CODE_HASH;
  if (envHash) {
    const bytes = hexToBytes(envHash);
    if (bytes.length !== 32) {
      throw new Error(
        "UNTRON_RECEIVER_INIT_CODE_HASH must be a 32-byte hex string (0x + 64 hex chars)."
      );
    }
    return bytes;
  }

  const rpcUrl = process.env.TRON_JSON_RPC_URL;
  if (!rpcUrl) {
    throw new Error(
      "Missing TRON_JSON_RPC_URL (required to hydrate initCodeHash from UntronController.receiverBytecode())."
    );
  }

  const client = createPublicClient({ transport: http(rpcUrl) });
  const receiverBytecode = await client.readContract({
    address: untronControllerAddress,
    abi: UntronControllerAbi,
    functionName: "receiverBytecode",
  });

  return keccak_256(hexToBytes(receiverBytecode));
}

const untronControllerAddress = tronToEVMAddress(process.env.UNTRON_CONTROLLER_ADDRESS!) as Hex;
const untronCreate2Prefix = parseBytes1(process.env.UNTRON_CONTROLLER_CREATE2_PREFIX ?? "0x41");
const tronReceiverInitCodeHash = await getTronReceiverInitCodeHash(untronControllerAddress);
const preknownReceiverAddresses = process.env.PREKNOWN_RECEIVER_SALTS!.split(",").map((salt) =>
  receiverSaltToEvmAddress(salt, {
    create2Prefix: untronCreate2Prefix,
    deployerAddress: untronControllerAddress,
    initCodeHash: tronReceiverInitCodeHash,
  })
) as `0x${string}`[];

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
  blocks: {
    mainnet: { chain: "mainnet", startBlock: "latest", interval: 1 },
    tron: { chain: "tron", startBlock: "latest", interval: 1 },
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
      address: untronControllerAddress as `0x${string}`,
      startBlock: parseInt(process.env.UNTRON_CONTROLLER_DEPLOYMENT_BLOCK!),
    },
    TRC20: {
      chain: "tron",
      abi: ERC20Abi, // TRC-20 is just how ERC-20 is called in Tron
      address: process.env
        .TRACKED_TRC20_TOKEN_ADDRESSES!.split(",")
        .map(tronToEVMAddress) as `0x${string}`[],
      // rationale: we don't care about transfers that happened before the V3 protocol was deployed
      startBlock: parseInt(process.env.UNTRON_CONTROLLER_DEPLOYMENT_BLOCK!),
      filter: {
        event: "Transfer",
        args: {
          to: preknownReceiverAddresses,
        },
      },
    },
  },
});
