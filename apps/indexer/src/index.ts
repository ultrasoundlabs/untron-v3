import { ponder } from "ponder:registry";

import { eventChainEvent, eventChainState } from "ponder:schema";
import { UntronV3Abi } from "../abis/evm/UntronV3Abi";
import { TronLightClientAbi } from "../abis/evm/TronLightClientAbi";
import { UntronControllerAbi } from "../abis/tron/UntronControllerAbi";

import { registerEventChainIndexer } from "./eventChainIndexer";

function requireEnv(name: string): string {
  const value = process.env[name];
  if (!value) throw new Error(`Missing env var ${name}`);
  return value;
}

registerEventChainIndexer({
  ponder,
  contractName: "UntronV3",
  indexName: "UntronV3Index",
  abi: UntronV3Abi,
  deploymentBlock: BigInt(requireEnv("UNTRON_V3_DEPLOYMENT_BLOCK")),
  stateTable: eventChainState,
  eventTable: eventChainEvent,
});

registerEventChainIndexer({
  ponder,
  contractName: "TronLightClient",
  indexName: "TronLightClientIndex",
  abi: TronLightClientAbi,
  deploymentBlock: BigInt(requireEnv("TRON_LIGHT_CLIENT_DEPLOYMENT_BLOCK")),
  stateTable: eventChainState,
  eventTable: eventChainEvent,
});

registerEventChainIndexer({
  ponder,
  contractName: "UntronController",
  indexName: "UntronControllerIndex",
  abi: UntronControllerAbi,
  deploymentBlock: BigInt(requireEnv("UNTRON_CONTROLLER_DEPLOYMENT_BLOCK")),
  onchainTipValidation: "head",
  stateTable: eventChainState,
  eventTable: eventChainEvent,
});
