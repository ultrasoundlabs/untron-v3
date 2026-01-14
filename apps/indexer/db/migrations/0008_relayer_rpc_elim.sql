/*
Relayer read-RPC minimization.

Goal: allow `apps/relayer` to replace hub `eth_call` reads with PostgREST reads derived
from already-indexed hub events (no new indexer RPC load).
*/

-- =========================
-- CHAIN HELPERS
-- =========================

create or replace function chain.bytes32_hex_to_bytea(p public.bytes32_hex)
returns bytea
language sql
immutable strict
set search_path = ''
as $$
  select decode(substr(p, 3), 'hex')
$$;

create or replace function chain.bytes_hex_to_bytea(p public.bytes_hex)
returns bytea
language sql
immutable strict
set search_path = ''
as $$
  select decode(substr(p, 3), 'hex')
$$;

create or replace function chain.u256_to_be_bytes32(p public.u256)
returns bytea
language plpgsql
immutable strict
set search_path = ''
as $$
declare
  n numeric := p;
  out bytea := decode(repeat('00', 32), 'hex');
  i int;
  b int;
begin
  -- Fill bytes from least significant to most significant.
  for i in reverse 31 .. 0 loop
    b := mod(n, 256)::int;
    out := set_byte(out, i, b);
    n := (n - b) / 256;
  end loop;

  if n <> 0 then
    raise exception 'u256 too large for 32-byte encoding';
  end if;

  return out;
end;
$$;

create or replace function chain.sha256_bytes32_hex(p bytea)
returns public.bytes32_hex
language sql
immutable strict
set search_path = ''
as $$
  select ('0x' || encode(extensions.digest(p, 'sha256'), 'hex'))::public.bytes32_hex
$$;

-- =========================
-- HUB: MATERIALIZED RELAYER STATE
-- =========================

-- Solidity reference: sha256(abi.encodePacked(prevTip, seq, blockNumber, blockTimestamp, sig, data))
create or replace function hub.controller_event_chain_tip_hash(
  p_prev_tip public.bytes32_hex,
  p_seq public.u256,
  p_block_number bigint,
  p_block_timestamp bigint,
  p_event_signature public.bytes32_hex,
  p_abi_encoded_event_data public.bytes_hex
)
returns public.bytes32_hex
language sql
immutable strict
set search_path = ''
as $$
  select chain.sha256_bytes32_hex(
    chain.bytes32_hex_to_bytea(p_prev_tip)
    || chain.u256_to_be_bytes32(p_seq)
    || chain.u256_to_be_bytes32((p_block_number::numeric)::public.u256)
    || chain.u256_to_be_bytes32((p_block_timestamp::numeric)::public.u256)
    || chain.bytes32_hex_to_bytea(p_event_signature)
    || chain.bytes_hex_to_bytea(p_abi_encoded_event_data)
  )
$$;

-- Hub-side view of controller sync state (singleton).
create table if not exists hub.controller_state_versions (
  valid_from_seq bigint primary key,
  valid_to_seq bigint null,

  -- Equivalent to UntronV3Base.lastControllerEventTip()
  last_controller_event_tip public.bytes32_hex not null,
  -- Equivalent to UntronV3Base.lastControllerEventSeq()
  last_controller_event_seq public.u256 not null,
  -- Equivalent to UntronV3Base.nextControllerEventIndex()
  next_controller_event_index public.u256 not null
);
create unique index if not exists hub_controller_state_current_unique
on hub.controller_state_versions ((1)) where valid_to_seq is null;

alter table hub.controller_state_versions
add constraint hub_controller_state_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

create or replace function hub.controller_state_init()
returns void
language plpgsql
set search_path = ''
as $$
declare
  desired_genesis public.bytes32_hex;
  cur hub.controller_state_versions%rowtype;
begin
  select genesis_tip into desired_genesis
  from chain.instance
  where stream = 'controller'
  limit 1;

  if not found then
    -- Allow hub-only deployments without a controller stream configured.
    -- If controller-related hub events are ever observed, we'll fail fast on tip mismatches.
    desired_genesis := ('0x' || repeat('00', 32))::public.bytes32_hex;
  end if;

  -- Ensure a baseline row exists (valid_from_seq = 0).
  select * into cur
  from hub.controller_state_versions
  where valid_to_seq is null
  limit 1;

  if found then
    -- Repair a common stale-db case: an earlier run seeded the baseline row with a zero genesis
    -- (hub-only), and later runs enable the controller stream, causing the first controller tip
    -- update (prev_tip = true genesis) to fail.
    if cur.valid_from_seq = 0
      and cur.last_controller_event_seq = 0::public.u256
      and cur.next_controller_event_index = 0::public.u256
      and cur.last_controller_event_tip <> desired_genesis
    then
      update hub.controller_state_versions
        set last_controller_event_tip = desired_genesis
      where valid_to_seq is null;
    end if;
    return;
  end if;

  insert into hub.controller_state_versions(
    valid_from_seq, valid_to_seq,
    last_controller_event_tip,
    last_controller_event_seq,
    next_controller_event_index
  ) values (
    0, null,
    desired_genesis,
    0::public.u256,
    0::public.u256
  );
end;
$$;

create or replace function hub.controller_state_apply_tip_update(
  p_seq bigint,
  p_previous_tip public.bytes32_hex,
  p_block_number bigint,
  p_block_timestamp bigint,
  p_event_signature public.bytes32_hex,
  p_abi_encoded_event_data public.bytes_hex
)
returns void
language plpgsql
set search_path = ''
as $$
declare
  cur hub.controller_state_versions%rowtype;
  seq_new public.u256;
  tip_new public.bytes32_hex;
begin
  perform hub.controller_state_init();

  select * into cur
  from hub.controller_state_versions
  where valid_to_seq is null
  limit 1;

  if cur.last_controller_event_tip <> p_previous_tip then
    raise exception 'controller previous_tip mismatch at hub seq %: expected %, got %',
      p_seq, cur.last_controller_event_tip, p_previous_tip;
  end if;

  seq_new := (cur.last_controller_event_seq + 1)::public.u256;
  tip_new := hub.controller_event_chain_tip_hash(
    p_previous_tip,
    seq_new,
    p_block_number,
    p_block_timestamp,
    p_event_signature,
    p_abi_encoded_event_data
  );

  update hub.controller_state_versions
    set valid_to_seq = p_seq
  where valid_to_seq is null;

  insert into hub.controller_state_versions(
    valid_from_seq, valid_to_seq,
    last_controller_event_tip,
    last_controller_event_seq,
    next_controller_event_index
  ) values (
    p_seq, null,
    tip_new,
    seq_new,
    cur.next_controller_event_index
  );
end;
$$;

create or replace function hub.controller_state_apply_processed(
  p_seq bigint,
  p_event_index public.u256
)
returns void
language plpgsql
set search_path = ''
as $$
declare
  cur hub.controller_state_versions%rowtype;
  next_new public.u256;
begin
  perform hub.controller_state_init();

  select * into cur
  from hub.controller_state_versions
  where valid_to_seq is null
  limit 1;

  next_new := greatest(cur.next_controller_event_index, (p_event_index + 1)::public.u256);
  if next_new = cur.next_controller_event_index then
    return;
  end if;

  update hub.controller_state_versions
    set valid_to_seq = p_seq
  where valid_to_seq is null;

  insert into hub.controller_state_versions(
    valid_from_seq, valid_to_seq,
    last_controller_event_tip,
    last_controller_event_seq,
    next_controller_event_index
  ) values (
    p_seq, null,
    cur.last_controller_event_tip,
    cur.last_controller_event_seq,
    next_new
  );
end;
$$;

-- =========================
-- BACKFILL (derive state for already-applied events)
-- =========================

create or replace function hub.backfill_controller_state()
returns void
language plpgsql
set search_path = ''
as $$
declare
  genesis public.bytes32_hex;
  cur_tip public.bytes32_hex;
  cur_seq public.u256;
  cur_next public.u256;
  rec record;
begin
  delete from hub.controller_state_versions;

  select genesis_tip into genesis
  from chain.instance
  where stream = 'controller'
  limit 1;

  if not found then
    if exists (select 1 from hub.controller_tip_updates_ledger limit 1)
      or exists (select 1 from hub.controller_processed_ledger limit 1)
    then
      raise exception 'cannot backfill controller_state: controller hub events exist but chain.instance stream=controller is missing';
    end if;
    genesis := ('0x' || repeat('00', 32))::public.bytes32_hex;
  end if;

  cur_tip := genesis;
  cur_seq := 0::public.u256;
  cur_next := 0::public.u256;

  insert into hub.controller_state_versions(
    valid_from_seq, valid_to_seq,
    last_controller_event_tip,
    last_controller_event_seq,
    next_controller_event_index
  ) values (
    0, null,
    cur_tip, cur_seq, cur_next
  );

  for rec in (
    select
      event_seq,
      kind,
      previous_tip,
      block_number,
      block_timestamp,
      event_signature,
      abi_encoded_event_data,
      event_index
    from (
      select
        event_seq,
        'tip'::text as kind,
        previous_tip,
        block_number,
        block_timestamp,
        event_signature,
        abi_encoded_event_data,
        null::public.u256 as event_index
      from hub.controller_tip_updates_ledger
      union all
      select
        event_seq,
        'processed'::text as kind,
        null::public.bytes32_hex as previous_tip,
        block_number,
        block_timestamp,
        event_signature,
        abi_encoded_event_data,
        event_index
      from hub.controller_processed_ledger
    ) x
    order by event_seq asc
  ) loop
    -- Close current row.
    update hub.controller_state_versions
      set valid_to_seq = rec.event_seq
    where valid_to_seq is null;

    if rec.kind = 'tip' then
      if rec.previous_tip <> cur_tip then
        raise exception 'controller previous_tip mismatch at hub seq %: expected %, got %',
          rec.event_seq, cur_tip, rec.previous_tip;
      end if;
      cur_seq := (cur_seq + 1)::public.u256;
      cur_tip := hub.controller_event_chain_tip_hash(
        cur_tip,
        cur_seq,
        rec.block_number,
        rec.block_timestamp,
        rec.event_signature,
        rec.abi_encoded_event_data
      );
    elsif rec.kind = 'processed' then
      cur_next := greatest(cur_next, (rec.event_index + 1)::public.u256);
    end if;

    insert into hub.controller_state_versions(
      valid_from_seq, valid_to_seq,
      last_controller_event_tip,
      last_controller_event_seq,
      next_controller_event_index
    ) values (
      rec.event_seq, null,
      cur_tip, cur_seq, cur_next
    );
  end loop;
end;
$$;

do $$
begin
  perform hub.backfill_controller_state();
end;
$$;

-- =========================
-- PROJECTOR HOOKS (wrap v7 functions)
-- =========================

alter function hub.apply_one(bigint, text, jsonb) rename to apply_one_v7;

create or replace function hub.apply_one(
    p_seq bigint,
    p_type text,
    p_args jsonb
)
returns void
language plpgsql
as $$
begin
  perform hub.apply_one_v7(p_seq, p_type, p_args);

  if p_type = 'ControllerEventChainTipUpdated' then
    perform hub.controller_state_apply_tip_update(
      p_seq,
      (p_args->>'previous_tip')::public.bytes32_hex,
      (p_args->>'block_number')::bigint,
      (p_args->>'block_timestamp')::bigint,
      (p_args->>'event_signature')::public.bytes32_hex,
      (p_args->>'abi_encoded_event_data')::public.bytes_hex
    );

  elsif p_type = 'ControllerEventProcessed' then
    perform hub.controller_state_apply_processed(p_seq, (p_args->>'event_index')::public.u256);

  end if;
end;
$$;

alter function hub.rollback_from(bigint) rename to rollback_from_v7;

create or replace function hub.rollback_from(rollback_seq bigint)
returns void
language plpgsql
as $$
begin
  perform hub.rollback_from_v7(rollback_seq);

  delete from hub.controller_state_versions where valid_from_seq >= rollback_seq;
  update hub.controller_state_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;
end;
$$;

-- =========================
-- API VIEWS
-- =========================

create or replace view api.hub_controller_state as
select *
from hub.controller_state_versions
where valid_to_seq is null;

comment on view api.hub_controller_state is
$$Hub controller sync state (materialized)

This single-row view is a DB projection of hub events and is intended to replace hub
read-RPC calls:

- `last_controller_event_tip` == UntronV3Base.lastControllerEventTip()
- `last_controller_event_seq` == UntronV3Base.lastControllerEventSeq()
- `next_controller_event_index` == UntronV3Base.nextControllerEventIndex()

Values reflect the hub's canonical event stream as applied by the indexer.$$;

create or replace view api.relayer_hub_state as
select
  cs.last_controller_event_tip,
  cs.last_controller_event_seq,
  cs.next_controller_event_index
from (
  select
    (select last_controller_event_tip from hub.controller_state_versions where valid_to_seq is null limit 1)
      as last_controller_event_tip,
    (select last_controller_event_seq from hub.controller_state_versions where valid_to_seq is null limit 1)
      as last_controller_event_seq,
    (select next_controller_event_index from hub.controller_state_versions where valid_to_seq is null limit 1)
      as next_controller_event_index
) cs
;

comment on view api.relayer_hub_state is
$$Relayer hub state bundle

Convenience single-row view combining:
- api.hub_controller_state

Intended for relayer use to minimize HTTP round trips.$$;
