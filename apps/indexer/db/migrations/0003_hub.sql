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
create type hub.claim_status as enum ('created', 'filled');

-- =========================
-- HUB VERSIONED TABLES (STATE)
-- =========================

-- OwnershipTransferred (singleton)
create table if not exists hub.ownership_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    old_owner evm_address not null,
    new_owner evm_address not null
);
create unique index if not exists hub_ownership_current_unique
on hub.ownership_versions ((1)) where valid_to_seq is null;

-- Protocol config snapshot (singleton)
create table if not exists hub.protocol_config_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,

    usdt evm_address null,
    tron_usdt tron_address null,
    tron_reader tron_address null,

    floor_ppm bigint null,
    floor_flat_fee u256 null,
    max_lease_duration_seconds bigint null,

    lessee_rate_max_updates u256 null,
    lessee_rate_window_seconds u256 null
);
create unique index if not exists hub_protocol_config_current_unique
on hub.protocol_config_versions ((1)) where valid_to_seq is null;

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
);
create unique index if not exists hub_realtor_current_unique
on hub.realtor_versions (realtor) where valid_to_seq is null;

-- LP allowlist (KV)
create table if not exists hub.lp_allowlist_versions (
    lp evm_address not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    allowed boolean not null,
    primary key (lp, valid_from_seq)
);
create unique index if not exists hub_lp_allowlist_current_unique
on hub.lp_allowlist_versions (lp) where valid_to_seq is null;

-- LP balance snapshot derived from deposit/withdraw (KV)
create table if not exists hub.lp_balance_versions (
    lp evm_address not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    balance u256 not null,
    primary key (lp, valid_from_seq)
);
create unique index if not exists hub_lp_balance_current_unique
on hub.lp_balance_versions (lp) where valid_to_seq is null;

-- ChainDeprecatedSet (KV by target_chain_id)
create table if not exists hub.chain_versions (
    target_chain_id bigint not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    deprecated boolean not null,
    primary key (target_chain_id, valid_from_seq)
);
create unique index if not exists hub_chain_current_unique
on hub.chain_versions (target_chain_id) where valid_to_seq is null;

-- SwapRateSet (KV by target_token)
create table if not exists hub.swap_rate_versions (
    target_token evm_address not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    rate_ppm bigint not null,
    primary key (target_token, valid_from_seq)
);
create unique index if not exists hub_swap_rate_current_unique
on hub.swap_rate_versions (target_token) where valid_to_seq is null;

-- BridgerSet (KV by (target_token, target_chain_id))
create table if not exists hub.bridger_versions (
    target_token evm_address not null,
    target_chain_id bigint not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    bridger evm_address not null,
    primary key (target_token, target_chain_id, valid_from_seq)
);
create unique index if not exists hub_bridger_current_unique
on hub.bridger_versions (
    target_token,
    target_chain_id
) where valid_to_seq is null;

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
);
create unique index if not exists hub_lease_current_unique
on hub.lease_versions (lease_id) where valid_to_seq is null;

-- PayoutConfigUpdated (KV by lease_id)
create table if not exists hub.payout_config_versions (
    lease_id u256 not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,

    target_chain_id bigint not null,
    target_token evm_address not null,
    beneficiary evm_address not null,

    primary key (lease_id, valid_from_seq)
);
create unique index if not exists hub_payout_config_current_unique
on hub.payout_config_versions (lease_id) where valid_to_seq is null;

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
    origin_token chain_address not null,
    origin_timestamp bigint not null,
    origin_raw_amount u256 not null,

    status hub.claim_status not null,

    primary key (lease_id, claim_id, valid_from_seq)
);
create unique index if not exists hub_claim_current_unique
on hub.claim_versions (lease_id, claim_id) where valid_to_seq is null;

-- LeaseNonceUpdated (KV by lease_id)
create table if not exists hub.lease_nonce_versions (
    lease_id u256 not null,
    valid_from_seq bigint not null,
    valid_to_seq bigint null,
    nonce u256 not null,
    primary key (lease_id, valid_from_seq)
);
create unique index if not exists hub_lease_nonce_current_unique
on hub.lease_nonce_versions (lease_id) where valid_to_seq is null;

-- ProtocolPnlUpdated (singleton snapshot)
create table if not exists hub.protocol_pnl_versions (
    valid_from_seq bigint primary key,
    valid_to_seq bigint null,
    pnl i256 not null,
    delta i256 not null,
    reason smallint not null
);
create unique index if not exists hub_protocol_pnl_current_unique
on hub.protocol_pnl_versions ((1)) where valid_to_seq is null;

-- =========================
-- VERSION RANGE CHECKS
-- =========================
alter table hub.ownership_versions
add constraint hub_ownership_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.protocol_config_versions
add constraint hub_protocol_config_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.realtor_versions
add constraint hub_realtor_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.lp_allowlist_versions
add constraint hub_lp_allowlist_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.lp_balance_versions
add constraint hub_lp_balance_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.chain_versions
add constraint hub_chain_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.swap_rate_versions
add constraint hub_swap_rate_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.bridger_versions
add constraint hub_bridger_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.lease_versions
add constraint hub_lease_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.payout_config_versions
add constraint hub_payout_config_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.claim_versions
add constraint hub_claim_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.claim_versions
add constraint hub_claim_origin_token_format_check
check (
    (origin = 2 and origin_token ~ '^T[1-9A-HJ-NP-Za-km-z]{33}$')
    or (
        origin <> 2
        and origin_token = '0x0000000000000000000000000000000000000000'
    )
);

alter table hub.lease_nonce_versions
add constraint hub_lease_nonce_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

alter table hub.protocol_pnl_versions
add constraint hub_protocol_pnl_versions_valid_range_check
check (valid_to_seq is null or valid_to_seq > valid_from_seq);

-- =========================
-- HUB LEDGERS (append-only actions)
-- =========================

create table if not exists hub.lp_vault_events (
    event_seq bigint primary key,
    kind text not null check (kind in ('deposit', 'withdraw')),
    lp evm_address not null,
    amount u256 not null
);

create table if not exists hub.tokens_rescued_ledger (
    event_seq bigint primary key,
    token evm_address not null,
    amount u256 not null
);

-- Controller ingestion related
-- (these are hub events carrying controller event bytes)
create table if not exists hub.controller_tip_updates_ledger (
    event_seq bigint primary key,
    previous_tip bytes32_hex not null,
    block_number bigint not null,
    block_timestamp bigint not null,
    event_signature bytes32_hex not null,
    abi_encoded_event_data bytes_hex not null
);

create table if not exists hub.controller_processed_ledger (
    event_seq bigint primary key,
    event_index u256 not null,
    block_number bigint not null,
    block_timestamp bigint not null,
    event_signature bytes32_hex not null,
    abi_encoded_event_data bytes_hex not null
);

-- =========================
-- HUB PATCH HELPERS (versioned updates)
-- =========================

-- Protocol config singleton patch
create or replace function hub.protocol_config_apply(
    p_seq bigint,
    p_usdt evm_address,
    p_tron_usdt tron_address,
    p_tron_reader tron_address,
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
end $$;

-- Realtor KV patch
create or replace function hub.realtor_apply(
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
end $$;

-- LP allowlist set
create or replace function hub.lp_allowlist_set(
    p_seq bigint,
    p_lp evm_address,
    p_allowed boolean
)
returns void language plpgsql as $$
begin
  update hub.lp_allowlist_versions
     set valid_to_seq = p_seq
   where lp = p_lp and valid_to_seq is null;

  insert into hub.lp_allowlist_versions(lp, valid_from_seq, valid_to_seq, allowed)
  values (p_lp, p_seq, null, p_allowed);
end $$;

-- LP balance delta apply (derived state)
create or replace function hub.lp_balance_apply_delta(
    p_seq bigint,
    p_lp evm_address,
    p_delta i256
)
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
end $$;

-- Chain deprecated set
create or replace function hub.chain_set(
    p_seq bigint,
    p_target_chain_id bigint,
    p_deprecated boolean
)
returns void language plpgsql as $$
begin
  update hub.chain_versions
     set valid_to_seq = p_seq
   where target_chain_id = p_target_chain_id and valid_to_seq is null;

  insert into hub.chain_versions(target_chain_id, valid_from_seq, valid_to_seq, deprecated)
  values (p_target_chain_id, p_seq, null, p_deprecated);
end $$;

-- Swap rate set
create or replace function hub.swap_rate_set(
    p_seq bigint,
    p_target_token evm_address,
    p_rate_ppm bigint
)
returns void language plpgsql as $$
begin
  update hub.swap_rate_versions
     set valid_to_seq = p_seq
   where target_token = p_target_token and valid_to_seq is null;

  insert into hub.swap_rate_versions(target_token, valid_from_seq, valid_to_seq, rate_ppm)
  values (p_target_token, p_seq, null, p_rate_ppm);
end $$;

-- Bridger set
create or replace function hub.bridger_set(
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
end $$;

-- Lease create (should be first creation of lease_id in a fork)
create or replace function hub.lease_create(
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
end $$;

-- Payout config set (versioned per lease)
create or replace function hub.payout_config_set(
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
end $$;

-- Claim created
create or replace function hub.claim_create(
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
    p_origin_token chain_address,
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
end $$;

-- Claim filled (versioned status update)
create or replace function hub.claim_fill(
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
end $$;

-- Lease nonce set
create or replace function hub.lease_nonce_set(
    p_seq bigint,
    p_lease_id u256,
    p_nonce u256
)
returns void language plpgsql as $$
begin
  update hub.lease_nonce_versions
     set valid_to_seq = p_seq
   where lease_id = p_lease_id and valid_to_seq is null;

  insert into hub.lease_nonce_versions(lease_id, valid_from_seq, valid_to_seq, nonce)
  values (p_lease_id, p_seq, null, p_nonce);
end $$;

-- Protocol pnl set (singleton snapshot)
create or replace function hub.protocol_pnl_set(
    p_seq bigint,
    p_pnl i256,
    p_delta i256,
    p_reason smallint
)
returns void language plpgsql as $$
begin
  update hub.protocol_pnl_versions
     set valid_to_seq = p_seq
   where valid_to_seq is null;

  insert into hub.protocol_pnl_versions(valid_from_seq, valid_to_seq, pnl, delta, reason)
  values (p_seq, null, p_pnl, p_delta, p_reason);
end $$;

-- =========================
-- HUB APPLY ONE (event interpreter)
-- =========================
create or replace function hub.apply_one(
    p_seq bigint,
    p_type text,
    p_args jsonb
)
returns void language plpgsql as $$
declare
  v_lp evm_address;
  v_amt u256;
  v_origin smallint;
  v_origin_token chain_address;
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
    perform hub.protocol_config_apply(
      p_seq,
      null,
      chain.tron_address_from_text(p_args->>'tron_usdt'),
      null,
      null, null, null, null, null
    );

  elsif p_type = 'TronReaderSet' then
    perform chain.require_json_keys(p_args, array['reader']);
    perform hub.protocol_config_apply(
      p_seq,
      null,
      null,
      chain.tron_address_from_text(p_args->>'reader'),
      null, null, null, null, null
    );

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
    v_origin := (p_args->>'origin')::smallint;
    if v_origin = 2 then
      v_origin_token := (chain.tron_address_from_text(p_args->>'origin_token'))::text::chain_address;
    else
      v_origin_token := ((p_args->>'origin_token')::evm_address)::text::chain_address;
    end if;

    perform hub.claim_create(
      p_seq,
      (p_args->>'lease_id')::u256,
      (p_args->>'claim_id')::u256,
      (p_args->>'target_token')::evm_address,
      (p_args->>'queue_index')::u256,
      (p_args->>'amount_usdt')::u256,
      (p_args->>'target_chain_id')::bigint,
      (p_args->>'beneficiary')::evm_address,
      v_origin,
      (p_args->>'origin_id')::bytes32_hex,
      (p_args->>'origin_actor')::evm_address,
      v_origin_token,
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
end $$;

-- =========================
-- HUB ROLLBACK (suffix-only)
-- =========================
create or replace function hub.rollback_from(rollback_seq bigint)
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
end $$;

-- =========================
-- HUB APPLY CATCHUP (contiguous canonical apply)
-- =========================
create or replace function hub.apply_catchup()
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

  if not found then
    raise exception 'stream cursor not initialized for hub (call chain.configure_instance(''hub'', ...))';
  end if;

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
end $$;

-- =============================================================================
-- POSTGREST / OPENAPI DOC COMMENTS (HUB PROJECTION)
-- =============================================================================

-- Enum types
comment on type hub.claim_status is
$$Hub claim lifecycle status

Claims are created by UntronV3 (EVM hub) when Tron-side deposits or controller pull profit volume
is recognized, and later filled (paid/bridged) by a filler via `UntronV3.fill(...)`.

- `created`: claim exists and is pending settlement
- `filled`: claim has been settled (locally transferred or bridged)$$;

-- Versioned state tables
comment on table hub.ownership_versions is
$$Hub owner history (versioned singleton)

Derived from UntronV3's `OwnershipTransferred` event (emitted through `UntronV3Index`).
There is exactly one "current" row where `valid_to_seq is null`.$$;

comment on column hub.ownership_versions.valid_from_seq is
$$Event sequence (`hub.event_seq`) at which this row became current$$;
comment on column hub.ownership_versions.valid_to_seq is
$$Event sequence at which this row stopped being current (NULL means current)$$;
comment on column hub.ownership_versions.old_owner is
$$Previous owner address (EVM)$$;
comment on column hub.ownership_versions.new_owner is
$$New owner address (EVM)$$;

comment on table hub.protocol_config_versions is
$$Hub protocol configuration snapshot (versioned singleton)

This is a convenience "current config" object derived from multiple hub events:
- `UsdtSet`              → `usdt` (EVM accounting token address on the hub chain)
- `TronUsdtSet`          → `tron_usdt` (Tron USDT contract address)
- `TronReaderSet`        → `tron_reader` (trusted Tron transaction reader contract)
- `ProtocolFloorSet`     → `floor_ppm` (minimum percentage fee floor)
- `ProtocolFlatFeeFloorSet` → `floor_flat_fee` (minimum flat fee floor)
- `ProtocolMaxLeaseDurationSet` → `max_lease_duration_seconds`
- `LesseePayoutConfigRateLimitSet` → payout-config update rate limiting parameters

Values are stored as "last known setting" as of the current canonical event sequence.$$;

comment on column hub.protocol_config_versions.usdt is
$$EVM USDT accounting token address for the hub chain (nullable until
configured)$$;
comment on column hub.protocol_config_versions.tron_usdt is
$$Tron USDT TRC-20 contract address (base58) (nullable until configured)$$;
comment on column hub.protocol_config_versions.tron_reader is
$$Trusted Tron transaction reader address (on the hub chain) used to
verify/parse Tron transactions$$;
comment on column hub.protocol_config_versions.floor_ppm is
$$Protocol-wide minimum percentage fee floor (parts-per-million of raw
volume)$$;
comment on column hub.protocol_config_versions.floor_flat_fee is
$$Protocol-wide minimum flat fee floor (USDT units, uint256)$$;
comment on column hub.protocol_config_versions.max_lease_duration_seconds is
$$Protocol-wide maximum lease duration in seconds (0/NULL means disabled)$$;
comment on column hub.protocol_config_versions.lessee_rate_max_updates is
$$Max number of payout-config updates allowed per window per lessee (NULL means
unset/disabled)$$;
comment on column hub.protocol_config_versions.lessee_rate_window_seconds is
$$Window size (seconds) for payout-config update rate limiting (NULL means
unset/disabled)$$;

comment on table hub.realtor_versions is
$$Realtor allowlist + realtor-specific constraints (versioned KV)

Realtors are the only addresses allowed to create new leases on the hub (`UntronV3.createLease`).
The owner can:
- allowlist or delist a realtor
- configure realtor-specific floors and rate limits

Current realtor state is the row with `valid_to_seq is null` per `realtor`.$$;

comment on column hub.realtor_versions.realtor is
$$Realtor address (EVM)$$;
comment on column hub.realtor_versions.allowed is
$$Whether this address is currently allowlisted to create leases$$;
comment on column hub.realtor_versions.min_fee_ppm is
$$Realtor-specific minimum percentage fee floor (ppm), applied in addition to
protocol floor$$;
comment on column hub.realtor_versions.min_flat_fee is
$$Realtor-specific minimum flat fee floor (USDT units), applied in addition to
protocol floor$$;
comment on column hub.realtor_versions.max_lease_duration_seconds is
$$Realtor-specific maximum lease duration cap in seconds (NULL means no
override)$$;
comment on column hub.realtor_versions.lease_rate_max_leases is
$$Max lease creations allowed per window for this realtor (NULL means
unset/disabled)$$;
comment on column hub.realtor_versions.lease_rate_window_seconds is
$$Window size for realtor lease creation rate limiting (NULL means
unset/disabled)$$;

comment on table hub.lp_allowlist_versions is
$$LP allowlist (versioned KV)

This governs who may deposit principal into the fast-fill vault on the hub (`UntronV3.deposit`).
Delisting does NOT prevent withdrawals; it only blocks new deposits.$$;

comment on column hub.lp_allowlist_versions.lp is
$$LP address (EVM) whose deposit permission is being tracked$$;
comment on column hub.lp_allowlist_versions.allowed is
$$Whether the LP is currently allowlisted to deposit into the hub vault$$;

comment on table hub.lp_balance_versions is
$$LP principal balance snapshot (derived, versioned KV)

This is NOT a direct onchain field; it is derived deterministically from the hub ledger events:
- `LpDeposited` increases balance
- `LpWithdrawn` decreases balance

This represents the "principal accounting" balance tracked by the protocol (0% APY by design).$$;

comment on column hub.lp_balance_versions.balance is
$$Current derived principal balance (uint256) for this LP$$;

comment on table hub.chain_versions is
$$Destination chain deprecation flags (versioned KV)

The hub owner can mark destination chains as deprecated. Lessees cannot set deprecated chains
in payout configs going forward (existing configs may remain).$$;

comment on column hub.chain_versions.target_chain_id is
$$EVM chainId for the payout destination$$;
comment on column hub.chain_versions.deprecated is
$$Whether this chainId is deprecated for new payout configurations$$;

comment on table hub.swap_rate_versions is
$$Swap rate table (versioned KV by target token)

When a claim is settled in a non-USDT target token, the hub expects to swap USDT → target token.
`rate_ppm` is the configured expected output rate used to:
- compute expected output totals for batches
- verify swaps produced at least the expected amount (SwapExecutor minimum output)

Interpretation: `outAmount = amountUsdt * rate_ppm / 1_000_000`.$$;

comment on column hub.swap_rate_versions.target_token is
$$EVM token address that claims will be settled in$$;
comment on column hub.swap_rate_versions.rate_ppm is
$$Expected output rate (target token units per 1e6 USDT units)$$;

comment on table hub.bridger_versions is
$$Bridger routing table (versioned KV by (target_token, target_chain_id))

If a claim's payout destination chain differs from the hub chain, the hub uses a configured
bridger adapter for the `(target_token, target_chain_id)` pair.$$;

comment on column hub.bridger_versions.target_token is
$$Token being bridged (EVM address on the hub chain)$$;
comment on column hub.bridger_versions.target_chain_id is
$$Destination EVM chainId for the bridge$$;
comment on column hub.bridger_versions.bridger is
$$Bridger adapter contract address that implements `IBridger.bridge(...)`$$;

comment on table hub.lease_versions is
$$Lease registry (versioned KV by lease_id)

A lease binds a receiver identity (`receiver_salt`) to:
- a realtor (creator)
- a lessee (controls payout config)
- a fee schedule (ppm + flat)
- a time window (`start_time` .. `nukeable_after`)

Deposits are attributed to the lease active at the Tron timestamp, and claims are created with
the payout config current at claim creation time.$$;

comment on column hub.lease_versions.lease_id is
$$Global lease identifier (Solidity uint256)$$;
comment on column hub.lease_versions.receiver_salt is
$$CREATE2 salt identifying the deterministic Tron receiver address$$;
comment on column hub.lease_versions.lease_number is
$$Per-receiver lease index (0-based) used for timeline ordering$$;
comment on column hub.lease_versions.realtor is
$$Realtor (EVM address) that created the lease$$;
comment on column hub.lease_versions.lessee is
$$Lessee (EVM address) that controls payout configuration$$;
comment on column hub.lease_versions.start_time is
$$Lease start time (seconds) on the hub chain$$;
comment on column hub.lease_versions.nukeable_after is
$$Earliest time when a subsequent lease for the same receiver_salt may be
created$$;
comment on column hub.lease_versions.lease_fee_ppm is
$$Percentage fee (ppm) applied to recognized raw USDT volume$$;
comment on column hub.lease_versions.flat_fee is
$$Flat fee (USDT units) subtracted after percentage fee$$;

comment on table hub.payout_config_versions is
$$Lease payout configuration (versioned KV by lease_id)

This stores the latest payout route for each lease, as set by the lessee:
- `target_chain_id`: destination chain (local transfer if equals hub chainId)
- `target_token`: token used for settlement on the hub chain (USDT or swapped token)
- `beneficiary`: recipient on the destination chain (EVM address)

Note: claims snapshot the payout config at creation time; later payout config updates do not
retroactively change existing claims.$$;

comment on column hub.payout_config_versions.target_chain_id is
$$Destination chainId for payouts created under this config$$;
comment on column hub.payout_config_versions.target_token is
$$Settlement token on the hub chain used for claim fills$$;
comment on column hub.payout_config_versions.beneficiary is
$$Recipient address (EVM) for payouts / bridged deliveries$$;

comment on table hub.claim_versions is
$$Claim registry (versioned KV by (lease_id, claim_id))

Claims are the hub-side representation of money owed to a beneficiary:
- Created by `preEntitle` (proven Tron USDT deposit), `subjectivePreEntitle` (LP-sponsored),
  or by processing controller pull profit volume.
- Filled by `UntronV3.fill(...)` either by transferring locally or bridging.

This table stores a row per claim version. Current state is `valid_to_seq is null`.
The `status` field reflects whether the claim is still pending or has been filled.$$;

comment on column hub.claim_versions.lease_id is
$$Lease that produced this claim$$;
comment on column hub.claim_versions.claim_id is
$$Per-lease claim identifier (0-indexed, uint256)$$;
comment on column hub.claim_versions.target_token is
$$Token used for settlement when filling this claim (EVM address on hub
chain)$$;
comment on column hub.claim_versions.queue_index is
$$Index of this claim in the per-target-token FIFO queue at creation time$$;
comment on column hub.claim_versions.amount_usdt is
$$Claim amount denominated in hub USDT accounting units (uint256)

Even if the claim is settled in another token, this is the USDT-denominated amount used for accounting.$$;
comment on column hub.claim_versions.target_chain_id is
$$Destination chainId; if equals hub chainId, payout is a local transfer, else
bridged$$;
comment on column hub.claim_versions.beneficiary is
$$Beneficiary address (EVM) receiving payout on the destination chain$$;
comment on column hub.claim_versions.origin is
$$Claim origin enum code (matches `UntronV3Index.ClaimOrigin`)

0 = subjective pre-entitle (LP-sponsored)
1 = pre-entitle (proven Tron deposit)
2 = receiver pull profit volume$$;
comment on column hub.claim_versions.origin_id is
$$Origin identifier (meaning depends on origin)

- pre-entitle: Tron txId (sha256(raw_data))
- subjective pre-entitle: anticipated txId
- receiver pull: receiver_salt$$;
comment on column hub.claim_versions.origin_actor is
$$Origin actor address (meaning depends on origin)

Used for subjective pre-entitle sponsor; otherwise zero address.$$;
comment on column hub.claim_versions.origin_token is
$$Origin token/address (meaning depends on origin)

For receiver pull origin, this is the token that was pulled on Tron (Tron address).
For other origins, this is the EVM zero address (enforced by a CHECK constraint).$$;
comment on column hub.claim_versions.origin_timestamp is
$$Origin timestamp (seconds, best-effort metadata)

For pre-entitle: Tron block timestamp of the proved deposit.
For receiver pull: controller dump timestamp.
For subjective pre-entitle: 0.$$;
comment on column hub.claim_versions.origin_raw_amount is
$$Raw amount before lease fees (uint256), denominated in USDT-equivalent
units$$;
comment on column hub.claim_versions.status is
$$Current claim status (`created` or `filled`)$$;

comment on table hub.lease_nonce_versions is
$$Lease nonce snapshots (versioned KV by lease_id)

Tracks the per-lease nonce used for EIP-712 signature-based payout config updates.
Derived from `LeaseNonceUpdated` events.$$;

comment on column hub.lease_nonce_versions.nonce is
$$Current nonce value (uint256) for signature replay protection$$;

comment on table hub.protocol_pnl_versions is
$$Protocol profit-and-loss snapshots (versioned singleton)

Derived from `ProtocolPnlUpdated` hub events. PnL is a signed value representing:
- positive deltas from fees, favorable rebalances, deposits into PnL
- negative deltas from owner withdrawals, unfavorable rebalances, controller executor spends

The hub contract uses this as an accounting abstraction; it is NOT necessarily equal to the contract's USDT balance.$$;

comment on column hub.protocol_pnl_versions.pnl is
$$Current protocol PnL value (int256) after applying `delta`$$;
comment on column hub.protocol_pnl_versions.delta is
$$Change applied at this event (int256)$$;
comment on column hub.protocol_pnl_versions.reason is
$$Reason code (smallint) matching `UntronV3Index.PnlReason`$$;

-- Ledgers
comment on table hub.lp_vault_events is
$$LP vault ledger (append-only)

Stores each `LpDeposited` / `LpWithdrawn` event by event_seq. Used to:
- provide an audit trail
- derive `hub.lp_balance_versions` snapshots deterministically.$$;

comment on table hub.tokens_rescued_ledger is
$$Rescue ledger (append-only)

Records `TokensRescued` actions (non-USDT tokens accidentally sent to hub contract) by event_seq.$$;

comment on table hub.controller_tip_updates_ledger is
$$Controller event-chain tip update ledger (append-only)

These hub events are emitted when the hub relayer submits controller events and hash-links them to a new tip.
The payload stores the raw controller event signature + ABI bytes that were hashed into the controller tip.$$;

comment on table hub.controller_processed_ledger is
$$Controller event processed ledger (append-only)

These hub events are emitted when the hub processes queued controller events (`processControllerEvents`).
They are hub-side bookkeeping to make controller reconciliation observable in indexed data.$$;

-- Projector functions
comment on function hub.apply_one(bigint, text, jsonb) is
$$Hub projector: apply a single semantic hub event to derived tables

This function is called only by `hub.apply_catchup`, which enforces:
- contiguous canonical `event_seq` order
- hash-chain continuity (`prev_tip` equals cursor `tip`)

It interprets `p_type` (e.g. `LeaseCreated`, `ClaimCreated`) and uses the JSON `p_args` produced by the ingestion worker
to update hub projection tables.$$;

comment on function hub.rollback_from(bigint) is
$$Hub projector: rollback derived hub state from an event sequence (inclusive)

Used to handle reorgs. When a previously-canonical `chain.event_appended` row becomes non-canonical,
the trigger computes the earliest affected `event_seq` and calls this function.

Rollback is suffix-only:
- delete projection rows created at/after `rollback_seq`
- re-open previous versions whose `valid_to_seq` was within the rolled-back range
- rewind `chain.stream_cursor` for the `hub` stream.$$;

comment on function hub.apply_catchup() is
$$Hub projector: apply all contiguous canonical hub events not yet applied

This advances `chain.stream_cursor(stream='hub')` by scanning `chain.event_appended` for canonical rows
starting at `applied_through_seq + 1`, verifying `prev_tip` continuity, and applying each event via `hub.apply_one`.

This function is invoked by ingestion triggers when new canonical hub events arrive or when reorgs flip canonical flags.$$;
