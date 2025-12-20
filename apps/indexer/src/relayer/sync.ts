import type { Context as PonderContext } from "ponder:registry";

import { eventChainState, relayerStatus } from "ponder:schema";

import type { ContractName } from "./types";

async function getHeadBlockNumber(context: PonderContext): Promise<bigint | null> {
  const hex = (await context.client.request({
    method: "eth_blockNumber",
  } as any)) as unknown;

  if (typeof hex !== "string") return null;
  return BigInt(hex);
}

export async function isProbablyLiveEvent({
  context,
  eventBlockNumber,
  maxLagBlocks,
}: {
  context: PonderContext;
  eventBlockNumber: bigint;
  maxLagBlocks: bigint;
}): Promise<boolean> {
  const status = await context.db.find(relayerStatus, { chainId: context.chain.id });
  const head =
    status?.isLive === true && typeof status.headBlockNumber === "bigint"
      ? status.headBlockNumber
      : await getHeadBlockNumber(context);

  if (head === null) return false;
  if (head < eventBlockNumber) return false;
  return head - eventBlockNumber <= maxLagBlocks;
}

function requireSingleAddress(
  address: PonderContext["contracts"][ContractName]["address"]
): `0x${string}` {
  if (typeof address === "string") return address;
  if (Array.isArray(address) && address.length === 1 && typeof address[0] === "string")
    return address[0];
  throw new Error("Expected a single contract address");
}

function makeEventChainStateId({
  chainId,
  contractName,
  contractAddress,
}: {
  chainId: number;
  contractName: ContractName;
  contractAddress: string;
}): string {
  return `${chainId}:${contractName}:${contractAddress.toLowerCase()}`;
}

export async function isSyncedForChain({
  context,
  blockNumber,
  maxLagBlocks,
  requiredContracts,
}: {
  context: PonderContext;
  blockNumber: bigint;
  maxLagBlocks: bigint;
  requiredContracts: readonly ContractName[];
}): Promise<boolean> {
  const minRequired = blockNumber > maxLagBlocks ? blockNumber - maxLagBlocks : 0n;

  for (const contractName of requiredContracts) {
    const contractConfig = context.contracts[contractName];
    const contractAddress = requireSingleAddress(contractConfig.address);

    const id = makeEventChainStateId({
      chainId: context.chain.id,
      contractName,
      contractAddress,
    });

    const state = await context.db.find(eventChainState, { id });
    if (!state) return false;
    if (state.lastEventBlockNumber < minRequired) return false;
  }

  return true;
}
