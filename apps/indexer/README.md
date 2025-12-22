# Reference implementation of an indexer for Untron V3 protocol

Written in TypeScript using Ponder framework.

## Quickstart

```
pnpm install
cp .env.example .env.local
nano .env.local # configure whatever you need to configure
pnpm run build
pnpm run start
```

## Important

Currently the indexer also implements relayer-role logic needed for operation of Untron V3. Thus, this app is monolithic and can run everything you need to operate Untron V3 protocol on your own in a single service. This, however, reduces the flexibility of the system (e.g. you can't just run the indexer to read but not write protocol data). We expect to split indexer and relayer into separate, independent modules in the near future.

## Architecture

- Ponder entrypoint: `apps/indexer/src/index.ts` registers the event-chain indexer + relayer handlers.
- Effect runtime: `apps/indexer/src/effect/runtime.ts` builds a `ManagedRuntime` used to run all Effect programs from Ponder callbacks.
- Env config: `apps/indexer/src/effect/config.ts` centralizes env parsing/validation with `ConfigProvider.fromEnv()`.
- Relayer:
  - `apps/indexer/src/relayer/register.ts`: Ponder event handlers → enqueue/process jobs.
  - `apps/indexer/src/relayer/queue.ts`: job queue primitives (claim/lock/retry).
  - `apps/indexer/src/relayer/processor.ts`: job dispatcher + per-job error handling.
  - `apps/indexer/src/relayer/deps/*`: Effect services for Tron + mainnet interactions.
