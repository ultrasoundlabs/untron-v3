-- =========================
-- STREAM INGEST CURSOR EXPOSURE
-- =========================
/*
Why:
- The relayer's main loop polls `get_now_block2` over Tron gRPC every tick just to know "what
  block are we at?" — pure read traffic against a paid RPC tier.
- The indexer already knows: `chain.ingest_cursor.next_block - 1` is the last block it has
  successfully scanned. With `confirmations=0` (controller stream) this tracks the chain head
  to within ~1 block.
- `api.stream_ingest_summary` already exposes projection state but NOT ingestion progress.
  `max_block_number` only counts blocks that emitted events, so on sparse streams (controller
  produces ~10-20 events/day across ~28k blocks) it can lag thousands of blocks behind head.

This migration extends the view with the ingestion cursor so relayers can replace per-tick
RPC head polls with a postgrest read.
*/

create or replace view api.stream_ingest_summary as
with last_event as (
    select
        stream,
        max(event_seq) as max_event_seq,
        max(block_number) as max_block_number,
        max(block_timestamp) as max_block_timestamp
    from chain.event_appended
    where canonical
    group by stream
)

select
    c.stream,
    c.applied_through_seq,
    c.tip,
    c.updated_at,

    e.max_event_seq,
    e.max_block_number,
    e.max_block_timestamp,
    to_timestamp(e.max_block_timestamp) as max_block_time,

    (
        e.max_event_seq is not null
        and c.applied_through_seq = e.max_event_seq
    ) as is_projection_caught_up,

    -- Ingestion progress. `ingest_next_block` is the next block the worker will scan, so
    -- the highest block confirmed scanned is `ingest_next_block - 1`. Differs from
    -- `max_block_number` which is "highest block that emitted an event" and lags far behind
    -- head on sparse streams.
    --
    -- `ingest_updated_at` is bumped only when the worker advances the cursor (i.e. processes
    -- a non-empty range). On an idle chain it stays old even when the indexer is healthy, so
    -- consumers should treat staleness here as a soft signal, not a liveness check.
    --
    -- Appended at the end of the column list so `create or replace view` succeeds against
    -- the prior shape (postgres refuses column-order changes on replace).
    ic.next_block as ingest_next_block,
    ic.updated_at as ingest_updated_at
from chain.stream_cursor c
left join last_event e using (stream)
left join chain.ingest_cursor ic using (stream);

comment on view api.stream_ingest_summary is
$$Per-stream ingestion/projection summary for relayers.

Now exposes both the projection cursor (`applied_through_seq`) and the ingestion cursor
(`ingest_next_block`). Relayers can use `ingest_next_block - 1` as a substitute for an RPC
head poll on streams configured with `confirmations=0`, eliminating per-tick RPC reads.

`is_projection_caught_up = false` still indicates derived "current state" views may be stale.$$;

-- The indexer's pre-deploy `migrate` is invoked with `--no-notify-pgrst`, so we explicitly
-- ask PostgREST to reload its schema cache here. Without this, the new columns are not
-- visible via the API until PostgREST restarts (relayer would skip ticks during the gap).
notify pgrst, 'reload schema';
