/*
Foundation:
- Schemas: chain/hub/ctl are internal; api is exposed via PostgREST.
- Domains/types: constrain addresses/hashes/bytes and large integers.
- Helpers: JSON key presence enforcement.
- PostgREST roles: authenticator (LOGIN) + web_anon (NOLOGIN).

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

-- =========================
-- POSTGREST ROLES
-- =========================
/*
PostgREST connects as authenticator, then SET ROLE web_anon.
Only api schema should be exposed to web_anon.
*/
do $$
declare
  pw text := current_setting('app.authenticator_password', true);
begin
  if not exists (select 1 from pg_roles where rolname = 'authenticator') then
    create role authenticator login;
  end if;

  -- Passwords are environment-specific; set out-of-band or via
  -- `SET app.authenticator_password = '...'` before running migrations.
  if pw is not null then
    execute format('alter role authenticator password %L', pw);
  end if;

  if not exists (select 1 from pg_roles where rolname = 'web_anon') then
    create role web_anon nologin;
  end if;
end $$;

grant web_anon to authenticator;

grant usage on schema api to web_anon;
revoke all on schema chain from web_anon;
revoke all on schema hub from web_anon;
revoke all on schema ctl from web_anon;

-- Make future api tables/views readable without extra GRANTs
alter default privileges in schema api grant select on tables to web_anon;
