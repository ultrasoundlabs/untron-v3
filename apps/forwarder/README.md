# Service for USDT0Forwarder

Simple orchestration service for [USDT0Forwarder](../../packages/contracts/src/evm/USDT0Forwarder.sol).

## Behavior

- Polls the forwarder contract's USDT0 balance.
- If balance is `> MIN_USDT` (default: `1`), sends `forward(tokenBalance)` with `msg.value = ~50%` of the signer's ETH balance (capped to leave room for gas).
- Any `msg.value` surplus is refunded by the forwarder contract.

## Config

See `apps/forwarder/.env.example`.

`RPC_URL` supports multiple endpoints separated by commas; the service uses a fallback transport and will try the next RPC if one fails.
