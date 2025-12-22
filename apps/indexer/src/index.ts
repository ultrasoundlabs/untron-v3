import { ponder } from "ponder:registry";

import { UntronV3Abi } from "../abis/evm/UntronV3Abi";
import { TronLightClientAbi } from "../abis/evm/TronLightClientAbi";
import { UntronControllerAbi } from "../abis/tron/UntronControllerAbi";

import { registerEventChainIndexer } from "./eventChainIndexer";
import { registerRelayer } from "./relayer";

registerEventChainIndexer({
  ponder,
  contractName: "UntronV3",
  indexName: "UntronV3Index",
  abi: UntronV3Abi,
});

registerEventChainIndexer({
  ponder,
  contractName: "TronLightClient",
  indexName: "TronLightClientIndex",
  abi: TronLightClientAbi,
});

registerEventChainIndexer({
  ponder,
  contractName: "UntronController",
  indexName: "UntronControllerIndex",
  abi: UntronControllerAbi,
  onchainTipValidation: "head",
});

registerRelayer({ ponder });
