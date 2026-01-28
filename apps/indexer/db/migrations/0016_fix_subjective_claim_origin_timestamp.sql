/*
Fix `origin_timestamp` for subjective pre-entitle claims (origin=0) in `api.lease_view`.

On-chain `subjectivePreEntitle(txId, leaseId, rawAmount)` does not carry the Tron
block timestamp of the underlying deposit, so `hub.claim_versions.origin_timestamp`
may be 0 for origin=0 claims.

However, we already compute `api.claim_usdt_deposit_attribution` for origin in (0,1)
by matching `origin_id` (tx_hash) against `ctl.receiver_usdt_transfers`, which
contains the true `block_timestamp`.

This migration makes the API response more ergonomic by overriding the JSON
`origin_timestamp` for origin=0 claims with the attributed deposit's
`block_timestamp` when available.
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
    coalesce(ch.claims_filled, 0) as claims_filled,

    -- Deposits that are still eligible for preEntitle (and not yet accounted for by hub).
    coalesce(pd.pending_usdt_deposits, '[]'::jsonb) as pending_usdt_deposits,
    coalesce(pd.pending_usdt_deposits_total, 0) as pending_usdt_deposits_total,
    coalesce(pd.pending_usdt_deposits_amount, '0') as pending_usdt_deposits_amount,
    coalesce(pd.pending_usdt_deposits_latest_block_timestamp, 0) as pending_usdt_deposits_latest_block_timestamp
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
                'origin_timestamp', (
                    case
                        when c.origin = 0 then coalesce(
                            nullif((a.usdt_deposit_attribution->0->>'block_timestamp')::bigint, 0),
                            nullif(c.origin_timestamp, 0),
                            0
                        )
                        else c.origin_timestamp
                    end
                ),
                'origin_raw_amount', c.origin_raw_amount,
                'status', c.status,
                'valid_from_seq', c.valid_from_seq,
                'valid_to_seq', c.valid_to_seq,
                'usdt_deposit_attribution', coalesce(a.usdt_deposit_attribution, '[]'::jsonb),
                'fill_tx_hash', e.tx_hash
            )
            order by c.claim_id
        ) as claims,
        count(*) as claims_total,
        count(*) filter (where c.status = 'filled') as claims_filled
    from hub.claim_versions c
    left join api.claim_usdt_deposit_attribution a
        on a.lease_id = c.lease_id and a.claim_id = c.claim_id
    left join chain.event_appended e
        on
            e.stream = 'hub'
            and e.canonical
            and e.event_seq = c.valid_from_seq
            and e.event_type = 'ClaimFilled'
    where c.lease_id = lv.lease_id and c.valid_to_seq is null
) ch on true
left join lateral (
    select
        jsonb_agg(
            jsonb_build_object(
                'tx_hash', t.tx_hash,
                'sender', t.sender,
                'amount', t.amount::text,
                'block_timestamp', t.block_timestamp,
                'log_index', t.log_index
            )
            order by t.block_timestamp, t.log_index
        ) as pending_usdt_deposits,
        count(*) as pending_usdt_deposits_total,
        coalesce(sum(t.amount), 0)::text as pending_usdt_deposits_amount,
        coalesce(max(t.block_timestamp), 0) as pending_usdt_deposits_latest_block_timestamp
    from api.receiver_usdt_transfer_actionability t
    where
        t.recommended_action = 'subjective_pre_entitle'
        and t.receiver_salt = lv.receiver_salt
        and t.expected_lease_id = lv.lease_id
) pd on true
where lv.valid_to_seq is null;

-- Ensure PostgREST anon role can read the updated api view (no-op if role missing).
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on api.lease_view to pgrst_anon;
  end if;
end $$;
