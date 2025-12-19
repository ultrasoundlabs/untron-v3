import { ponder } from "ponder:registry";

import { untronV3Event, untronV3State } from "ponder:schema";
import { UntronV3Abi } from "../abis/evm/UntronV3Abi";
import {
  encodeAbiParameters,
  encodePacked,
  sha256,
  type AbiEvent,
  type AbiParameter,
  type Hex,
} from "viem";

type AbiEventItem<TAbi extends readonly unknown[]> = Extract<
  TAbi[number],
  { type: "event"; name: string }
>;
type AbiEventName<TAbi extends readonly unknown[]> = AbiEventItem<TAbi>["name"];

type UntronV3ChainedEventName = AbiEventName<typeof UntronV3Abi>;

const EVENT_CHAIN_DECLARATION =
  "Justin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.";

const EVENT_CHAIN_GENESIS_UNTRON_V3_INDEX = sha256(
  encodePacked(["string", "string"], ["UntronV3Index\n", EVENT_CHAIN_DECLARATION])
);

function getAbiEventNames<TAbi extends readonly unknown[]>(
  abi: TAbi
): readonly AbiEventName<TAbi>[] {
  return abi
    .filter((entry): entry is AbiEventItem<TAbi> => {
      if (typeof entry !== "object" || entry === null) return false;
      const maybeEvent = entry as { type?: unknown; name?: unknown };
      return maybeEvent.type === "event" && typeof maybeEvent.name === "string";
    })
    .map((event) => event.name);
}

const UNTRON_V3_CHAINED_EVENTS: readonly UntronV3ChainedEventName[] = getAbiEventNames(UntronV3Abi);

function getAbiEvent(abi: readonly unknown[], eventName: string): AbiEvent {
  const eventItem = abi.find(
    (entry): entry is AbiEvent =>
      typeof entry === "object" &&
      entry !== null &&
      (entry as AbiEvent).type === "event" &&
      (entry as AbiEvent).name === eventName
  );

  if (!eventItem) {
    throw new Error(`Event "${eventName}" not found in ABI`);
  }

  return eventItem;
}

function getArgValue(args: unknown, index: number, name: string | undefined) {
  if (args && typeof args === "object" && !Array.isArray(args)) {
    if (name && name in args) return (args as Record<string, unknown>)[name];
  }
  if (Array.isArray(args)) return args[index];
  return undefined;
}

function encodeEventArgs({ abiEvent, args }: { abiEvent: AbiEvent; args: unknown }): Hex {
  const parameters = abiEvent.inputs as readonly AbiParameter[];
  const values = parameters.map((parameter, index) => getArgValue(args, index, parameter.name));

  if (values.some((value) => value === undefined)) {
    const missing = values
      .map((value, index) =>
        value === undefined ? (abiEvent.inputs[index]?.name ?? `#${index}`) : null
      )
      .filter((value): value is string => value !== null)
      .join(", ");

    throw new Error(`Missing event args for "${abiEvent.name}": ${missing}`);
  }

  return encodeAbiParameters(parameters, values as readonly unknown[]);
}

function computeNextEventChainTip({
  previousTip,
  blockNumber,
  blockTimestamp,
  eventSignature,
  encodedEventData,
}: {
  previousTip: Hex;
  blockNumber: bigint;
  blockTimestamp: bigint;
  eventSignature: Hex;
  encodedEventData: Hex;
}): Hex {
  return sha256(
    encodePacked(
      ["bytes32", "uint256", "uint256", "bytes32", "bytes"],
      [previousTip, blockNumber, blockTimestamp, eventSignature, encodedEventData]
    )
  );
}

function safeJsonStringify(value: unknown): string {
  return JSON.stringify(value, (_key, val) => (typeof val === "bigint" ? val.toString() : val));
}

function makeStateId({
  chainId,
  contractAddress,
}: {
  chainId: number;
  contractAddress: string;
}): string {
  return `${chainId}:${contractAddress.toLowerCase()}`;
}

async function handleUntronV3ChainedEvent({
  eventName,
  event,
  context,
}: {
  eventName: UntronV3ChainedEventName;
  event: any;
  context: any;
}) {
  const chainId = context.chain.id;
  const contractAddress = event.log.address as Hex;
  const deploymentBlockEnv = process.env.UNTRON_V3_DEPLOYMENT_BLOCK;
  if (!deploymentBlockEnv) {
    throw new Error("Missing env var UNTRON_V3_DEPLOYMENT_BLOCK");
  }
  const deploymentBlock = BigInt(deploymentBlockEnv);

  const stateId = makeStateId({ chainId, contractAddress });
  let state = await context.db.find(untronV3State, { id: stateId });

  if (!state) {
    const priorBlockNumber =
      event.block.number > deploymentBlock ? event.block.number - 1n : deploymentBlock;
    const initialTip =
      event.block.number > deploymentBlock
        ? await context.client.readContract({
            abi: UntronV3Abi,
            address: contractAddress,
            functionName: "eventChainTip",
            blockNumber: priorBlockNumber,
          })
        : EVENT_CHAIN_GENESIS_UNTRON_V3_INDEX;

    await context.db.insert(untronV3State).values({
      id: stateId,
      chainId,
      contractAddress,
      eventChainTip: initialTip,
      lastEventBlockNumber: priorBlockNumber,
      sequence: 0n,
    });

    state = await context.db.find(untronV3State, { id: stateId });
    if (!state) throw new Error("Failed to initialize untronV3State row");
  }

  if (event.block.number > state.lastEventBlockNumber) {
    const onchainTip = await context.client.readContract({
      abi: UntronV3Abi,
      address: contractAddress,
      functionName: "eventChainTip",
      blockNumber: state.lastEventBlockNumber,
    });

    if (onchainTip.toLowerCase() !== state.eventChainTip.toLowerCase()) {
      throw new Error(
        [
          "UntronV3 event chain tip mismatch at block boundary.",
          `chainId=${chainId}`,
          `contract=${contractAddress}`,
          `validatedBlock=${state.lastEventBlockNumber}`,
          `dbTip=${state.eventChainTip}`,
          `onchainTip=${onchainTip}`,
        ].join(" ")
      );
    }
  }

  const topic0 = event.log.topics[0];
  if (!topic0) throw new Error(`Missing topic0 for "${eventName}"`);

  const abiEvent = getAbiEvent(UntronV3Abi, eventName);
  const encodedEventData = encodeEventArgs({ abiEvent, args: event.args });
  const nextTip = computeNextEventChainTip({
    previousTip: state.eventChainTip,
    blockNumber: event.block.number,
    blockTimestamp: event.block.timestamp,
    eventSignature: topic0,
    encodedEventData,
  });

  const nextSequence = state.sequence + 1n;

  await context.db.insert(untronV3Event).values({
    tip: nextTip,
    previousTip: state.eventChainTip,
    sequence: nextSequence,
    chainId,
    contractAddress,
    blockNumber: event.block.number,
    blockTimestamp: event.block.timestamp,
    transactionHash: event.transaction.hash,
    transactionIndex: event.transaction.transactionIndex,
    logIndex: event.log.logIndex,
    eventName,
    eventSignature: topic0,
    encodedEventData,
    argsJson: safeJsonStringify(event.args),
  });

  await context.db.update(untronV3State, { id: stateId }).set({
    eventChainTip: nextTip,
    lastEventBlockNumber: event.block.number,
    sequence: nextSequence,
  });
}

for (const eventName of UNTRON_V3_CHAINED_EVENTS) {
  ponder.on(`UntronV3:${eventName}`, async ({ event, context }) => {
    await handleUntronV3ChainedEvent({
      eventName,
      event: event as any,
      context: context as any,
    });
  });
}
