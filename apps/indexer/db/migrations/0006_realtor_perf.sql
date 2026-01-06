/*
Realtor performance primitives.

Goals:
- Keep PostgREST schema read-only (views in `api`).
- Reduce per-request DB load by avoiding global aggregations.
- Expose higher-level "candidate" views so clients can use fewer queries.

Notes:
- All logic lives in the DB schema (no Rust/indexer changes required).
- This migration intentionally changes/overwrites some `api.*` view definitions.
*/

-- =============================================================================
-- INDEXES (match realtor query patterns)
-- =============================================================================

-- Latest lease for a receiver salt (ORDER BY lease_number DESC LIMIT 1).
create index if not exists hub_lease_current_by_receiver_salt_lease_number
on hub.lease_versions (receiver_salt, lease_number desc)
where valid_to_seq is null;

-- Rate-limit window scans
-- (realtor + start_time >= ... ORDER BY start_time DESC).
create index if not exists hub_lease_current_by_realtor_start_time
on hub.lease_versions (realtor, start_time desc)
where valid_to_seq is null;

-- Beneficiary "has any filled claim" existence checks.
create index if not exists hub_claim_current_filled_by_beneficiary
on hub.claim_versions (beneficiary)
where valid_to_seq is null and status = 'filled';

-- =============================================================================
-- CACHED RECEIVER BALANCES (avoid global GROUP BY per request)
-- =============================================================================

create table if not exists ctl.receiver_token_balances (
    receiver_salt public.bytes32_hex not null,
    token public.tron_address not null,
    incoming_amount public.u256 not null default 0,
    pulled_amount public.u256 not null default 0,
    balance_amount public.u256 not null default 0,
    updated_at timestamptz not null default now(),
    primary key (receiver_salt, token)
);

create index if not exists ctl_receiver_token_balances_token_balance
on ctl.receiver_token_balances (token, balance_amount);

create index if not exists ctl_receiver_token_balances_token_nonzero
on ctl.receiver_token_balances (token, balance_amount)
where balance_amount > 0;

create or replace function ctl.recompute_receiver_token_balance(
    p_receiver_salt public.bytes32_hex,
    p_token public.tron_address
) returns void language plpgsql as $$
begin
  update ctl.receiver_token_balances b
     set balance_amount = greatest(b.incoming_amount - b.pulled_amount, 0::public.u256),
         updated_at = now()
   where b.receiver_salt = p_receiver_salt and b.token = p_token;
end $$;

-- Receiver transfer changes (canonical insert/update/reorg invalidation).
create or replace function ctl.on_receiver_usdt_transfers_change()
returns trigger language plpgsql as $$
declare
  old_salt public.bytes32_hex;
  old_token public.tron_address;
  old_amount public.u256;
  old_canonical boolean;

  new_salt public.bytes32_hex;
  new_token public.tron_address;
  new_amount public.u256;
  new_canonical boolean;
begin
  if tg_op = 'INSERT' then
    old_canonical := false;
    new_canonical := new.canonical;
    new_salt := new.receiver_salt;
    new_token := new.token;
    new_amount := new.amount;
  elsif tg_op = 'UPDATE' then
    old_canonical := old.canonical;
    old_salt := old.receiver_salt;
    old_token := old.token;
    old_amount := old.amount;

    new_canonical := new.canonical;
    new_salt := new.receiver_salt;
    new_token := new.token;
    new_amount := new.amount;
  elsif tg_op = 'DELETE' then
    old_canonical := old.canonical;
    old_salt := old.receiver_salt;
    old_token := old.token;
    old_amount := old.amount;
    new_canonical := false;
  else
    return null;
  end if;

  -- Subtract old contribution if it was canonical.
  if old_canonical then
    insert into ctl.receiver_token_balances (receiver_salt, token, incoming_amount, pulled_amount, balance_amount)
    values (old_salt, old_token, 0::public.u256, 0::public.u256, 0::public.u256)
    on conflict (receiver_salt, token) do nothing;

    update ctl.receiver_token_balances
       set incoming_amount = greatest(incoming_amount - old_amount, 0::public.u256),
           updated_at = now()
     where receiver_salt = old_salt and token = old_token;

    perform ctl.recompute_receiver_token_balance(old_salt, old_token);
  end if;

  -- Add new contribution if it is canonical.
  if new_canonical then
    insert into ctl.receiver_token_balances (receiver_salt, token, incoming_amount, pulled_amount, balance_amount)
    values (new_salt, new_token, new_amount, 0::public.u256, new_amount)
    on conflict (receiver_salt, token) do update
      set incoming_amount = ctl.receiver_token_balances.incoming_amount + excluded.incoming_amount,
          updated_at = now();

    perform ctl.recompute_receiver_token_balance(new_salt, new_token);
  end if;

  return null;
end $$;

drop trigger if exists trg_receiver_usdt_transfers_insert
on ctl.receiver_usdt_transfers;
create trigger trg_receiver_usdt_transfers_insert
after insert on ctl.receiver_usdt_transfers
for each row execute function ctl.on_receiver_usdt_transfers_change();

drop trigger if exists trg_receiver_usdt_transfers_update
on ctl.receiver_usdt_transfers;
create trigger trg_receiver_usdt_transfers_update
after update on ctl.receiver_usdt_transfers
for each row execute function ctl.on_receiver_usdt_transfers_change();

drop trigger if exists trg_receiver_usdt_transfers_delete
on ctl.receiver_usdt_transfers;
create trigger trg_receiver_usdt_transfers_delete
after delete on ctl.receiver_usdt_transfers
for each row execute function ctl.on_receiver_usdt_transfers_change();

-- Controller pull ledger changes (rollback deletes).
create or replace function ctl.on_pulled_from_receiver_ledger_change()
returns trigger language plpgsql as $$
declare
  old_salt public.bytes32_hex;
  old_token public.tron_address;
  old_amount public.u256;

  new_salt public.bytes32_hex;
  new_token public.tron_address;
  new_amount public.u256;
begin
  if tg_op = 'INSERT' then
    new_salt := new.receiver_salt;
    new_token := new.token;
    new_amount := new.token_amount;
  elsif tg_op = 'UPDATE' then
    old_salt := old.receiver_salt;
    old_token := old.token;
    old_amount := old.token_amount;
    new_salt := new.receiver_salt;
    new_token := new.token;
    new_amount := new.token_amount;
  elsif tg_op = 'DELETE' then
    old_salt := old.receiver_salt;
    old_token := old.token;
    old_amount := old.token_amount;
  else
    return null;
  end if;

  -- Subtract old contribution (for UPDATE/DELETE).
  if tg_op in ('UPDATE', 'DELETE') then
    insert into ctl.receiver_token_balances (receiver_salt, token, incoming_amount, pulled_amount, balance_amount)
    values (old_salt, old_token, 0::public.u256, 0::public.u256, 0::public.u256)
    on conflict (receiver_salt, token) do nothing;

    update ctl.receiver_token_balances
       set pulled_amount = greatest(pulled_amount - old_amount, 0::public.u256),
           updated_at = now()
     where receiver_salt = old_salt and token = old_token;

    perform ctl.recompute_receiver_token_balance(old_salt, old_token);
  end if;

  -- Add new contribution (for INSERT/UPDATE).
  if tg_op in ('INSERT', 'UPDATE') then
    insert into ctl.receiver_token_balances (receiver_salt, token, incoming_amount, pulled_amount, balance_amount)
    values (new_salt, new_token, 0::public.u256, new_amount, 0::public.u256)
    on conflict (receiver_salt, token) do update
      set pulled_amount = ctl.receiver_token_balances.pulled_amount + excluded.pulled_amount,
          updated_at = now();

    perform ctl.recompute_receiver_token_balance(new_salt, new_token);
  end if;

  return null;
end $$;

drop trigger if exists trg_pulled_from_receiver_ledger_insert
on ctl.pulled_from_receiver_ledger;
create trigger trg_pulled_from_receiver_ledger_insert
after insert on ctl.pulled_from_receiver_ledger
for each row execute function ctl.on_pulled_from_receiver_ledger_change();

drop trigger if exists trg_pulled_from_receiver_ledger_update
on ctl.pulled_from_receiver_ledger;
create trigger trg_pulled_from_receiver_ledger_update
after update on ctl.pulled_from_receiver_ledger
for each row execute function ctl.on_pulled_from_receiver_ledger_change();

drop trigger if exists trg_pulled_from_receiver_ledger_delete
on ctl.pulled_from_receiver_ledger;
create trigger trg_pulled_from_receiver_ledger_delete
after delete on ctl.pulled_from_receiver_ledger
for each row execute function ctl.on_pulled_from_receiver_ledger_change();

-- One-time seed of the cache for the current USDT token (safe to re-run).
do $$
declare
  cur_usdt public.tron_address;
begin
  select u.usdt into cur_usdt
  from ctl.usdt_versions u
  where u.valid_to_seq is null
  limit 1;

  if cur_usdt is not null then
    insert into ctl.receiver_token_balances (receiver_salt, token, incoming_amount, pulled_amount, balance_amount)
    with incoming as (
      select t.receiver_salt, t.token, sum(t.amount) as incoming_amount
      from ctl.receiver_usdt_transfers t
      where t.canonical and t.token = cur_usdt
      group by t.receiver_salt, t.token
    ),
    pulled as (
      select l.receiver_salt, l.token, sum(l.token_amount) as pulled_amount
      from ctl.pulled_from_receiver_ledger l
      where l.token = cur_usdt
      group by l.receiver_salt, l.token
    )
    select
      w.receiver_salt,
      cur_usdt as token,
      coalesce(i.incoming_amount, 0::public.u256) as incoming_amount,
      coalesce(p.pulled_amount, 0::public.u256) as pulled_amount,
      greatest(
        coalesce(i.incoming_amount, 0::public.u256) - coalesce(p.pulled_amount, 0::public.u256),
        0::public.u256
      ) as balance_amount
    from ctl.receiver_watchlist w
    left join incoming i on i.receiver_salt = w.receiver_salt and i.token = cur_usdt
    left join pulled p on p.receiver_salt = w.receiver_salt and p.token = cur_usdt
    on conflict (receiver_salt, token) do update
      set incoming_amount = excluded.incoming_amount,
          pulled_amount = excluded.pulled_amount,
          balance_amount = excluded.balance_amount,
          updated_at = now();
  end if;
end $$;

-- =============================================================================
-- API VIEW UPGRADES / NEW HIGH-LEVEL VIEWS
-- =============================================================================

-- Overwrite api.receiver_usdt_balances to read from the cache instead of
-- aggregating.
drop view if exists api.receiver_usdt_balances;
create or replace view api.receiver_usdt_balances as
with current_usdt as (
    select u.usdt
    from ctl.usdt_versions u
    where u.valid_to_seq is null
    limit 1
)

select
    w.receiver_salt,
    w.receiver,
    w.receiver_evm,
    u.usdt as token,
    coalesce(b.incoming_amount, 0::public.u256) as incoming_amount,
    coalesce(b.pulled_amount, 0::public.u256) as pulled_amount,
    coalesce(b.balance_amount, 0::public.u256) as balance_amount
from ctl.receiver_watchlist w
cross join current_usdt u
left join ctl.receiver_token_balances b
    on b.receiver_salt = w.receiver_salt and b.token = u.usdt;

-- Realtor constraints (protocol+realtor merged) + optionally rate remaining.
create or replace view api.realtor_effective_config as
with protocol as (
    select *
    from hub.protocol_config_versions
    where valid_to_seq is null
    limit 1
),

realtors as (
    select *
    from hub.realtor_versions
    where valid_to_seq is null
)

select
    r.realtor,
    coalesce(r.allowed, false) as allowed,

    greatest(
        coalesce(p.floor_ppm, 0), coalesce(r.min_fee_ppm, 0)
    ) as min_fee_ppm,
    greatest(
        coalesce(p.floor_flat_fee, 0::public.u256),
        coalesce(r.min_flat_fee, 0::public.u256)
    ) as min_flat_fee,

    (
        case
            when
                coalesce(p.max_lease_duration_seconds, 0) = 0
                and coalesce(r.max_lease_duration_seconds, 0) = 0
                then 0
            when
                coalesce(p.max_lease_duration_seconds, 0) = 0
                then coalesce(r.max_lease_duration_seconds, 0)
            when
                coalesce(r.max_lease_duration_seconds, 0) = 0
                then coalesce(p.max_lease_duration_seconds, 0)
            else
                least(
                    coalesce(p.max_lease_duration_seconds, 0),
                    coalesce(r.max_lease_duration_seconds, 0)
                )
        end
    ) as max_duration_seconds,

    coalesce(r.lease_rate_max_leases, 0::public.u256) as lease_rate_max_leases,
    coalesce(
        r.lease_rate_window_seconds, 0::public.u256
    ) as lease_rate_window_seconds,

    (
        case
            when
                coalesce(r.lease_rate_max_leases, 0::public.u256)
                = 0::public.u256
                or coalesce(r.lease_rate_window_seconds, 0::public.u256)
                = 0::public.u256
                then null
            else greatest(
                coalesce(r.lease_rate_max_leases, 0::public.u256)
                - (
                    select count(*)::numeric(78, 0)
                    from hub.lease_versions lv
                    where
                        lv.valid_to_seq is null
                        and lv.realtor = r.realtor
                        and lv.start_time >= (
                            extract(epoch from now())::bigint
                            - coalesce(
                                r.lease_rate_window_seconds, 0::public.u256
                            )::bigint
                        )
                ),
                0::public.u256
            )
        end
    ) as lease_rate_remaining
from realtors r
left join protocol p on true;

-- Candidate receiver salts with balance + free status for a single-query
-- selection.
create or replace view api.receiver_salt_candidates as
select
    cr.receiver_salt,
    coalesce(w.receiver, cr.receiver) as receiver,
    w.receiver_evm,

    coalesce(b.balance_amount, 0::public.u256) as balance_amount,
    (
        coalesce(b.balance_amount, 0::public.u256) > 0::public.u256
    ) as has_balance,

    l.nukeable_after,
    (
        l.nukeable_after is null
        or l.nukeable_after <= extract(epoch from now())::bigint
    ) as is_free
from api.controller_receivers cr
left join ctl.receiver_watchlist w
    on w.receiver_salt = cr.receiver_salt
left join api.receiver_usdt_balances b
    on b.receiver_salt = cr.receiver_salt
left join lateral (
    select lv.nukeable_after
    from hub.lease_versions lv
    where lv.valid_to_seq is null and lv.receiver_salt = cr.receiver_salt
    order by lv.lease_number desc
    limit 1
) l on true;

comment on view api.receiver_usdt_balances is
$$Net receiver USDT balances derived from indexed transfer logs and pull
ledgers.

This is a deterministic approximation of each receiver's USDT balance:
  sum(incoming TRC-20 transfers into the receiver) - sum(controller pulls from that receiver)

It assumes receiver addresses do not have other outflows besides controller pulls.$$;

comment on view api.realtor_effective_config is
$$Realtor effective config
(protocol floors + realtor overrides + rate remaining).

This view merges the protocol-wide floor limits with the current realtor row and returns:
- `allowed`
- `min_fee_ppm`, `min_flat_fee`, `max_duration_seconds` (effective minima/maxima)
- `lease_rate_*` and `lease_rate_remaining` (best-effort, computed from current leases within the window)$$;

comment on view api.receiver_salt_candidates is
$$Receiver salt candidates for realtor selection.

Joins:
- `api.controller_receivers` (allowed salts)
- `api.receiver_usdt_balances` (cached balance view)
- latest hub lease by receiver_salt (for `nukeable_after`)

Computed fields:
- `has_balance`: `balance_amount > 0`
- `is_free`: receiver has no current lease or lease is nukeable (based on `nukeable_after <= now`)$$;

-- Ensure PostgREST anon role can read new api views
-- (no-ops if roles don't exist).
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on all tables in schema api to pgrst_anon;
    alter default privileges in schema api grant select on tables to pgrst_anon;
  end if;
end $$;
