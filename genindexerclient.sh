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

merger_ref="a5c098c43edef67dbd24edac4661d8f47b935870"
merger_img="openapi-merger:${merger_ref}"

docker build -q -t "${merger_img}" "https://github.com/ultrasoundlabs/openapi-merger.git#${merger_ref}" >/dev/null
cat "$tmp" | docker run --rm -i -e OPENAPI_ALLOWED_METHODS=get "${merger_img}" --stdin > crates/indexer-client/openapi.json
echo "wrote crates/indexer-client/openapi.json"
