import type { createTronClients } from "@untron/tron-protocol";
import type { Address, Hash, Hex } from "viem";

export type TronGrpcClients = ReturnType<typeof createTronClients>;
export type EvmChainName = "mainnet" | "tron";

export type EntryPointVersion = "0.6" | "0.7";

export type MainnetUserOperationCall = {
  to: Address;
  data?: Hex;
  value?: bigint;
};

export type SendMainnetUserOperationResult = {
  bundlerUrl: string;
  userOpHash: Hash;
  transactionHash: Hash;
  blockNumber: bigint;
};

export type TronReceiverMapEntry = {
  receiverAddress: Address;
  receiverSalt: Hex;
};

export type SendTronTransactionResult = {
  txid: string;
};
