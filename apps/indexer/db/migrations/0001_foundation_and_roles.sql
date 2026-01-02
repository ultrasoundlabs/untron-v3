/*
Foundation:
- Schemas: chain/hub/ctl are internal; api is exposed via PostgREST.
- Domains/types: constrain addresses/hashes/bytes and large integers.
- Helpers: JSON key presence enforcement.

Design choice: store addresses/hashes as TEXT with strict regex constraints.
This keeps data human-readable and PostgREST outputs friendly.
*/

-- =========================
-- SCHEMAS
-- =========================
create schema if not exists chain;
create schema if not exists hub;
create schema if not exists ctl;
create schema if not exists api;

-- =========================
-- DOMAINS (canonical text representations)
-- =========================

-- EVM addresses must be stored in a checksummed form.
-- All other byte types must be stored in all lowercase hex.
create domain evm_address as text
check (value ~ '^0x[0-9a-fA-F]{40}$');

create domain bytes32_hex as text
check (value ~ '^0x[0-9a-f]{64}$');

create domain txhash_hex as text
check (value ~ '^0x[0-9a-f]{64}$');

-- Variable-length bytes as 0x-prefixed even-length hex.
-- (Used for ABI blobs / payloads.)
create domain bytes_hex as text
check (value ~ '^0x([0-9a-f]{2})*$');

-- uint256 fits within 78 decimal digits (2^256 ~ 1.15e77)
create domain u256 as numeric(78, 0)
check (value > = 0);

-- int256 fits in numeric(78,0) comfortably
create domain i256 as numeric(78, 0);

-- =========================
-- STREAM ENUM
-- =========================
create type chain.stream as enum ('hub', 'controller');

-- =========================
-- JSON VALIDATION HELPERS
-- =========================
/*
Projection depends on worker-decoded JSON being correct.
We fail loudly on missing keys to avoid silent state corruption.
*/
create or replace function chain.require_json_keys(
    p_args jsonb,
    p_keys text []
)
returns void language plpgsql as $$
declare
  k text;
begin
  foreach k in array p_keys loop
    if not (p_args ? k) then
      raise exception 'missing required json key: % in %', k, p_args;
    end if;
  end loop;
end $$;

-- Note: PostgREST roles/passwords/grants are intentionally managed out-of-band
-- (e.g. via docker init scripts / IaC), not in migrations.
