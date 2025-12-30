# AGENTS.md guide for AI agents

Whenever you start a task inside apps/backend, you must read this file and follow it closely. If you find any inconsistency between this file and the code, you must change the AGENTS.md file accordingly to match the code, even if the task prompt doesn't ask you to do anything with the code. If you change anything in apps/backend, you must read this file and see if anything is no longer up-to-date, and if yes, change the AGENTS.md file accordingly.

## What this app is

- `apps/backend` is a Ponder project that:
  1. Indexes onchain events into a DB, and
  2. Optionally runs “relayer” logic that sends transactions (currently mostly on Tron) based on what the indexer sees.
- Ponder is still the outer framework (it runs the node process, calls handlers, owns the DB + API server).
- Effect is used inside handlers to structure the code: dependency injection, config parsing, caching, and typed error handling.

## Useful commands (what’s actually wired in `package.json`)

- `pnpm dev` → `ponder dev` (hot reloading)
- `pnpm start` → `ponder start` (indexer + API)
- `ponder serve` (not in `package.json` scripts) → production HTTP API without running the indexer
- `pnpm test` → `vitest run`
- `pnpm test:watch` → `vitest` (watch mode)
- `pnpm lint` → ESLint
- `pnpm typecheck` → `tsc`
- `pnpm codegen` → `ponder codegen` (generates `ponder-env.d.ts`)
- `pnpm db` → `ponder db …` (management commands like `list`, `prune`, `create-views`)

Ponder manages the database for you. If you don’t point `DATABASE_URL` at an external Postgres, it will use its default local DB and store state under `apps/backend/.ponder/`.

---

# 1) Ponder lifecycle: what runs when

## Ponder loads config + schema

- `apps/backend/ponder.config.ts` is executed at startup and defines:
  - Chains (mainnet, tron) and their RPC endpoints.
  - Block triggers (`mainnet:block`, `tron:block`) at interval 1.
  - Contracts and what events to index (names like `UntronV3`, `TronLightClient`, `UntronController`, etc).
  - A Tron-specific filter for `TRC20:Transfer` that only watches transfers to a computed set of receiver addresses (derived from `PREKNOWN_RECEIVER_SALTS` and controller `CREATE2` parameters).
    - Note: if `UNTRON_RECEIVER_INIT_CODE_HASH` is not set, `ponder.config.ts` will make a JSON-RPC call at startup to read `UntronController.receiverBytecode()` and hash it.
- `apps/backend/ponder.schema.ts` defines the database schema (tables/views) Ponder will keep updated.

## Ponder loads the app code that registers handlers

- `apps/backend/src/index.ts` is the Ponder entrypoint. It registers:
  - The event-chain indexer for 3 contracts (`UntronV3`, `TronLightClient`, `UntronController`).
  - The UntronV3 derived indexer (tracks payout config, swap rates, bridger routes, and claim queue/claim rows for relayer decisions).
  - The relayer handlers.

## Ponder exposes an HTTP API (independent of Effect)

- `apps/backend/src/api/index.ts` is a small Hono app:
  - `/sql/*` exposes Ponder SQL.
  - `/` and `/graphql` expose a GraphQL API for the schema.

---

# 2) Effect primer (just enough to follow this code)

Effect gives you a way to write side-effecting programs as values with explicit types for:

- Success type (what you get if it works),
- Error type (what it can fail with),
- Environment (what dependencies it needs, like a “TronRelayer service”).

In this repo you’ll mostly see:

## `Effect.Effect<A, E>`

- Think: “a description of a program that eventually produces an `A` or fails with `E`”.
- It doesn’t run until you call something like `runtime.runPromise(effect)`.

## `Effect.gen(function* () { ... })`

- Effect’s do-notation: write sequential logic like async/await, but using `yield*`:
  - `const x = yield* someEffect` is analogous to `const x = await somePromise`.
  - Errors propagate through the typed error channel instead of implicit throws.

## `Effect.Tag` (dependency key + accessors)

- A Tag declares a service interface.
- Effect turns the service’s fields into static accessors on the tag class (so you can call `TronRelayer.getReceiverMap()` without manually threading context).
- Example used all over: `apps/backend/src/relayer/deps/tron/relayer.ts` defines `TronRelayer` as a Tag with methods like `getReceiverMap`, `sendTronControllerPullFromReceivers`, etc.

## `Layer`

- A Layer is “how to build services and wire dependencies”.
- `Layer.effect(Tag, Effect.gen(...))` means “construct the service by running this Effect”.

## `ManagedRuntime`

- `ManagedRuntime.make(layer)` builds a runtime that knows how to provide all those services.
- You run programs from non-Effect code (Ponder callbacks) via `BackendRuntime.runPromise(effect)`.

## `Effect.cached`

- `Effect.cached(effect)` returns a memoized version of `effect`, computed once on first use and reused after.
- Used heavily to avoid re-creating clients / re-parsing config / re-computing maps.

---

# 3) The bridge: Ponder callback → Effect program

Ponder expects handler functions that return a `Promise` (or are `async`). This code instead writes handlers as Effects and then runs them via the shared runtime.

- The runtime is defined once in `apps/backend/src/effect/runtime.ts`:
  - Sets the config provider to env vars.
  - Builds the service graph (`AppConfig`, `TronRelayer`, etc).
  - Exports `BackendRuntime`.

Then each Ponder handler does:

- Build an `Effect.gen(...)` program
- Run it with `BackendRuntime.runPromise(...)`

You can see that pattern in:

- `apps/backend/src/eventChainIndexer.ts` (event handlers)
- `apps/backend/src/relayer/register.ts` (block + transfer handlers)

This is the key architectural seam: Ponder stays imperative/event-driven; inside each event you get a structured FP program.

---

# 4) Configuration: one place, typed, validated, lazy

## Ponder config env vs Effect config env

There are two env readers:

- Ponder reads `process.env` in `apps/backend/ponder.config.ts` because it needs chain/contract info before anything else exists.
- The application code reads env through Effect Config inside `apps/backend/src/effect/config.ts`.

## AppConfig service

`apps/backend/src/effect/config.ts` defines `AppConfig` as a Tag. It exposes three lazy config groups:

- `relayerRuntime()` → `RelayerRuntimeConfig`
- `tronNetwork()` → `TronNetworkConfig`
- `mainnetRelayer()` → `MainnetRelayerConfig`

Each group is:

- Parsed/validated via `Config.*` combinators,
- Cached via `Effect.cached`,
- Returned as an Effect (so failures are typed as `ConfigError.ConfigError`).

Key design choice: most relayer config is optional until you actually run relayer code.

- Example: `TRON_GRPC_HOST` is optional in the config shape, but `TronGrpc` will `requireSome` it when you call `TronGrpc.get()` (`apps/backend/src/relayer/deps/tron/grpcClient.ts`).
- This lets you run “indexer-only” without filling every relayer env var, as long as you don’t execute relayer paths.

Secrets are wrapped in `Redacted` so you don’t accidentally log them.

## Relayer safety toggles (defaults matter)

These defaults are intentionally conservative:

- `RELAYER_ENABLED` defaults to `false` (no jobs enqueued, no tx sent).
- `RELAYER_DRY_RUN` defaults to `true` (jobs may enqueue/process, but handlers return early before sending txs).
- `RELAYER_EMBEDDED_EXECUTOR_ENABLED` defaults to `false` (jobs enqueue into `relay_job`, but won’t be claimed/processed by this same process).

---

# 5) The Event-Chain Indexer: what it stores and why

## What it’s doing conceptually

For each contract, the indexer builds a deterministic hash chain (`eventChainTip`) over that contract’s events:

- Start at a genesis hash,
- For each new event, compute:
  - `nextTip = H(previousTip, blockNumber, blockTimestamp, eventSignature, encodedEventData)`
- Store both the evolving tip and a full per-event record.

This gives you a compact fingerprint of the event history and supports cross-checking against onchain `eventChainTip` in contracts that maintain the same hash.

## Where the hash function lives

- `apps/backend/src/eventChain/tip.ts` defines `computeNextEventChainTip`.
- It uses `sha256(encodePacked([...]))` with deterministic fields.

This function is reused by:

- The indexer (when ingesting logs), and
- The Tron relayer (when predicting what a transaction should do to the event chain).

## DB schema for event chains

Defined in `apps/backend/ponder.schema.ts`:

- `event_chain_state` (`eventChainState`) stores the current tip + last block + sequence for `(chainId, contractName, contractAddress)`.
- `event_chain_event` (`eventChainEvent`) stores each step: tip, previous tip, sequence, and log metadata.

There are also views like `untronV3Event`, `untronControllerState`, etc, that just filter by `contractName`.

## How handlers are registered

- `apps/backend/src/index.ts` calls `registerEventChainIndexer` three times.

`apps/backend/src/eventChainIndexer.ts`:

- Takes `{ ponder, contractName, indexName, abi, onchainTipValidation }`
- Introspects the ABI to find all event names (`getAbiEventNames`)
- Registers a Ponder handler for each `${contractName}:${eventName}`

That dynamic registration is why you’ll see some `as any` around Ponder typing in `apps/backend/src/eventChainIndexer.ts`.

## What happens per event

Inside the Effect program in `apps/backend/src/eventChainIndexer.ts`:

1. Identify the chain + contract address and compute a deterministic `stateId`.
2. Load `eventChainState` row.
3. If missing, initialize it:
   - Compute genesis tip (`computeEventChainGenesis`)
   - Optionally read `eventChainTip` from chain at a specific block to seed/validate (`onchainTipValidation === "blockTag"`).
4. Optionally validate that the DB tip matches onchain tip at block boundaries (`"blockTag"`).
5. Encode the event arguments deterministically:
   - Find the event in the ABI (`getAbiEvent`)
   - Extract args from Ponder’s `event.args` (handles both array + object shapes)
   - `encodeAbiParameters(inputs, values)`
6. Compute `nextTip` using `computeNextEventChainTip`.
7. Insert `event_chain_event` row and update `event_chain_state`.
8. If `onchainTipValidation === "head"`, and this event is at the current head block, read the onchain `eventChainTip` and compare.

If a tip mismatch is detected, the handler fails the Effect on purpose: this is treated as a correctness invariant.

### Tron JSON-RPC limitation (important)

Some Tron JSON-RPC endpoints reject `eth_call` when the 2nd parameter is a QUANTITY block number (they only accept the TAG `"latest"`). This means:

- `onchainTipValidation: "blockTag"` (historical block reads) is not supported on Tron with those endpoints.
- `onchainTipValidation: "head"` is supported; the code forces an explicit `"latest"` call for Tron to avoid Ponder/viem emitting QUANTITY block parameters.

---

# 6) The Relayer: job-queue + optional embedded executor

## Why it’s designed as jobs

Relaying (sending transactions) is conceptually separate from indexing (reading and storing events). Even though the app is currently monolithic, the relayer is implemented as:

- Enqueue jobs based on indexed data
- Process jobs with a claim/lock/retry queue
- Chain-specific services that isolate IO and make behavior testable

That shape keeps relayer logic explicit and robust while still running in-process as part of the backend.

## DB schema for relayer state

`apps/backend/ponder.schema.ts` defines:

- `relayer_status`: per-chain “isLive/head block” bookkeeping.
- `relay_job`: job queue table (status, attempts, lock metadata, retry scheduling).
- `trc20_transfer`: a denormalized record of the filtered TRC20 transfers the relayer cares about.

## How the relayer is registered

`apps/backend/src/relayer/register.ts` wires Ponder events into the job system:

- `mainnet:block` → enqueue `mainnet_heartbeat` job
- `tron:block` → enqueue `tron_heartbeat` job (+ optionally process TRC20 transfer jobs)
- `TRC20:Transfer` → store transfer row + enqueue `trc20_transfer` job (only if relayer is enabled, live-ish, and synced)

All of this is executed as Effect programs via `BackendRuntime.runPromise(...)`.

## “Should we act now?” gating

There are two important guards:

### 1) Is this event live-ish?

- `apps/backend/src/relayer/register.ts` has `isProbablyLiveEvent`:
  - Uses `relayer_status` head if available.
  - Falls back to asking the chain head via `eth_blockNumber`.
  - Treats an event as probably live if `head - eventBlockNumber <= maxLagBlocks`.

### 2) Is the indexer caught up enough?

Currently, the relayer does **not** enforce an additional “is the indexer caught up to head?” guard beyond the “live-ish” check above. If you reintroduce a backfill safety gate, it should be based on `event_chain_state.lastEventBlockNumber` for the relevant contracts.

## Enqueueing jobs

- `apps/backend/src/relayer/queue.ts` `enqueueRelayJob` inserts into `relay_job` and uses `.onConflictDoNothing()` for dedupe.
- Job IDs are constructed so duplicate triggers won’t create duplicates (e.g. block heartbeats use `${chainId}:${kind}:${blockNumber}`).

## Claim/lock/retry model (core queue mechanics)

`apps/backend/src/relayer/queue.ts` `claimRelayJobs`:

- Computes `eligibleBlock = head - minConfirmations`.
- In SQL:
  - Selects candidate jobs with:
    - `status = 'pending'`
    - `createdAtBlockNumber <= eligibleBlock`
    - Retry is due (`nextRetryBlockNumber` is null or `<= head`)
  - `FOR UPDATE SKIP LOCKED` so multiple workers can safely race
  - Updates them to:
    - `status = 'processing'`
    - Sets lock fields (`lockedAt…`, `lockedBy`)
    - Clears `lastError`
  - Returns the claimed rows.

`markRelayJobFailed` in `apps/backend/src/relayer/queue.ts` implements:

- Attempt counter
- Terminal failure when attempts `>= maxAttempts`
- Otherwise requeue to pending with `nextRetryBlockNumber = head + retryDelayBlocks`

## Processing jobs

`apps/backend/src/relayer/processor.ts` `processRelayJobs`:

- Claims a batch,
- Builds a `RelayJobHandlerContext` (`apps/backend/src/relayer/jobs/types.ts`) carrying:
  - `ponderContext` (DB + RPC access),
  - Head block metadata,
  - `dryRun` flag.
- Runs each job via `handleRelayJob` and marks it sent on success.
- On error, catches and marks failed/retry with the error message.

## Job handlers (what relaying actually does today)

- `apps/backend/src/relayer/jobs/heartbeat/mainnetHeartbeat.ts`:
  - If `dryRun`, do nothing.
  - Runs a list of heartbeat handlers sequentially via `runHeartbeatHandlers` (`apps/backend/src/relayer/jobs/heartbeat/runHeartbeatHandlers.ts`) (each wrapped in `Effect.exit` so later handlers still run if one fails).
  - Current handlers:
    - `fill_claims_from_untron_balance`:
      - Uses `apps/backend/src/relayer/claimFiller/buildMainnetFillCalls.ts` to:
        - Read derived queue state (`untron_v3_claim_queue`) and per-claim rows (`untron_v3_claim`) from the DB.
        - Read onchain `UntronV3.nextIndexByTargetToken(targetToken)` to determine the pending head per queue.
        - Build a single EIP-4337 UserOperation (via `MainnetRelayer`) that batches:
          - any required pre-calls (e.g. swap executor prefunding), then
          - `UntronV3.fill(targetToken, maxClaims, calls)` for each planned queue.
      - Swap support is designed to be pluggable via the `SwapPlanner` service (`apps/backend/src/relayer/claimFiller/swapPlanner.ts`); if no providers are configured, non-USDT queues are skipped.
    - `sweep_tron_receivers_if_pending_claims`:
      - Reads `untron_v3_claim_queue` rows (max observed claim index + 1 per `targetToken`).
      - Compares that `queueLength` to onchain `UntronV3.nextIndexByTargetToken(targetToken)` at latest state.
      - If any queue has pending claims (`queueLength > nextIndex`), sweeps Tron **USDT** from known receivers into the controller.
        - Rationale: sweeping USDT is significantly more expensive than sweeping TRX, so it’s only done when there’s evidence it’s needed (pending claims).
- `apps/backend/src/relayer/jobs/heartbeat/tronHeartbeat.ts`:
  - If `dryRun`, do nothing.
  - Runs a list of heartbeat handlers sequentially via `runHeartbeatHandlers` (`apps/backend/src/relayer/jobs/heartbeat/runHeartbeatHandlers.ts`).
  - Current handlers:
    - `sweep_tron_receivers_trx`: sweeps TRX (native token; modeled as `0x000…000`) from known receivers into the controller.
      - It uses the controller’s USDT position to budget how many receivers to sweep in a single tx.
    - `rebalance_pulled_usdt`: if configured, calls `UntronController.rebalanceUsdt` for `pulledUsdt - 1` when `pulledUsdt` is above a threshold.
    - `ensure_is_event_chain_tip_called`: calls `UntronController.isEventChainTip(...)` on Tron when the onchain controller tip matches the indexed tip but no recent call is observed.
    - `publish_tron_light_client`: consumes demand-driven publish requests and calls `TronLightClient.proveBlocks(...)` on mainnet to store txTrieRoots for exact Tron blocks needed by relaying jobs (`apps/backend/src/relayer/tronLightClientPublisher/publish.ts`).
- `apps/backend/src/relayer/jobs/trc20Transfer.ts`:
  - If `dryRun`, do nothing.
  - Parse and validate payload fields from `job.payloadJson` (no guessing types).
  - Map the receiver address back to a known receiver salt.
  - For Tron USDT (as defined by `UntronV3.tronUsdt()` on mainnet):
    - Prove the Tron tx (inclusion proof against `TronLightClient`), ensure the block is published, and call `UntronV3.preEntitle(...)` via `MainnetRelayer`.
  - For all other tokens:
    - Read actual token balance on the receiver and sweep `balance - 1` (leaving 1 unit) via `sendTronControllerPullFromReceivers`.
- `apps/backend/src/relayer/jobs/relayControllerEventChain.ts`:
  - If `dryRun`, do nothing.
  - For each indexed `IsEventChainTipCalled` on Tron, prove the calling transaction (direct call or multicall including `isEventChainTip`).
  - Ensure the Tron block is published in `TronLightClient` (demand-driven publish request + tron heartbeat publisher).
  - Call `UntronV3.relayControllerEventChain(...)` via `MainnetRelayer` to enqueue controller events on mainnet.

Handlers are intentionally small: chain-specific mechanics live in services in `relayer/deps/*`.

---

# 7) Chain integrations as Effect services (`relayer/deps/*`)

These files are the ports that the relayer uses. They’re structured as `Effect.Tag` services + Live layers.

## PublicClients: JSON-RPC clients

- `apps/backend/src/relayer/deps/publicClients.ts`
- Provides `PublicClients.get("mainnet" | "tron")` returning a cached viem `PublicClient`.
- Reads RPC URLs from env via `Config.nonEmptyString`.

## TronGrpc: gRPC client factory

- `apps/backend/src/relayer/deps/tron/grpcClient.ts`
- Wraps `@untron/tron-protocol`’s `createTronClients`.
- Reads `TRON_GRPC_HOST` (required when used), optional API key, and insecure flag from `AppConfig.tronNetwork()`.
- Returns cached clients via `TronGrpc.get()`.

This module also contains low-level gRPC utilities:

- `makeGrpcUnary` (Promise wrapper around unary calls with `callOpts.metadata`)
- helpers to detect NotFound/Unimplemented errors by gRPC `code`

## `protocol.ts`: pure helpers

- `apps/backend/src/relayer/deps/tron/protocol.ts`
- Deterministic utilities:
  - Tron base58 ↔ bytes21 ↔ EVM `0x` address conversion.
  - Tron private key normalization + address derivation.
  - Tron transaction signing (`signTronTransaction`) using secp256k1.

Imported by both:

- `AppConfig` (to validate env vars), and
- `TronRelayer` (to sign + convert).

## Tron planning (pure, unit-testable)

- `apps/backend/src/relayer/tron/controllerMulticallPlanner.ts`
- Computes which controller events should be indexed for a given `UntronController.multicall(...)` payload (used to predict/validate `eventChainTip` updates).

## TronRelayer: Tron read/write port

- `apps/backend/src/relayer/deps/tron/relayer.ts`
- Encapsulates Tron quirks:
  - Constant calls (readContract-like) via `apps/backend/src/relayer/deps/tron/contractCalls.ts`,
  - Building unsigned transactions + sending controller ops via `apps/backend/src/relayer/deps/tron/untronController.ts`,
  - Signing + broadcasting via `apps/backend/src/relayer/deps/tron/transactions.ts`,
  - Deterministic address/key utilities via `apps/backend/src/relayer/deps/tron/protocol.ts`.

Key internal pieces:

### 1) Cached config + derived identities

- Pulls Tron network config from `AppConfig` and memoizes it.
- Memoizes:
  - Controller address (bytes21 + EVM hex),
  - Receiver salts,
  - Relayer address (base58 + bytes21).

### 2) `tronReadContract`

- Uses gRPC `triggerConstantContract` (`apps/backend/src/relayer/deps/tron/contractCalls.ts`)
- Encodes calldata with viem `encodeFunctionData` and decodes with `decodeFunctionResult`.

### 3) Receiver map

- `getReceiverMap` is cached (`apps/backend/src/relayer/deps/tron/relayer.ts`)
- For each `PREKNOWN_RECEIVER_SALTS`, calls controller’s `predictReceiverAddress` and builds a lowercase lookup map.

### 4) Building and sending transactions

- `buildControllerMulticallTx` (`apps/backend/src/relayer/deps/tron/untronController.ts`)
  - Uses gRPC `triggerContract` to get an unsigned tx calling `UntronController.multicall(calls)`.
  - Enforces `RELAYER_TRON_CALL_VALUE === 0` because Solady multicall forbids `msg.value`.
- `broadcastTronTx` (`apps/backend/src/relayer/deps/tron/transactions.ts`)
  - Sets `feeLimit`,
  - Signs with `signTronTransaction`,
  - Broadcasts,
  - Polls for receipt (`waitForTronTransaction`).

### 5) Predicting emitted events → predicting `eventChainTip`

- The `UntronController` contract maintains an onchain `eventChainTip`.
- The indexer computes the same tip offchain from events (`eventChainIndexer` + `computeNextEventChainTip`).
- When the relayer sends a transaction, it wants strong assurance that:
  - It’s not racing another relayer changing the tip, and
  - It correctly predicted which events will be emitted (and thus what the tip should become).

To do that, `TronRelayer`:

- Reads `preTip = controller.eventChainTip` (`apps/backend/src/relayer/deps/tron/untronController.ts`)
- Runs the multicall event planner (`apps/backend/src/relayer/tron/controllerMulticallPlanner.ts`) to simulate which events will be emitted by the calls, by reading current chain state (balances, deployment status, exchange rates, etc)
- Computes `expectedTip = fold(computeNextEventChainTip, preTip, plannedEvents)` using the tx’s block/time fields
- Previously appended a final checkpoint call `isEventChainTip(expectedTip)` into controller multicalls, but this is currently disabled (hotpatch) because the computed tip can mismatch on Tron.

That last call acts as an onchain assertion: if anything about the tip prediction is wrong (or the tip changed), the tx fails, and the relayer retries a few times (`apps/backend/src/relayer/deps/tron/untronController.ts`).

Public methods are small wrappers:

- `sendTronControllerPullFromReceivers` (`apps/backend/src/relayer/deps/tron/untronController.ts`)
- `sendTronControllerRebalanceUsdt` (`apps/backend/src/relayer/deps/tron/untronController.ts`)

## MainnetRelayer: account abstraction (EIP-4337)

- `apps/backend/src/relayer/deps/mainnet/index.ts` (plus helpers in `apps/backend/src/relayer/deps/mainnet/*`)
- Wraps `permissionless` to send EIP-4337 UserOperations via bundlers:
  - Builds a Safe smart account (cached)
  - Sends UOs to one or more bundlers
  - Polls for inclusion by scanning EntryPoint `UserOperationEvent` logs
  - Tries bundlers sequentially and aggregates errors

Used by mainnet heartbeat claim-filling (`fill_claims_from_untron_balance`) to batch `UntronV3.fill(...)` calls into a single UserOperation.

---

# 8) How everything ties together (follow the data)

If you start the app with `ponder start`:

1. Ponder indexes events for the configured contracts.
2. For each event on `UntronV3` / `TronLightClient` / `UntronController`:
   - `apps/backend/src/eventChainIndexer.ts` stores it into `event_chain_event` and updates `event_chain_state`.
   - For `UntronV3` events, `apps/backend/src/index.ts` wires `apps/backend/src/untronV3DerivedIndexer.ts` in as an `afterEvent` hook to maintain UntronV3-derived state tables like:
     - `untron_v3_lease_payout_config`
     - `untron_v3_swap_rate`
     - `untron_v3_bridger_route`
     - `untron_v3_claim_queue`
     - `untron_v3_claim`
   - For `TronLightClient` events, `apps/backend/src/index.ts` handles a small `afterEvent` hook inline to maintain:
     - `tron_light_client_checkpoint` (observability: which Tron blocks have stored txTrieRoots on mainnet)
   - For `UntronController:IsEventChainTipCalled`, `apps/backend/src/index.ts` handles a small `afterEvent` hook inline to:
     - store the call into `untron_controller_is_event_chain_tip_called`,
     - insert a `tron_light_client_publish_request` for that Tron block, and
     - enqueue a `relay_controller_event_chain` job.
3. For each new Tron block:
   - `apps/backend/src/relayer/register.ts` updates `relayer_status`.
   - If relayer enabled and indexer synced, it enqueues a `tron_heartbeat` job.
   - If embedded executor enabled, it claims and runs jobs:
     - `tron_heartbeat` (possibly sweeping multiple receivers),
     - then `trc20_transfer` and `relay_controller_event_chain` jobs.
4. For each filtered TRC20 transfer into a receiver:
  - `apps/backend/src/relayer/register.ts` stores a `trc20_transfer` row,
  - Inserts a `tron_light_client_publish_request` row for the transfer’s Tron block number,
  - Enqueues a `trc20_transfer` job (only if it looks live + synced),
  - Which later either pre-entitles the deposit on mainnet (Tron USDT only) or triggers a targeted sweep (all other tokens).

Effect’s role in all of this is to make each step:

- Explicit about dependencies (services),
- Explicit about failure modes (typed errors),
- Easier to test and evolve (layers/services),
- Less ambient (no scattered `process.env` reads).

---

# 9) A couple of “don’t be surprised” notes

- You’ll still see a few `as any` casts around `context.client.request({ method: "eth_blockNumber" })` in:
  - `apps/backend/src/eventChainIndexer.ts`
  - `apps/backend/src/relayer/register.ts`
  - This works around Ponder/viem typing of raw RPC calls.
- `apps/backend/ponder.config.ts` does nontrivial work (computing receiver addresses, possibly reading controller bytecode). That’s intentional: it keeps the `TRC20:Transfer` subscription small by filtering in the indexer layer rather than in the relayer.
