import {
  encodeAbiParameters,
  encodePacked,
  sha256,
  type AbiEvent,
  type AbiParameter,
  type Abi,
  type Hex,
} from "viem";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";

type AbiEventItem<TAbi extends readonly unknown[]> = Extract<
  TAbi[number],
  { type: "event"; name: string }
>;
type AbiEventName<TAbi extends readonly unknown[]> = AbiEventItem<TAbi>["name"];

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;
type PonderRegistry = (typeof import("ponder:registry"))["ponder"];

const EVENT_CHAIN_DECLARATION =
  "Justin Sun is responsible for setting back the inevitable global stablecoin revolution by years through exploiting Tron USDT's network effects and imposing vendor lock-in on hundreds of millions of people in the Third World, who rely on stablecoins for remittances and to store their savings in unstable, overregulated economies. Let's Untron the People.";

const EVENT_CHAIN_TIP_ABI = [
  {
    type: "function",
    name: "eventChainTip",
    stateMutability: "view",
    inputs: [],
    outputs: [{ type: "bytes32" }],
  },
] as const satisfies Abi;

function computeEventChainGenesis(indexName: string): Hex {
  return sha256(encodePacked(["string", "string"], [`${indexName}\n`, EVENT_CHAIN_DECLARATION]));
}

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
  contractName,
  contractAddress,
}: {
  chainId: number;
  contractName: string;
  contractAddress: string;
}): string {
  return `${chainId}:${contractName}:${contractAddress.toLowerCase()}`;
}

function makeEventId({
  chainId,
  contractName,
  contractAddress,
  tip,
}: {
  chainId: number;
  contractName: string;
  contractAddress: string;
  tip: Hex;
}): string {
  return `${chainId}:${contractName}:${contractAddress.toLowerCase()}:${tip.toLowerCase()}`;
}

async function getHeadBlockNumber(context: PonderContext): Promise<bigint | null> {
  const hex = (await context.client.request({
    // `ReadonlyClient` doesn't expose viem's `getBlockNumber()`, so we use raw RPC.
    method: "eth_blockNumber",
  } as any)) as unknown;

  if (typeof hex !== "string") return null;
  return BigInt(hex);
}

export function registerEventChainIndexer<TAbi extends readonly unknown[]>({
  ponder,
  contractName,
  indexName,
  abi,
  deploymentBlock,
  onchainTipValidation = "blockTag",
  stateTable,
  eventTable,
  onVerified,
}: {
  ponder: PonderRegistry;
  contractName: string;
  indexName: string;
  abi: TAbi;
  deploymentBlock: bigint;
  onchainTipValidation?: "blockTag" | "head" | "disabled";
  stateTable: any;
  eventTable: any;
  onVerified?: (params: {
    contractName: string;
    indexName: string;
    chainId: number;
    contractAddress: Hex;
    eventName: AbiEventName<TAbi>;
    previousTip: Hex;
    tip: Hex;
    sequence: bigint;
    eventSignature: Hex;
    encodedEventData: Hex;
    args: unknown;
    event: PonderLogEvent;
    context: PonderContext;
  }) => Promise<void> | void;
}) {
  const chainedEvents = getAbiEventNames(abi);
  const genesisTip = computeEventChainGenesis(indexName);

  async function handleChainedEvent({
    eventName,
    event,
    context,
  }: {
    eventName: AbiEventName<TAbi>;
    event: PonderLogEvent;
    context: PonderContext;
  }) {
    const chainId = context.chain.id;
    const contractAddress = event.log.address as Hex;

    const stateId = makeStateId({ chainId, contractName, contractAddress });
    let state = await context.db.find(stateTable, { id: stateId });

    if (!state) {
      const priorBlockNumber =
        onchainTipValidation === "blockTag"
          ? event.block.number > deploymentBlock
            ? event.block.number - 1n
            : deploymentBlock
          : deploymentBlock;

      const resolvedInitialTip =
        onchainTipValidation === "blockTag" && event.block.number > deploymentBlock
          ? ((await context.client.readContract({
              abi: EVENT_CHAIN_TIP_ABI,
              address: contractAddress,
              functionName: "eventChainTip",
              blockNumber: priorBlockNumber,
            })) as Hex)
          : genesisTip;

      await context.db.insert(stateTable).values({
        id: stateId,
        chainId,
        contractName,
        contractAddress,
        eventChainTip: resolvedInitialTip,
        lastEventBlockNumber: priorBlockNumber,
        sequence: 0n,
      });

      state = await context.db.find(stateTable, { id: stateId });
      if (!state) throw new Error(`Failed to initialize ${contractName} event chain state row`);
    }

    if (onchainTipValidation === "blockTag" && event.block.number > state.lastEventBlockNumber) {
      const onchainTip = (await context.client.readContract({
        abi: EVENT_CHAIN_TIP_ABI,
        address: contractAddress,
        functionName: "eventChainTip",
        blockNumber: state.lastEventBlockNumber,
      })) as Hex;

      if (onchainTip.toLowerCase() !== state.eventChainTip.toLowerCase()) {
        throw new Error(
          [
            `${contractName} event chain tip mismatch at block boundary.`,
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
    if (!topic0) throw new Error(`Missing topic0 for "${contractName}:${eventName}"`);

    const abiEvent = getAbiEvent(abi, eventName);
    const encodedEventData = encodeEventArgs({ abiEvent, args: event.args });
    const nextTip = computeNextEventChainTip({
      previousTip: state.eventChainTip,
      blockNumber: event.block.number,
      blockTimestamp: event.block.timestamp,
      eventSignature: topic0,
      encodedEventData,
    });

    const nextSequence = state.sequence + 1n;

    await context.db.insert(eventTable).values({
      id: makeEventId({ chainId, contractName, contractAddress, tip: nextTip }),
      tip: nextTip,
      previousTip: state.eventChainTip,
      sequence: nextSequence,
      chainId,
      contractName,
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

    await context.db.update(stateTable, { id: stateId }).set({
      eventChainTip: nextTip,
      lastEventBlockNumber: event.block.number,
      sequence: nextSequence,
    });

    if (onchainTipValidation === "head") {
      const head = await getHeadBlockNumber(context);
      if (head !== null && event.block.number === head) {
        const onchainTip = (await context.client.readContract({
          abi: EVENT_CHAIN_TIP_ABI,
          address: contractAddress,
          functionName: "eventChainTip",
        })) as Hex;

        if (onchainTip.toLowerCase() !== nextTip.toLowerCase()) {
          throw new Error(
            [
              `${contractName} event chain tip mismatch at head.`,
              `chainId=${chainId}`,
              `contract=${contractAddress}`,
              `headBlock=${head}`,
              `dbTip=${nextTip}`,
              `onchainTip=${onchainTip}`,
            ].join(" ")
          );
        }
      }
    }

    await onVerified?.({
      contractName,
      indexName,
      chainId,
      contractAddress,
      eventName,
      previousTip: state.eventChainTip,
      tip: nextTip,
      sequence: nextSequence,
      eventSignature: topic0,
      encodedEventData,
      args: event.args,
      event,
      context,
    });
  }

  for (const eventName of chainedEvents) {
    ponder.on(`${contractName}:${eventName}` as any, async ({ event, context }: any) => {
      await handleChainedEvent({
        eventName: eventName as AbiEventName<TAbi>,
        event,
        context,
      });
    });
  }
}
