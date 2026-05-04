-- =========================
-- CONTROLLER USDT BALANCE (event-derived)
-- =========================
/*
Why:
- The relayer's rebalance plan polls `usdt.balanceOf(controller)` over Tron RPC every tick to
  decide whether the controller's USDT balance has crossed the rebalance threshold. Pure read
  traffic (~12 calls/min, ~520k/month) against a paid RPC tier.
- The indexer already records every event that moves USDT in or out of the controller. This
  view aggregates those ledgers to give the same number without an RPC hop.

Balance arithmetic (all amounts are u256 stored as numeric(78,0), so Postgres handles them
without overflow):

    + sum(ctl.pulled_from_receiver_ledger.usdt_amount)   -- inflow: receiver -> controller
    + sum(ctl.usdt_rebalanced_ledger.in_amount)          -- inflow: rebalancer top-ups
    - sum(ctl.usdt_rebalanced_ledger.out_amount)         -- outflow: rebalanced out
    - sum(ctl.controller_usdt_transfer_ledger.amount)    -- outflow: generic controller transfer

Caveats:
- This counts only USDT movements emitted as controller events. Direct TRC-20 transfers TO the
  controller (e.g. an operator manually topping it up outside the protocol) are not tracked,
  so the event-derived balance UNDER-estimates the on-chain balance in those cases. The
  rebalance threshold check uses `>=`, so an underestimate produces a false negative (we skip
  a rebalance we could have done) — never a false positive that broadcasts a doomed tx.
- The rebalance and controller_usdt_transfer ledgers do not record the token address (the
  controller emits them implicitly against the active USDT version). This view assumes a
  single active USDT at a time, which is true today. If the controller ever rotates USDT
  versions, residual amounts of the old token would skew the new token's balance — fix at
  that point by scoping each ledger sum by event_seq within each `usdt_versions` validity
  range.
- Stale projection: like every event-derived view, this is only as fresh as
  `chain.stream_cursor.applied_through_seq`. Consumers should already be checking
  `api.stream_ingest_summary.is_projection_caught_up` before trusting derived state.
*/

create or replace view api.controller_usdt_balance as
select
  cu.usdt as token,
  -- Clamp to >= 0. The on-chain controller balance can never go negative, but during indexer
  -- catch-up an outflow event may briefly land before the matching inflow, producing a
  -- transient negative sum that would otherwise fail the u256 domain check (which forbids
  -- negative values). Clamping returns a conservative-but-valid 0 in that window; the
  -- relayer reads this as "nothing to rebalance" and skips, which is safe.
  greatest(
    0,
    coalesce(
      (select sum(usdt_amount) from ctl.pulled_from_receiver_ledger where token = cu.usdt),
      0
    )
    + coalesce((select sum(in_amount) from ctl.usdt_rebalanced_ledger), 0)
    - coalesce((select sum(out_amount) from ctl.usdt_rebalanced_ledger), 0)
    - coalesce((select sum(amount) from ctl.controller_usdt_transfer_ledger), 0)
  )::public.u256::text as balance
from api.controller_usdt cu;

comment on view api.controller_usdt_balance is
$$Event-derived USDT balance held by the controller, for the currently active USDT.

Lets relayers replace per-tick `usdt.balanceOf(controller)` Tron RPC reads with a
postgrest query. See the migration source for caveats around manual deposits and
multi-token rotation.$$;

notify pgrst, 'reload schema';
