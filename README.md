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
