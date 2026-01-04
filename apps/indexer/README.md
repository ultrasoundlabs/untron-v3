# Rust polling indexer (HTTP-only)

This indexer is a small Rust/Tokio + Alloy worker that:

- Polls `eth_getLogs` for `EventAppended` (and on controller, `IsEventChainTipCalled`) in block ranges
- Enriches logs with block timestamp
- Bulk upserts into Postgres (`chain.event_appended`, `chain.controller_tip_proofs`)
- Detects reorgs by block-hash mismatch and invalidates via `canonical=false`
- Optionally indexes TRC-20 USDT `Transfer` logs into deterministic receiver addresses on the controller chain

## Run

1) Migrate DB:

`pnpm --filter @untron/v3-indexer db:migrate`

2) Start the indexer:

`pnpm --filter @untron/v3-indexer dev`

## Run via Docker Compose

`infra/docker-compose.yml` includes:

- `db_migrate`: runs `apps/indexer` SQLx migrations against the Compose Postgres
- `indexer` (profile `indexer`): runs the polling worker

1) Bring up infra services:

`docker compose -f infra/docker-compose.yml up -d`

2) Start the indexer (requires RPC URLs in `infra/indexer.env`):

`docker compose -f infra/docker-compose.yml --profile indexer up -d indexer`

## Env

Common:

- `DATABASE_URL` (required)
- `RUST_LOG` (optional, defaults to `info`)
- `INDEXER_PROGRESS_INTERVAL_SECS` (default `5`; INFO progress summary per stream)
- `OTEL_TRACES_SAMPLE_RATIO` (default `0.01`; set `1` for full tracing)
- `OTEL_DISABLED` (optional; set `1` to disable OTLP exports)

Hub stream (EVM):

- `HUB_RPC_URLS` (required; comma/space separated)
- `HUB_CHAIN_ID` (required)
- `HUB_CONTRACT_ADDRESS` (required; `0x…`)
- `HUB_DEPLOYMENT_BLOCK` (required)
- `HUB_CONFIRMATIONS` (default `0`)
- `HUB_POLL_INTERVAL_SECS` (default `1`)
- `HUB_CHUNK_BLOCKS` (default `2000`)
- `HUB_REORG_SCAN_DEPTH` (default `128`)

Controller stream (Tron JSON-RPC):

- `CONTROLLER_RPC_URLS` (required; comma/space separated)
- `CONTROLLER_CHAIN_ID` (required)
- `CONTROLLER_CONTRACT_ADDRESS` (required; `T…` base58 or `0x…`)
- `CONTROLLER_DEPLOYMENT_BLOCK` (required)
- `CONTROLLER_CONFIRMATIONS` (default `0`)
- `CONTROLLER_POLL_INTERVAL_SECS` (default `1`)
- `CONTROLLER_CHUNK_BLOCKS` (default `2000`)
- `CONTROLLER_REORG_SCAN_DEPTH` (default `256`)

RPC retry/backoff (applies to all streams):

- `RPC_MAX_RATE_LIMIT_RETRIES` (default `8`)
- `RPC_INITIAL_BACKOFF_MS` (default `250`)
- `RPC_COMPUTE_UNITS_PER_SECOND` (default `500`)

Block timestamp enrichment:

- `BLOCK_HEADER_CONCURRENCY` (default `16`)
- `BLOCK_TIMESTAMP_CACHE_SIZE` (default `2048`)

DB pool:

- `DB_MAX_CONNECTIONS` (default `5`)

Receiver USDT transfer indexing (controller chain):

- `TRC20_ENABLED` (default `true`)
- `PREKNOWN_RECEIVER_SALTS` (optional; comma/space separated `0x…` bytes32 salts)
- `UNTRON_CONTROLLER_CREATE2_PREFIX` (default `0x41`; set `0xff` for EVM-only test chains)
- `TRC20_POLL_INTERVAL_SECS` (default `2`)
- `TRC20_CHUNK_BLOCKS` (default `2000`)
- `TRC20_TO_BATCH_SIZE` (default `50`)
- `TRC20_BACKFILL_CONCURRENCY` (default `2`)
- `TRC20_DISCOVERY_INTERVAL_SECS` (default `30`)

## Stream selection

- `INDEXER_STREAM` (optional: `hub` | `controller` | `all`; default: `all`)

## Useful logging presets

- Minimal (default): `RUST_LOG=info`
- See per-range/tick internals: `RUST_LOG=indexer=debug`
- Include SQLx query logs (very noisy): `RUST_LOG=indexer=debug,sqlx=trace`
