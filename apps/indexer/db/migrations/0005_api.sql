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
    to_timestamp(block_timestamp) as block_time,
    block_hash,
    tx_hash,
    log_index
from chain.event_appended
where canonical;

create or replace view api.controller_tip_proofs as
select
    block_number, block_timestamp, block_hash,
    to_timestamp(block_timestamp) as block_time,
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
-- CONTROLLER: RECEIVER USDT TRANSFERS (TRC-20)
-- =========================

-- Canonical receiver transfers that have not yet been accounted for by hub
-- pre-entitlements.
create or replace view api.unaccounted_receiver_usdt_transfers as
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
    -- best-effort attribution: expected active lease at this Tron timestamp
    (
        select lv.lease_id
        from hub.lease_versions lv
        where
            lv.receiver_salt = t.receiver_salt
            and lv.start_time <= t.block_timestamp
            and lv.nukeable_after > t.block_timestamp
        order by lv.start_time desc
        limit 1
    ) as expected_lease_id
from ctl.receiver_usdt_transfers t
left join hub.claim_versions c
    on
        c.valid_to_seq is null
        and c.origin in (0, 1)
        and c.origin_id = t.tx_hash
where t.canonical and c.lease_id is null;

comment on view api.unaccounted_receiver_usdt_transfers is
$$Unaccounted receiver USDT deposits

Tron TRC-20 Transfer logs not yet reflected as hub claims

Rows in this view correspond to canonical TRC-20 USDT transfers into
deterministic receivers that do NOT yet have a hub-side claim with
`origin in (pre-entitle, subjective pre-entitle)` matching
`origin_id = tx_hash`.

This view is intended for operators to identify deposits that may require
action (preEntitle or receiver pull).$$;

-- Operator-oriented view: include pre-entitle eligibility + suggested next
-- action.
create or replace view api.receiver_usdt_transfer_actionability as
with last_pull as (
    select
        l.receiver_salt,
        l.token,
        max(e.block_timestamp) as last_pull_timestamp
    from ctl.pulled_from_receiver_ledger l
    join chain.event_appended e
        on
            e.stream = 'controller'
            and e.canonical
            and e.event_seq = l.event_seq
    group by l.receiver_salt, l.token
)

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

    c.origin as claim_origin,
    c.lease_id as claim_lease_id,
    c.claim_id,
    c.status as claim_status,
    c.amount_usdt as claim_amount_usdt,

    (
        select lv.lease_id
        from hub.lease_versions lv
        where
            lv.receiver_salt = t.receiver_salt
            and lv.start_time <= t.block_timestamp
            and lv.nukeable_after > t.block_timestamp
        order by lv.start_time desc
        limit 1
    ) as expected_lease_id,

    lp.last_pull_timestamp,
    (
        lp.last_pull_timestamp is null
        or t.block_timestamp > lp.last_pull_timestamp
    ) as preentitle_time_ok,

    case
        when c.lease_id is not null then 'already_accounted'
        when
            (
                lp.last_pull_timestamp is null
                or t.block_timestamp > lp.last_pull_timestamp
            )
            then 'pre_entitle'
        else 'pull'
    end as recommended_action
from ctl.receiver_usdt_transfers t
left join hub.claim_versions c
    on
        c.valid_to_seq is null
        and c.origin in (0, 1)
        and c.origin_id = t.tx_hash
left join last_pull lp
    on
        lp.receiver_salt = t.receiver_salt
        and lp.token = t.token
where t.canonical;

comment on view api.receiver_usdt_transfer_actionability is
$$Receiver USDT deposits + actionability hints (preEntitle vs pull)

For each canonical TRC-20 USDT transfer into a deterministic receiver, this view shows:
- whether the hub has already accounted for it (matching claim origin_id == tx_hash),
- the latest observed receiver pull timestamp for (receiver_salt, token), and
- whether `preEntitle` is still time-eligible (`transfer_ts > last_pull_ts`).

`recommended_action` is a best-effort operator hint:
- 'already_accounted' => hub claim exists (origin pre-entitle or subjective pre-entitle)
- 'pre_entitle'        => no claim yet and pre-entitle timing is still allowed
- 'pull'               => no claim yet and a later pull timestamp suggests pre-entitle would revert$$;

-- =========================
-- POSTGREST GRANTS
-- These are safe to run in all environments; they no-op if roles don't exist.
-- =========================
do $$
begin
  if exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    grant usage on schema api to pgrst_anon;
    grant select on all tables in schema api to pgrst_anon;

    revoke all on schema chain from pgrst_anon;
    revoke all on schema hub from pgrst_anon;
    revoke all on schema ctl from pgrst_anon;

    -- Make future api tables/views readable without extra GRANTs.
    alter default privileges in schema api grant select on tables to pgrst_anon;
  end if;
end $$;

-- =============================================================================
-- POSTGREST / OPENAPI DOC COMMENTS (PUBLIC API VIEWS)
-- =============================================================================
-- PostgREST uses:
-- - COMMENT ON SCHEMA to populate the OpenAPI title/description
-- - COMMENT ON TABLE/VIEW/COLUMN to populate OpenAPI resource descriptions

comment on schema api is
$$Untron V3 Indexer API

Read-only HTTP API served by PostgREST.

This schema contains only views. Each view exposes either:
- canonical raw event data (`api.event_appended`), or
- "current state" derived from events (views over `hub.*` and `ctl.*` versioned tables), or
- append-only ledgers for actions (views over `hub.*_ledger` and `ctl.*_ledger` tables).

The underlying protocol is Untron V3:
- hub (EVM): `UntronV3Index` emits a hash-chained event stream
- controller (Tron): `UntronControllerIndex` emits a hash-chained event stream

All state shown here is derived deterministically from those streams and is reorg-safe.$$;

comment on view api.health is
$$Health check endpoint

Always returns a single row with `status = 'ok'` if PostgREST can query the database.$$;

comment on view api.stream_cursor is
$$Projection cursors (per stream)

Shows how far the database has applied canonical events to derived state tables.

Fields:
- `applied_through_seq`: highest contiguous canonical event_seq applied
- `tip`: expected prev_tip for the next event
- `updated_at`: last time the cursor advanced/rolled back$$;

comment on view api.event_appended is
$$Canonical raw EventAppended stream (both hub + controller)

This is the canonical ordered stream of Untron "semantic events", as emitted by the onchain index contracts.
Each row corresponds to one onchain `EventAppended` log and includes:
- hash-chain linkage (prev_tip/new_tip/event_seq)
- the semantic event name (`event_type`) and decoded arguments (`args`)
- block/tx metadata for auditability

This view filters to `canonical=true` only.$$;

comment on view api.controller_tip_proofs is
$$Controller tip proof logs (IsEventChainTipCalled)

On Tron, the controller exposes `isEventChainTip(bytes32)` which emits `IsEventChainTipCalled`.
This event is NOT appended into the controller's hash chain; it is used as a proof-carrying log to
anchor controller event sequences.

This view filters to `canonical=true` only.$$;

-- Hub "current state" views
comment on view api.hub_ownership is
$$Current hub owner (singleton)

Derived from `OwnershipTransferred` events emitted via `UntronV3Index`.$$;

comment on view api.hub_protocol_config is
$$Current hub protocol configuration (singleton)

Derived from a set of hub config events (USDT, Tron reader, fee floors, rate limits).$$;

comment on view api.hub_realtors is
$$Current realtor allowlist + realtor config (KV)

Realtors are addresses allowed to create leases on the hub (`UntronV3.createLease`).$$;

comment on view api.hub_lp_allowlist is
$$Current LP allowlist (KV)

LPs must be allowlisted to deposit principal into the hub fast-fill vault.$$;

comment on view api.hub_lp_balances is
$$Current derived LP principal balances (KV)

Derived deterministically from hub `LpDeposited`/`LpWithdrawn` events.$$;

comment on view api.hub_chains is
$$Current chain deprecation flags (KV)

Deprecated destination chains cannot be selected in new payout configs.$$;

comment on view api.hub_swap_rates is
$$Current swap rates for settlement tokens (KV)

Rates are used by fillers to swap USDT into the target token during claim settlement.$$;

comment on view api.hub_bridgers is
$$Current bridger routing table (KV)

If a claim targets a different chain than the hub chain, the configured bridger is used to deliver funds.$$;

comment on view api.hub_leases is
$$Current lease registry (KV)

Leases define who controls payouts (lessee), which receiver salt they apply to, and fee schedule parameters.$$;

comment on view api.hub_payout_configs is
$$Current payout configuration per lease (KV)

Defines destination chain, settlement token, and beneficiary for newly created claims under each lease.$$;

comment on view api.hub_claims is
$$Current claim states (KV)

Claims are created by proven Tron deposits / LP-sponsored subjective pre-entitlement / controller profit volume,
and transition to `filled` when a filler settles them.$$;

comment on view api.hub_lease_nonces is
$$Current per-lease nonces (KV)

Used for replay protection on EIP-712 signature-based payout config updates.$$;

comment on view api.hub_protocol_pnl is
$$Current protocol PnL snapshot (singleton)

Tracks fee revenue and rebalance/withdrawal deltas as emitted by the hub contract.$$;

-- Hub ledgers
comment on view api.hub_lp_vault_events is
$$Hub LP vault deposit/withdraw ledger (append-only)$$;

comment on view api.hub_tokens_rescued is
$$Hub token rescue ledger (append-only)$$;

comment on view api.hub_controller_tip_updates is
$$Hub-side controller tip update ledger (append-only)

Records the raw controller event bytes that were hash-linked into the controller tip as seen by the hub.$$;

comment on view api.hub_controller_processed is
$$Hub-side controller processed events ledger (append-only)

Records that the hub processed a queued controller event during reconciliation.$$;

-- Controller "current state" views
comment on view api.controller_owner is
$$Current controller owner (singleton)$$;

comment on view api.controller_executor is
$$Current controller executor (singleton)$$;

comment on view api.controller_usdt is
$$Current controller canonical USDT token (singleton)$$;

comment on view api.controller_lp is
$$Current controller LP address (singleton)$$;

comment on view api.controller_payloads is
$$Current controller rebalancer payloads (KV)

Payloads configure how USDT is bridged out of Tron for each rebalancer implementation.$$;

comment on view api.controller_receivers is
$$Current mapping of receiver salts to deployed receiver addresses (KV)$$;

comment on view api.controller_lp_exchange_rates is
$$Current LP exchange rates for non-USDT tokens (KV)

Used by controller sweeps to compute USDT-equivalent amounts.$$;

-- Controller ledgers
comment on view api.controller_pulled_from_receiver is
$$Controller sweep ledger (append-only)

Records each receiver sweep and the computed USDT-equivalent amount.$$;

comment on view api.controller_usdt_rebalanced is
$$Controller USDT rebalance ledger (append-only)$$;

comment on view api.controller_usdt_transfers is
$$Controller executor USDT transfer ledger (append-only)$$;

comment on view api.controller_lp_tokens_withdrawn is
$$Controller LP token withdrawal ledger (append-only)$$;

-- =============================================================================
-- API VIEW COLUMN COMMENTS (for OpenAPI field descriptions)
-- =============================================================================

-- api.stream_cursor
comment on column api.stream_cursor.stream is
$$Stream name (`hub` or `controller`)$$;
comment on column api.stream_cursor.applied_through_seq is
$$Highest contiguous canonical event sequence already applied to derived
tables$$;
comment on column api.stream_cursor.tip is
$$Expected `prev_tip` for the next event to apply (hash-chain continuity
check)$$;
comment on column api.stream_cursor.updated_at is
$$Timestamp when the cursor last advanced or rolled back$$;

-- api.event_appended
comment on column api.event_appended.stream is
$$Which stream emitted this EventAppended log (`hub` or `controller`)$$;
comment on column api.event_appended.event_seq is
$$Monotonic sequence number in the stream's onchain event hash-chain$$;
comment on column api.event_appended.prev_tip is
$$Previous onchain event hash-chain tip before this event$$;
comment on column api.event_appended.new_tip is
$$New onchain event hash-chain tip after this event$$;
comment on column api.event_appended.event_signature is
$$Keccak256 hash of the semantic event signature string (bytes32 hex)$$;
comment on column api.event_appended.abi_encoded_event_data is
$$Exact ABI-encoded event payload bytes that were hashed onchain (0x hex)$$;
comment on column api.event_appended.event_type is
$$Worker-decoded semantic event name (e.g. `LeaseCreated`, `ClaimCreated`)$$;
comment on column api.event_appended.args is
$$Worker-decoded event arguments as JSON (snake_case keys; values as
strings/hex)$$;
comment on column api.event_appended.block_number is
$$Block number containing the EventAppended log$$;
comment on column api.event_appended.block_timestamp is
$$Block timestamp (seconds since epoch) containing the EventAppended log$$;
comment on column api.event_appended.block_time is
$$Convenience timestamp (block_timestamp converted to timestamptz)$$;
comment on column api.event_appended.block_hash is
$$Block hash of the log's block (bytes32 hex)$$;
comment on column api.event_appended.tx_hash is
$$Transaction hash of the log's transaction (bytes32 hex)$$;
comment on column api.event_appended.log_index is
$$Log index within the transaction receipt (0-based)$$;

-- api.controller_tip_proofs
comment on column api.controller_tip_proofs.block_number is
$$Block number containing the controller `IsEventChainTipCalled` log$$;
comment on column api.controller_tip_proofs.block_timestamp is
$$Block timestamp (seconds since epoch) containing the log$$;
comment on column api.controller_tip_proofs.block_time is
$$Convenience timestamp (block_timestamp converted to timestamptz)$$;
comment on column api.controller_tip_proofs.block_hash is
$$Block hash of the log's block (bytes32 hex)$$;
comment on column api.controller_tip_proofs.tx_hash is
$$Transaction hash of the transaction containing the log (bytes32 hex)$$;
comment on column api.controller_tip_proofs.log_index is
$$Log index within the transaction receipt (0-based)$$;
comment on column api.controller_tip_proofs.caller is
$$Tron address that called `isEventChainTip(bytes32)`$$;
comment on column api.controller_tip_proofs.proved_tip is
$$Hash-chain tip value that the caller asserted as the controller's current
tip$$;

-- api.hub_ownership
comment on column api.hub_ownership.valid_from_seq is
$$Event sequence at which this owner transition became current$$;
comment on column api.hub_ownership.old_owner is
$$Previous hub owner (EVM address)$$;
comment on column api.hub_ownership.new_owner is
$$New hub owner (EVM address)$$;

-- api.hub_protocol_config
comment on column api.hub_protocol_config.valid_from_seq is
$$Event sequence at which this config snapshot became current$$;
comment on column api.hub_protocol_config.usdt is
$$EVM USDT accounting token address on the hub chain$$;
comment on column api.hub_protocol_config.tron_usdt is
$$Tron USDT TRC-20 contract address (base58) accepted by `preEntitle`$$;
comment on column api.hub_protocol_config.tron_reader is
$$Trusted Tron transaction reader address used to verify + decode Tron
transactions$$;
comment on column api.hub_protocol_config.floor_ppm is
$$Protocol-wide minimum percentage fee floor (ppm)$$;
comment on column api.hub_protocol_config.floor_flat_fee is
$$Protocol-wide minimum flat fee floor (USDT units)$$;
comment on column api.hub_protocol_config.max_lease_duration_seconds is
$$Protocol-wide maximum lease duration in seconds (NULL/0 means disabled)$$;
comment on column api.hub_protocol_config.lessee_rate_max_updates is
$$Max payout config updates allowed per window per lessee (NULL/0 means
disabled)$$;
comment on column api.hub_protocol_config.lessee_rate_window_seconds is
$$Window size (seconds) for payout config update rate limiting (NULL/0 means
disabled)$$;

-- api.hub_realtors
comment on column api.hub_realtors.realtor is
$$Realtor address (EVM)$$;
comment on column api.hub_realtors.valid_from_seq is
$$Event sequence at which this realtor snapshot became current$$;
comment on column api.hub_realtors.allowed is
$$Whether this address is currently allowlisted to create leases$$;
comment on column api.hub_realtors.min_fee_ppm is
$$Realtor-specific minimum percentage fee floor (ppm)$$;
comment on column api.hub_realtors.min_flat_fee is
$$Realtor-specific minimum flat fee floor (USDT units)$$;
comment on column api.hub_realtors.max_lease_duration_seconds is
$$Realtor-specific maximum lease duration in seconds (NULL means no override)$$;
comment on column api.hub_realtors.lease_rate_max_leases is
$$Max lease creations allowed per window (NULL/0 means disabled)$$;
comment on column api.hub_realtors.lease_rate_window_seconds is
$$Window size (seconds) for lease creation rate limiting (NULL/0 means
disabled)$$;

-- api.hub_lp_allowlist
comment on column api.hub_lp_allowlist.lp is
$$LP address (EVM)$$;
comment on column api.hub_lp_allowlist.valid_from_seq is
$$Event sequence at which this allowlist entry became current$$;
comment on column api.hub_lp_allowlist.allowed is
$$Whether this LP may deposit into the fast-fill vault (withdrawals are always
allowed)$$;

-- api.hub_lp_balances
comment on column api.hub_lp_balances.lp is
$$LP address (EVM)$$;
comment on column api.hub_lp_balances.valid_from_seq is
$$Event sequence at which this balance snapshot became current$$;
comment on column api.hub_lp_balances.balance is
$$Derived LP principal balance (uint256), based on deposits/withdrawals$$;

-- api.hub_chains
comment on column api.hub_chains.target_chain_id is
$$Destination EVM chainId$$;
comment on column api.hub_chains.valid_from_seq is
$$Event sequence at which this deprecation flag became current$$;
comment on column api.hub_chains.deprecated is
$$Whether this destination chain is deprecated for new payout configs$$;

-- api.hub_swap_rates
comment on column api.hub_swap_rates.target_token is
$$Settlement token (EVM) on the hub chain$$;
comment on column api.hub_swap_rates.valid_from_seq is
$$Event sequence at which this rate became current$$;
comment on column api.hub_swap_rates.rate_ppm is
$$Expected output rate: targetToken units per 1e6 USDT units$$;

-- api.hub_bridgers
comment on column api.hub_bridgers.target_token is
$$Token being bridged (EVM on hub chain)$$;
comment on column api.hub_bridgers.target_chain_id is
$$Destination EVM chainId$$;
comment on column api.hub_bridgers.valid_from_seq is
$$Event sequence at which this bridger route became current$$;
comment on column api.hub_bridgers.bridger is
$$Bridger adapter contract address (EVM)$$;

-- api.hub_leases
comment on column api.hub_leases.lease_id is
$$Global lease id (uint256)$$;
comment on column api.hub_leases.valid_from_seq is
$$Event sequence at which this lease became current$$;
comment on column api.hub_leases.receiver_salt is
$$Receiver salt (bytes32) used to derive deterministic Tron receiver
addresses$$;
comment on column api.hub_leases.lease_number is
$$Per-receiver lease index (0-based) for timeline ordering$$;
comment on column api.hub_leases.realtor is
$$Realtor (EVM) that created this lease$$;
comment on column api.hub_leases.lessee is
$$Lessee (EVM) who controls payout configuration$$;
comment on column api.hub_leases.start_time is
$$Lease start time on the hub chain (seconds)$$;
comment on column api.hub_leases.nukeable_after is
$$Earliest timestamp when the lease can be replaced by a new one for this
receiver_salt$$;
comment on column api.hub_leases.lease_fee_ppm is
$$Percentage fee (ppm) applied to recognized raw volume$$;
comment on column api.hub_leases.flat_fee is
$$Flat fee (USDT units) applied after percentage fee$$;

-- api.hub_payout_configs
comment on column api.hub_payout_configs.lease_id is
$$Lease id this payout config applies to$$;
comment on column api.hub_payout_configs.valid_from_seq is
$$Event sequence at which this payout config became current$$;
comment on column api.hub_payout_configs.target_chain_id is
$$Destination chainId for payouts created under this config$$;
comment on column api.hub_payout_configs.target_token is
$$Settlement token on the hub chain used for fills (USDT or swapped
token)$$;
comment on column api.hub_payout_configs.beneficiary is
$$Recipient (EVM) for payouts / bridged delivery$$;

-- api.hub_claims
comment on column api.hub_claims.lease_id is
$$Lease id that produced this claim$$;
comment on column api.hub_claims.claim_id is
$$Per-lease claim id (uint256, 0-indexed)$$;
comment on column api.hub_claims.valid_from_seq is
$$Event sequence at which this claim version became current$$;
comment on column api.hub_claims.target_token is
$$Token used for settlement when filling this claim (EVM on hub chain)$$;
comment on column api.hub_claims.queue_index is
$$Index in the FIFO queue (per target_token) where this claim was enqueued$$;
comment on column api.hub_claims.amount_usdt is
$$USDT-denominated claim amount (uint256) used for accounting$$;
comment on column api.hub_claims.target_chain_id is
$$Destination chainId for payout (local if equals hub chainId)$$;
comment on column api.hub_claims.beneficiary is
$$Beneficiary address (EVM) receiving payout$$;
comment on column api.hub_claims.origin is
$$Claim origin code (matches `UntronV3Index.ClaimOrigin`)$$;
comment on column api.hub_claims.origin_id is
$$Origin identifier (txId for pre-entitle, receiver_salt for receiver pull,
etc.)$$;
comment on column api.hub_claims.origin_actor is
$$Origin actor (e.g. subjective pre-entitle sponsor; otherwise zero)$$;
comment on column api.hub_claims.origin_token is
$$Origin token/address (Tron token for receiver pull; zero address otherwise)$$;
comment on column api.hub_claims.origin_timestamp is
$$Origin timestamp (seconds) (Tron block time or controller dump time; 0 if
not applicable)$$;
comment on column api.hub_claims.origin_raw_amount is
$$Raw amount before fees (USDT-equivalent units)$$;
comment on column api.hub_claims.status is
$$Claim lifecycle status (`created` or `filled`)$$;

-- api.hub_lease_nonces
comment on column api.hub_lease_nonces.lease_id is
$$Lease id$$;
comment on column api.hub_lease_nonces.valid_from_seq is
$$Event sequence at which this nonce became current$$;
comment on column api.hub_lease_nonces.nonce is
$$Current nonce value used for EIP-712 signature replay protection$$;

-- api.hub_protocol_pnl
comment on column api.hub_protocol_pnl.valid_from_seq is
$$Event sequence at which this PnL snapshot became current$$;
comment on column api.hub_protocol_pnl.pnl is
$$Current protocol PnL value (int256)$$;
comment on column api.hub_protocol_pnl.delta is
$$Delta applied at this event (int256)$$;
comment on column api.hub_protocol_pnl.reason is
$$PnL reason code (matches `UntronV3Index.PnlReason`)$$;

-- Hub ledgers
comment on column api.hub_lp_vault_events.event_seq is
$$Hub event sequence for this vault event$$;
comment on column api.hub_lp_vault_events.kind is
$$`deposit` or `withdraw`$$;
comment on column api.hub_lp_vault_events.lp is
$$LP address (EVM)$$;
comment on column api.hub_lp_vault_events.amount is
$$Amount deposited/withdrawn (uint256, USDT units)$$;

comment on column api.hub_tokens_rescued.event_seq is
$$Hub event sequence for this rescue$$;
comment on column api.hub_tokens_rescued.token is
$$Token rescued (EVM address on hub chain; must not be USDT)$$;
comment on column api.hub_tokens_rescued.amount is
$$Amount rescued (uint256)$$;

comment on column api.hub_controller_tip_updates.event_seq is
$$Hub event sequence for this controller tip update record$$;
comment on column api.hub_controller_tip_updates.previous_tip is
$$Controller tip that this hop links from$$;
comment on column api.hub_controller_tip_updates.block_number is
$$Controller event block number (as embedded in the hub event payload)$$;
comment on column api.hub_controller_tip_updates.block_timestamp is
$$Controller event block timestamp (seconds) (as embedded in the hub event
payload)$$;
comment on column api.hub_controller_tip_updates.event_signature is
$$Controller event signature hash (bytes32 hex)$$;
comment on column api.hub_controller_tip_updates.abi_encoded_event_data is
$$Controller ABI-encoded event payload (0x hex)$$;

comment on column api.hub_controller_processed.event_seq is
$$Hub event sequence for this controller processing record$$;
comment on column api.hub_controller_processed.event_index is
$$Index within the hub's controller event queue that was processed$$;
comment on column api.hub_controller_processed.block_number is
$$Controller event block number (as embedded in the hub event payload)$$;
comment on column api.hub_controller_processed.block_timestamp is
$$Controller event block timestamp (seconds) (as embedded in the hub event
payload)$$;
comment on column api.hub_controller_processed.event_signature is
$$Controller event signature hash (bytes32 hex)$$;
comment on column api.hub_controller_processed.abi_encoded_event_data is
$$Controller ABI-encoded event payload (0x hex)$$;

-- Controller current-state views
comment on column api.controller_owner.valid_from_seq is
$$Controller event sequence at which this owner became current$$;
comment on column api.controller_owner.owner is
$$Controller owner (Tron address)$$;

comment on column api.controller_executor.valid_from_seq is
$$Controller event sequence at which this executor became current$$;
comment on column api.controller_executor.executor is
$$Controller executor (Tron address)$$;

comment on column api.controller_usdt.valid_from_seq is
$$Controller event sequence at which this canonical USDT became current$$;
comment on column api.controller_usdt.usdt is
$$Controller canonical USDT token contract (Tron address)$$;

comment on column api.controller_lp.valid_from_seq is
$$Controller event sequence at which this LP became current$$;
comment on column api.controller_lp.lp is
$$Controller LP address (Tron address)$$;

comment on column api.controller_payloads.rebalancer is
$$Rebalancer address (Tron)$$;
comment on column api.controller_payloads.valid_from_seq is
$$Controller event sequence at which this payload became current$$;
comment on column api.controller_payloads.payload is
$$Rebalancer-specific payload bytes (0x hex) used for delegatecall bridging$$;

comment on column api.controller_receivers.receiver_salt is
$$Receiver salt (bytes32) identifying the deterministic receiver$$;
comment on column api.controller_receivers.valid_from_seq is
$$Controller event sequence at which this receiver mapping became current$$;
comment on column api.controller_receivers.receiver is
$$Receiver contract address on Tron (base58)$$;

comment on column api.controller_lp_exchange_rates.token is
$$Token address on Tron whose exchange rate is configured$$;
comment on column api.controller_lp_exchange_rates.valid_from_seq is
$$Controller event sequence at which this exchange rate became current$$;
comment on column api.controller_lp_exchange_rates.exchange_rate is
$$Scaled exchange rate used to convert token amounts into USDT-equivalent
amounts$$;

-- Controller ledgers
comment on column api.controller_pulled_from_receiver.event_seq is
$$Controller event sequence for this receiver pull$$;
comment on column api.controller_pulled_from_receiver.receiver_salt is
$$Receiver salt identifying which deterministic receiver was swept$$;
comment on column api.controller_pulled_from_receiver.token is
$$Token pulled from receiver (Tron address)$$;
comment on column api.controller_pulled_from_receiver.token_amount is
$$Raw token amount pulled (uint256)$$;
comment on column api.controller_pulled_from_receiver.exchange_rate is
$$Exchange rate used (1e18-scaled); 1:1 for USDT sweeps$$;
comment on column api.controller_pulled_from_receiver.usdt_amount is
$$USDT-equivalent amount accounted for this pull (uint256)$$;

comment on column api.controller_usdt_rebalanced.event_seq is
$$Controller event sequence for this rebalance$$;
comment on column api.controller_usdt_rebalanced.in_amount is
$$USDT amount bridged in (uint256)$$;
comment on column api.controller_usdt_rebalanced.out_amount is
$$Expected USDT amount out on destination (uint256)$$;
comment on column api.controller_usdt_rebalanced.rebalancer is
$$Rebalancer used (Tron address)$$;

comment on column api.controller_usdt_transfers.event_seq is
$$Controller event sequence for this executor transfer$$;
comment on column api.controller_usdt_transfers.recipient is
$$Recipient of USDT from controller (Tron address)$$;
comment on column api.controller_usdt_transfers.amount is
$$Amount transferred (uint256)$$;

comment on column api.controller_lp_tokens_withdrawn.event_seq is
$$Controller event sequence for this LP withdrawal$$;
comment on column api.controller_lp_tokens_withdrawn.token is
$$Token withdrawn by LP (Tron address)$$;
comment on column api.controller_lp_tokens_withdrawn.amount is
$$Amount withdrawn (uint256)$$;
