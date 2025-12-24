import { Effect } from "effect";
import { sql } from "ponder";

import { UntronV3Abi } from "../../../../../abis/evm/UntronV3Abi";
import { tryPromise } from "../../../../effect/tryPromise";
import { getRows } from "../../../sqlRows";
import { tronSweepUsdtFromReceivers } from "./usdtSweep";
import type { RelayJobHandlerContext } from "../../types";

export const sweepTronReceiversIfPendingClaims = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = ctx.ponderContext.contracts.UntronV3.address as `0x${string}`;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          target_token AS targetToken,
          queue_length AS queueLength
        FROM "untron_v3_claim_queue"
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
          AND queue_length > 0;
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
        })
      )) as bigint;

      if (queue.queueLength > nextIndex) {
        hasPendingClaims = true;
        break;
      }
    }

    if (!hasPendingClaims) return;

    yield* tronSweepUsdtFromReceivers();
  });
