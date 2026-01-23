/*
USDT deposit attribution for claims.

Adds `usdt_deposit_attribution` to each claim object returned by `api.lease_view`.

Semantics:
- For claim origins SUBJECTIVE_PRE_ENTITLE / PRE_ENTITLE (0 / 1):
  attribute to the canonical TRC-20 Transfer log into the lease receiver with tx_hash == origin_id.
- For claim origin RECEIVER_PULL (2) where origin_token is a Tron USDT address (T...):
  best-effort FIFO attribution of `origin_raw_amount` across canonical receiver USDT transfers
  between the previous pull timestamp and this claim's origin_timestamp.

This is intentionally a deterministic approximation and is not meant to be a perfect accounting proof.
*/

-- Per-claim USDT deposit attribution (JSON array).
create or replace view api.claim_usdt_deposit_attribution as
select
    c.lease_id,
    c.claim_id,
    coalesce(a.usdt_deposit_attribution, '[]'::jsonb) as usdt_deposit_attribution
from hub.claim_versions c
join hub.lease_versions lv
    on lv.lease_id = c.lease_id and lv.valid_to_seq is null
left join lateral (
    select
        case
            -- SUBJECTIVE_PRE_ENTITLE / PRE_ENTITLE: match by txId (origin_id) and receiver_salt.
            when c.origin in (0, 1) then (
                select
                    coalesce(
                        jsonb_agg(
                            jsonb_build_object(
                                'tx_hash', t.tx_hash,
                                'sender', t.sender,
                                'amount', t.amount,
                                'block_timestamp', t.block_timestamp,
                                'log_index', t.log_index
                            )
                            order by t.log_index
                        ),
                        '[]'::jsonb
                    )
                from ctl.receiver_usdt_transfers t
                where
                    t.canonical
                    and t.receiver_salt = lv.receiver_salt
                    and t.tx_hash = c.origin_id
            )

            -- RECEIVER_PULL: best-effort FIFO attribution against USDT transfers only (origin_token is Tron address).
            when c.origin = 2 and c.origin_token like 'T%' then (
                with prev_pull as (
                    select
                        max(e.block_timestamp) as prev_pull_timestamp
                    from ctl.pulled_from_receiver_ledger l
                    join chain.event_appended e
                        on
                            e.stream = 'controller'
                            and e.canonical
                            and e.event_seq = l.event_seq
                    where
                        l.receiver_salt = lv.receiver_salt
                        and l.token = c.origin_token::public.tron_address
                        and e.block_timestamp < c.origin_timestamp
                ),
                transfers as (
                    select
                        t.id,
                        t.tx_hash,
                        t.sender,
                        t.amount,
                        t.block_timestamp,
                        t.log_index,
                        sum(t.amount) over (
                            order by t.block_timestamp asc, t.log_index asc, t.id asc
                        ) as cum_amount
                    from ctl.receiver_usdt_transfers t
                    cross join prev_pull p
                    where
                        t.canonical
                        and t.receiver_salt = lv.receiver_salt
                        and t.token = c.origin_token::public.tron_address
                        and t.block_timestamp <= c.origin_timestamp
                        and t.block_timestamp > coalesce(p.prev_pull_timestamp, 0)
                ),
                allocated as (
                    select
                        tx_hash,
                        sender,
                        block_timestamp,
                        log_index,
                        greatest(
                            0::numeric,
                            least(
                                amount,
                                c.origin_raw_amount - (cum_amount - amount)
                            )
                        )::public.u256 as allocated_amount
                    from transfers
                )
                select
                    coalesce(
                        jsonb_agg(
                            jsonb_build_object(
                                'tx_hash', tx_hash,
                                'sender', sender,
                                'amount', allocated_amount,
                                'block_timestamp', block_timestamp,
                                'log_index', log_index
                            )
                            order by block_timestamp asc, log_index asc
                        ),
                        '[]'::jsonb
                    )
                from allocated
                where allocated_amount > 0
            )

            else '[]'::jsonb
        end as usdt_deposit_attribution
) a on true
where c.valid_to_seq is null;

comment on view api.claim_usdt_deposit_attribution is
$$USDT deposit attribution per claim (best-effort).

Returns a JSON array of deposit attribution entries for each claim:
- Pre-entitle origins (0/1): matched by txId to `ctl.receiver_usdt_transfers`.
- Receiver-pull origin (2): FIFO allocation of `origin_raw_amount` across receiver USDT transfers since previous pull.$$;

-- Update `api.lease_view` claims JSON to include `usdt_deposit_attribution`.
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
                'valid_to_seq', c.valid_to_seq,
                'usdt_deposit_attribution', coalesce(a.usdt_deposit_attribution, '[]'::jsonb)
            )
            order by c.claim_id
        ) as claims,
        count(*) as claims_total,
        count(*) filter (where c.status = 'filled') as claims_filled
    from hub.claim_versions c
    left join api.claim_usdt_deposit_attribution a
        on a.lease_id = c.lease_id and a.claim_id = c.claim_id
    where c.lease_id = lv.lease_id and c.valid_to_seq is null
) ch on true
where lv.valid_to_seq is null;

-- Ensure PostgREST anon role can read the new api view (no-op if role missing).
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on api.claim_usdt_deposit_attribution to pgrst_anon;
    grant select on api.lease_view to pgrst_anon;
  end if;
end $$;

