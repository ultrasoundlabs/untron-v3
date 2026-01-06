## Local V3 Stack (Docker)

```bash
cp infra/indexer.env.example infra/indexer.env
cp infra/realtor.env.example infra/realtor.env
cp infra/relayer.env.example infra/relayer.env

docker compose -f infra/docker-compose.yml up -d
```

- Swagger UI: `http://localhost:8080/docs`
- OpenAPI (3.x): `http://localhost:8080/openapi.json`

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

If you change `infra/openapi-sidecar/src/*`, rebuild the bundled runtime used by Docker:

```bash
pnpm openapi:build # from monorepo root
```
