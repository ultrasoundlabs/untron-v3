import { Effect } from "effect";
import { sql } from "ponder";
import { eventChainState } from "ponder:schema";

import { tryPromise } from "../../../../effect/tryPromise";
import { TronRelayer } from "../../../deps/tron";
import { getRows } from "../../../sqlRows";
import type { RelayJobHandlerContext } from "../../types";

export const ensureIsEventChainTipCalled = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const controllerAddress = (
      ctx.ponderContext.contracts.UntronController.address as `0x${string}`
    ).toLowerCase() as `0x${string}`;

    const state = yield* tryPromise(() =>
      ctx.ponderContext.db.find(eventChainState, {
        id: `${chainId}:UntronController:${controllerAddress}`,
      })
    );
    if (!state) return;

    const result = yield* tryPromise(() =>
      ctx.ponderContext.db.sql.execute(sql`
        SELECT
          event_chain_tip AS "eventChainTip"
        FROM "untron_controller_is_event_chain_tip_called"
        WHERE chain_id = ${chainId}
          AND contract_address = ${controllerAddress}
        ORDER BY block_number DESC,
          log_index DESC
        LIMIT 1;
      `)
    );

    const rows = getRows(result) as Array<{ eventChainTip: `0x${string}` }>;
    const lastCalledTip = rows[0]?.eventChainTip;
    if (lastCalledTip && lastCalledTip.toLowerCase() === state.eventChainTip.toLowerCase()) {
      return;
    }

    const onchainTip = yield* TronRelayer.getControllerEventChainTip();
    if (onchainTip.toLowerCase() !== state.eventChainTip.toLowerCase()) return;

    yield* TronRelayer.sendTronControllerIsEventChainTip();
  });
