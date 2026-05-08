-- =========================
-- FIX: controller_usdt_balance event-sign error
-- =========================
/*
0024 introduced this view with the rebalance term added instead of subtracted, and
with `out_amount` (a destination-chain figure) treated as a Tron-side outflow. Plug
the on-chain semantics back in:

    rebalanceUsdt(rebalancer, inAmount):
      - _enforceAccounting(inAmount)            -- pulledUsdt -= inAmount
      - rebalancer.delegatecall(rebalance(usdt, inAmount, payload))
                                                -- transfers inAmount USDT OUT of
                                                -- the controller (LegacyMesh OFT.send
                                                -- or NearIntents pool.transfer)
      - emits UsdtRebalanced(inAmount, outAmount, rebalancer)

So `inAmount` is the Tron-side outflow; `outAmount` is what arrives on the bridge's
destination chain and never touches the controller's Tron USDT balance. Effective
on-chain balance is exactly:

    + pulled_from_receiver_ledger.usdt_amount
    - usdt_rebalanced_ledger.in_amount
    - controller_usdt_transfer_ledger.amount

This matches `controller.pulledUsdt()` and `usdt.balanceOf(controller)` on Tron.

Carry-forward caveats from 0024 (still apply): manual TRC-20 deposits to the controller
are not tracked, so the event-derived balance UNDER-estimates real balance in those
cases. The relayer's threshold check uses `>=`, so an underestimate produces a false
negative (skip a rebalance we could have done) — never a false positive that broadcasts
a doomed tx. Multi-USDT-version rotation still requires scoping each ledger sum by
event_seq within `usdt_versions` validity ranges; punt to that fix when version rotation
actually ships.
*/

create or replace view api.controller_usdt_balance as
select
  cu.usdt as token,
  greatest(
    0,
    coalesce(
      (select sum(usdt_amount) from ctl.pulled_from_receiver_ledger where token = cu.usdt),
      0
    )
    - coalesce((select sum(in_amount) from ctl.usdt_rebalanced_ledger), 0)
    - coalesce((select sum(amount) from ctl.controller_usdt_transfer_ledger), 0)
  )::public.u256::text as balance
from api.controller_usdt cu;

notify pgrst, 'reload schema';
