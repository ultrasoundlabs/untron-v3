/*
Public read API (PostgREST schema).

We expose only views in api schema, sourced from internal tables.
This lets us refactor internal storage without breaking API consumers.
*/

-- =========================
-- BASIC
-- =========================
create or replace view api.health as
select 'ok'::text as status;

create or replace view api.stream_cursor as
select stream, applied_through_seq, tip, updated_at
from chain.stream_cursor;

-- =========================
-- RAW EVENTS (canonical)
-- =========================
create or replace view api.event_appended as
select
    stream,
    event_seq,
    prev_tip,
    new_tip,
    event_signature,
    abi_encoded_event_data,
    event_type,
    args,
    block_number,
    block_timestamp,
    block_hash,
    tx_hash,
    log_index
from chain.event_appended
where canonical;

create or replace view api.controller_tip_proofs as
select
    block_number, block_timestamp, block_hash,
    tx_hash, log_index,
    caller, proved_tip
from chain.controller_tip_proofs
where canonical;

-- =========================
-- HUB: CURRENT STATE VIEWS
-- =========================
create or replace view api.hub_ownership as
select valid_from_seq, old_owner, new_owner
from hub.ownership_versions
where valid_to_seq is null;

create or replace view api.hub_protocol_config as
select *
from hub.protocol_config_versions
where valid_to_seq is null;

create or replace view api.hub_realtors as
select *
from hub.realtor_versions
where valid_to_seq is null;

create or replace view api.hub_lp_allowlist as
select *
from hub.lp_allowlist_versions
where valid_to_seq is null;

create or replace view api.hub_lp_balances as
select *
from hub.lp_balance_versions
where valid_to_seq is null;

create or replace view api.hub_chains as
select *
from hub.chain_versions
where valid_to_seq is null;

create or replace view api.hub_swap_rates as
select *
from hub.swap_rate_versions
where valid_to_seq is null;

create or replace view api.hub_bridgers as
select *
from hub.bridger_versions
where valid_to_seq is null;

create or replace view api.hub_leases as
select *
from hub.lease_versions
where valid_to_seq is null;

create or replace view api.hub_payout_configs as
select *
from hub.payout_config_versions
where valid_to_seq is null;

create or replace view api.hub_claims as
select *
from hub.claim_versions
where valid_to_seq is null;

create or replace view api.hub_lease_nonces as
select *
from hub.lease_nonce_versions
where valid_to_seq is null;

create or replace view api.hub_protocol_pnl as
select *
from hub.protocol_pnl_versions
where valid_to_seq is null;

-- =========================
-- HUB: LEDGERS
-- =========================
create or replace view api.hub_lp_vault_events as
select * from hub.lp_vault_events;

create or replace view api.hub_tokens_rescued as
select * from hub.tokens_rescued_ledger;

create or replace view api.hub_controller_tip_updates as
select * from hub.controller_tip_updates_ledger;

create or replace view api.hub_controller_processed as
select * from hub.controller_processed_ledger;

-- =========================
-- CONTROLLER: CURRENT STATE VIEWS
-- =========================
create or replace view api.controller_owner as
select * from ctl.owner_versions where valid_to_seq is null;

create or replace view api.controller_executor as
select * from ctl.executor_versions where valid_to_seq is null;

create or replace view api.controller_usdt as
select * from ctl.usdt_versions where valid_to_seq is null;

create or replace view api.controller_lp as
select * from ctl.lp_versions where valid_to_seq is null;

create or replace view api.controller_payloads as
select * from ctl.payload_versions where valid_to_seq is null;

create or replace view api.controller_receivers as
select * from ctl.receiver_versions where valid_to_seq is null;

create or replace view api.controller_lp_exchange_rates as
select * from ctl.lp_exchange_rate_versions where valid_to_seq is null;

-- =========================
-- CONTROLLER: LEDGERS
-- =========================
create or replace view api.controller_pulled_from_receiver as
select * from ctl.pulled_from_receiver_ledger;

create or replace view api.controller_usdt_rebalanced as
select * from ctl.usdt_rebalanced_ledger;

create or replace view api.controller_usdt_transfers as
select * from ctl.controller_usdt_transfer_ledger;

create or replace view api.controller_lp_tokens_withdrawn as
select * from ctl.lp_tokens_withdrawn_ledger;

-- =========================
-- GRANTS
-- =========================
grant usage on schema api to web_anon;
grant select on all tables in schema api to web_anon;
alter default privileges in schema api grant select on tables to web_anon ;
