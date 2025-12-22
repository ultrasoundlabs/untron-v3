import type { createTronClients } from "@untron/tron-protocol";
import type { Address, Hash, Hex, PublicClient } from "viem";

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

export type RelayerDeps = {
  getPublicClient: (chain: EvmChainName) => PublicClient;
  getTronGrpcClients: () => TronGrpcClients;

  getMainnetRelayerAddress: () => Promise<Address>;
  sendMainnetUserOperation: (args: {
    calls: readonly MainnetUserOperationCall[];
    bundlerUrls?: readonly string[];
    timeoutBlocks?: bigint;
    pollIntervalMs?: number;
  }) => Promise<SendMainnetUserOperationResult>;

  getTronRelayerAddress: () => string;
  getTronControllerEvmAddress: () => Address;
  getTronReceiverMap: () => Promise<ReadonlyMap<string, TronReceiverMapEntry>>;
  sendTronControllerPullFromReceivers: (args: {
    tokenAddress: Address;
    receiverSalts: readonly Hex[];
  }) => Promise<SendTronTransactionResult>;
  sendTronControllerRebalanceUsdt: (args: {
    rebalancer: Address;
    inAmount: bigint;
  }) => Promise<SendTronTransactionResult>;
};
