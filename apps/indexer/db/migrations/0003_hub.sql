/*
Hub projection (UntronV3Index).

Model:
- Versioned tables for "latest state":
  (entity_id, valid_from_seq) PK, with valid_to_seq NULL = current row.
- Ledger tables for naturally-append-only actions.
- Rollback is suffix-only (reorgs) driven by canonical=false flips.

Integrity:
- apply canonical events strictly in event_seq order
- require ev.prev_tip == cursor.tip for every applied event
*/

-- =========================
-- HUB TYPES
-- =========================
create type hub.claim_status as enum ('created', 'filled') ;

-- =========================
-- HUB VERSIONED TABLES (STATE)
-- =========================

-- OwnershipTransferred (singleton)
create table if not exists hub.ownership_versions (
valid_from_seq bigint primary key,
valid_to_seq bigint null,
old_owner evm_address not null,
new_owner evm_address not null
) ;
create unique index if not exists hub_ownership_current_unique
on hub.ownership_versions ((1)) where valid_to_seq is null ;

-- Protocol config snapshot (singleton)
create table if not exists hub.protocol_config_versions (
valid_from_seq bigint primary key,
valid_to_seq bigint null,

usdt evm_address null,
tron_usdt evm_address null,
tron_reader evm_address null,

floor_ppm bigint null,
floor_flat_fee u256 null,
max_lease_duration_seconds bigint null,

lessee_rate_max_updates u256 null,
lessee_rate_window_seconds u256 null
) ;
create unique index if not exists hub_protocol_config_current_unique
on hub.protocol_config_versions ((1)) where valid_to_seq is null ;

-- Realtor allowlist + per-realtor config (KV)
create table if not exists hub.realtor_versions (
realtor evm_address not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,

allowed boolean null,

min_fee_ppm bigint null,
min_flat_fee u256 null,
max_lease_duration_seconds bigint null,
lease_rate_max_leases u256 null,
lease_rate_window_seconds u256 null,

primary key (realtor, valid_from_seq)
) ;
create unique index if not exists hub_realtor_current_unique
on hub.realtor_versions (realtor) where valid_to_seq is null ;

-- LP allowlist (KV)
create table if not exists hub.lp_allowlist_versions (
lp evm_address not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
allowed boolean not null,
primary key (lp, valid_from_seq)
) ;
create unique index if not exists hub_lp_allowlist_current_unique
on hub.lp_allowlist_versions (lp) where valid_to_seq is null ;

-- LP balance snapshot derived from deposit/withdraw (KV)
create table if not exists hub.lp_balance_versions (
lp evm_address not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
balance u256 not null,
primary key (lp, valid_from_seq)
) ;
create unique index if not exists hub_lp_balance_current_unique
on hub.lp_balance_versions (lp) where valid_to_seq is null ;

-- ChainDeprecatedSet (KV by target_chain_id)
create table if not exists hub.chain_versions (
target_chain_id bigint not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
deprecated boolean not null,
primary key (target_chain_id, valid_from_seq)
) ;
create unique index if not exists hub_chain_current_unique
on hub.chain_versions (target_chain_id) where valid_to_seq is null ;

-- SwapRateSet (KV by target_token)
create table if not exists hub.swap_rate_versions (
target_token evm_address not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
rate_ppm bigint not null,
primary key (target_token, valid_from_seq)
) ;
create unique index if not exists hub_swap_rate_current_unique
on hub.swap_rate_versions (target_token) where valid_to_seq is null ;

-- BridgerSet (KV by (target_token, target_chain_id))
create table if not exists hub.bridger_versions (
target_token evm_address not null,
target_chain_id bigint not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
bridger evm_address not null,
primary key (target_token, target_chain_id, valid_from_seq)
) ;
create unique index if not exists hub_bridger_current_unique
on hub.bridger_versions (target_token,
target_chain_id) where valid_to_seq is null ;

-- LeaseCreated (KV by lease_id)
create table if not exists hub.lease_versions (
lease_id u256 not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,

receiver_salt bytes32_hex not null,
lease_number u256 not null,

realtor evm_address not null,
lessee evm_address not null,

start_time bigint not null,
nukeable_after bigint not null,

lease_fee_ppm bigint not null,
flat_fee u256 not null,

primary key (lease_id, valid_from_seq)
) ;
create unique index if not exists hub_lease_current_unique
on hub.lease_versions (lease_id) where valid_to_seq is null ;

-- PayoutConfigUpdated (KV by lease_id)
create table if not exists hub.payout_config_versions (
lease_id u256 not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,

target_chain_id bigint not null,
target_token evm_address not null,
beneficiary evm_address not null,

primary key (lease_id, valid_from_seq)
) ;
create unique index if not exists hub_payout_config_current_unique
on hub.payout_config_versions (lease_id) where valid_to_seq is null ;

-- Claim state (KV by (lease_id, claim_id))
create table if not exists hub.claim_versions (
lease_id u256 not null,
claim_id u256 not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,

target_token evm_address not null,
queue_index u256 not null,
amount_usdt u256 not null,
target_chain_id bigint not null,
beneficiary evm_address not null,

origin smallint not null,
origin_id bytes32_hex not null,
origin_actor evm_address not null,
origin_token evm_address not null,
origin_timestamp bigint not null,
origin_raw_amount u256 not null,

status hub.claim_status not null,

primary key (lease_id, claim_id, valid_from_seq)
) ;
create unique index if not exists hub_claim_current_unique
on hub.claim_versions (lease_id, claim_id) where valid_to_seq is null ;

-- LeaseNonceUpdated (KV by lease_id)
create table if not exists hub.lease_nonce_versions (
lease_id u256 not null,
valid_from_seq bigint not null,
valid_to_seq bigint null,
nonce u256 not null,
primary key (lease_id, valid_from_seq)
) ;
create unique index if not exists hub_lease_nonce_current_unique
on hub.lease_nonce_versions (lease_id) where valid_to_seq is null ;

-- ProtocolPnlUpdated (singleton snapshot)
create table if not exists hub.protocol_pnl_versions (
valid_from_seq bigint primary key,
valid_to_seq bigint null,
pnl i256 not null,
delta i256 not null,
reason smallint not null
) ;
create unique index if not exists hub_protocol_pnl_current_unique
on hub.protocol_pnl_versions ((1)) where valid_to_seq is null ;

-- =========================
-- HUB LEDGERS (append-only actions)
-- =========================

create table if not exists hub.lp_vault_events (
event_seq bigint primary key,
kind text not null check (kind in ('deposit', 'withdraw')),
lp evm_address not null,
amount u256 not null
) ;

create table if not exists hub.tokens_rescued_ledger (
event_seq bigint primary key,
token evm_address not null,
amount u256 not null
) ;

-- Controller ingestion related
-- (these are hub events carrying controller event bytes)
create table if not exists hub.controller_tip_updates_ledger (
event_seq bigint primary key,
previous_tip bytes32_hex not null,
block_number bigint not null,
block_timestamp bigint not null,
event_signature bytes32_hex not null,
abi_encoded_event_data bytes_hex not null
) ;

create table if not exists hub.controller_processed_ledger (
event_seq bigint primary key,
event_index u256 not null,
block_number bigint not null,
block_timestamp bigint not null,
event_signature bytes32_hex not null,
abi_encoded_event_data bytes_hex not null
) ;

-- =========================
-- HUB PATCH HELPERS (versioned updates)
-- =========================

-- Protocol config singleton patch
create or replace function hub.protocol_config_apply (
p_seq bigint,
p_usdt evm_address,
p_tron_usdt evm_address,
p_tron_reader evm_address,
p_floor_ppm bigint,
p_floor_flat_fee u256,
p_max_lease_duration_seconds bigint,
p_lessee_rate_max_updates u256,
p_lessee_rate_window_seconds u256
) returns void language plpgsql as $$
declare
  cur hub.protocol_config_versions%rowtype;
begin
  select * into cur
    from hub.protocol_config_versions
   where valid_to_seq is null
   limit 1;

  update hub.protocol_config_versions
     set valid_to_seq = p_seq
   where valid_to_seq is null;

  insert into hub.protocol_config_versions(
    valid_from_seq, valid_to_seq,
    usdt, tron_usdt, tron_reader,
    floor_ppm, floor_flat_fee, max_lease_duration_seconds,
    lessee_rate_max_updates, lessee_rate_window_seconds
  ) values (
    p_seq, null,
    coalesce(p_usdt, cur.usdt),
    coalesce(p_tron_usdt, cur.tron_usdt),
    coalesce(p_tron_reader, cur.tron_reader),
    coalesce(p_floor_ppm, cur.floor_ppm),
    coalesce(p_floor_flat_fee, cur.floor_flat_fee),
    coalesce(p_max_lease_duration_seconds, cur.max_lease_duration_seconds),
    coalesce(p_lessee_rate_max_updates, cur.lessee_rate_max_updates),
    coalesce(p_lessee_rate_window_seconds, cur.lessee_rate_window_seconds)
  );
end $$ ;

-- Realtor KV patch
create or replace function hub.realtor_apply (
p_seq bigint,
p_realtor evm_address,
p_allowed boolean,
p_min_fee_ppm bigint,
p_min_flat_fee u256,
p_max_lease_duration_seconds bigint,
p_lease_rate_max_leases u256,
p_lease_rate_window_seconds u256
) returns void language plpgsql as $$
declare
  cur hub.realtor_versions%rowtype;
begin
  select * into cur
    from hub.realtor_versions
   where realtor = p_realtor and valid_to_seq is null
   limit 1;

  update hub.realtor_versions
     set valid_to_seq = p_seq
   where realtor = p_realtor and valid_to_seq is null;

  insert into hub.realtor_versions(
    realtor, valid_from_seq, valid_to_seq,
    allowed, min_fee_ppm, min_flat_fee, max_lease_duration_seconds,
    lease_rate_max_leases, lease_rate_window_seconds
  ) values (
    p_realtor, p_seq, null,
    coalesce(p_allowed, cur.allowed),
    coalesce(p_min_fee_ppm, cur.min_fee_ppm),
    coalesce(p_min_flat_fee, cur.min_flat_fee),
    coalesce(p_max_lease_duration_seconds, cur.max_lease_duration_seconds),
    coalesce(p_lease_rate_max_leases, cur.lease_rate_max_leases),
    coalesce(p_lease_rate_window_seconds, cur.lease_rate_window_seconds)
  );
end $$ ;

-- LP allowlist set
create or replace function hub.lp_allowlist_set (p_seq bigint,
p_lp evm_address,
p_allowed boolean)
returns void language plpgsql as $$
begin
  update hub.lp_allowlist_versions
     set valid_to_seq = p_seq
   where lp = p_lp and valid_to_seq is null;

  insert into hub.lp_allowlist_versions(lp, valid_from_seq, valid_to_seq, allowed)
  values (p_lp, p_seq, null, p_allowed);
end $$ ;

-- LP balance delta apply (derived state)
create or replace function hub.lp_balance_apply_delta (p_seq bigint,
p_lp evm_address,
p_delta i256)
returns void language plpgsql as $$
declare
  cur_bal u256;
  new_bal i256;
begin
  select balance into cur_bal
    from hub.lp_balance_versions
   where lp = p_lp and valid_to_seq is null
   limit 1;

  if cur_bal is null then
    cur_bal := 0;
  end if;

  -- compute as signed then cast back to u256 (domain check prevents negative)
  new_bal := cur_bal + p_delta;

  update hub.lp_balance_versions
     set valid_to_seq = p_seq
   where lp = p_lp and valid_to_seq is null;

  insert into hub.lp_balance_versions(lp, valid_from_seq, valid_to_seq, balance)
  values (p_lp, p_seq, null, new_bal::u256);
end $$ ;

-- Chain deprecated set
create or replace function hub.chain_set (p_seq bigint,
p_target_chain_id bigint,
p_deprecated boolean)
returns void language plpgsql as $$
begin
  update hub.chain_versions
     set valid_to_seq = p_seq
   where target_chain_id = p_target_chain_id and valid_to_seq is null;

  insert into hub.chain_versions(target_chain_id, valid_from_seq, valid_to_seq, deprecated)
  values (p_target_chain_id, p_seq, null, p_deprecated);
end $$ ;

-- Swap rate set
create or replace function hub.swap_rate_set (p_seq bigint,
p_target_token evm_address,
p_rate_ppm bigint)
returns void language plpgsql as $$
begin
  update hub.swap_rate_versions
     set valid_to_seq = p_seq
   where target_token = p_target_token and valid_to_seq is null;

  insert into hub.swap_rate_versions(target_token, valid_from_seq, valid_to_seq, rate_ppm)
  values (p_target_token, p_seq, null, p_rate_ppm);
end $$ ;

-- Bridger set
create or replace function hub.bridger_set (
p_seq bigint,
p_target_token evm_address,
p_target_chain_id bigint,
p_bridger evm_address
) returns void language plpgsql as $$
begin
  update hub.bridger_versions
     set valid_to_seq = p_seq
   where target_token = p_target_token
     and target_chain_id = p_target_chain_id
     and valid_to_seq is null;

  insert into hub.bridger_versions(target_token, target_chain_id, valid_from_seq, valid_to_seq, bridger)
  values (p_target_token, p_target_chain_id, p_seq, null, p_bridger);
end $$ ;

-- Lease create (should be first creation of lease_id in a fork)
create or replace function hub.lease_create (
p_seq bigint,
p_lease_id u256,
p_receiver_salt bytes32_hex,
p_lease_number u256,
p_realtor evm_address,
p_lessee evm_address,
p_start_time bigint,
p_nukeable_after bigint,
p_lease_fee_ppm bigint,
p_flat_fee u256
) returns void language plpgsql as $$
begin
  -- close any current row (should not exist in a consistent canonical fork)
  update hub.lease_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and valid_to_seq is null;

  insert into hub.lease_versions(
    lease_id, valid_from_seq, valid_to_seq,
    receiver_salt, lease_number, realtor, lessee,
    start_time, nukeable_after, lease_fee_ppm, flat_fee
  ) values (
    p_lease_id, p_seq, null,
    p_receiver_salt, p_lease_number, p_realtor, p_lessee,
    p_start_time, p_nukeable_after, p_lease_fee_ppm, p_flat_fee
  );
end $$ ;

-- Payout config set (versioned per lease)
create or replace function hub.payout_config_set (
p_seq bigint,
p_lease_id u256,
p_target_chain_id bigint,
p_target_token evm_address,
p_beneficiary evm_address
) returns void language plpgsql as $$
begin
  update hub.payout_config_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and valid_to_seq is null;

  insert into hub.payout_config_versions(
    lease_id, valid_from_seq, valid_to_seq,
    target_chain_id, target_token, beneficiary
  ) values (
    p_lease_id, p_seq, null,
    p_target_chain_id, p_target_token, p_beneficiary
  );
end $$ ;

-- Claim created
create or replace function hub.claim_create (
p_seq bigint,
p_lease_id u256,
p_claim_id u256,
p_target_token evm_address,
p_queue_index u256,
p_amount_usdt u256,
p_target_chain_id bigint,
p_beneficiary evm_address,
p_origin smallint,
p_origin_id bytes32_hex,
p_origin_actor evm_address,
p_origin_token evm_address,
p_origin_timestamp bigint,
p_origin_raw_amount u256
) returns void language plpgsql as $$
begin
  update hub.claim_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and claim_id = p_claim_id and valid_to_seq is null;

  insert into hub.claim_versions(
    lease_id, claim_id, valid_from_seq, valid_to_seq,
    target_token, queue_index, amount_usdt, target_chain_id, beneficiary,
    origin, origin_id, origin_actor, origin_token, origin_timestamp, origin_raw_amount,
    status
  ) values (
    p_lease_id, p_claim_id, p_seq, null,
    p_target_token, p_queue_index, p_amount_usdt, p_target_chain_id, p_beneficiary,
    p_origin, p_origin_id, p_origin_actor, p_origin_token, p_origin_timestamp, p_origin_raw_amount,
    'created'
  );
end $$ ;

-- Claim filled (versioned status update)
create or replace function hub.claim_fill (
p_seq bigint,
p_lease_id u256,
p_claim_id u256
) returns void language plpgsql as $$
declare
  cur hub.claim_versions%rowtype;
begin
  select * into cur
    from hub.claim_versions
   where lease_id = p_lease_id and claim_id = p_claim_id and valid_to_seq is null
   limit 1;

  if not found then
    raise exception 'ClaimFilled without existing current ClaimCreated: lease_id %, claim_id %', p_lease_id, p_claim_id;
  end if;

  update hub.claim_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and claim_id = p_claim_id and valid_to_seq is null;

  insert into hub.claim_versions(
    lease_id, claim_id, valid_from_seq, valid_to_seq,
    target_token, queue_index, amount_usdt, target_chain_id, beneficiary,
    origin, origin_id, origin_actor, origin_token, origin_timestamp, origin_raw_amount,
    status
  ) values (
    cur.lease_id, cur.claim_id, p_seq, null,
    cur.target_token, cur.queue_index, cur.amount_usdt, cur.target_chain_id, cur.beneficiary,
    cur.origin, cur.origin_id, cur.origin_actor, cur.origin_token, cur.origin_timestamp, cur.origin_raw_amount,
    'filled'
  );
end $$ ;

-- Lease nonce set
create or replace function hub.lease_nonce_set (p_seq bigint,
p_lease_id u256,
p_nonce u256)
returns void language plpgsql as $$
begin
  update hub.lease_nonce_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and valid_to_seq is null;

  insert into hub.lease_nonce_versions(lease_id, valid_from_seq, valid_to_seq, nonce)
  values (p_lease_id, p_seq, null, p_nonce);
end $$ ;

-- Protocol pnl set (singleton snapshot)
create or replace function hub.protocol_pnl_set (p_seq bigint,
p_pnl i256,
p_delta i256,
p_reason smallint)
returns void language plpgsql as $$
begin
  update hub.protocol_pnl_versions
     set valid_to_seq = p_seq
   where valid_to_seq is null;

  insert into hub.protocol_pnl_versions(valid_from_seq, valid_to_seq, pnl, delta, reason)
  values (p_seq, null, p_pnl, p_delta, p_reason);
end $$ ;

-- =========================
-- HUB APPLY ONE (event interpreter)
-- =========================
create or replace function hub.apply_one (p_seq bigint,
p_type text,
p_args jsonb)
returns void language plpgsql as $$
declare
  v_lp evm_address;
  v_amt u256;
begin
  -- OwnershipTransferred
  if p_type = 'OwnershipTransferred' then
    perform chain.require_json_keys(p_args, array['old_owner','new_owner']);
    update hub.ownership_versions set valid_to_seq = p_seq where valid_to_seq is null;
    insert into hub.ownership_versions(valid_from_seq, valid_to_seq, old_owner, new_owner)
    values (p_seq, null, (p_args->>'old_owner')::evm_address, (p_args->>'new_owner')::evm_address);

  -- Protocol config fields
  elsif p_type = 'UsdtSet' then
    perform chain.require_json_keys(p_args, array['usdt']);
    perform hub.protocol_config_apply(p_seq, (p_args->>'usdt')::evm_address, null, null, null, null, null, null, null);

  elsif p_type = 'TronUsdtSet' then
    perform chain.require_json_keys(p_args, array['tron_usdt']);
    perform hub.protocol_config_apply(p_seq, null, (p_args->>'tron_usdt')::evm_address, null, null, null, null, null, null);

  elsif p_type = 'TronReaderSet' then
    perform chain.require_json_keys(p_args, array['reader']);
    perform hub.protocol_config_apply(p_seq, null, null, (p_args->>'reader')::evm_address, null, null, null, null, null);

  elsif p_type = 'ProtocolFloorSet' then
    perform chain.require_json_keys(p_args, array['floor_ppm']);
    perform hub.protocol_config_apply(p_seq, null, null, null, (p_args->>'floor_ppm')::bigint, null, null, null, null);

  elsif p_type = 'ProtocolFlatFeeFloorSet' then
    perform chain.require_json_keys(p_args, array['floor_flat_fee']);
    perform hub.protocol_config_apply(p_seq, null, null, null, null, (p_args->>'floor_flat_fee')::u256, null, null, null);

  elsif p_type = 'ProtocolMaxLeaseDurationSet' then
    perform chain.require_json_keys(p_args, array['max_lease_duration_seconds']);
    perform hub.protocol_config_apply(p_seq, null, null, null, null, null, (p_args->>'max_lease_duration_seconds')::bigint, null, null);

  elsif p_type = 'LesseePayoutConfigRateLimitSet' then
    perform chain.require_json_keys(p_args, array['max_updates','window_seconds']);
    perform hub.protocol_config_apply(
      p_seq, null, null, null, null, null, null,
      (p_args->>'max_updates')::u256,
      (p_args->>'window_seconds')::u256
    );

  -- Realtors
  elsif p_type = 'RealtorSet' then
    perform chain.require_json_keys(p_args, array['realtor','allowed']);
    perform hub.realtor_apply(p_seq, (p_args->>'realtor')::evm_address, (p_args->>'allowed')::boolean, null, null, null, null, null);

  elsif p_type = 'RealtorMinFeeSet' then
    perform chain.require_json_keys(p_args, array['realtor','min_fee_ppm']);
    perform hub.realtor_apply(p_seq, (p_args->>'realtor')::evm_address, null, (p_args->>'min_fee_ppm')::bigint, null, null, null, null);

  elsif p_type = 'RealtorMinFlatFeeSet' then
    perform chain.require_json_keys(p_args, array['realtor','min_flat_fee']);
    perform hub.realtor_apply(p_seq, (p_args->>'realtor')::evm_address, null, null, (p_args->>'min_flat_fee')::u256, null, null, null);

  elsif p_type = 'RealtorMaxLeaseDurationSet' then
    perform chain.require_json_keys(p_args, array['realtor','max_lease_duration_seconds']);
    perform hub.realtor_apply(p_seq, (p_args->>'realtor')::evm_address, null, null, null, (p_args->>'max_lease_duration_seconds')::bigint, null, null);

  elsif p_type = 'RealtorLeaseRateLimitSet' then
    perform chain.require_json_keys(p_args, array['realtor','max_leases','window_seconds']);
    perform hub.realtor_apply(
      p_seq,
      (p_args->>'realtor')::evm_address,
      null, null, null, null,
      (p_args->>'max_leases')::u256,
      (p_args->>'window_seconds')::u256
    );

  -- LP allowlist + vault events
  elsif p_type = 'LpSet' then
    perform chain.require_json_keys(p_args, array['lp','allowed']);
    perform hub.lp_allowlist_set(p_seq, (p_args->>'lp')::evm_address, (p_args->>'allowed')::boolean);

  elsif p_type = 'LpDeposited' then
    perform chain.require_json_keys(p_args, array['lp','amount']);
    v_lp := (p_args->>'lp')::evm_address;
    v_amt := (p_args->>'amount')::u256;
    insert into hub.lp_vault_events(event_seq, kind, lp, amount) values (p_seq, 'deposit', v_lp, v_amt);
    perform hub.lp_balance_apply_delta(p_seq, v_lp, v_amt::i256);

  elsif p_type = 'LpWithdrawn' then
    perform chain.require_json_keys(p_args, array['lp','amount']);
    v_lp := (p_args->>'lp')::evm_address;
    v_amt := (p_args->>'amount')::u256;
    insert into hub.lp_vault_events(event_seq, kind, lp, amount) values (p_seq, 'withdraw', v_lp, v_amt);
    perform hub.lp_balance_apply_delta(p_seq, v_lp, - (v_amt::i256));

  -- Chains / swap / bridgers
  elsif p_type = 'ChainDeprecatedSet' then
    perform chain.require_json_keys(p_args, array['target_chain_id','deprecated']);
    perform hub.chain_set(p_seq, (p_args->>'target_chain_id')::bigint, (p_args->>'deprecated')::boolean);

  elsif p_type = 'SwapRateSet' then
    perform chain.require_json_keys(p_args, array['target_token','rate_ppm']);
    perform hub.swap_rate_set(p_seq, (p_args->>'target_token')::evm_address, (p_args->>'rate_ppm')::bigint);

  elsif p_type = 'BridgerSet' then
    perform chain.require_json_keys(p_args, array['target_token','target_chain_id','bridger']);
    perform hub.bridger_set(
      p_seq,
      (p_args->>'target_token')::evm_address,
      (p_args->>'target_chain_id')::bigint,
      (p_args->>'bridger')::evm_address
    );

  -- Leases / payout
  elsif p_type = 'LeaseCreated' then
    perform chain.require_json_keys(p_args, array[
      'lease_id','receiver_salt','lease_number','realtor','lessee',
      'start_time','nukeable_after','lease_fee_ppm','flat_fee'
    ]);
    perform hub.lease_create(
      p_seq,
      (p_args->>'lease_id')::u256,
      (p_args->>'receiver_salt')::bytes32_hex,
      (p_args->>'lease_number')::u256,
      (p_args->>'realtor')::evm_address,
      (p_args->>'lessee')::evm_address,
      (p_args->>'start_time')::bigint,
      (p_args->>'nukeable_after')::bigint,
      (p_args->>'lease_fee_ppm')::bigint,
      (p_args->>'flat_fee')::u256
    );

  elsif p_type = 'PayoutConfigUpdated' then
    perform chain.require_json_keys(p_args, array['lease_id','target_chain_id','target_token','beneficiary']);
    perform hub.payout_config_set(
      p_seq,
      (p_args->>'lease_id')::u256,
      (p_args->>'target_chain_id')::bigint,
      (p_args->>'target_token')::evm_address,
      (p_args->>'beneficiary')::evm_address
    );

  -- Claims
  elsif p_type = 'ClaimCreated' then
    perform chain.require_json_keys(p_args, array[
      'lease_id','claim_id','target_token','queue_index','amount_usdt','target_chain_id','beneficiary',
      'origin','origin_id','origin_actor','origin_token','origin_timestamp','origin_raw_amount'
    ]);
    perform hub.claim_create(
      p_seq,
      (p_args->>'lease_id')::u256,
      (p_args->>'claim_id')::u256,
      (p_args->>'target_token')::evm_address,
      (p_args->>'queue_index')::u256,
      (p_args->>'amount_usdt')::u256,
      (p_args->>'target_chain_id')::bigint,
      (p_args->>'beneficiary')::evm_address,
      (p_args->>'origin')::smallint,
      (p_args->>'origin_id')::bytes32_hex,
      (p_args->>'origin_actor')::evm_address,
      (p_args->>'origin_token')::evm_address,
      (p_args->>'origin_timestamp')::bigint,
      (p_args->>'origin_raw_amount')::u256
    );

  elsif p_type = 'ClaimFilled' then
    perform chain.require_json_keys(p_args, array['lease_id','claim_id']);
    perform hub.claim_fill(p_seq, (p_args->>'lease_id')::u256, (p_args->>'claim_id')::u256);

  -- PnL / nonce / rescue / controller ingestion
  elsif p_type = 'ProtocolPnlUpdated' then
    perform chain.require_json_keys(p_args, array['pnl','delta','reason']);
    perform hub.protocol_pnl_set(p_seq, (p_args->>'pnl')::i256, (p_args->>'delta')::i256, (p_args->>'reason')::smallint);

  elsif p_type = 'LeaseNonceUpdated' then
    perform chain.require_json_keys(p_args, array['lease_id','nonce']);
    perform hub.lease_nonce_set(p_seq, (p_args->>'lease_id')::u256, (p_args->>'nonce')::u256);

  elsif p_type = 'TokensRescued' then
    perform chain.require_json_keys(p_args, array['token','amount']);
    insert into hub.tokens_rescued_ledger(event_seq, token, amount)
    values (p_seq, (p_args->>'token')::evm_address, (p_args->>'amount')::u256);

  elsif p_type = 'ControllerEventChainTipUpdated' then
    perform chain.require_json_keys(p_args, array['previous_tip','block_number','block_timestamp','event_signature','abi_encoded_event_data']);
    insert into hub.controller_tip_updates_ledger(
      event_seq, previous_tip, block_number, block_timestamp, event_signature, abi_encoded_event_data
    ) values (
      p_seq,
      (p_args->>'previous_tip')::bytes32_hex,
      (p_args->>'block_number')::bigint,
      (p_args->>'block_timestamp')::bigint,
      (p_args->>'event_signature')::bytes32_hex,
      (p_args->>'abi_encoded_event_data')::bytes_hex
    );

  elsif p_type = 'ControllerEventProcessed' then
    perform chain.require_json_keys(p_args, array['event_index','block_number','block_timestamp','event_signature','abi_encoded_event_data']);
    insert into hub.controller_processed_ledger(
      event_seq, event_index, block_number, block_timestamp, event_signature, abi_encoded_event_data
    ) values (
      p_seq,
      (p_args->>'event_index')::u256,
      (p_args->>'block_number')::bigint,
      (p_args->>'block_timestamp')::bigint,
      (p_args->>'event_signature')::bytes32_hex,
      (p_args->>'abi_encoded_event_data')::bytes_hex
    );

  else
    -- Forward-compatibility: ignore unknown event types.
    null;
  end if;
end $$ ;

-- =========================
-- HUB ROLLBACK (suffix-only)
-- =========================
create or replace function hub.rollback_from (rollback_seq bigint)
returns void language plpgsql as $$
begin
  -- ledgers: delete suffix
  delete from hub.controller_processed_ledger where event_seq >= rollback_seq;
  delete from hub.controller_tip_updates_ledger where event_seq >= rollback_seq;
  delete from hub.tokens_rescued_ledger where event_seq >= rollback_seq;
  delete from hub.lp_vault_events where event_seq >= rollback_seq;

  -- versioned: delete suffix + reopen rows closed by suffix
  delete from hub.protocol_pnl_versions where valid_from_seq >= rollback_seq;
  update hub.protocol_pnl_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.lease_nonce_versions where valid_from_seq >= rollback_seq;
  update hub.lease_nonce_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.claim_versions where valid_from_seq >= rollback_seq;
  update hub.claim_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.payout_config_versions where valid_from_seq >= rollback_seq;
  update hub.payout_config_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.lease_versions where valid_from_seq >= rollback_seq;
  update hub.lease_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.bridger_versions where valid_from_seq >= rollback_seq;
  update hub.bridger_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.swap_rate_versions where valid_from_seq >= rollback_seq;
  update hub.swap_rate_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.chain_versions where valid_from_seq >= rollback_seq;
  update hub.chain_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.lp_balance_versions where valid_from_seq >= rollback_seq;
  update hub.lp_balance_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.lp_allowlist_versions where valid_from_seq >= rollback_seq;
  update hub.lp_allowlist_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.realtor_versions where valid_from_seq >= rollback_seq;
  update hub.realtor_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.protocol_config_versions where valid_from_seq >= rollback_seq;
  update hub.protocol_config_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  delete from hub.ownership_versions where valid_from_seq >= rollback_seq;
  update hub.ownership_versions set valid_to_seq = null where valid_to_seq >= rollback_seq;

  -- cursor rewind
  update chain.stream_cursor
     set applied_through_seq = rollback_seq - 1,
         updated_at = now()
   where stream = 'hub';

  -- recompute cursor tip (genesis if seq=0 else new_tip at applied seq)
  update chain.stream_cursor c
     set tip =
       case when c.applied_through_seq = 0
            then (select genesis_tip from chain.instance where stream='hub')
            else (select e.new_tip from chain.event_appended e
                   where e.stream='hub' and e.canonical and e.event_seq = c.applied_through_seq
                   limit 1)
       end
   where c.stream='hub';
end $$ ;

-- =========================
-- HUB APPLY CATCHUP (contiguous canonical apply)
-- =========================
create or replace function hub.apply_catchup ()
returns void language plpgsql as $$
declare
  cur_seq bigint;
  cur_tip bytes32_hex;
  next_seq bigint;
  ev record;
begin
  -- one projector per transaction
  perform pg_advisory_xact_lock(9101, 1);

  select applied_through_seq, tip
    into cur_seq, cur_tip
    from chain.stream_cursor
   where stream='hub'
   for update;

  loop
    next_seq := cur_seq + 1;

    select *
      into ev
      from chain.event_appended
     where stream='hub' and canonical and event_seq = next_seq
     limit 1;

    exit when not found;

    -- hash-chain link integrity
    if ev.prev_tip <> cur_tip then
      raise exception 'hub tip mismatch at seq %, expected %, got %', next_seq, cur_tip, ev.prev_tip;
    end if;

    perform hub.apply_one(ev.event_seq, ev.event_type, ev.args);

    cur_seq := next_seq;
    cur_tip := ev.new_tip;
  end loop;

  update chain.stream_cursor
     set applied_through_seq = cur_seq,
         tip = cur_tip,
         updated_at = now()
   where stream='hub';
end $$ ;

-- =========================
-- INGEST TRIGGERS (hub-only in this migration)
-- (controller stream is added in 0004 by CREATE OR REPLACE)
-- =========================
create or replace function chain.on_event_appended_insert ()
returns trigger language plpgsql as $$
begin
  if exists (select 1 from new_rows where stream='hub' and canonical) then
    perform hub.apply_catchup();
  end if;
  return null;
end $$ ;

drop trigger if exists trg_event_appended_insert on chain.event_appended ;
create trigger trg_event_appended_insert
after insert on chain.event_appended
referencing new table as new_rows
for each statement execute function chain.on_event_appended_insert () ;

create or replace function chain.on_event_appended_canonical_update ()
returns trigger language plpgsql as $$
declare
  rollback_seq bigint;
begin
  -- only care about hub canonical TRUE -> FALSE flips (suffix rollback)
  select min(o.event_seq)
    into rollback_seq
    from old_rows o
    join new_rows n using (id)
   where o.stream='hub' and o.canonical is true and n.canonical is false;

  if rollback_seq is not null then
    perform hub.rollback_from(rollback_seq);
  end if;

  if exists (
    select 1
      from old_rows o join new_rows n using (id)
     where o.stream='hub' and o.canonical is distinct from n.canonical
  ) then
    perform hub.apply_catchup();
  end if;

  return null;
end $$ ;

drop trigger if exists trg_event_appended_canonical_update
on chain.event_appended ;
create trigger trg_event_appended_canonical_update
after update of canonical on chain.event_appended
referencing old table as old_rows new table as new_rows
for each statement
execute function chain.on_event_appended_canonical_update () ;
