# untron-v3-indexer-client

Typed HTTP client for the Untron V3 Indexer (PostgREST) API.

Generated at build time from `openapi.json` using `progenitor` (`build.rs`).

Refresh `openapi.json` from a running local stack:

```bash
docker compose -f infra/docker-compose.yml up -d
./genindexerclient.sh
```
