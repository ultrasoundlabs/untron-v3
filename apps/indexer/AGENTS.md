# AGENTS.md guide for AI agents

Whenever you start a task inside apps/indexer, you must read this file and follow it closely. If you find any inconsistency between this file and the code, you must change the AGENTS.md file accordingly to match the code, even if the task prompt doesn't ask you to do anything with the code. If you change anything in apps/indexer, you must read this file and see if anything is no longer up-to-date, and if yes, change the AGENTS.md file accordingly.

## What this app is

- `apps/indexer` is a Ponder project that:
  1. Indexes onchain events into a DB, and
  2. Optionally runs “relayer” logic that sends transactions (currently mostly on Tron) based on what the indexer sees.
- Ponder is still the outer framework (it runs the node process, calls handlers, owns the DB + API server).
- Effect is used inside handlers to structure the code: dependency injection, config parsing, caching, and typed error handling.

---

# 1) Ponder lifecycle: what runs when

## Ponder loads config + schema

- `apps/indexer/ponder.config.ts` is executed at startup and defines:
  - Chains (mainnet, tron) and their RPC endpoints.
  - Block triggers (`mainnet:block`, `tron:block`) at interval 1.
  - Contracts and what events to index (names like `UntronV3`, `UntronController`, etc).
  - A Tron-specific filter for `TRC20:Transfer` that only watches transfers to a computed set of receiver addresses (derived from `PREKNOWN_RECEIVER_SALTS` and controller `CREATE2` parameters).
- `apps/indexer/ponder.schema.ts` defines the database schema (tables/views) Ponder will keep updated.

## Ponder loads the app code that registers handlers

- `apps/indexer/src/index.ts` is the Ponder entrypoint. It registers:
  - The event-chain indexer for 3 contracts (`UntronV3`, `TronLightClient`, `UntronController`).
  - The UntronV3 derived indexer (tracks payout config, swap rates, bridger routes, and claim queue/claim rows for relayer decisions).
  - The relayer handlers.

## Ponder exposes an HTTP API (independent of Effect)

- `apps/indexer/src/api/index.ts` is a small Hono app:
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
- Example used all over: `apps/indexer/src/relayer/deps/tron.ts` defines `TronRelayer` as a Tag with methods like `getReceiverMap`, `sendTronControllerPullFromReceivers`, etc.

## `Layer`

- A Layer is “how to build services and wire dependencies”.
- `Layer.effect(Tag, Effect.gen(...))` means “construct the service by running this Effect”.

## `ManagedRuntime`

- `ManagedRuntime.make(layer)` builds a runtime that knows how to provide all those services.
- You run programs from non-Effect code (Ponder callbacks) via `IndexerRuntime.runPromise(effect)`.

## `Effect.cached`

- `Effect.cached(effect)` returns a memoized version of `effect`, computed once on first use and reused after.
- Used heavily to avoid re-creating clients / re-parsing config / re-computing maps.

---

# 3) The bridge: Ponder callback → Effect program

Ponder expects handler functions that return a `Promise` (or are `async`). This code instead writes handlers as Effects and then runs them via the shared runtime.

- The runtime is defined once in `apps/indexer/src/effect/runtime.ts`:
  - Sets the config provider to env vars.
  - Builds the service graph (`AppConfig`, `TronRelayer`, etc).
  - Exports `IndexerRuntime`.

Then each Ponder handler does:

- Build an `Effect.gen(...)` program
- Run it with `IndexerRuntime.runPromise(...)`

You can see that pattern in:

- `apps/indexer/src/eventChainIndexer.ts` (event handlers)
- `apps/indexer/src/relayer/register.ts` (block + transfer handlers)

This is the key architectural seam: Ponder stays imperative/event-driven; inside each event you get a structured FP program.

---

# 4) Configuration: one place, typed, validated, lazy

## Ponder config env vs Effect config env

There are two env readers:

- Ponder reads `process.env` in `apps/indexer/ponder.config.ts` because it needs chain/contract info before anything else exists.
- The application code reads env through Effect Config inside `apps/indexer/src/effect/config.ts`.

## AppConfig service

`apps/indexer/src/effect/config.ts` defines `AppConfig` as a Tag. It exposes three lazy config groups:

- `relayerRuntime()` → `RelayerRuntimeConfig`
- `tronNetwork()` → `TronNetworkConfig`
- `mainnetRelayer()` → `MainnetRelayerConfig`

Each group is:

- Parsed/validated via `Config.*` combinators,
- Cached via `Effect.cached`,
- Returned as an Effect (so failures are typed as `ConfigError.ConfigError`).

Key design choice: most relayer config is optional until you actually run relayer code.

- Example: `TRON_GRPC_HOST` is optional in the config shape, but `TronGrpc` will `requireSome` it when you call `TronGrpc.get()` (`apps/indexer/src/relayer/deps/tronGrpc.ts`).
- This lets you run “indexer-only” without filling every relayer env var, as long as you don’t execute relayer paths.

Secrets are wrapped in `Redacted` so you don’t accidentally log them.

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

- `apps/indexer/src/eventChain/tip.ts` defines `computeNextEventChainTip`.
- It uses `sha256(encodePacked([...]))` with deterministic fields.

This function is reused by:

- The indexer (when ingesting logs), and
- The Tron relayer (when predicting what a transaction should do to the event chain).

## DB schema for event chains

Defined in `apps/indexer/ponder.schema.ts`:

- `event_chain_state` (`eventChainState`) stores the current tip + last block + sequence for `(chainId, contractName, contractAddress)`.
- `event_chain_event` (`eventChainEvent`) stores each step: tip, previous tip, sequence, and log metadata.

There are also views like `untronV3Event`, `untronControllerState`, etc, that just filter by `contractName`.

## How handlers are registered

- `apps/indexer/src/index.ts` calls `registerEventChainIndexer` three times.

`apps/indexer/src/eventChainIndexer.ts`:

- Takes `{ ponder, contractName, indexName, abi, onchainTipValidation }`
- Introspects the ABI to find all event names (`getAbiEventNames`)
- Registers a Ponder handler for each `${contractName}:${eventName}`

That dynamic registration is why you’ll see some `as any` around Ponder typing in `apps/indexer/src/eventChainIndexer.ts`.

## What happens per event

Inside the Effect program in `apps/indexer/src/eventChainIndexer.ts`:

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

---

# 6) The Relayer: job-queue + optional embedded executor

## Why it’s designed as jobs

Relaying (sending transactions) is conceptually separate from indexing (reading and storing events). Even though the app is currently monolithic, the relayer is implemented as:

- Enqueue jobs based on indexed data
- Process jobs with a claim/lock/retry queue
- Chain-specific services that can be moved into a separate worker later

That shape makes it straightforward to split into indexer vs relayer processes in the future.

## DB schema for relayer state

`apps/indexer/ponder.schema.ts` defines:

- `relayer_status`: per-chain “isLive/head block” bookkeeping.
- `relay_job`: job queue table (status, attempts, lock metadata, retry scheduling).
- `trc20_transfer`: a denormalized record of the filtered TRC20 transfers the relayer cares about.

## How the relayer is registered

`apps/indexer/src/relayer/register.ts` wires Ponder events into the job system:

- `mainnet:block` → enqueue `mainnet_heartbeat` job
- `tron:block` → enqueue `tron_heartbeat` job (+ optionally process TRC20 transfer jobs)
- `TRC20:Transfer` → store transfer row + enqueue `trc20_transfer` job (only if live and synced)

All of this is executed as Effect programs via `IndexerRuntime.runPromise(...)`.

## “Should we act now?” gating

There are two important guards:

### 1) Is this event live-ish?

- `apps/indexer/src/relayer/sync.ts` has `isProbablyLiveEvent`:
  - Uses `relayer_status` head if available.
  - Falls back to asking the chain head via `eth_blockNumber`.
  - Treats an event as probably live if `head - eventBlockNumber <= maxLagBlocks`.

### 2) Is the indexer caught up enough?

- `apps/indexer/src/relayer/sync.ts` has `isSyncedForChain`:
  - For a list of required contracts, checks `event_chain_state.lastEventBlockNumber`
  - Ensures they’re within `maxLagBlocks` of the block we’re acting on.
- This prevents the relayer from acting while the indexer is still backfilling.

`registerRelayer` supplies the required contracts per chain:

- Mainnet heartbeat requires `["UntronV3", "TronLightClient"]` (`apps/indexer/src/relayer/register.ts`)
- Tron heartbeat requires `["UntronController"]` (`apps/indexer/src/relayer/register.ts`)

## Enqueueing jobs

- `apps/indexer/src/relayer/queue.ts` `enqueueRelayJob` inserts into `relay_job` and uses `.onConflictDoNothing()` for dedupe.
- Job IDs are constructed so duplicate triggers won’t create duplicates (e.g. block heartbeats use `${chainId}:${kind}:${blockNumber}`).

## Claim/lock/retry model (core queue mechanics)

`apps/indexer/src/relayer/queue.ts` `claimRelayJobs`:

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

`markRelayJobFailed` in `apps/indexer/src/relayer/queue.ts` implements:

- Attempt counter
- Terminal failure when attempts `>= maxAttempts`
- Otherwise requeue to pending with `nextRetryBlockNumber = head + retryDelayBlocks`

## Processing jobs

`apps/indexer/src/relayer/processor.ts` `processRelayJobs`:

- Claims a batch,
- Builds a `RelayJobHandlerContext` (`apps/indexer/src/relayer/jobs/types.ts`) carrying:
  - `ponderContext` (DB + RPC access),
  - Head block metadata,
  - `dryRun` flag.
- Runs each job via `handleRelayJob` and marks it sent on success.
- On error, catches and marks failed/retry with the error message.

## Job handlers (what relaying actually does today)

- `apps/indexer/src/relayer/jobs/heartbeat/mainnetHeartbeat.ts`:
  - If `dryRun`, do nothing.
  - Runs a list of heartbeat handlers sequentially via `runHeartbeatHandlers` (each wrapped in `Effect.exit` so later handlers still run if one fails).
  - Current handlers:
    - `fill_claims_from_untron_balance`:
      - Implemented in `apps/indexer/src/relayer/jobs/heartbeat/handlers/fillClaimsFromUntronBalance.ts`.
      - Uses `apps/indexer/src/relayer/claimFiller/buildMainnetFillCalls.ts` to:
        - Read derived queue state (`untron_v3_claim_queue`) and per-claim rows (`untron_v3_claim`) from the DB.
        - Read onchain `UntronV3.nextIndexByTargetToken(targetToken)` to determine the pending head per queue.
        - Build a single EIP-4337 UserOperation (via `MainnetRelayer`) that batches:
          - any required pre-calls (e.g. swap executor prefunding), then
          - `UntronV3.fill(targetToken, maxClaims, calls)` for each planned queue.
      - Swap support is designed to be pluggable via the `SwapPlanner` service (`apps/indexer/src/relayer/claimFiller/swapPlanner.ts`); if no providers are configured, non-USDT queues are skipped.
    - `sweep_tron_receivers_if_pending_claims`:
    - Implemented in `apps/indexer/src/relayer/jobs/heartbeat/handlers/sweepTronReceiversIfPendingClaims.ts`.
    - Reads `untron_v3_claim_queue` rows (max observed claim index + 1 per `targetToken`).
    - Compares that `queueLength` to onchain `UntronV3.nextIndexByTargetToken(targetToken)` at latest state.
    - If any queue has pending claims (`queueLength > nextIndex`), runs the same Tron sweep logic as `tron_heartbeat`.
- `apps/indexer/src/relayer/jobs/heartbeat/tronHeartbeat.ts`:
  - If `dryRun`, do nothing.
  - Runs a list of heartbeat handlers sequentially via `runHeartbeatHandlers`.
  - Current handler (`sweep_tron_receivers`): sweeps nonzero USDT balances from known receivers into the controller.
- `apps/indexer/src/relayer/jobs/trc20Transfer.ts`:
  - If `dryRun`, do nothing.
  - Parse and validate payload fields from `job.payloadJson` (no guessing types).
  - Map the receiver address back to a known receiver salt.
  - Read actual token balance on the receiver.
  - Sweep `balance - 1` (leaving 1 unit) via `sendTronControllerPullFromReceivers`.

Handlers are intentionally small: chain-specific mechanics live in services in `relayer/deps/*`.

---

# 7) Chain integrations as Effect services (`relayer/deps/*`)

These files are the ports that the relayer uses. They’re structured as `Effect.Tag` services + Live layers.

## PublicClients: JSON-RPC clients

- `apps/indexer/src/relayer/deps/publicClients.ts`
- Provides `PublicClients.get("mainnet" | "tron")` returning a cached viem `PublicClient`.
- Reads RPC URLs from env via `Config.nonEmptyString`.

## TronGrpc: gRPC client factory

- `apps/indexer/src/relayer/deps/tronGrpc.ts`
- Wraps `@untron/tron-protocol`’s `createTronClients`.
- Reads `TRON_GRPC_HOST` (required when used), optional API key, and insecure flag from `AppConfig.tronNetwork()`.
- Returns cached clients via `TronGrpc.get()`.

## `tronProtocol.ts`: pure helpers

- `apps/indexer/src/relayer/deps/tronProtocol.ts`
- Deterministic utilities:
  - Tron base58 ↔ bytes21 ↔ EVM `0x` address conversion.
  - Tron private key normalization + address derivation.
  - Tron transaction signing (`signTronTransaction`) using secp256k1.

Imported by both:

- `AppConfig` (to validate env vars), and
- `TronRelayer` (to sign + convert).

## TronRelayer: Tron read/write port

- `apps/indexer/src/relayer/deps/tron.ts`
- Encapsulates Tron quirks:
  - Constant calls (readContract-like),
  - Building unsigned transactions via gRPC,
  - Signing + broadcasting,
  - Predicting emitted events to compute an expected event-chain tip.

Key internal pieces:

### 1) Cached config + derived identities

- Pulls Tron network config from `AppConfig` and memoizes it.
- Memoizes:
  - Controller address (bytes21 + EVM hex),
  - Receiver salts,
  - Relayer address (base58 + bytes21).

### 2) `tronReadContract`

- Uses gRPC `triggerConstantContract` (`apps/indexer/src/relayer/deps/tron.ts`)
- Encodes calldata with viem `encodeFunctionData` and decodes with `decodeFunctionResult`.

### 3) Receiver map

- `getReceiverMap` is cached (`apps/indexer/src/relayer/deps/tron.ts`)
- For each `PREKNOWN_RECEIVER_SALTS`, calls controller’s `predictReceiverAddress` and builds a lowercase lookup map.

### 4) Building and sending transactions

- `buildControllerMulticallTx` (`apps/indexer/src/relayer/deps/tron.ts`)
  - Uses gRPC `triggerContract` to get an unsigned tx calling `UntronController.multicall(calls)`.
  - Enforces `RELAYER_TRON_CALL_VALUE === 0` because Solady multicall forbids `msg.value`.
- `broadcastTronTx` (`apps/indexer/src/relayer/deps/tron.ts`)
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

- Reads `preTip = controller.eventChainTip` (`apps/indexer/src/relayer/deps/tron.ts`)
- Runs `planIndexedEventsForCalls(calls)` (`apps/indexer/src/relayer/deps/tron.ts`) to simulate which events will be emitted by the calls, by reading current chain state (balances, deployment status, exchange rates, etc)
- Computes `expectedTip = fold(computeNextEventChainTip, preTip, plannedEvents)` using the tx’s block/time fields
- Appends a final checkpoint call `isEventChainTip(expectedTip)` into the multicall (`apps/indexer/src/relayer/deps/tron.ts`)

That last call acts as an onchain assertion: if anything about the tip prediction is wrong (or the tip changed), the tx fails, and the relayer retries a few times (`apps/indexer/src/relayer/deps/tron.ts`).

Public methods are small wrappers:

- `sendTronControllerPullFromReceivers` (`apps/indexer/src/relayer/deps/tron.ts`)
- `sendTronControllerRebalanceUsdt` (`apps/indexer/src/relayer/deps/tron.ts`)

## MainnetRelayer: account abstraction (EIP-4337)

- `apps/indexer/src/relayer/deps/mainnet.ts`
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
   - `apps/indexer/src/eventChainIndexer.ts` stores it into `event_chain_event` and updates `event_chain_state`.
   - For `UntronV3` events, `apps/indexer/src/index.ts` wires `apps/indexer/src/untronV3DerivedIndexer.ts` in as an `afterEvent` hook to maintain UntronV3-derived state tables like:
     - `untron_v3_lease_payout_config`
     - `untron_v3_swap_rate`
     - `untron_v3_bridger_route`
     - `untron_v3_claim_queue`
     - `untron_v3_claim`
3. For each new Tron block:
   - `apps/indexer/src/relayer/register.ts` updates `relayer_status`.
   - If relayer enabled and indexer synced, it enqueues a `tron_heartbeat` job.
   - If embedded executor enabled, it claims and runs jobs:
     - `tron_heartbeat` (possibly sweeping multiple receivers),
     - then `trc20_transfer` jobs.
4. For each filtered TRC20 transfer into a receiver:
   - `apps/indexer/src/relayer/register.ts` stores a `trc20_transfer` row,
   - Enqueues a `trc20_transfer` job (only if it looks live + synced),
   - Which later triggers a targeted sweep.

Effect’s role in all of this is to make each step:

- Explicit about dependencies (services),
- Explicit about failure modes (typed errors),
- Easier to test/split later (layers/services),
- Less ambient (no scattered `process.env` reads).

---

# 9) A couple of “don’t be surprised” notes

- You’ll still see a few `as any` casts around `context.client.request({ method: "eth_blockNumber" })` in:
  - `apps/indexer/src/eventChainIndexer.ts`
  - `apps/indexer/src/relayer/sync.ts`
  - This works around Ponder/viem typing of raw RPC calls.
- `apps/indexer/ponder.config.ts` does nontrivial work (computing receiver addresses, possibly reading controller bytecode). That’s intentional: it keeps the `TRC20:Transfer` subscription small by filtering in the indexer layer rather than in the relayer.
