# Ponder SQL API (frontend notes)

Ponder exposes a read-only SQL endpoint you can call directly from a frontend.

## Endpoints

- One-shot query: `GET /sql/db?sql=...`
- Live query (SSE): `GET /sql/live?sql=...`

Default dev URL is `http://localhost:42069`.

## Request format (`sql` query param)

The `sql` query param is a **SuperJSON string** of an object shaped like:

```ts
{ sql: string; params?: unknown[] }
```

For plain JSON-safe values (strings/numbers/booleans/null/arrays/objects), you can also send:

```js
const encoded = JSON.stringify({ json: { sql, params } });
```

## Minimal fetch helper

```js
export async function ponderSqlDb(url, sql, params = []) {
  const fullUrl = new URL(url); // e.g. http://localhost:42069/sql/db
  fullUrl.searchParams.set("sql", JSON.stringify({ json: { sql, params } }));
  const res = await fetch(fullUrl.toString());
  if (!res.ok) throw new Error(await res.text());
  return await res.json(); // { rows: [...] }
}
```

Notes:
- Always parameterize user input (`$1`, `$2`, …) and pass values via `params`.
- Response keys are `snake_case`.
- `bigint` columns come back as strings (use `BigInt(x)` if needed).

## Common queries

### Leases (by lease id / lessee / beneficiary / active)

```sql
SELECT *
FROM untron_v3_lease_full
WHERE chain_id = $1
  AND contract_address = $2
  AND is_active = true
  AND (lessee = $3 OR beneficiary = $4)
ORDER BY lease_id DESC
LIMIT 20 OFFSET 0;
```

### Claims (by id / lease id / status)

```sql
SELECT *
FROM untron_v3_claim_full
WHERE chain_id = $1
  AND contract_address = $2
  AND status = $3 -- 'pending' | 'filled'
  AND lease_id = $4
ORDER BY target_token ASC, claim_index DESC
LIMIT 50;
```

### Realtors (by address)

```sql
SELECT *
FROM untron_v3_realtor_full
WHERE chain_id = $1
  AND contract_address = $2
  AND realtor = $3
LIMIT 1;
```

## Views used

- `untron_v3_lease_full` (lease + payout config + related config/limits when present)
- `untron_v3_claim_full` (claim + linked lease + swap/route + status)
- `untron_v3_realtor_full` (realtor + lease stats)

