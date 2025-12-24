import { Effect } from "effect";
import type { Context as PonderContext } from "ponder:registry";

import { eventChainState, relayerStatus } from "ponder:schema";

import type { ContractName } from "./types";

export const getRpcHeadBlockNumber = (
  context: PonderContext
): Effect.Effect<bigint | null, Error> =>
  Effect.tryPromise({
    try: async () => {
      const hex = (await context.client.request({
        method: "eth_blockNumber",
      } as any)) as unknown;

      if (typeof hex !== "string") return null;
      return BigInt(hex);
    },
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });

export const isProbablyLiveEvent = (args: {
  context: PonderContext;
  eventBlockNumber: bigint;
  maxLagBlocks: bigint;
}): Effect.Effect<boolean, Error> =>
  Effect.gen(function* () {
    const status = yield* Effect.tryPromise({
      try: () => args.context.db.find(relayerStatus, { chainId: args.context.chain.id }),
      catch: (error) => (error instanceof Error ? error : new Error(String(error))),
    });

    const head =
      status?.isLive === true && typeof status.headBlockNumber === "bigint"
        ? status.headBlockNumber
        : yield* getRpcHeadBlockNumber(args.context);

    if (head === null) return false;
    if (head < args.eventBlockNumber) return false;
    return head - args.eventBlockNumber <= args.maxLagBlocks;
  });

const requireSingleAddress = (
  address: PonderContext["contracts"][ContractName]["address"]
): `0x${string}` => {
  if (typeof address === "string") return address;
  if (Array.isArray(address) && address.length === 1 && typeof address[0] === "string")
    return address[0];
  throw new Error("Expected a single contract address");
};

const makeEventChainStateId = (args: {
  chainId: number;
  contractName: ContractName;
  contractAddress: string;
}): string => `${args.chainId}:${args.contractName}:${args.contractAddress.toLowerCase()}`;

export const isSyncedForChain = (args: {
  context: PonderContext;
  blockNumber: bigint;
  maxLagBlocks: bigint;
  requiredContracts: readonly ContractName[];
}): Effect.Effect<boolean, Error> =>
  Effect.gen(function* () {
    const minRequired =
      args.blockNumber > args.maxLagBlocks ? args.blockNumber - args.maxLagBlocks : 0n;

    for (const contractName of args.requiredContracts) {
      const contractConfig = args.context.contracts[contractName];
      const contractAddress = requireSingleAddress(contractConfig.address);

      const id = makeEventChainStateId({
        chainId: args.context.chain.id,
        contractName,
        contractAddress,
      });

      const state = yield* Effect.tryPromise({
        try: () => args.context.db.find(eventChainState, { id }),
        catch: (error) => (error instanceof Error ? error : new Error(String(error))),
      });
      if (!state) return false;
      if (state.lastEventBlockNumber < minRequired) return false;
    }

    return true;
  });
