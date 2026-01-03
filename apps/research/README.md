# Untron V3's scratchpad scripts for research

The only one you'll probably need here is tlcStream, which is a long-range golden test runner for TronLightClient that uses Anvil and Tron's gRPC API to simulate real light client proofs of a real Tron chain being published on a kinda real EVM chain.

```
TRON_GRPC_HOST=... ANVIL_PRIVATE_KEY=0x... pnpm research tlcStream --start 78000000 --end 78100000 --batch 10000 --concurrency 16 --anvil http://127.0.0.1:8545
```

## Mock indexer playground (two anvils)

Spin up two local chains (hub + “tron”), deploy both sides, and write an indexer env file:

`pnpm research mockBothSides --spawn-anvil`

Generate lots of activity (events) on both sides for the indexer to ingest:

`pnpm research generateBothSidesActivity --receiver-salts 200 --deposits 1000 --controller-pulls 50 --relay-batch-size 50 --fills 50`

Also some scripts like genTronFixture or genTrc20TxFixture are needed for reproducibility of artifacts inside the test folder in @untron/v3-contracts.

Fetch full on-chain details for a transaction (raw tx + receipt/status + return data):

```
TRON_GRPC_HOST=... TRON_API_KEY=... pnpm research fetchTronTx 0x<txid>
TRON_GRPC_HOST=... TRON_API_KEY=... pnpm research fetchTronTx 0x<txid> --summary
```
