import { Effect } from "effect";
import { sql } from "ponder";
import { untronV3ClaimQueue } from "ponder:schema";

import { UntronV3Abi } from "../../../../../abis/evm/UntronV3Abi";
import { tryPromise } from "../../../../effect/tryPromise";
import { getRows } from "../../../sqlRows";
import { tronSweepFromReceivers } from "./tronSweep";
import type { RelayJobHandlerContext } from "../../types";

export const sweepTronReceiversIfPendingClaims = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = ctx.ponderContext.contracts.UntronV3.address as `0x${string}`;

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

    const nonEmptyQueues = getRows(result) as Array<{
      targetToken: `0x${string}`;
      queueLength: bigint;
    }>;
    if (nonEmptyQueues.length === 0) return;

    let hasPendingClaims = false;
    for (const queue of nonEmptyQueues) {
      const nextIndex = (yield* tryPromise(() =>
        ctx.ponderContext.client.readContract({
          address: untronV3Address,
          abi: UntronV3Abi,
          functionName: "nextIndexByTargetToken",
          args: [queue.targetToken],
          blockNumber: ctx.headBlockNumber,
        })
      )) as bigint;

      if (queue.queueLength > nextIndex) {
        hasPendingClaims = true;
        break;
      }
    }

    if (!hasPendingClaims) return;

    yield* tronSweepFromReceivers(ctx);
  });
