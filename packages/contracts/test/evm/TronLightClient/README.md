# TronLightClient's multi-layered testing

- [Layer 0](./layer0.t.sol): local math and packing is correct
- [Layer 1](./layer1.t.sol): given valid Tron data, the client accepts and stores it
- [Layer 2](./layer2.t.sol): given broken data, it rejects in the right ways
- [Layer 3](./layer3.t.sol): long-range behavior & ring buffer invariants
- [Layer 4](./layer4.t.sol): proveBlockRange wiring with mocked prover