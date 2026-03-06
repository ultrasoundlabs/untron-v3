# Repository Guidelines

## Project Structure & Module Organization
- `apps/` – runnable services and tools:
  - Rust services: `apps/indexer`, `apps/relayer`, `apps/realtor`, `apps/pool`
  - TypeScript utilities: `apps/forwarder`, `apps/research`
- `crates/` – shared Rust crates (e.g. `crates/tron`, `crates/observability`, generated bindings in `crates/bindings`)
- `packages/` – workspace packages:
  - `packages/contracts` – Solidity (Foundry) + generated TypeScript ABI/types
  - `packages/tron-protocol` – protobuf/gRPC bindings + build scripts
- `infra/` – local docker-compose stack, env templates, observability
- `docs/` – architecture diagram sources (`docs/untron-v3.mmd`)

## Build, Test, and Development Commands
Prereqs: Node.js (20+ recommended), `pnpm` (see `package.json`), Rust toolchain, and Foundry (`forge`). For the full local stack, you’ll also need Docker.

After cloning: `git submodule update --init --recursive` (contracts dependencies live under `packages/contracts/lib/`).

```bash
pnpm install          # install workspace deps
pnpm build            # build all packages/apps
pnpm test             # run tests across the workspace
pnpm verify           # CI gate (fmt + tests + prod build + codegen + typecheck)
pnpm format           # format TS + Solidity
pnpm codegen          # wagmi types + contracts ABI generation
```

Target a single component:
```bash
pnpm --filter @untron/v3-contracts test   # forge test (Foundry)
pnpm --filter @untron/v3-indexer dev      # cargo run -p indexer
```

## Coding Style & Naming Conventions
- TS/JS: Prettier (`tabWidth: 2`, `printWidth: 100`, semicolons, double quotes).
- Solidity: `forge fmt` + `solhint`; tests are typically `*.t.sol`.
- Rust: `cargo fmt`; keep `cargo clippy … -D warnings` clean (see `apps/*/package.json` scripts).
- Avoid hand-editing generated artifacts; commit updates to `packages/contracts/abi/generated.ts` after running `pnpm codegen`.

## Testing Guidelines
- Contracts: `forge test` (via `pnpm --filter @untron/v3-contracts test`).
- Rust: `cargo test -p <crate>` (or `pnpm --filter @untron/v3-<app> test`).
- Indexer SQL: lint/format in `apps/indexer/db` via `pnpm --filter @untron/v3-indexer run db:lint` / `db:format`.
- Migrations: `bash infra/verify-migrations.sh` (Docker required) to validate a clean DB apply.

## Commit & Pull Request Guidelines
- Prefer Conventional Commit style used in history: `feat(scope): …`, `fix(scope): …`, `refactor(scope): …`, `chore(scope): …` (scopes like `indexer`, `realtor`, `relayer`, `tron`).
- PRs should include: what/why, how to test (commands), and any contract/API/migration impact. Ensure `pnpm verify` passes; Husky hooks run on commit/push (`--no-verify` only for emergencies).

## Security & Configuration Tips
- Use the checked-in `.env.example` files (root/apps/infra) as templates; don’t commit secrets.
- Local stack: `docker compose -f infra/docker-compose.yml up -d` (see `infra/README.md`).
