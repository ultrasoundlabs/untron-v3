/*
Convenient UI views for PostgREST + dashboards/graphs.

Includes:
- A chronological "all USDT deposits" view (one row per TRC-20 Transfer log).
- Time-series rollups and KPI views for charting.
*/

-- =============================================================================
-- BASE: CHRONOLOGICAL USDT DEPOSIT LOGS (UI LIST + JOIN POINT FOR GRAPHS)
-- =============================================================================

/*
Chronological USDT deposit transactions (TRC-20 Transfer logs into deterministic receivers),
enriched with best-effort lease + claim linkage and operator hints.

Notes:
- Rows are per Transfer log (tx_hash + log_index), not per Tron transaction.
- Ordering is not guaranteed by Postgres for views; consumers should use PostgREST `order=`.
*/
create or replace view api.usdt_deposit_txs as
with deposits as (
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
        t.inserted_at
    from ctl.receiver_usdt_transfers t
    where t.canonical
),
claim_links_raw as (
    select
        a.lease_id,
        a.claim_id,
        (e->>'tx_hash')::public.txhash_hex as tx_hash,
        (e->>'log_index')::int as log_index,
        (e->>'amount')::public.u256 as attributed_amount
    from api.claim_usdt_deposit_attribution a
    cross join lateral jsonb_array_elements(a.usdt_deposit_attribution) e
),
claim_links_enriched as (
    select
        l.tx_hash,
        l.log_index,
        l.lease_id,
        l.claim_id,
        l.attributed_amount,
        c.origin as claim_origin,
        c.status as claim_status,
        c.amount_usdt as claim_amount_usdt,
        lv.lease_number,
        lv.realtor,
        lv.lessee
    from claim_links_raw l
    left join hub.claim_versions c
        on
            c.lease_id = l.lease_id
            and c.claim_id = l.claim_id
            and c.valid_to_seq is null
    left join hub.lease_versions lv
        on lv.lease_id = l.lease_id and lv.valid_to_seq is null
),
links_by_deposit as (
    select
        tx_hash,
        log_index,
        jsonb_agg(
            jsonb_build_object(
                'lease_id', lease_id,
                'lease_number', lease_number,
                'realtor', realtor,
                'lessee', lessee,
                'claim_id', claim_id,
                'claim_origin', claim_origin,
                'claim_status', claim_status,
                'claim_amount_usdt', claim_amount_usdt::text,
                'attributed_amount', attributed_amount::text
            )
            order by lease_id, claim_id
        ) as linked_claims,
        count(*) as linked_claims_total,
        coalesce(sum(attributed_amount), 0) as linked_claims_amount
    from claim_links_enriched
    group by tx_hash, log_index
)
select
    d.chain_id,
    d.token,
    d.receiver_salt,
    d.sender,
    d.recipient,
    d.amount,
    d.block_number,
    d.block_timestamp,
    d.block_time,
    d.block_hash,
    d.tx_hash,
    d.log_index,
    d.inserted_at,

    a.recommended_action,
    a.expected_lease_id,
    elv.lease_number as expected_lease_number,
    elv.realtor as expected_realtor,
    elv.lessee as expected_lessee,

    -- Direct (pre-entitle) claim match by origin_id=tx_hash (if any).
    a.claim_lease_id,
    a.claim_id,
    a.claim_origin,
    a.claim_status,
    a.claim_amount_usdt,
    clv.lease_number as claim_lease_number,
    clv.realtor as claim_realtor,
    clv.lessee as claim_lessee,

    dp.processed as deposit_processed,
    dp.last_checked_at as deposit_processed_last_checked_at,

    coalesce(l.linked_claims, '[]'::jsonb) as linked_claims,
    coalesce(l.linked_claims_total, 0) as linked_claims_total,
    coalesce(l.linked_claims_amount, 0)::text as linked_claims_amount
from deposits d
left join api.receiver_usdt_transfer_actionability a
    on
        a.chain_id = d.chain_id
        and a.token = d.token
        and a.receiver_salt = d.receiver_salt
        and a.tx_hash = d.tx_hash
        and a.log_index = d.log_index
left join hub.deposit_processed_cache dp
    on dp.tx_hash::text = d.tx_hash::text
left join links_by_deposit l
    on l.tx_hash = d.tx_hash and l.log_index = d.log_index
left join hub.lease_versions elv
    on elv.lease_id = a.expected_lease_id and elv.valid_to_seq is null
left join hub.lease_versions clv
    on clv.lease_id = a.claim_lease_id and clv.valid_to_seq is null;

comment on view api.usdt_deposit_txs is
$$Chronological USDT deposit Transfer logs into deterministic receivers, enriched with lease/claim linkage.

Rows correspond to canonical TRC-20 `Transfer` logs (tx_hash + log_index) into deterministic receivers.

Linkage fields:
- `expected_*` => best-effort attribution by (receiver_salt, timestamp) to the active lease window.
- `claim_*`    => direct hub claim match for pre-entitle origins (origin_id == tx_hash), if any.
- `linked_claims` => JSON array of best-effort claim attributions from `api.claim_usdt_deposit_attribution`
  (includes receiver-pull FIFO allocations).$$;

-- =============================================================================
-- DASHBOARD INDEXES
-- =============================================================================

create index if not exists ctl_receiver_usdt_transfers_canonical_blocktime_desc
on ctl.receiver_usdt_transfers (block_timestamp desc, log_index desc)
where canonical;

create index if not exists hub_claim_versions_current_origin_timestamp_desc
on hub.claim_versions (origin_timestamp desc)
where valid_to_seq is null;

-- =============================================================================
-- USDT DEPOSITS: DAILY + CUMULATIVE + FUNNEL
-- =============================================================================

create or replace view api.usdt_deposits_daily as
select
    date_trunc('day', t.block_time) as day,
    count(*)::bigint as deposits_total,
    coalesce(sum(t.amount), 0)::text as amount_total,
    count(distinct t.sender)::bigint as unique_senders,
    count(distinct t.receiver_salt)::bigint as unique_receivers,
    count(distinct t.expected_lease_id)::bigint as leases_touched
from api.usdt_deposit_txs t
group by 1
order by 1;

comment on view api.usdt_deposits_daily is
$$Daily rollup of canonical USDT deposit logs (count + sum + uniques).$$;

create or replace view api.usdt_deposits_daily_by_action as
select
    date_trunc('day', t.block_time) as day,
    t.recommended_action,
    count(*)::bigint as deposits_total,
    coalesce(sum(t.amount), 0)::text as amount_total
from api.usdt_deposit_txs t
group by 1, 2
order by 1, 2;

comment on view api.usdt_deposits_daily_by_action is
$$Daily USDT deposit rollup split by `recommended_action` (operator funnel stage).$$;

create or replace view api.usdt_deposits_cumulative as
with daily as (
    select
        date_trunc('day', t.block_time) as day,
        count(*)::bigint as deposits_total,
        coalesce(sum(t.amount), 0) as amount_total_numeric
    from api.usdt_deposit_txs t
    group by 1
)
select
    day,
    deposits_total,
    amount_total_numeric::text as amount_total,
    sum(deposits_total) over (order by day asc)::bigint as deposits_total_cum,
    sum(amount_total_numeric) over (order by day asc)::text as amount_total_cum
from daily
order by day asc;

comment on view api.usdt_deposits_cumulative is
$$Daily USDT deposits plus cumulative totals (running sums).$$;

create or replace view api.usdt_deposit_funnel_daily as
select
    date_trunc('day', t.block_time) as day,
    t.recommended_action as stage,
    count(*)::bigint as deposits_total,
    coalesce(sum(t.amount), 0)::text as amount_total
from api.usdt_deposit_txs t
group by 1, 2
union all
select
    date_trunc('day', t.block_time) as day,
    'all'::text as stage,
    count(*)::bigint as deposits_total,
    coalesce(sum(t.amount), 0)::text as amount_total
from api.usdt_deposit_txs t
group by 1
order by 1, 2;

comment on view api.usdt_deposit_funnel_daily is
$$Deposit funnel time-series. `stage` is `recommended_action` plus an `all` row per day.$$;

-- =============================================================================
-- CLAIMS: CREATED/FILLED + LATENCY
-- =============================================================================

-- First version per claim (approximates "created" time).
create or replace view api.hub_claim_first_versions as
select distinct on (c.lease_id, c.claim_id)
    c.lease_id,
    c.claim_id,
    c.valid_from_seq as created_from_seq,
    c.origin,
    c.origin_timestamp,
    c.amount_usdt,
    c.target_chain_id,
    c.target_token,
    c.beneficiary
from hub.claim_versions c
order by c.lease_id, c.claim_id, c.valid_from_seq asc;

comment on view api.hub_claim_first_versions is
$$First persisted version per (lease_id, claim_id), used for claim-created time-series.$$;

create or replace view api.claims_created_daily as
select
    date_trunc('day', to_timestamp(e.block_timestamp)) as day,
    count(*)::bigint as claims_created_total,
    count(*) filter (where fv.origin = 0)::bigint as claims_created_subjective_pre_entitle,
    count(*) filter (where fv.origin = 1)::bigint as claims_created_pre_entitle,
    count(*) filter (where fv.origin = 2)::bigint as claims_created_receiver_pull,
    coalesce(sum(fv.amount_usdt), 0)::text as amount_usdt_total
from api.hub_claim_first_versions fv
join chain.event_appended e
    on
        e.stream = 'hub'
        and e.canonical
        and e.event_seq = fv.created_from_seq
group by 1
order by 1;

comment on view api.claims_created_daily is
$$Daily rollup of claims created (first version per claim; timestamp from hub event stream).$$;

create or replace view api.claims_filled_daily as
select
    date_trunc('day', to_timestamp(e.block_timestamp)) as day,
    count(*)::bigint as claims_filled_total,
    coalesce(sum(c.amount_usdt), 0)::text as amount_usdt_total
from hub.claim_versions c
join chain.event_appended e
    on
        e.stream = 'hub'
        and e.canonical
        and e.event_seq = c.valid_from_seq
        and e.event_type = 'ClaimFilled'
where c.valid_to_seq is null and c.status = 'filled'
group by 1
order by 1;

comment on view api.claims_filled_daily is
$$Daily rollup of filled claims (current rows only; timestamp from ClaimFilled event).$$;

create or replace view api.claim_fill_latency_daily as
select
    date_trunc('day', to_timestamp(e.block_timestamp)) as day,
    count(*)::bigint as filled_claims_total,
    avg(extract(epoch from (to_timestamp(e.block_timestamp) - to_timestamp(c.origin_timestamp)))) as avg_seconds,
    percentile_cont(0.50) within group (
        order by extract(epoch from (to_timestamp(e.block_timestamp) - to_timestamp(c.origin_timestamp)))
    ) as p50_seconds,
    percentile_cont(0.90) within group (
        order by extract(epoch from (to_timestamp(e.block_timestamp) - to_timestamp(c.origin_timestamp)))
    ) as p90_seconds
from hub.claim_versions c
join chain.event_appended e
    on
        e.stream = 'hub'
        and e.canonical
        and e.event_seq = c.valid_from_seq
        and e.event_type = 'ClaimFilled'
where c.valid_to_seq is null and c.status = 'filled'
group by 1
order by 1;

comment on view api.claim_fill_latency_daily is
$$Daily latency stats for filled claims: (ClaimFilled block_time - origin_timestamp).$$;

-- =============================================================================
-- PROTOCOL PNL: TIME-SERIES
-- =============================================================================

create or replace view api.protocol_pnl_timeseries as
select
    e.block_timestamp,
    to_timestamp(e.block_timestamp) as block_time,
    p.pnl,
    p.delta,
    p.reason,
    p.valid_from_seq
from hub.protocol_pnl_versions p
join chain.event_appended e
    on
        e.stream = 'hub'
        and e.canonical
        and e.event_seq = p.valid_from_seq
order by e.block_timestamp asc;

comment on view api.protocol_pnl_timeseries is
$$Protocol PnL snapshots as a time-series (timestamped by hub event stream).$$;

-- =============================================================================
-- LEASES: LIFECYCLE + ACTIVE (DERIVABLE) + PER-LEASE KPIS
-- =============================================================================

create or replace view api.leases_started_daily as
select
    date_trunc('day', to_timestamp(lv.start_time)) as day,
    count(*)::bigint as leases_started_total
from hub.lease_versions lv
where lv.valid_to_seq is null
group by 1
order by 1;

comment on view api.leases_started_daily is
$$Daily count of current leases by start_time (not historical versions).$$;

create or replace view api.leases_ending_daily as
select
    date_trunc('day', to_timestamp(lv.nukeable_after)) as day,
    count(*)::bigint as leases_ending_total
from hub.lease_versions lv
where lv.valid_to_seq is null
group by 1
order by 1;

comment on view api.leases_ending_daily is
$$Daily count of current leases by nukeable_after (scheduled/expected end time).$$;

create or replace view api.active_leases_daily as
with bounds as (
    select
        date_trunc('day', least(
            coalesce(min(to_timestamp(start_time)), now()),
            now()
        )) as min_day,
        date_trunc('day', greatest(
            coalesce(max(to_timestamp(nukeable_after)), now()),
            now()
        )) as max_day
    from hub.lease_versions
    where valid_to_seq is null
),
days as (
    select generate_series(b.min_day, b.max_day, interval '1 day') as day
    from bounds b
),
starts as (
    select date_trunc('day', to_timestamp(start_time)) as day, count(*)::bigint as n
    from hub.lease_versions
    where valid_to_seq is null
    group by 1
),
ends as (
    select date_trunc('day', to_timestamp(nukeable_after)) as day, count(*)::bigint as n
    from hub.lease_versions
    where valid_to_seq is null
    group by 1
),
deltas as (
    select d.day,
        coalesce(s.n, 0) as started,
        coalesce(e.n, 0) as ended
    from days d
    left join starts s on s.day = d.day
    left join ends e on e.day = d.day
)
select
    day,
    started as leases_started,
    ended as leases_ended,
    sum(started - ended) over (order by day asc)::bigint as leases_active
from deltas
order by day asc;

comment on view api.active_leases_daily is
$$Daily active lease count (running starts minus ends) over a continuous day series.$$;

create or replace view api.lease_kpis as
select
    lv.lease_id,
    lv.lease_number,
    lv.realtor,
    lv.lessee,
    lv.receiver_salt,
    lv.start_time,
    lv.nukeable_after,
    to_timestamp(lv.start_time) as start_time_utc,
    to_timestamp(lv.nukeable_after) as nukeable_after_utc,

    coalesce(d.deposits_total, 0)::bigint as deposits_total,
    coalesce(d.amount_total, 0)::text as deposits_amount_total,
    coalesce(d.deposits_latest_block_timestamp, 0)::bigint as deposits_latest_block_timestamp,

    coalesce(c.claims_total, 0)::bigint as claims_total,
    coalesce(c.claims_filled, 0)::bigint as claims_filled,
    coalesce(c.claims_amount_total, 0)::text as claims_amount_total,
    coalesce(c.claims_latest_origin_timestamp, 0)::bigint as claims_latest_origin_timestamp
from hub.lease_versions lv
left join lateral (
    select
        count(*) as deposits_total,
        coalesce(sum(t.amount), 0) as amount_total,
        coalesce(max(t.block_timestamp), 0) as deposits_latest_block_timestamp
    from api.usdt_deposit_txs t
    where t.expected_lease_id = lv.lease_id
) d on true
left join lateral (
    select
        count(*) as claims_total,
        count(*) filter (where c.status = 'filled') as claims_filled,
        coalesce(sum(c.amount_usdt), 0) as claims_amount_total,
        coalesce(max(c.origin_timestamp), 0) as claims_latest_origin_timestamp
    from hub.claim_versions c
    where c.valid_to_seq is null and c.lease_id = lv.lease_id
) c on true
where lv.valid_to_seq is null;

comment on view api.lease_kpis is
$$Per-lease KPIs: deposits, claims, and last activity timestamps (best-effort).$$;

-- =============================================================================
-- OPS BACKLOG: ACTIONABILITY + AGE BUCKETS
-- =============================================================================

create or replace view api.usdt_deposit_backlog_summary as
select
    a.recommended_action,
    count(*)::bigint as deposits_total,
    coalesce(sum(a.amount), 0)::text as amount_total,
    coalesce(min(a.block_timestamp), 0)::bigint as oldest_block_timestamp,
    coalesce(max(a.block_timestamp), 0)::bigint as newest_block_timestamp
from api.receiver_usdt_transfer_actionability a
group by 1
order by 1;

comment on view api.usdt_deposit_backlog_summary is
$$Current deposit actionability backlog summary grouped by `recommended_action`.$$;

create or replace view api.usdt_deposit_backlog_age_buckets as
with base as (
    select
        a.recommended_action,
        a.block_time,
        a.amount
    from api.receiver_usdt_transfer_actionability a
),
bucketed as (
    select
        recommended_action,
        case
            when now() - block_time < interval '1 hour' then '00_01h'
            when now() - block_time < interval '6 hours' then '01_06h'
            when now() - block_time < interval '24 hours' then '06_24h'
            when now() - block_time < interval '7 days' then '01_07d'
            when now() - block_time < interval '30 days' then '07_30d'
            else '30d_plus'
        end as age_bucket,
        amount
    from base
)
select
    recommended_action,
    age_bucket,
    count(*)::bigint as deposits_total,
    coalesce(sum(amount), 0)::text as amount_total
from bucketed
group by 1, 2
order by 1, 2;

comment on view api.usdt_deposit_backlog_age_buckets is
$$Deposit backlog grouped into age buckets (relative to now()).$$;

-- =============================================================================
-- REALTOR: DAILY PERFORMANCE
-- =============================================================================

create or replace view api.realtor_deposits_daily as
select
    date_trunc('day', t.block_time) as day,
    t.expected_realtor as realtor,
    count(*)::bigint as deposits_total,
    coalesce(sum(t.amount), 0)::text as amount_total,
    count(distinct t.expected_lease_id)::bigint as leases_touched
from api.usdt_deposit_txs t
where t.expected_realtor is not null
group by 1, 2
order by 1, 2;

comment on view api.realtor_deposits_daily is
$$Daily USDT deposits attributed to realtor (via expected_lease_id -> lease.realtor).$$;

-- =============================================================================
-- POSTGREST GRANTS
-- =============================================================================

do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on
      api.usdt_deposit_txs,
      api.usdt_deposits_daily,
      api.usdt_deposits_daily_by_action,
      api.usdt_deposits_cumulative,
      api.usdt_deposit_funnel_daily,
      api.hub_claim_first_versions,
      api.claims_created_daily,
      api.claims_filled_daily,
      api.claim_fill_latency_daily,
      api.protocol_pnl_timeseries,
      api.leases_started_daily,
      api.leases_ending_daily,
      api.active_leases_daily,
      api.lease_kpis,
      api.usdt_deposit_backlog_summary,
      api.usdt_deposit_backlog_age_buckets,
      api.realtor_deposits_daily
    to pgrst_anon;
  end if;
end $$;
