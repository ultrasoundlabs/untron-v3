#!/usr/bin/env bash
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_EXPORTER_PASSWORD="${POSTGRES_EXPORTER_PASSWORD:-}"
UI_READONLY_PASSWORD="${UI_READONLY_PASSWORD:-}"
PGRST_AUTH_PASSWORD="${PGRST_AUTH_PASSWORD:-}"

if [ -z "$POSTGRES_EXPORTER_PASSWORD" ]; then
  echo "POSTGRES_EXPORTER_PASSWORD must be set" >&2
  exit 1
fi

if [ -z "$UI_READONLY_PASSWORD" ]; then
  echo "UI_READONLY_PASSWORD must be set" >&2
  exit 1
fi
if [ -z "$PGRST_AUTH_PASSWORD" ]; then
  echo "PGRST_AUTH_PASSWORD must be set" >&2
  exit 1
fi

psql -v ON_ERROR_STOP=1 \
  --username "$POSTGRES_USER" \
  --dbname "untron" \
  -v postgres_exporter_password="$POSTGRES_EXPORTER_PASSWORD" \
  -v ui_readonly_password="$UI_READONLY_PASSWORD" \
  -v pgrst_auth_password="$PGRST_AUTH_PASSWORD" <<'EOSQL' || exit 1

-- 1) Schemas
create schema if not exists api;

-- 2) Roles
do $$
declare
  pgrst_pw text := :'pgrst_auth_password';
  ui_pw    text := :'ui_readonly_password';
begin
  if pgrst_pw is null or length(pgrst_pw) = 0 then
    raise exception 'PGRST_AUTH_PASSWORD is empty';
  end if;
  if ui_pw is null or length(ui_pw) = 0 then
    raise exception 'UI_READONLY_PASSWORD is empty';
  end if;

  -- PostgREST roles
  if not exists (select 1 from pg_roles where rolname = 'pgrst_authenticator') then
    create role pgrst_authenticator login noinherit;
  end if;

  if not exists (select 1 from pg_roles where rolname = 'pgrst_anon') then
    create role pgrst_anon nologin;
  end if;

  execute format('alter role pgrst_authenticator password %L', pgrst_pw);

  -- Read-only browsing roles
  if not exists (select 1 from pg_roles where rolname = 'db_readonly') then
    create role db_readonly nologin;
  end if;

  if not exists (select 1 from pg_roles where rolname = 'ui_readonly') then
    create role ui_readonly login noinherit;
  end if;

  execute format('alter role ui_readonly password %L', ui_pw);

  grant pgrst_anon to pgrst_authenticator;
  grant db_readonly to ui_readonly;

  -- Optional safety: read-only transactions for UI login
  execute 'alter role ui_readonly set default_transaction_read_only = on';

  -- Postgres exporter (metrics)
  if not exists (select 1 from pg_roles where rolname = 'postgres_exporter') then
    create role postgres_exporter login noinherit;
  end if;

  -- set password from a psql var you pass in (like you do for the others)
  execute format('alter role postgres_exporter password %L', :'postgres_exporter_password');

  -- give it monitoring privileges (Postgres >= 10)
  grant pg_monitor to postgres_exporter;

  -- because you revoked CONNECT from PUBLIC, you must grant it explicitly
  grant connect on database untron to postgres_exporter;

  -- optional safety
  execute 'alter role postgres_exporter set default_transaction_read_only = on';
end $$;

-- 3) Baseline hardening (optional but I recommend it early)
-- Prevent accidental grants to everyone
revoke all on database untron from public;
revoke all on schema public from public;

-- 4) Permissions for PostgREST anon (start minimal; expand deliberately)
grant usage on schema api to pgrst_anon;
grant select on all tables in schema api to pgrst_anon;
alter default privileges in schema api grant select on tables to pgrst_anon;

-- 5) Permissions for UI read-only browsing
grant connect on database untron to db_readonly;
grant usage on schema api to db_readonly;

grant select on all tables in schema api to db_readonly;
grant select on all sequences in schema api to db_readonly;

-- Future-proof: objects created by *this role* (postgres, right now) will inherit grants
alter default privileges in schema api grant select on tables to db_readonly;
alter default privileges in schema api grant select on sequences to db_readonly;

EOSQL
