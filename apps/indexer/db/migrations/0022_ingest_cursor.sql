-- =========================
-- INGEST CURSOR (ingestion resume state)
-- =========================
/*
Why:
- The event-chain projector (`chain.stream_cursor`) requires contiguous `event_seq`.
- If ingestion ever skips a block range (RPC flake, transient failure) but later ingests newer blocks,
  resuming from `max(block_number)+1` permanently leaves holes.

This table makes ingestion resume explicit and monotonic:
- `next_block` is the *next* block number the ingestion worker should scan (inclusive).
- The worker advances it only after a block-range is successfully processed and committed.
*/

create table if not exists chain.ingest_cursor (
    stream chain.stream primary key,
    next_block bigint not null default 0,
    updated_at timestamptz not null default now(),

    constraint ingest_cursor_instance_fk
    foreign key (stream)
    references chain.instance (stream),

    constraint ingest_cursor_nonnegative
    check (next_block >= 0)
);

comment on table chain.ingest_cursor is
$$Ingestion resume cursor (block-based)

Unlike `chain.stream_cursor` (projection/apply cursor), this table tracks where the ingestion worker
should resume scanning chain logs. It prevents permanent gaps caused by resuming from `max(block_number)+1`.

`next_block=0` means "unset" and the worker should fall back to its configured deployment_block.$$;

comment on column chain.ingest_cursor.next_block is
$$Next block to scan (inclusive) for this stream.

Advanced monotonically by the ingestion worker after successfully committing a processed block range.$$;

-- Ensure configure_instance seeds ingest_cursor too.
create or replace function chain.configure_instance(
    p_stream chain.stream,
    p_chain_id bigint,
    p_contract_address chain_address,
    p_genesis_tip bytes32_hex
) returns void language plpgsql as $$
declare
  cur_applied bigint;
begin
  if p_chain_id <= 0 then
    raise exception 'chain_id must be > 0 (got %)', p_chain_id;
  end if;

  select applied_through_seq
    into cur_applied
    from chain.stream_cursor
   where stream = p_stream
   for update;

  if found and cur_applied <> 0 then
    raise exception 'cannot reconfigure stream %, already applied through seq %', p_stream, cur_applied;
  end if;

  insert into chain.instance(stream, chain_id, contract_address, genesis_tip)
  values (p_stream, p_chain_id, p_contract_address, p_genesis_tip)
  on conflict (stream) do update
    set chain_id = excluded.chain_id,
        contract_address = excluded.contract_address,
        genesis_tip = excluded.genesis_tip;

  insert into chain.stream_cursor(stream, applied_through_seq, tip)
  values (p_stream, 0, p_genesis_tip)
  on conflict (stream) do update
    set applied_through_seq = 0,
        tip = excluded.tip,
        updated_at = now();

  insert into chain.ingest_cursor(stream, next_block)
  values (p_stream, 0)
  on conflict (stream) do update
    set next_block = 0,
        updated_at = now();
end $$;
