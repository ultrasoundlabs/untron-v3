-- Ensure relayer/operator views ignore receivers that are still backfilling.
--
-- Motivation:
-- The receiver_usdt indexer maintains per-receiver backfill cursors in
-- `ctl.receiver_watchlist.backfill_next_block`. When a new receiver is
-- discovered, it may require a large historical backfill, but we still want
-- relayers to keep operating for receivers whose backfills are already done.
--
-- This migration makes the receiver-USDT-derived API views (used by relayers)
-- only include transfers for receivers with `backfill_next_block IS NULL`.

-- Canonical receiver transfers that have not yet been accounted for by hub
-- pre-entitlements.
create or replace view api.unaccounted_receiver_usdt_transfers as
select
    t.chain_id,
    t.token,
    t.receiver_salt,
    t.sender,
    t.recipient,
    t.amount,
    t.block_number,
    t.block_timestamp,
    to_timestamp(t.block_timestamp) as block_time,
    t.block_hash,
    t.tx_hash,
    t.log_index,
    -- best-effort attribution: expected active lease at this Tron timestamp
    (
        select lv.lease_id
        from hub.lease_versions lv
        where
            lv.receiver_salt = t.receiver_salt
            and lv.start_time <= t.block_timestamp
            and lv.nukeable_after > t.block_timestamp
        order by lv.start_time desc
        limit 1
    ) as expected_lease_id
from ctl.receiver_usdt_transfers t
join ctl.receiver_watchlist w
    on w.receiver_salt = t.receiver_salt
left join hub.claim_versions c
    on
        c.valid_to_seq is null
        and c.origin in (0, 1)
        and c.origin_id = t.tx_hash
where
    t.canonical
    and c.lease_id is null
    and w.backfill_next_block is null;

comment on view api.unaccounted_receiver_usdt_transfers is
$$Unaccounted receiver USDT deposits

Tron TRC-20 Transfer logs not yet reflected as hub claims

Rows in this view correspond to canonical TRC-20 USDT transfers into
fully-indexed deterministic receivers that do NOT yet have a hub-side claim with
`origin in (pre-entitle, subjective pre-entitle)` matching
`origin_id = tx_hash`.

Note: receivers that are still backfilling (`ctl.receiver_watchlist.backfill_next_block is not null`)
are intentionally excluded so relayers can continue operating for already-synced receivers.$$;

-- Operator-oriented view: include pre-entitle eligibility + suggested next
-- action.
--
-- NOTE: Semantics follow 0012_actionability_subjective_pre_entitle.sql; we only
-- add the backfill gating join.
create or replace view api.receiver_usdt_transfer_actionability as
with last_pull as (
    select
        l.receiver_salt,
        l.token,
        max(e.block_timestamp) as last_pull_timestamp
    from ctl.pulled_from_receiver_ledger l
    join chain.event_appended e
        on
            e.stream = 'controller'
            and e.canonical
            and e.event_seq = l.event_seq
    group by l.receiver_salt, l.token
)

select
    t.chain_id,
    t.token,
    t.receiver_salt,
    t.sender,
    t.recipient,
    t.amount,
    t.block_number,
    t.block_timestamp,
    to_timestamp(t.block_timestamp) as block_time,
    t.block_hash,
    t.tx_hash,
    t.log_index,

    c.origin as claim_origin,
    c.lease_id as claim_lease_id,
    c.claim_id,
    c.status as claim_status,
    c.amount_usdt as claim_amount_usdt,

    (
        select lv.lease_id
        from hub.lease_versions lv
        where
            lv.receiver_salt = t.receiver_salt
            and lv.start_time <= t.block_timestamp
            and lv.nukeable_after > t.block_timestamp
        order by lv.start_time desc
        limit 1
    ) as expected_lease_id,

    lp.last_pull_timestamp,
    (
        lp.last_pull_timestamp is null
        or t.block_timestamp > lp.last_pull_timestamp
    ) as preentitle_time_ok,

    case
        when c.lease_id is not null and c.origin = 1 then 'already_accounted'
        when c.lease_id is not null and c.origin = 0 then 'pre_entitle'
        when
            (
                lp.last_pull_timestamp is null
                or t.block_timestamp > lp.last_pull_timestamp
            )
            then 'subjective_pre_entitle'
        else 'pull'
    end as recommended_action

from ctl.receiver_usdt_transfers t
join ctl.receiver_watchlist w
    on w.receiver_salt = t.receiver_salt
left join hub.claim_versions c
    on
        c.valid_to_seq is null
        and c.origin in (0, 1)
        and c.origin_id = t.tx_hash
left join last_pull lp
    on
        lp.receiver_salt = t.receiver_salt
        and lp.token = t.token
where
    t.canonical
    and w.backfill_next_block is null;

comment on view api.receiver_usdt_transfer_actionability is
$$Receiver USDT deposits + actionability hints (subjective_pre_entitle vs pre_entitle vs pull)

For each canonical TRC-20 USDT transfer into a deterministic receiver, this view shows:
- whether the hub has already created a claim for it (subjective or objective),
- the latest observed receiver pull timestamp for (receiver_salt, token), and
- whether `preEntitle` is still time-eligible (`transfer_ts > last_pull_ts`).

`recommended_action` is a best-effort operator hint:
- 'already_accounted'        => PRE_ENTITLE claim exists (objective proven)
- 'pre_entitle'              => SUBJECTIVE_PRE_ENTITLE claim exists (prove tx to reimburse sponsor)
- 'subjective_pre_entitle'   => no claim yet and pre-entitle timing is still allowed
- 'pull'                     => no claim yet and a later pull timestamp suggests pre-entitle would revert

Receivers that are still backfilling (`ctl.receiver_watchlist.backfill_next_block is not null`)
are intentionally excluded so relayers can continue operating for already-synced receivers.$$;
