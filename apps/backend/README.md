# Untron V3 backend (indexer + relayer)

Written in TypeScript using Ponder framework.

## Quickstart

```
pnpm install
cp .env.example .env.local
nano .env.local # configure whatever you need to configure
pnpm run start
```

## Important

This app is the monolithic service you run to operate Untron V3. It includes both the onchain event indexer (Ponder) and the relayer logic.

## Architecture

For introduction into the codebase, read [AGENTS.md](./AGENTS.md). It's written for AI agents but good for humans too.

- Ponder entrypoint: `apps/backend/src/index.ts` registers the event-chain indexer + relayer handlers.
- Effect runtime: `apps/backend/src/effect/runtime.ts` builds a `ManagedRuntime` used to run all Effect programs from Ponder callbacks.
- Env config: `apps/backend/src/effect/config.ts` centralizes env parsing/validation with `ConfigProvider.fromEnv()`.
- Relayer:
  - `apps/backend/src/relayer/register.ts`: Ponder event handlers → enqueue/process jobs.
  - `apps/backend/src/relayer/queue.ts`: job queue primitives (claim/lock/retry).
  - `apps/backend/src/relayer/processor.ts`: job dispatcher + per-job error handling.
  - `apps/backend/src/relayer/deps/*`: Effect services for Tron + mainnet interactions.

### Logging

The indexer uses Effect logging (logfmt). Control verbosity with `LOG_LEVEL` (e.g. `Info`, `Debug`, `Trace`).
