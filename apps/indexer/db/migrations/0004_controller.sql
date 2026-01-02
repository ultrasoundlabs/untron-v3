/*
Controller projection (UntronControllerIndex).

Same model as hub:
- versioned tables for current singleton/KV state
- ledgers for actions
- apply_catchup + rollback_from driven by event_seq
- strict prev_tip == cursor.tip

Also upgrades the chain triggers created in 0003 to:
- handle BOTH streams
- compute rollback points independently per stream
*/

-- =========================
-- CONTROLLER VERSIONED TABLES
-- =========================

-- OwnerChanged (singleton)
create table if not exists ctl.owner_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    owner tron_address not null
);
create unique index if not exists ctl_owner_current_unique
on ctl.owner_versions ((1)) where valid_to_seq is null;

-- ExecutorChanged (singleton)
create table if not exists ctl.executor_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    executor tron_address not null
);
create unique index if not exists ctl_executor_current_unique
on ctl.executor_versions ((1)) where valid_to_seq is null;

-- UsdtSet (singleton)
create table if not exists ctl.usdt_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    usdt tron_address not null
);
create unique index if not exists ctl_usdt_current_unique
on ctl.usdt_versions ((1)) where valid_to_seq is null;

-- LpSet (singleton)
create table if not exists ctl.lp_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    lp tron_address not null
);
create unique index if not exists ctl_lp_current_unique
on ctl.lp_versions ((1)) where valid_to_seq is null;

-- PayloadSet (KV by rebalancer)
create table if not exists ctl.payload_versions (
    rebalancer tron_address not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    payload bytes_hex not null,
    primary key (rebalancer, valid_from_seq)
);
create unique index if not exists ctl_payload_current_unique
on ctl.payload_versions (rebalancer) where valid_to_seq is null;

-- ReceiverDeployed (KV by receiver_salt)
create table if not exists ctl.receiver_versions (
    receiver_salt bytes32_hex not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    receiver tron_address not null,
    primary key (receiver_salt, valid_from_seq)
);
create unique index if not exists ctl_receiver_current_unique
on ctl.receiver_versions (receiver_salt) where valid_to_seq is null;

-- LpExchangeRateSet (KV by token)
create table if not exists ctl.lp_exchange_rate_versions (
    token tron_address not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    exchange_rate u256 not null,
    primary key (token, valid_from_seq)
);
create unique index if not exists ctl_lp_exchange_rate_current_unique
on ctl.lp_exchange_rate_versions (token) where valid_to_seq is null;

-- =========================
-- VERSION RANGE CHECKS
-- =========================
alter table ctl.owner_versions
add constraint ctl_owner_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.executor_versions
add constraint ctl_executor_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.usdt_versions
add constraint ctl_usdt_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.lp_versions
add constraint ctl_lp_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.payload_versions
add constraint ctl_payload_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.receiver_versions
add constraint ctl_receiver_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table ctl.lp_exchange_rate_versions
add constraint ctl_lp_exchange_rate_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

-- =========================
-- CONTROLLER LEDGERS
-- =========================

create table if not exists ctl.pulled_from_receiver_ledger (
    event_seq bigint primary key,
    receiver_salt bytes32_hex not null,
    token tron_address not null,
    token_amount u256 not null,
    exchange_rate u256 not null,
    usdt_amount u256 not null
);

create table if not exists ctl.usdt_rebalanced_ledger (
    event_seq bigint primary key,
    in_amount u256 not null,
    out_amount u256 not null,
    rebalancer tron_address not null
);

create table if not exists ctl.controller_usdt_transfer_ledger (
    event_seq bigint primary key,
    recipient tron_address not null,
    amount u256 not null
);

create table if not exists ctl.lp_tokens_withdrawn_ledger (
    event_seq bigint primary key,
    token tron_address not null,
    amount u256 not null
);

-- =========================
-- CONTROLLER APPLY ONE
-- =========================
create or replace function ctl.apply_one(
    p_seq bigint,
    p_type text,
    p_args jsonb
)
returns void language plpgsql as $$
begin
  if p_type = 'OwnerChanged' then
    perform chain.require_json_keys(p_args, array['new_owner']);
    update ctl.owner_versions set valid_to_seq = p_seq where valid_to_seq is null;
    insert into ctl.owner_versions(valid_from_seq, valid_to_seq, owner)
    values (p_seq, null, chain.tron_address_from_text(p_args->>'new_owner'));

  elsif p_type = 'ExecutorChanged' then
    perform chain.require_json_keys(p_args, array['new_executor']);
    update ctl.executor_versions set valid_to_seq = p_seq where valid_to_seq is null;
    insert into ctl.executor_versions(valid_from_seq, valid_to_seq, executor)
    values (p_seq, null, chain.tron_address_from_text(p_args->>'new_executor'));

  elsif p_type = 'UsdtSet' then
    perform chain.require_json_keys(p_args, array['new_usdt']);
    update ctl.usdt_versions set valid_to_seq = p_seq where valid_to_seq is null;
    insert into ctl.usdt_versions(valid_from_seq, valid_to_seq, usdt)
    values (p_seq, null, chain.tron_address_from_text(p_args->>'new_usdt'));

  elsif p_type = 'LpSet' then
    perform chain.require_json_keys(p_args, array['new_lp']);
    update ctl.lp_versions set valid_to_seq = p_seq where valid_to_seq is null;
    insert into ctl.lp_versions(valid_from_seq, valid_to_seq, lp)
    values (p_seq, null, chain.tron_address_from_text(p_args->>'new_lp'));

  elsif p_type = 'PayloadSet' then
    perform chain.require_json_keys(p_args, array['rebalancer','payload']);
    update ctl.payload_versions set valid_to_seq = p_seq
      where rebalancer = chain.tron_address_from_text(p_args->>'rebalancer') and valid_to_seq is null;
    insert into ctl.payload_versions(rebalancer, valid_from_seq, valid_to_seq, payload)
    values (chain.tron_address_from_text(p_args->>'rebalancer'), p_seq, null, (p_args->>'payload')::bytes_hex);

  elsif p_type = 'ReceiverDeployed' then
    perform chain.require_json_keys(p_args, array['receiver','salt']);
    update ctl.receiver_versions set valid_to_seq = p_seq
      where receiver_salt = (p_args->>'salt')::bytes32_hex and valid_to_seq is null;
    insert into ctl.receiver_versions(receiver_salt, valid_from_seq, valid_to_seq, receiver)
    values ((p_args->>'salt')::bytes32_hex, p_seq, null, chain.tron_address_from_text(p_args->>'receiver'));

  elsif p_type = 'LpExchangeRateSet' then
    perform chain.require_json_keys(p_args, array['token','exchange_rate']);
    update ctl.lp_exchange_rate_versions set valid_to_seq = p_seq
      where token = chain.tron_address_from_text(p_args->>'token') and valid_to_seq is null;
    insert into ctl.lp_exchange_rate_versions(token, valid_from_seq, valid_to_seq, exchange_rate)
    values (chain.tron_address_from_text(p_args->>'token'), p_seq, null, (p_args->>'exchange_rate')::u256);

  -- ledgers
  elsif p_type = 'PulledFromReceiver' then
    perform chain.require_json_keys(p_args, array['receiver_salt','token','token_amount','exchange_rate','usdt_amount']);
    insert into ctl.pulled_from_receiver_ledger(event_seq, receiver_salt, token, token_amount, exchange_rate, usdt_amount)
    values (
      p_seq,
      (p_args->>'receiver_salt')::bytes32_hex,
      chain.tron_address_from_text(p_args->>'token'),
      (p_args->>'token_amount')::u256,
      (p_args->>'exchange_rate')::u256,
      (p_args->>'usdt_amount')::u256
    );

  elsif p_type = 'UsdtRebalanced' then
    perform chain.require_json_keys(p_args, array['in_amount','out_amount','rebalancer']);
    insert into ctl.usdt_rebalanced_ledger(event_seq, in_amount, out_amount, rebalancer)
    values (
      p_seq,
      (p_args->>'in_amount')::u256,
      (p_args->>'out_amount')::u256,
      chain.tron_address_from_text(p_args->>'rebalancer')
    );

  elsif p_type = 'ControllerUsdtTransfer' then
    perform chain.require_json_keys(p_args, array['recipient','amount']);
    insert into ctl.controller_usdt_transfer_ledger(event_seq, recipient, amount)
    values (p_seq, chain.tron_address_from_text(p_args->>'recipient'), (p_args->>'amount')::u256);

  elsif p_type = 'LpTokensWithdrawn' then
    perform chain.require_json_keys(p_args, array['token','amount']);
    insert into ctl.lp_tokens_withdrawn_ledger(event_seq, token, amount)
    values (p_seq, chain.tron_address_from_text(p_args->>'token'), (p_args->>'amount')::u256);

  else
    null;
  end if;
end $$;

-- =========================
-- CONTROLLER ROLLBACK
-- =========================
create or replace function ctl.rollback_from(rollback_seq bigint)
returns void language plpgsql as $$
begin
  -- ledgers
  delete from ctl.lp_tokens_withdrawn_ledger where event_seq >= rollback_seq;
  delete from ctl.controller_usdt_transfer_ledger where event_seq >= rollback_seq;
  delete from ctl.usdt_rebalanced_ledger where event_seq >= rollback_seq;
  delete from ctl.pulled_from_receiver_ledger where event_seq >= rollback_seq;

  -- versioned
  delete from ctl.lp_exchange_rate_versions where valid_from_seq >= rollback_seq;
  update ctl.lp_exchange_rate_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.receiver_versions where valid_from_seq >= rollback_seq;
  update ctl.receiver_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.payload_versions where valid_from_seq >= rollback_seq;
  update ctl.payload_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.lp_versions where valid_from_seq >= rollback_seq;
  update ctl.lp_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.usdt_versions where valid_from_seq >= rollback_seq;
  update ctl.usdt_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.executor_versions where valid_from_seq >= rollback_seq;
  update ctl.executor_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from ctl.owner_versions where valid_from_seq >= rollback_seq;
  update ctl.owner_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  -- cursor rewind
  update chain.stream_cursor
     set applied_through_seq = rollback_seq - 1,
         updated_at = now()
   where stream = 'controller';

  update chain.stream_cursor c
     set tip =
       case when c.applied_through_seq = 0
            then (select genesis_tip from chain.instance where stream='controller')
            else (select e.new_tip from chain.event_appended e
                   where e.stream='controller' and e.canonical and e.event_seq = c.applied_through_seq
                   limit 1)
       end
   where c.stream='controller';
end $$;

-- =========================
-- CONTROLLER APPLY CATCHUP
-- =========================
create or replace function ctl.apply_catchup()
returns void language plpgsql as $$
declare
  cur_seq bigint;
  cur_tip bytes32_hex;
  next_seq bigint;
  ev record;
begin
  perform pg_advisory_xact_lock(9101, 2);

  select applied_through_seq, tip
    into cur_seq, cur_tip
    from chain.stream_cursor
   where stream='controller'
   for update;

  if not found then
    raise exception 'stream cursor not initialized for controller (call chain.configure_instance(''controller'', ...))';
  end if;

  loop
    next_seq := cur_seq + 1;

    select *
      into ev
      from chain.event_appended
     where stream='controller' and canonical and event_seq = next_seq
     limit 1;

    exit when not found;

    if ev.prev_tip <> cur_tip then
      raise exception 'controller tip mismatch at seq %, expected %, got %', next_seq, cur_tip, ev.prev_tip;
    end if;

    perform ctl.apply_one(ev.event_seq, ev.event_type, ev.args);

    cur_seq := next_seq;
    cur_tip := ev.new_tip;
  end loop;

  update chain.stream_cursor
     set applied_through_seq = cur_seq,
         tip = cur_tip,
         updated_at = now()
   where stream='controller';
end $$;

-- =========================
-- INGEST TRIGGERS (both streams)
-- =========================
create or replace function chain.on_event_appended_insert()
returns trigger language plpgsql as $$
begin
  if exists (select 1 from new_rows where stream='hub' and canonical) then
    perform hub.apply_catchup();
  end if;

  if exists (select 1 from new_rows where stream='controller' and canonical) then
    perform ctl.apply_catchup();
  end if;

  return null;
end $$;

create or replace function chain.on_event_appended_canonical_update()
returns trigger language plpgsql as $$
declare
  hub_rollback bigint;
  ctl_rollback bigint;
begin
  -- compute per-stream rollback points for canonical TRUE -> FALSE flips
  select min(o.event_seq)
    into hub_rollback
    from old_rows o join new_rows n using (id)
   where o.stream='hub' and o.canonical is true and n.canonical is false;

  select min(o.event_seq)
    into ctl_rollback
    from old_rows o join new_rows n using (id)
   where o.stream='controller' and o.canonical is true and n.canonical is false;

  if hub_rollback is not null then
    perform hub.rollback_from(hub_rollback);
  end if;

  if ctl_rollback is not null then
    perform ctl.rollback_from(ctl_rollback);
  end if;

  -- re-apply catchup if anything changed
  if exists (
    select 1 from old_rows o join new_rows n using (id)
     where o.stream='hub' and o.canonical is distinct from n.canonical
  ) then
    perform hub.apply_catchup();
  end if;

  if exists (
    select 1 from old_rows o join new_rows n using (id)
     where o.stream='controller' and o.canonical is distinct from n.canonical
  ) then
    perform ctl.apply_catchup();
  end if;

  return null;
end $$;

drop trigger if exists trg_event_appended_insert on chain.event_appended;
create trigger trg_event_appended_insert
after insert on chain.event_appended
referencing new table as new_rows
for each statement execute function chain.on_event_appended_insert();

drop trigger if exists trg_event_appended_canonical_update
on chain.event_appended;
create trigger trg_event_appended_canonical_update
after update of canonical on chain.event_appended
referencing old table as old_rows new table as new_rows
for each statement execute function chain.on_event_appended_canonical_update();
