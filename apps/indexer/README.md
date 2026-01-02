# Rust polling indexer (HTTP-only)

This indexer is a small Rust/Tokio + Alloy worker that:

- Polls `eth_getLogs` for `EventAppended` (and on controller, `IsEventChainTipCalled`) in block ranges
- Enriches logs with block timestamp
- Bulk upserts into Postgres (`chain.event_appended`, `chain.controller_tip_proofs`)
- Detects reorgs by block-hash mismatch and invalidates via `canonical=false`

## Run

1) Migrate DB:

`pnpm --filter @untron/v3-indexer db:migrate`

2) Start the indexer:

`pnpm --filter @untron/v3-indexer dev`

## Env

Common:

- `DATABASE_URL` (required)
- `RUST_LOG` (optional, defaults to `info`)

Hub stream (EVM):

- `HUB_RPC_URLS` (required; comma/space separated)
- `HUB_CHAIN_ID` (required)
- `HUB_CONTRACT_ADDRESS` (required; `0x…`)
- `HUB_DEPLOYMENT_BLOCK` (required)
- `HUB_CONFIRMATIONS` (default `12`)
- `HUB_POLL_INTERVAL_SECS` (default `5`)
- `HUB_CHUNK_BLOCKS` (default `5000`)
- `HUB_REORG_SCAN_DEPTH` (default `512`)

Controller stream (Tron JSON-RPC):

- `CONTROLLER_RPC_URLS` (required; comma/space separated)
- `CONTROLLER_CHAIN_ID` (required)
- `CONTROLLER_CONTRACT_ADDRESS` (required; `T…` base58 or `0x…`)
- `CONTROLLER_DEPLOYMENT_BLOCK` (required)
- `CONTROLLER_CONFIRMATIONS` (default `20`)
- `CONTROLLER_POLL_INTERVAL_SECS` (default `5`)
- `CONTROLLER_CHUNK_BLOCKS` (default `1000`)
- `CONTROLLER_REORG_SCAN_DEPTH` (default `1024`)

RPC retry/backoff (applies to all streams):

- `RPC_MAX_RATE_LIMIT_RETRIES` (default `8`)
- `RPC_INITIAL_BACKOFF_MS` (default `250`)
- `RPC_COMPUTE_UNITS_PER_SECOND` (default `500`)

Block timestamp enrichment:

- `BLOCK_HEADER_CONCURRENCY` (default `16`)
- `BLOCK_TIMESTAMP_CACHE_SIZE` (default `2048`)

DB pool:

- `DB_MAX_CONNECTIONS` (default `5`)

## CLI

- `cargo run -p indexer -- --stream hub|controller|all`

