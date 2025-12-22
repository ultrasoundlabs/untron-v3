import { Effect } from "effect";
import type { Context as PonderContext, IndexingFunctionArgs } from "ponder:registry";

import { relayerStatus, trc20Transfer } from "ponder:schema";

import { AppConfig } from "../effect/config";
import { IndexerRuntime } from "../effect/runtime";

import { enqueueRelayJob } from "./queue";
import { processRelayJobs } from "./processor";
import { isProbablyLiveEvent, isSyncedForChain } from "./sync";
import type { BlockEventName, ContractName, PonderRegistry, RelayJobKind } from "./types";

const upsertRelayerStatus = (args: {
  context: PonderContext;
  isLive: boolean;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
}) =>
  Effect.tryPromise(() =>
    args.context.db
      .insert(relayerStatus)
      .values({
        chainId: args.context.chain.id,
        isLive: args.isLive,
        headBlockNumber: args.headBlockNumber,
        headBlockTimestamp: args.headBlockTimestamp,
      })
      .onConflictDoUpdate({
        isLive: args.isLive,
        headBlockNumber: args.headBlockNumber,
        headBlockTimestamp: args.headBlockTimestamp,
      })
  );

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
  const onBlock = <TEventName extends BlockEventName>(
    blockEventName: TEventName,
    heartbeatKind: RelayJobKind,
    requiredContracts: readonly ContractName[]
  ) => {
    ponder.on(blockEventName, ({ event, context }: IndexingFunctionArgs<TEventName>) =>
      IndexerRuntime.runPromise(
        Effect.gen(function* () {
          const runtime = yield* AppConfig.relayerRuntime();
          const resolvedEnabled = enabled ?? runtime.enabled;
          const resolvedEmbeddedExecutorEnabled =
            embeddedExecutorEnabled ?? runtime.embeddedExecutorEnabled;
          const resolvedDryRun = dryRun ?? runtime.dryRun;

          const blockNumber = event.block.number as bigint;
          const blockTimestamp = event.block.timestamp as bigint;

          yield* upsertRelayerStatus({
            context: context as PonderContext,
            isLive: true,
            headBlockNumber: blockNumber,
            headBlockTimestamp: blockTimestamp,
          });

          if (!resolvedEnabled) return;

          const isSynced = yield* isSyncedForChain({
            context: context as PonderContext,
            blockNumber,
            maxLagBlocks,
            requiredContracts,
          });
          if (!isSynced) return;

          yield* enqueueRelayJob({
            context: context as PonderContext,
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

          yield* processRelayJobs({
            context: context as PonderContext,
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
          });

          if (heartbeatKind === "tron_heartbeat") {
            yield* processRelayJobs({
              context: context as PonderContext,
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
            });
          }
        })
      )
    );
  };

  onBlock("mainnet:block", "mainnet_heartbeat", ["UntronV3", "TronLightClient"]);
  onBlock("tron:block", "tron_heartbeat", ["UntronController"]);

  ponder.on("TRC20:Transfer", ({ event, context }: IndexingFunctionArgs<"TRC20:Transfer">) =>
    IndexerRuntime.runPromise(
      Effect.gen(function* () {
        const runtime = yield* AppConfig.relayerRuntime();
        const resolvedEnabled = enabled ?? runtime.enabled;

        const chainId = context.chain.id;
        const blockNumber = event.block.number;
        const blockTimestamp = event.block.timestamp;
        const tokenAddress = event.log.address as `0x${string}`;
        const transactionHash = event.transaction.hash as `0x${string}`;
        const logIndex = event.log.logIndex as number;

        const { from, to, value } = event.args;

        yield* Effect.tryPromise(() =>
          (context as PonderContext).db
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
            .onConflictDoNothing()
        );

        if (!resolvedEnabled) return;

        const isLive = yield* isProbablyLiveEvent({
          context: context as PonderContext,
          eventBlockNumber: blockNumber,
          maxLagBlocks,
        });
        if (!isLive) return;

        const isSynced = yield* isSyncedForChain({
          context: context as PonderContext,
          blockNumber,
          maxLagBlocks,
          requiredContracts: ["UntronController"],
        });
        if (!isSynced) return;

        yield* enqueueRelayJob({
          context: context as PonderContext,
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
      })
    )
  );
}
