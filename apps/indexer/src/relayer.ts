import type { Context as PonderContext, EventNames, IndexingFunctionArgs } from "ponder:registry";

import { eventChainState, relayJob, relayerStatus, trc20Transfer } from "ponder:schema";

type BlockEventName = Extract<EventNames, `${string}:block`>;
type ContractName = keyof PonderContext["contracts"];
type PonderRegistry = (typeof import("ponder:registry"))["ponder"];

async function getHeadBlockNumber(context: PonderContext): Promise<bigint | null> {
  const hex = (await context.client.request({
    method: "eth_blockNumber",
  } as any)) as unknown;

  if (typeof hex !== "string") return null;
  return BigInt(hex);
}

async function isProbablyLiveEvent({
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

async function isSyncedForChain({
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

export function registerRelayer({
  ponder,
  enabled = process.env.RELAYER_ENABLED === "true",
  maxLagBlocks = 5n,
}: {
  ponder: PonderRegistry;
  enabled?: boolean;
  maxLagBlocks?: bigint;
}) {
  const onBlock = <TEventName extends BlockEventName>(
    blockEventName: TEventName,
    requiredContracts: readonly ContractName[]
  ) => {
    ponder.on(blockEventName, async ({ event, context }: IndexingFunctionArgs<TEventName>) => {
      const blockNumber = event.block.number as bigint;
      const blockTimestamp = event.block.timestamp as bigint;

      await context.db
        .insert(relayerStatus)
        .values({
          chainId: context.chain.id,
          isLive: true,
          headBlockNumber: blockNumber,
          headBlockTimestamp: blockTimestamp,
        })
        .onConflictDoUpdate({
          isLive: true,
          headBlockNumber: blockNumber,
          headBlockTimestamp: blockTimestamp,
        });

      if (!enabled) return;

      const isSynced = await isSyncedForChain({
        context: context as PonderContext,
        blockNumber,
        maxLagBlocks,
        requiredContracts,
      });
      if (!isSynced) return;

      const id = `${context.chain.id}:tick:${blockNumber.toString()}`;
      await context.db
        .insert(relayJob)
        .values({
          id,
          chainId: context.chain.id,
          createdAtBlockNumber: blockNumber,
          createdAtBlockTimestamp: blockTimestamp,
          kind: "tick",
          status: "pending",
          attempts: 0,
          payloadJson: { chainName: context.chain.name, blockNumber: blockNumber.toString() },
        })
        .onConflictDoNothing();
    });
  };

  onBlock("mainnet:block", ["UntronV3", "TronLightClient", "TronTxReader"]);
  onBlock("tron:block", ["UntronController"]);

  ponder.on(
    "TRC20:Transfer",
    async ({ event, context }: IndexingFunctionArgs<"TRC20:Transfer">) => {
      const chainId = context.chain.id;
      const blockNumber = event.block.number;
      const blockTimestamp = event.block.timestamp;
      const tokenAddress = event.log.address as `0x${string}`;
      const transactionHash = event.transaction.hash as `0x${string}`;
      const logIndex = event.log.logIndex as number;

      const { from, to, value } = event.args;

      await context.db
        .insert(trc20Transfer)
        .values({
          id: `${chainId}:${transactionHash.toLowerCase()}:${logIndex}`,
          chainId,
          tokenAddress,
          from,
          to,
          value,
          blockNumber,
          blockTimestamp,
          transactionHash,
          logIndex,
        })
        .onConflictDoNothing();

      if (!enabled) return;

      const isLive = await isProbablyLiveEvent({
        context: context as PonderContext,
        eventBlockNumber: blockNumber,
        maxLagBlocks,
      });
      if (!isLive) return;

      const isSynced = await isSyncedForChain({
        context,
        blockNumber,
        maxLagBlocks,
        requiredContracts: ["UntronController"],
      });
      if (!isSynced) return;

      const id = `${chainId}:trc20_transfer:${transactionHash.toLowerCase()}:${logIndex}`;
      await context.db
        .insert(relayJob)
        .values({
          id,
          chainId,
          createdAtBlockNumber: blockNumber,
          createdAtBlockTimestamp: blockTimestamp,
          kind: "trc20_transfer",
          status: "pending",
          attempts: 0,
          payloadJson: {
            tokenAddress,
            from,
            to,
            value: value.toString(),
            transactionHash,
            logIndex,
            blockNumber: blockNumber.toString(),
          },
        })
        .onConflictDoNothing();
    }
  );
}
