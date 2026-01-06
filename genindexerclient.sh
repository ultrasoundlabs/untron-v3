#!/usr/bin/env bash
set -euo pipefail

# Updates the checked-in OpenAPI 3.x spec used by the Progenitor-generated client
# in `crates/indexer-client`.
#
# Requires the compose gateway to be running and proxying PostgREST at `/`:
#   docker compose -f infra/docker-compose.yml up -d
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

curl -sSf -H "Accept: application/openapi+json" "http://localhost:8080/" -o "$tmp"

if [[ ! -f "infra/openapi-sidecar/dist/cli.cjs" ]]; then
  node infra/openapi-sidecar/build.mjs
fi

cat "$tmp" | node infra/openapi-sidecar/dist/cli.cjs > crates/indexer-client/openapi.json
echo "wrote crates/indexer-client/openapi.json"
