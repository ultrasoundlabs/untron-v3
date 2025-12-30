import { Cause, Effect } from "effect";
import type { Context as PonderContext, IndexingFunctionArgs } from "ponder:registry";

import { relayerStatus, trc20Transfer, tronLightClientPublishRequest } from "ponder:schema";
import { sql } from "ponder";

import { getTronLightClientAddress } from "../contracts";
import { AppConfig } from "../effect/config";
import { BackendRuntime } from "../effect/runtime";
import { MAINNET_CHAIN_ID } from "../env";

import { enqueueRelayJob } from "./queue";
import { processRelayJobs } from "./processor";
import { getRows } from "./sqlRows";
import type { BlockEventName, PonderRegistry, RelayJobKind } from "./types";

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

const getRpcHeadBlockNumber = (context: PonderContext): Effect.Effect<bigint | null, Error> =>
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

const isProbablyLiveEvent = (args: {
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

export function registerRelayer({
  ponder,
  enabled,
  embeddedExecutorEnabled,
  dryRun,
  maxLagBlocks,
}: {
  ponder: PonderRegistry;
  enabled?: boolean;
  embeddedExecutorEnabled?: boolean;
  dryRun?: boolean;
  maxLagBlocks?: bigint;
}) {
  const hasOutstandingHeartbeatJob = (args: {
    context: PonderContext;
    chainId: number;
    kind: "mainnet_heartbeat" | "tron_heartbeat";
    workerId: string;
  }) =>
    Effect.tryPromise({
      try: async () => {
        const result = await args.context.db.sql.execute(sql`
          SELECT 1 AS one
          FROM "relay_job"
          WHERE chain_id = ${args.chainId}
            AND "kind" = ${args.kind}
            AND (
              "status" = 'pending'
              OR ("status" = 'processing' AND locked_by = ${args.workerId})
            )
          LIMIT 1;
        `);
        return getRows(result).length > 0;
      },
      catch: (error) => (error instanceof Error ? error : new Error(String(error))),
    });

  const onBlock = <TEventName extends BlockEventName>(
    blockEventName: TEventName,
    heartbeatKind: RelayJobKind
  ) => {
    ponder.on(blockEventName, ({ event, context }: IndexingFunctionArgs<TEventName>) =>
      BackendRuntime.runPromise(
        Effect.gen(function* () {
          const runtime = yield* AppConfig.relayerRuntime();
          const resolvedEnabled = enabled ?? runtime.enabled;
          const resolvedEmbeddedExecutorEnabled =
            embeddedExecutorEnabled ?? runtime.embeddedExecutorEnabled;
          const resolvedDryRun = dryRun ?? runtime.dryRun;
          const resolvedMaxLagBlocks = maxLagBlocks ?? runtime.maxLagBlocks;

          const blockNumber = event.block.number as bigint;
          const blockTimestamp = event.block.timestamp as bigint;

          const rpcHead = yield* getRpcHeadBlockNumber(context as PonderContext);
          const isLive =
            rpcHead !== null &&
            rpcHead >= blockNumber &&
            rpcHead - blockNumber <= resolvedMaxLagBlocks;

          yield* upsertRelayerStatus({
            context: context as PonderContext,
            isLive,
            headBlockNumber: rpcHead ?? blockNumber,
            headBlockTimestamp: blockTimestamp,
          });

          if (!resolvedEnabled) return;

          if (!isLive) {
            const lag = rpcHead !== null && rpcHead >= blockNumber ? rpcHead - blockNumber : null;
            yield* Effect.logDebug("[relayer] skipping enqueue (not live)").pipe(
              Effect.annotateLogs({
                chainId: context.chain.id,
                blockNumber: blockNumber.toString(),
                rpcHead: rpcHead?.toString() ?? "null",
                lag: lag?.toString() ?? "null",
                maxLagBlocks: resolvedMaxLagBlocks.toString(),
              })
            );
            return;
          }

          if (heartbeatKind === "mainnet_heartbeat" || heartbeatKind === "tron_heartbeat") {
            const outstanding = yield* hasOutstandingHeartbeatJob({
              context: context as PonderContext,
              chainId: context.chain.id,
              kind: heartbeatKind,
              workerId: runtime.workerId,
            });
            if (outstanding) return;
          }

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

            yield* processRelayJobs({
              context: context as PonderContext,
              chainId: context.chain.id,
              kind: "relay_controller_event_chain",
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
        }).pipe(
          Effect.tapErrorCause(
            (cause): Effect.Effect<void, never, never> =>
              Effect.logError("[relayer] block handler failed").pipe(
                Effect.annotateLogs({
                  blockEventName,
                  chainId: context.chain.id,
                  blockNumber: String(event.block.number),
                  cause: Cause.pretty(cause),
                })
              )
          )
        )
      )
    );
  };

  onBlock("mainnet:block", "mainnet_heartbeat");
  onBlock("tron:block", "tron_heartbeat");

  ponder.on("TRC20:Transfer", ({ event, context }: IndexingFunctionArgs<"TRC20:Transfer">) =>
    BackendRuntime.runPromise(
      Effect.gen(function* () {
        const runtime = yield* AppConfig.relayerRuntime();
        const resolvedEnabled = enabled ?? runtime.enabled;
        const resolvedMaxLagBlocks = maxLagBlocks ?? runtime.maxLagBlocks;

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
          maxLagBlocks: resolvedMaxLagBlocks,
        });
        if (!isLive) return;

        const tronLightClientAddress = getTronLightClientAddress() as `0x${string}`;
        yield* Effect.tryPromise(() =>
          (context as PonderContext).db
            .insert(tronLightClientPublishRequest)
            .values({
              id: `${MAINNET_CHAIN_ID}:${tronLightClientAddress}:${blockNumber.toString()}`,
              chainId: MAINNET_CHAIN_ID,
              tronLightClientAddress,
              tronBlockNumber: blockNumber,
              requestedAtTronBlockTimestamp: blockTimestamp,
              source: "trc20_transfer",
            })
            .onConflictDoNothing()
        );

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
      }).pipe(
        Effect.tapErrorCause(
          (cause): Effect.Effect<void, never, never> =>
            Effect.logError("[relayer] TRC20:Transfer handler failed").pipe(
              Effect.annotateLogs({
                chainId: context.chain.id,
                blockNumber: String(event.block.number),
                transactionHash: event.transaction.hash,
                logIndex: String(event.log.logIndex),
                cause: Cause.pretty(cause),
              })
            )
        )
      )
    )
  );
}
