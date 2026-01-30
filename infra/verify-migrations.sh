#!/usr/bin/env bash
set -euo pipefail

# Validates that `apps/indexer/db/migrations` apply cleanly to an empty Postgres,
# using the same Docker images as the local infra stack.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

project="untron_migtest_$(date +%s)"
tmpdir="$(mktemp -d)"
override_compose="${tmpdir}/docker-compose.override.yml"

cleanup() {
  set +e
  docker compose -p "${project}" -f "${ROOT_DIR}/infra/docker-compose.yml" -f "${override_compose}" down -v --remove-orphans >/dev/null 2>&1
  rm -rf "${tmpdir}" >/dev/null 2>&1
}
trap cleanup EXIT

cat >"${override_compose}" <<'YAML'
services:
  db:
    ports: []
  gateway:
    ports: []
  pgweb:
    ports: []
YAML

POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-$(openssl rand -hex 16)}"
POSTGRES_EXPORTER_PASSWORD="${POSTGRES_EXPORTER_PASSWORD:-$(openssl rand -hex 16)}"
UI_READONLY_PASSWORD="${UI_READONLY_PASSWORD:-$(openssl rand -hex 16)}"
PGRST_AUTH_PASSWORD="${PGRST_AUTH_PASSWORD:-$(openssl rand -hex 16)}"

export POSTGRES_PASSWORD POSTGRES_EXPORTER_PASSWORD UI_READONLY_PASSWORD PGRST_AUTH_PASSWORD

echo "==> Starting clean DB + running migrations (project=${project})"
docker compose \
  -p "${project}" \
  -f "${ROOT_DIR}/infra/docker-compose.yml" \
  -f "${override_compose}" \
  up -d --build db

echo "==> Waiting for Postgres to accept connections"
for _ in $(seq 1 60); do
  if docker compose \
    -p "${project}" \
    -f "${ROOT_DIR}/infra/docker-compose.yml" \
    -f "${override_compose}" \
    exec -T db pg_isready -U postgres -d untron >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

echo "==> Running migrations"
docker compose \
  -p "${project}" \
  -f "${ROOT_DIR}/infra/docker-compose.yml" \
  -f "${override_compose}" \
  run --rm db_migrate

echo "==> Verifying expected views exist and compile"
db_cid="$(docker compose -p "${project}" -f "${ROOT_DIR}/infra/docker-compose.yml" -f "${override_compose}" ps -q db)"
if [[ -z "${db_cid}" ]]; then
  echo "ERROR: db container not found" >&2
  exit 1
fi

psql_cmd=(
  docker exec -e "PGPASSWORD=${POSTGRES_PASSWORD}" -i "${db_cid}"
  psql -v ON_ERROR_STOP=1 -U postgres -d untron
)

expected_views=(
  usdt_deposit_txs
  usdt_deposits_daily
  usdt_deposits_daily_by_action
  usdt_deposits_cumulative
  usdt_deposit_funnel_daily
  hub_claim_first_versions
  claims_created_daily
  claims_filled_daily
  claim_fill_latency_daily
  protocol_pnl_timeseries
  leases_started_daily
  leases_ending_daily
  active_leases_daily
  lease_kpis
  usdt_deposit_backlog_summary
  usdt_deposit_backlog_age_buckets
  realtor_deposits_daily
)

for v in "${expected_views[@]}"; do
  "${psql_cmd[@]}" -qAtc "select 1 from pg_views where schemaname='api' and viewname='${v}'" | grep -qx "1"
  "${psql_cmd[@]}" -qAtc "select 1 from api.${v} limit 0" >/dev/null
done

echo "==> Verifying derived billing views exist and compile"
"${psql_cmd[@]}" -qAtc "select 1 from pg_views where schemaname='realtor' and viewname='principal_leases'" | grep -qx "1"
"${psql_cmd[@]}" -qAtc "select 1 from realtor.principal_leases limit 0" >/dev/null

echo "==> OK: migrations applied and views verified (${#expected_views[@]} views)"
