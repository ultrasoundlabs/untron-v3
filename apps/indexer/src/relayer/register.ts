import type { Context as PonderContext, IndexingFunctionArgs } from "ponder:registry";

import { relayerStatus, trc20Transfer } from "ponder:schema";

import { createRelayerDeps } from "./deps";
import { getRelayerRuntimeConfig } from "./env";
import { enqueueRelayJob } from "./queue";
import { processRelayJobs } from "./processor";
import { isProbablyLiveEvent, isSyncedForChain } from "./sync";
import type { BlockEventName, ContractName, PonderRegistry, RelayJobKind } from "./types";

export function registerRelayer({
  ponder,
  enabled,
  embeddedExecutorEnabled,
  dryRun,
  maxLagBlocks = 2n,
}: {
  ponder: PonderRegistry;
  enabled?: boolean;
  embeddedExecutorEnabled?: boolean;
  dryRun?: boolean;
  maxLagBlocks?: bigint;
}) {
  const runtime = getRelayerRuntimeConfig();
  const resolvedEnabled = enabled ?? runtime.enabled;
  const resolvedEmbeddedExecutorEnabled =
    embeddedExecutorEnabled ?? runtime.embeddedExecutorEnabled;
  const resolvedDryRun = dryRun ?? runtime.dryRun;

  const deps = createRelayerDeps();

  const onBlock = <TEventName extends BlockEventName>(
    blockEventName: TEventName,
    heartbeatKind: RelayJobKind,
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

      if (!resolvedEnabled) return;

      const isSynced = await isSyncedForChain({
        context: context as PonderContext,
        blockNumber,
        maxLagBlocks,
        requiredContracts,
      });
      if (!isSynced) return;

      await enqueueRelayJob({
        context,
        id: `${context.chain.id}:${heartbeatKind}:${blockNumber.toString()}`,
        chainId: context.chain.id,
        createdAtBlockNumber: blockNumber,
        createdAtBlockTimestamp: blockTimestamp,
        kind: heartbeatKind,
        payloadJson: { chainName: context.chain.name, blockNumber: blockNumber.toString() },
      });

      if (!resolvedEmbeddedExecutorEnabled) return;

      const minConfirmations =
        heartbeatKind === "mainnet_heartbeat"
          ? runtime.mainnetConfirmations
          : runtime.tronConfirmations;

      await processRelayJobs({
        context,
        chainId: context.chain.id,
        kind: heartbeatKind,
        headBlockNumber: blockNumber,
        headBlockTimestamp: blockTimestamp,
        minConfirmations,
        workerId: runtime.workerId,
        limit: runtime.claimLimit,
        dryRun: resolvedDryRun,
        maxAttempts: runtime.maxAttempts,
        retryDelayBlocks: runtime.retryDelayBlocks,
        deps,
      });

      if (heartbeatKind === "tron_heartbeat") {
        await processRelayJobs({
          context,
          chainId: context.chain.id,
          kind: "trc20_transfer",
          headBlockNumber: blockNumber,
          headBlockTimestamp: blockTimestamp,
          minConfirmations: runtime.tronConfirmations,
          workerId: runtime.workerId,
          limit: runtime.claimLimit,
          dryRun: resolvedDryRun,
          maxAttempts: runtime.maxAttempts,
          retryDelayBlocks: runtime.retryDelayBlocks,
          deps,
        });
      }
    });
  };

  onBlock("mainnet:block", "mainnet_heartbeat", ["UntronV3", "TronLightClient", "TronTxReader"]);
  onBlock("tron:block", "tron_heartbeat", ["UntronController"]);

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

      if (!resolvedEnabled) return;

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

      await enqueueRelayJob({
        context,
        id: `${chainId}:trc20_transfer:${transactionHash.toLowerCase()}:${logIndex}`,
        chainId,
        createdAtBlockNumber: blockNumber,
        createdAtBlockTimestamp: blockTimestamp,
        kind: "trc20_transfer",
        payloadJson: {
          tokenAddress,
          from,
          to,
          value: value.toString(),
          transactionHash,
          logIndex,
          blockNumber: blockNumber.toString(),
        },
      });
    }
  );
}
