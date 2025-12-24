import { ponder } from "ponder:registry";
import { Effect } from "effect";

import { UntronV3Abi } from "../abis/evm/UntronV3Abi";
import { TronLightClientAbi } from "../abis/evm/TronLightClientAbi";
import { UntronControllerAbi } from "../abis/tron/UntronControllerAbi";

import { registerEventChainIndexer } from "./eventChainIndexer";
import { registerRelayer } from "./relayer";
import { handleTronLightClientDerivedEvent } from "./tronLightClientDerivedIndexer";
import { handleUntronControllerIsEventChainTipCalled } from "./untronControllerIsEventChainTipCalled";
import { handleUntronV3DerivedEvent } from "./untronV3DerivedIndexer";

registerEventChainIndexer({
  ponder,
  contractName: "UntronV3",
  indexName: "UntronV3Index",
  abi: UntronV3Abi,
  afterEvent: ({ eventName, event, context }) =>
    handleUntronV3DerivedEvent({ eventName, event, context }),
});

registerEventChainIndexer({
  ponder,
  contractName: "TronLightClient",
  indexName: "TronLightClientIndex",
  abi: TronLightClientAbi,
  afterEvent: ({ eventName, event, context }) =>
    eventName === "TronBlockStored"
      ? handleTronLightClientDerivedEvent({
          eventName,
          event,
          context,
        })
      : Effect.void,
});

registerEventChainIndexer({
  ponder,
  contractName: "UntronController",
  indexName: "UntronControllerIndex",
  abi: UntronControllerAbi,
  onchainTipValidation: "head",
  skipTipUpdateEvents: ["IsEventChainTipCalled" as any],
  afterEvent: ({ eventName, event, context }) =>
    eventName === ("IsEventChainTipCalled" as any)
      ? handleUntronControllerIsEventChainTipCalled({ event, context })
      : Effect.void,
});

registerRelayer({ ponder });
