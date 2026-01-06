/*
Lease aggregated views for efficient lease introspection.

Goals:
- Provide a single-query lease "detail" payload for clients that want an
  aggregated view (lease snapshot + payout config + claims + nonce).
- Keep the public PostgREST API schema read-only (views in `api`).

Notes:
- This is intended to reduce round-trips for services like `apps/realtor`
  which otherwise need multiple PostgREST requests to build a lease view.
- This view uses JSON aggregation; large leases with many claims will return
  large payloads by design.
*/

create or replace view api.lease_view as
select
    lv.lease_id,
    lv.receiver_salt,
    lv.lease_number,
    lv.realtor,
    lv.lessee,
    lv.start_time,
    lv.nukeable_after,
    lv.lease_fee_ppm,
    lv.flat_fee,

    ln.nonce as lease_nonce,

    pc.target_chain_id as payout_target_chain_id,
    pc.target_token as payout_target_token,
    pc.beneficiary as payout_beneficiary,

    -- Payout config history for this lease (all versions).
    coalesce(pch.payout_config_history, '[]'::jsonb) as payout_config_history,

    -- Current claim states for this lease (one per claim_id).
    coalesce(ch.claims, '[]'::jsonb) as claims,
    coalesce(ch.claims_total, 0) as claims_total,
    coalesce(ch.claims_filled, 0) as claims_filled
from hub.lease_versions lv
left join hub.lease_nonce_versions ln
    on ln.lease_id = lv.lease_id and ln.valid_to_seq is null
left join hub.payout_config_versions pc
    on pc.lease_id = lv.lease_id and pc.valid_to_seq is null
left join lateral (
    select jsonb_agg(
        jsonb_build_object(
            'target_chain_id', v.target_chain_id,
            'target_token', v.target_token,
            'beneficiary', v.beneficiary,
            'valid_from_seq', v.valid_from_seq,
            'valid_to_seq', v.valid_to_seq
        )
        order by v.valid_from_seq
    ) as payout_config_history
    from hub.payout_config_versions v
    where v.lease_id = lv.lease_id
) pch on true
left join lateral (
    select
        jsonb_agg(
            jsonb_build_object(
                'claim_id', c.claim_id,
                'target_token', c.target_token,
                'queue_index', c.queue_index,
                'amount_usdt', c.amount_usdt,
                'target_chain_id', c.target_chain_id,
                'beneficiary', c.beneficiary,
                'origin', c.origin,
                'origin_id', c.origin_id,
                'origin_actor', c.origin_actor,
                'origin_token', c.origin_token,
                'origin_timestamp', c.origin_timestamp,
                'origin_raw_amount', c.origin_raw_amount,
                'status', c.status,
                'valid_from_seq', c.valid_from_seq,
                'valid_to_seq', c.valid_to_seq
            )
            order by c.claim_id
        ) as claims,
        count(*) as claims_total,
        count(*) filter (where c.status = 'filled') as claims_filled
    from hub.claim_versions c
    where c.lease_id = lv.lease_id and c.valid_to_seq is null
) ch on true
where lv.valid_to_seq is null;

comment on view api.lease_view is
$$Aggregated lease view (single-row per lease_id).

This view joins:
- current hub lease (hub.lease_versions)
- current payout config (hub.payout_config_versions)
- payout config history (hub.payout_config_versions, all versions)
- current per-lease nonce (hub.lease_nonce_versions)
- current claim states (hub.claim_versions)

It is intended for clients that want a single PostgREST request to fetch a full
lease "detail" payload.$$;

-- Ensure PostgREST anon role can read the new api view (no-op if role missing).
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on api.lease_view to pgrst_anon;
  end if;
end $$;

