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
create schema if not exists extensions;

-- =========================
-- EXTENSIONS
-- =========================
-- digest() for checksums
create extension if not exists pgcrypto with schema extensions;

-- =========================
-- DOMAINS (canonical text representations)
-- =========================

-- EVM addresses must be stored in a checksummed form.
-- All other byte types must be stored in all lowercase hex.
create domain public.evm_address as text
check (value ~ '^0x[0-9a-fA-F]{40}$');

-- Tron mainnet base58check addresses (0x41 || addr20).
-- Note: regex validation only (checksum not verified here).
create domain public.tron_address as text
check (value ~ '^T[1-9A-HJ-NP-Za-km-z]{33}$');

-- Multi-chain "address string" for tables that can contain either chain.
create domain public.chain_address as text
check (
    value ~ '^0x[0-9a-fA-F]{40}$'
    or value ~ '^T[1-9A-HJ-NP-Za-km-z]{33}$'
);

create domain public.bytes32_hex as text
check (value ~ '^0x[0-9a-f]{64}$');

create domain public.txhash_hex as text
check (value ~ '^0x[0-9a-f]{64}$');

-- Variable-length bytes as 0x-prefixed even-length hex.
-- (Used for ABI blobs / payloads.)
create domain public.bytes_hex as text
check (value ~ '^0x([0-9a-f]{2})*$');

-- uint256 fits within 78 decimal digits (2^256 ~ 1.15e77)
create domain public.u256 as numeric(78, 0)
check (value >= 0);

-- int256 fits in numeric(78,0) comfortably
create domain public.i256 as numeric(78, 0);

-- =========================
-- TRON ADDRESS HELPERS
-- =========================
/*
Tron "base58check" address encoding:
- payload = 0x41 || addr20  (21 bytes)
- checksum = first 4 bytes of SHA256(SHA256(payload))
- b58 = base58(payload || checksum)

We keep this in the DB mainly for deterministic projection + API output.
If performance becomes an issue, do the conversion in the worker instead.
*/
create or replace function chain.tron_b58check_encode(addr20 bytea)
returns public.tron_address
language plpgsql
set search_path = ''
immutable strict
as $$
declare
    alphabet constant text[] := array[
      '1','2','3','4','5','6','7','8','9',
      'A','B','C','D','E','F','G','H','J','K','L','M','N','P','Q','R','S','T','U','V','W','X','Y','Z',
      'a','b','c','d','e','f','g','h','i','j','k','m','n','o','p','q','r','s','t','u','v','w','x','y','z'
    ];
    payload   bytea;          -- 21-byte (41 || addr20)
    checksum  bytea;          -- 4-byte double-SHA256
    num       numeric := 0;   -- big-int accumulator
    digit     int;
    enc       text   := '';
    i         int;
begin
    if length(addr20) <> 20 then
        raise exception 'Expected 20-byte raw address, got % bytes', length(addr20);
    end if;

    payload  := E'\\x41'::bytea || addr20;
    checksum := substring(extensions.digest(extensions.digest(payload,'sha256'),'sha256') for 4);
    payload  := payload || checksum; -- 25 bytes total

    -- binary -> numeric (base-256)
    for i in 0 .. length(payload)-1 loop
        num := num * 256 + get_byte(payload,i);
    end loop;

    -- numeric -> base-58 string
    while num > 0 loop
        digit := mod(num, 58)::int;
        num   := (num - digit) / 58;
        enc   := alphabet[digit+1] || enc;
    end loop;

    -- leading zero bytes -> leading '1'
    i := 0;
    while i < length(payload) and get_byte(payload,i) = 0 loop
        enc := '1' || enc;
        i   := i + 1;
    end loop;

    return enc::public.tron_address;
end;
$$;

create or replace function chain.evm_address_to_bytes20(p_addr public.evm_address)
returns bytea
language sql
immutable strict
set search_path = ''
as $$
  select decode(substr(p_addr, 3), 'hex')
$$;

create or replace function chain.tron_address_from_evm(p_addr public.evm_address)
returns public.tron_address
language sql
immutable strict
set search_path = ''
as $$
  select chain.tron_b58check_encode(chain.evm_address_to_bytes20(p_addr))
$$;

create or replace function chain.tron_address_from_text(p_addr text)
returns public.tron_address
language plpgsql
immutable strict
set search_path = ''
as $$
begin
  if p_addr ~ '^T[1-9A-HJ-NP-Za-km-z]{33}$' then
    return p_addr::public.tron_address;
  elsif p_addr ~ '^0x[0-9a-fA-F]{40}$' then
    return chain.tron_address_from_evm(p_addr::public.evm_address);
  end if;

  raise exception 'Invalid Tron address text: %', p_addr;
end;
$$;

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
