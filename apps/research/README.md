# Untron V3's scratchpad scripts for research

The only one you'll probably need here is tlcStream, which is a long-range golden test runner for TronLightClient that uses Anvil and Tron's gRPC API to simulate real light client proofs of a real Tron chain being published on a kinda real EVM chain.

```
TRON_GRPC_HOST=... ANVIL_PRIVATE_KEY=0x... pnpm research tlcStream --start 78000000 --end 78100000 --batch 10000 --concurrency 16 --anvil http://127.0.0.1:8545
```

Also some scripts like genTronFixture or genTrc20TxFixture are needed for reproducibility of artifacts inside the test folder in @untron/v3-contracts.
