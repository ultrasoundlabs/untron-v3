/*
Ingestion layer written by the worker.

Core invariants:
-- chain.event_appended stores the canonical total order via
--   EventAppended(event_seq).
- reorgs are represented by flipping canonical=false (never deleting rows).
- projection tables are derived deterministically from canonical rows.

Also introduces chain.instance to enforce:
- one DB indexes exactly one deployment instance per stream
- the DB rejects "wrong chain / wrong contract" ingestion bugs
*/

-- =========================
-- INSTANCE CONFIG (one deployment per stream)
-- =========================
create table if not exists chain.instance (
    stream chain.stream primary key,

    -- numeric chain_id
    chain_id bigint not null,

    -- deployed index contract address for that stream
    contract_address evm_address not null,

    -- EventChainGenesis constant for that contract
    genesis_tip bytes32_hex not null
);

-- Fill these before production (placeholders here).
insert into chain.instance (stream, chain_id, contract_address, genesis_tip)
values
(
    'hub',
    0,
    '0x0000000000000000000000000000000000000000',
    '0x0000000000000000000000000000000000000000000000000000000000000000'
),
(
    'controller',
    0,
    '0x0000000000000000000000000000000000000000',
    '0x0000000000000000000000000000000000000000000000000000000000000000'
)
on conflict (stream) do nothing ;

-- =========================
-- CANONICAL EVENT STREAM: EventAppended logs
-- =========================
create table if not exists chain.event_appended (
id bigserial primary key,

-- "hub" or "controller"
stream chain.stream not null,

-- must match chain.instance for the given stream
chain_id bigint not null,
contract_address evm_address not null,

block_number bigint not null,
block_timestamp bigint not null,
block_hash bytes32_hex not null,

tx_hash txhash_hex not null,
log_index integer not null,

-- canonicality for reorg handling:
-- worker flips canonical=false on removed fork logs
canonical boolean not null default true,

-- hash-chain payload from EventAppended
event_seq bigint not null,
prev_tip bytes32_hex not null,
new_tip bytes32_hex not null,
event_signature bytes32_hex not null,

-- EXACT ABI bytes blob used in the onchain hash chain (hex string)
abi_encoded_event_data bytes_hex not null,

-- worker-decoded "semantic event"
event_type text not null,   -- e.g. 'LeaseCreated'
-- snake_case keys, values as strings/hex
args jsonb not null default '{}'::jsonb,

inserted_at timestamptz not null default now ()
) ;

-- idempotency: chain log identity
create unique index if not exists event_appended_uid
on chain.event_appended (chain_id, tx_hash, log_index) ;

-- at most one canonical event at a given seq per stream
create unique index if not exists event_appended_canonical_seq
on chain.event_appended (stream, event_seq)
where canonical ;

-- projector scan index
create index if not exists event_appended_scan
on chain.event_appended (stream, canonical, event_seq) ;

-- =========================
-- ENFORCE SINGLE INSTANCE ON INGEST
-- =========================
create or replace function chain.enforce_instance_on_event_appended ()
returns trigger language plpgsql as $$
declare
  inst record;
begin
  select * into inst from chain.instance where stream = new.stream;
  if not found then
    raise exception 'no chain.instance row for stream=%', new.stream;
  end if;

  if new.chain_id <> inst.chain_id then
    raise exception 'wrong chain_id for stream %, expected %, got %',
      new.stream, inst.chain_id, new.chain_id;
  end if;

  if new.contract_address <> inst.contract_address then
    raise exception 'wrong contract_address for stream %, expected %, got %',
      new.stream, inst.contract_address, new.contract_address;
  end if;

  return new;
end $$ ;

drop trigger if exists trg_enforce_instance_event_appended
on chain.event_appended ;
create trigger trg_enforce_instance_event_appended
before insert on chain.event_appended
for each row execute function chain.enforce_instance_on_event_appended () ;

-- =========================
-- CONTROLLER NON-CHAINED EVENT: IsEventChainTipCalled
-- =========================
/*
This event is intentionally NOT appended to the controller hash chain.
We store it separately as a "proof-carrying" log.
*/
create table if not exists chain.controller_tip_proofs (
id bigserial primary key,

chain_id bigint not null,
contract_address evm_address not null,

block_number bigint not null,
block_timestamp bigint not null,
block_hash bytes32_hex not null,

tx_hash txhash_hex not null,
log_index integer not null,

canonical boolean not null default true,

caller evm_address not null,
proved_tip bytes32_hex not null,

inserted_at timestamptz not null default now ()
) ;

create unique index if not exists controller_tip_proofs_uid
on chain.controller_tip_proofs (chain_id, tx_hash, log_index) ;

create or replace function chain.enforce_instance_on_tip_proofs ()
returns trigger language plpgsql as $$
declare
  inst record;
begin
  select * into inst from chain.instance where stream = 'controller';
  if new.chain_id <> inst.chain_id or new.contract_address <> inst.contract_address then
    raise exception 'controller tip proof does not match configured controller instance';
  end if;
  return new;
end $$ ;

drop trigger if exists trg_enforce_instance_tip_proofs
on chain.controller_tip_proofs ;
create trigger trg_enforce_instance_tip_proofs
before insert on chain.controller_tip_proofs
for each row execute function chain.enforce_instance_on_tip_proofs () ;

-- =========================
-- STREAM CURSORS (projection state machine)
-- =========================
/*
applied_through_seq:
- highest canonical event_seq already applied to projections

tip:
- expected prev_tip for the next event (event_seq = applied_through_seq + 1)

Initialize tip to genesis_tip for each stream.
*/
create table if not exists chain.stream_cursor (
stream chain.stream primary key,
applied_through_seq bigint not null default 0,
tip bytes32_hex not null,
updated_at timestamptz not null default now ()
) ;

insert into chain.stream_cursor (stream, applied_through_seq, tip)
select stream, 0, genesis_tip
from chain.instance
on conflict (stream) do nothing ;
