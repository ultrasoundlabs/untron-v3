#!/usr/bin/env bash
# This runs once when the Postgres container initializes an empty data dir.
# Keep auth secrets out of git: provide `AUTHENTICATOR_PASSWORD` via env/secrets.

POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_DB="${POSTGRES_DB:-untron}"
AUTHENTICATOR_PASSWORD="${AUTHENTICATOR_PASSWORD:-}"

if [ -z "$AUTHENTICATOR_PASSWORD" ]; then
  echo "AUTHENTICATOR_PASSWORD must be set for PostgREST login" >&2
  exit 1
fi

psql -v ON_ERROR_STOP=1 \
  --username "$POSTGRES_USER" \
  --dbname "$POSTGRES_DB" \
  -v authenticator_password="$AUTHENTICATOR_PASSWORD" <<'EOSQL' || exit 1
create schema if not exists api;

do $$
declare
  pw text := :'authenticator_password';
begin
  if pw is null or length(pw) = 0 then
    raise exception 'AUTHENTICATOR_PASSWORD is empty';
  end if;

  if not exists (select 1 from pg_roles where rolname = 'authenticator') then
    create role authenticator login;
  end if;

  if not exists (select 1 from pg_roles where rolname = 'web_anon') then
    create role web_anon nologin;
  end if;

  execute format('alter role authenticator password %L', pw);
end $$;

grant web_anon to authenticator;
EOSQL
