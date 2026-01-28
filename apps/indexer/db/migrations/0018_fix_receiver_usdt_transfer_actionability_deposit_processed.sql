/*
Fix `api.receiver_usdt_transfer_actionability` to consult `hub.deposit_processed_cache`
*and* to ignore receivers that are still backfilling.

Background:
- 0013_deposit_processed_cache.sql updated the view to treat `depositProcessed(txId)=true`
  as `recommended_action = 'already_accounted'`, even if the only persisted claim is
  origin=SUBJECTIVE_PRE_ENTITLE (0).
- 0014_receiver_backfill_actionability.sql later replaced the same view to exclude
  receivers whose watchlist backfill is not complete (`backfill_next_block is not null`),
  but that replacement unintentionally dropped the `deposit_processed_cache` logic.

This migration re-creates the view, merging both behaviors.
*/

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
        -- Objective proof confirmed onchain (via cached depositProcessed), regardless of claim origin.
        when dp.processed then 'already_accounted'

        -- Claim origin PRE_ENTITLE exists (objective proven via event chain).
        when c.lease_id is not null and c.origin = 1 then 'already_accounted'

        -- Subjective claim exists; objective preEntitle still needed (typically for reimbursement).
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
left join hub.deposit_processed_cache dp
    on dp.tx_hash::text = t.tx_hash::text
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
- whether the hub has objectively processed the deposit (`depositProcessed[txId]`, best-effort cached),
- the latest observed receiver pull timestamp for (receiver_salt, token), and
- whether `preEntitle` is still time-eligible (`transfer_ts > last_pull_ts`).

`recommended_action` is a best-effort operator hint:
- 'already_accounted'        => objective preEntitle is already processed (via claim origin=PRE_ENTITLE or cached depositProcessed)
- 'pre_entitle'              => SUBJECTIVE_PRE_ENTITLE claim exists; objective proof may be needed (e.g. reimbursement)
- 'subjective_pre_entitle'   => no claim yet and pre-entitle timing is still allowed
- 'pull'                     => no claim yet and a later pull timestamp suggests pre-entitle would revert

Receivers that are still backfilling (`ctl.receiver_watchlist.backfill_next_block is not null`)
are intentionally excluded so relayers can continue operating for already-synced receivers.$$;
