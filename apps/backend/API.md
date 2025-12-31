# Untron V3 backend: HTTP API

Base URL (dev): `http://localhost:42069`

This service exposes:

- REST endpoints (custom, UntronV3-specific).
- Ponder SQL API under `/sql/*`.
- Ponder GraphQL API under `/graphql` (only).

## Conventions

- EVM addresses are returned as EIP-55 checksums (`viem.getAddress`).
- Tron addresses are returned as Base58Check strings (usually starting with `T`).
- `uint256`/`uint64`/`int` values are encoded as **decimal strings** in JSON requests/responses.
- Errors return JSON:
  - `{ "ok": false, "error": { "message": string, "details": any | null } }`

Note: some endpoints return “raw DB rows” (snake_case). Those rows may contain additional address fields beyond the ones documented here; the API tries to checksum/convert key address fields but does not guarantee full normalization for every column.

## REST (UntronV3)

Implementation: `apps/backend/src/api/untronV3.ts`.

### `GET /protocol`

Returns protocol-wide configuration and route tables used by the frontend to decide what is creatable/updatable.

Response (200):

```json
{
  "ok": true,
  "hub": {
    "chainId": 137,
    "contractAddress": "0x…",
    "protocol": { "...": "untron_v3_protocol_full (snake_case), tron_usdt is Base58" },
    "deprecatedChains": [{ "...": "untron_v3_chain_deprecated rows" }],
    "swapRates": [{ "...": "untron_v3_swap_rate rows (target_token checksummed)" }],
    "bridgerRoutes": [{ "...": "untron_v3_bridger_route rows (addresses checksummed)" }]
  },
  "controller": {
    "chainId": 728126428,
    "address": "T…",
    "state": { "...": "untron_controller_state row" },
    "latestIsEventChainTipCalled": { "...": "untron_controller_is_event_chain_tip_called row (caller is Base58)" }
  }
}
```

Notes:
- No top-level aliases are provided; read everything from `hub` and `controller`.
- Rows returned inside `hub.*` and `controller.*` omit DB-only `id` as well as redundant `chain_id` and `contract_address` fields.

### Toy client

There is a runnable toy script that:

1) generates a fresh EOA lessee,
2) calls `POST /leases`,
3) signs an EIP-712 `PayoutConfigUpdate` as the lessee,
4) calls `PUT /leases/:leaseId`.

Run (with `pnpm dev` running):

```
pnpm -C apps/backend rest:lease:toy
```

Optional env var:
- `UNTRON_API_URL` (default `http://localhost:42069`)
- `TOY_OMIT_RECEIVER_SALT` (set to `1`/`true` to omit `receiverSalt` and let the backend auto-pick from `PREKNOWN_RECEIVER_SALTS`)

### `GET /realtors`

Returns the backend’s **relayer address** (Safe smart account address) and the indexed realtor row for that address (if present).

Response (200):

```json
{
  "ok": true,
  "chainId": 137,
  "contractAddress": "0x…",
  "relayerAddress": "0x…",
  "realtor": { "...": "snake_case DB columns" } 
}
```

Notes:
- If the backend relayer isn’t configured, this returns 501 with an error (because `MainnetRelayer.getAddress()` fails).
- `realtor` is either a row from `untron_v3_realtor_full` (snake_case keys) or `null`.
- `untron_v3_realtor_full` includes `protocol_floor_ppm` and `effective_min_fee_ppm` (effective minimum is `max(protocol_floor_ppm, min_fee_ppm)`).

### `GET /realtors/:address`

Looks up an address in the derived DB.

Response (200):

```json
{
  "ok": true,
  "chainId": 137,
  "contractAddress": "0x…",
  "realtor": "0x…",
  "result": { "...": "snake_case DB columns" }
}
```

`result` is either the row (snake_case keys) or `null`.

### `POST /leases`

Relays `UntronV3.createLease(...)` using the backend’s Safe/4337 relayer.

Body:

```json
{
  "receiverSalt": "0x<32 bytes>",
  "lessee": "0x…",
  "nukeableAfter": "9999999999",
  "leaseFeePpm": "100",
  "flatFee": "0",
  "targetChainId": "137",
  "targetToken": "0x…",
  "beneficiary": "0x…"
}
```

Notes:
- `receiverSalt` is optional. If omitted/empty, the backend picks the first “free” salt from `PREKNOWN_RECEIVER_SALTS` (first salt that has no lease yet, or whose latest lease is already nukeable).
- Auto-picking uses the backend’s indexed DB state; if the indexer is behind the chain, it may temporarily fail with “not yet nukeable” until the latest `LeaseCreated` is indexed.

Behavior:
- Checks the backend relayer address is a realtor onchain (`isRealtor(relayerAddress)`), returning HTTP 403 if not.
- Simulates the call first (fails fast on reverts).
- Sends a UserOperation via bundlers.
- Waits for inclusion and returns the bundler tx hash.
- Attempts to parse `LeaseCreated` logs from the included tx to return `leaseId`.

Response (200):

```json
{
  "ok": true,
  "chainId": 137,
  "contractAddress": "0x…",
  "receiverSalt": "0x…",
  "leaseId": "1",
  "userOperation": {
    "bundlerUrl": "…",
    "userOpHash": "0x…",
    "transactionHash": "0x…",
    "blockNumber": "…",
    "success": true
  }
}
```

Notes:
- This requires the backend relayer address to be a **realtor** onchain (`createLease` checks `isRealtor[msg.sender]`).
- `leaseId` may be `null` if event parsing fails (tx still succeeded).

### `PUT /leases/:leaseId`

Relays `UntronV3.setPayoutConfigWithSig(...)` using the backend’s Safe/4337 relayer.

Path params:
- `leaseId`: decimal string

Body:

```json
{
  "targetChainId": "137",
  "targetToken": "0x…",
  "beneficiary": "0x…",
  "deadline": "1735689600",
  "signature": "0x…"
}
```

Behavior:
- Simulates the call first.
- Sends a UserOperation via bundlers.
- Waits for inclusion and returns the bundler tx hash.
- Attempts to parse `PayoutConfigUpdated` to confirm the update.

Response (200):

```json
{
  "ok": true,
  "chainId": 137,
  "contractAddress": "0x…",
  "leaseId": "1",
  "updated": true,
  "userOperation": {
    "bundlerUrl": "…",
    "userOpHash": "0x…",
    "transactionHash": "0x…",
    "blockNumber": "…",
    "success": true
  }
}
```

Notes:
- Any caller can relay `setPayoutConfigWithSig`; authorization comes from the **lessee signature**.

## SQL schema (derived views used for reads)

These are Postgres views (see `apps/backend/ponder.schema.ts`) designed for efficient querying:

- `untron_v3_lease_full`: lease + payout config + derived limits/config when present (`is_active`, `is_nukeable_yet` computed).
- `untron_v3_claim_full`: claim + status (`pending`/`filled`) + linked lease fields + swap/bridger info when present.
- `untron_v3_realtor_full`: realtor config + aggregated lease stats.

Query them via `/sql/db` (see `apps/backend/SQL.md`).
