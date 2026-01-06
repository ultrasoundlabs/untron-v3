# untron-v3

Research and experiments related to a potential next iteration of the Untron protocol.

## Quick Start

```bash
# Install dependencies
pnpm install

# Build contracts
pnpm --filter @untron/v3-contracts build

# Run tests
pnpm --filter @untron/v3-contracts test

# Generate TypeScript bindings
pnpm wagmi generate

# Build Tron's gRPC bindings
pnpm --filter @untron/tron-protocol run build
```

## Development

**Stack**: pnpm workspaces, Foundry, Husky, GitHub Actions

**Git hooks** (auto-installed):
- Pre-commit: `forge fmt --check`, prettier
- Pre-push: `FOUNDRY_PROFILE=ci forge test --gas-report`
- Bypass: `git push --no-verify`

**Foundry profiles**: `dev` (default), `ci` (strict), `production` (optimized)

**Fix formatting**: `forge fmt` for Solidity, `pnpm prettier --write "**/*.{ts,js}"` for TS/JS

TODO: configure wagmi.config.ts to only include contracts that we use

## Local Indexer Stack (Docker)

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
./genindexerclient.sh
```

If you change `infra/openapi-sidecar/src/*`, rebuild the bundled runtime used by Docker:

```bash
pnpm openapi:build
```
