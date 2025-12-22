import { Effect } from "effect";
import { sql } from "ponder";
import { untronV3ClaimQueue } from "ponder:schema";
import { encodeFunctionData, type Address } from "viem";

import { UntronV3Abi } from "../../../../../abis/evm/UntronV3Abi";
import { tryPromise } from "../../../../effect/tryPromise";
import { MainnetRelayer } from "../../../deps/mainnet";
import { getRows } from "../../../sqlRows";
import type { RelayJobHandlerContext } from "../../types";

const lower = (address: Address) => address.toLowerCase();

export const fillClaimsFromUntronBalance = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = ctx.ponderContext.contracts.UntronV3.address as Address;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          ${untronV3ClaimQueue.targetToken} AS targetToken,
          ${untronV3ClaimQueue.queueLength} AS queueLength
        FROM ${untronV3ClaimQueue}
        WHERE ${untronV3ClaimQueue.chainId} = ${chainId}
          AND ${untronV3ClaimQueue.contractAddress} = ${untronV3Address}
          AND ${untronV3ClaimQueue.queueLength} > 0;
      `)
    );

    const queues = getRows(result) as Array<{
      targetToken: Address;
      queueLength: bigint;
    }>;
    if (queues.length === 0) return;

    const usdtBalance = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: UntronV3Abi,
        functionName: "usdtBalance",
        blockNumber: ctx.headBlockNumber,
      })
    )) as bigint;

    if (usdtBalance === 0n) return;

    const usdtAddress = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: UntronV3Abi,
        functionName: "usdt",
        blockNumber: ctx.headBlockNumber,
      })
    )) as Address;

    const pendingQueues = yield* Effect.forEach(queues, (queue) =>
      tryPromise(() =>
        ctx.ponderContext.client.readContract({
          address: untronV3Address,
          abi: UntronV3Abi,
          functionName: "nextIndexByTargetToken",
          args: [queue.targetToken],
          blockNumber: ctx.headBlockNumber,
        })
      ).pipe(
        Effect.map((nextIndex) => {
          const next = nextIndex as bigint;
          return {
            targetToken: queue.targetToken,
            pendingCount: queue.queueLength > next ? queue.queueLength - next : 0n,
          };
        })
      )
    );

    const fillCalls = pendingQueues
      .filter((q) => q.pendingCount > 0n)
      .filter((q) => lower(q.targetToken) === lower(usdtAddress))
      .map((q) => ({
        to: untronV3Address,
        data: encodeFunctionData({
          abi: UntronV3Abi,
          functionName: "fill",
          args: [q.targetToken, q.pendingCount, []],
        }),
        value: 0n,
      }));

    if (fillCalls.length === 0) return;

    const relayer = yield* MainnetRelayer;
    yield* relayer.sendUserOperation({ calls: fillCalls });
  });
