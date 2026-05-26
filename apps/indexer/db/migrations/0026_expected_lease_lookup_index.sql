-- =========================
-- PERF: expected lease lookup for deposit actionability
-- =========================
/*
The PostgREST-facing api.usdt_deposit_txs view joins
api.receiver_usdt_transfer_actionability, which derives expected_lease_id for
each receiver USDT transfer with this lookup:

    select lv.lease_id
    from hub.lease_versions lv
    where lv.receiver_salt = t.receiver_salt
      and lv.start_time <= t.block_timestamp
      and lv.nukeable_after > t.block_timestamp
    order by lv.start_time desc
    limit 1

In production this ran as a repeated seq scan over hub.lease_versions for every
deposit row (~57k rows scanned per transfer), which made simple paginated
/usdt_deposit_txs reads take seconds and exact-count reads long enough to trip
the api.untron.finance gateway timeout.

This index is intentionally NOT partial on valid_to_seq: the lookup above is a
historical time-window lookup and does not currently constrain to the current
version. Existing current-only lease indexes therefore cannot support it.
*/

create index if not exists hub_lease_versions_receiver_salt_start_time_lookup
on hub.lease_versions (receiver_salt, start_time desc, nukeable_after)
include (lease_id);
