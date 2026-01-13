## Local V3 Stack (Docker)

```bash
cp infra/.env.example infra/.env
cp infra/indexer.env.example infra/indexer.env
cp infra/realtor.env.example infra/realtor.env
cp infra/relayer.env.example infra/relayer.env

docker compose -f infra/docker-compose.yml up -d
```

- Swagger UI: `http://localhost:8080/docs`
- OpenAPI (3.x): `http://localhost:8080/openapi.json`

### Running behind a path prefix (external reverse proxy)

If an upstream reverse proxy publishes the gateway under a path prefix (for example `https://example.com/v3/*`)
and strips `/v3` before forwarding to this stack, set:

```bash
export EXTERNAL_PROXY_BASE_PATH=/v3 # no trailing slash
docker compose -f infra/docker-compose.yml up -d
```

This makes the Scalar docs and the OpenAPI `servers[0].url` reflect the externally-visible prefix.

### Observability (Prometheus + Grafana + Tempo)

```bash
docker compose -f infra/docker-compose.yml --profile observability up -d
```

- Grafana: `http://localhost:3000` (admin/admin)
- Prometheus: `http://localhost:9090`
- Tempo: `http://localhost:3200`

Optional services (profiles):

```bash
docker compose -f infra/docker-compose.yml --profile realtor up -d
docker compose -f infra/docker-compose.yml --profile relayer up -d
```

Update the checked-in OpenAPI spec used by `crates/indexer-client`:

```bash
cd .. # monorepo root
./genindexerclient.sh
```

If you need to rebuild the OpenAPI sidecar image:

```bash
docker compose -f infra/docker-compose.yml build openapi_sidecar
```
