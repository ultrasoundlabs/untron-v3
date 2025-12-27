import { ponder } from "ponder:registry";
import { Effect } from "effect";

import { tronLightClientAbi, untronControllerAbi, untronV3Abi } from "@untron/v3-contracts";

import { registerEventChainIndexer } from "./eventChainIndexer";
import { registerRelayer } from "./relayer";
import { handleTronLightClientDerivedEvent } from "./tronLightClientDerivedIndexer";
import { handleUntronControllerIsEventChainTipCalled } from "./untronControllerIsEventChainTipCalled";
import { handleUntronV3DerivedEvent } from "./untronV3DerivedIndexer";

registerEventChainIndexer({
  ponder,
  contractName: "UntronV3",
  indexName: "UntronV3Index",
  abi: untronV3Abi,
  afterEvent: ({ eventName, event, context }) =>
    handleUntronV3DerivedEvent({ eventName, event, context }),
});

registerEventChainIndexer({
  ponder,
  contractName: "TronLightClient",
  indexName: "TronLightClientIndex",
  abi: tronLightClientAbi,
  afterEvent: ({ eventName, event, context }) =>
    eventName === "TronBlockStored" || eventName === "TronLightClientConfigured"
      ? handleTronLightClientDerivedEvent({
          eventName: eventName as any,
          event,
          context,
        })
      : Effect.void,
});

registerEventChainIndexer({
  ponder,
  contractName: "UntronController",
  indexName: "UntronControllerIndex",
  abi: untronControllerAbi,
  onchainTipValidation: "head",
  skipTipUpdateEvents: ["IsEventChainTipCalled" as any],
  afterEvent: ({ eventName, event, context }) =>
    eventName === ("IsEventChainTipCalled" as any)
      ? handleUntronControllerIsEventChainTipCalled({ event, context })
      : Effect.void,
});

registerRelayer({ ponder });
