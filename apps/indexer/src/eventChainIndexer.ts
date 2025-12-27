import { Cause, Effect } from "effect";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";
import { eventChainEvent, eventChainState } from "ponder:schema";
import {
  encodeAbiParameters,
  encodePacked,
  sha256,
  type Abi,
  type AbiEvent,
  type AbiParameter,
  type Hex,
} from "viem";

import { tryPromise } from "./effect/tryPromise";
import { IndexerRuntime } from "./effect/runtime";
import { computeNextEventChainTip } from "./eventChain/tip";
import { getArgValue } from "./parse";

type AbiEventItem<TAbi extends readonly unknown[]> = Extract<
  TAbi[number],
  { type: "event"; name: string }
>;
type AbiEventName<TAbi extends readonly unknown[]> = AbiEventItem<TAbi>["name"];

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;
type PonderRegistry = (typeof import("ponder:registry"))["ponder"];
type ContractName = keyof PonderContext["contracts"];

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

function getDeploymentBlock(context: PonderContext, contractName: ContractName): bigint {
  const startBlock = context.contracts[contractName].startBlock;

  if (typeof startBlock === "bigint") return startBlock;
  if (typeof startBlock === "number") return BigInt(startBlock);
  if (typeof startBlock === "string") {
    if (startBlock === "latest")
      throw new Error(`Unsupported startBlock "latest" for ${contractName}`);
    return BigInt(startBlock);
  }

  throw new Error(`Unsupported startBlock type for ${contractName}`);
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

  if (!eventItem) throw new Error(`Event "${eventName}" not found in ABI`);
  return eventItem;
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

const getHeadBlockNumber = (context: PonderContext): Effect.Effect<bigint | null, Error> =>
  tryPromise(async () => {
    const hex = (await context.client.request({
      method: "eth_blockNumber",
    } as any)) as unknown;

    if (typeof hex !== "string") return null;
    return BigInt(hex);
  });

export function registerEventChainIndexer<TAbi extends readonly unknown[]>({
  ponder,
  contractName,
  indexName,
  abi,
  onchainTipValidation = "blockTag",
  skipTipUpdateEvents,
  afterEvent,
}: {
  ponder: PonderRegistry;
  contractName: ContractName;
  indexName: string;
  abi: TAbi;
  onchainTipValidation?: "blockTag" | "head" | "disabled";
  skipTipUpdateEvents?: readonly AbiEventName<TAbi>[];
  afterEvent?: (args: {
    eventName: AbiEventName<TAbi>;
    event: PonderLogEvent;
    context: PonderContext;
  }) => Effect.Effect<void, unknown, any>;
}) {
  const chainedEvents = getAbiEventNames(abi);
  const genesisTip = computeEventChainGenesis(indexName);
  const skipTipUpdateEventSet = new Set<string>(skipTipUpdateEvents ?? []);

  const handleChainedEvent = (args: {
    eventName: AbiEventName<TAbi>;
    event: PonderLogEvent;
    context: PonderContext;
  }) =>
    Effect.gen(function* () {
      const chainId = args.context.chain.id;
      const contractAddress = args.event.log.address as Hex;
      const deploymentBlock = getDeploymentBlock(args.context, contractName);

      const stateId = makeStateId({ chainId, contractName, contractAddress });
      let state = yield* tryPromise(() => args.context.db.find(eventChainState, { id: stateId }));

      if (!state) {
        const priorBlockNumber =
          onchainTipValidation === "blockTag"
            ? args.event.block.number > deploymentBlock
              ? args.event.block.number - 1n
              : deploymentBlock
            : deploymentBlock;

        const resolvedInitialTip =
          onchainTipValidation === "blockTag" && args.event.block.number > deploymentBlock
            ? ((yield* tryPromise(() =>
                args.context.client.readContract({
                  abi: EVENT_CHAIN_TIP_ABI,
                  address: contractAddress,
                  functionName: "eventChainTip",
                  blockNumber: priorBlockNumber,
                })
              )) as Hex)
            : genesisTip;

        yield* tryPromise(() =>
          args.context.db.insert(eventChainState).values({
            id: stateId,
            chainId,
            contractName,
            contractAddress,
            eventChainTip: resolvedInitialTip,
            lastEventBlockNumber: priorBlockNumber,
            sequence: 0n,
          })
        );

        state = yield* tryPromise(() => args.context.db.find(eventChainState, { id: stateId }));
        if (!state) throw new Error(`Failed to initialize ${contractName} event chain state row`);
      }

      if (
        onchainTipValidation === "blockTag" &&
        args.event.block.number > state.lastEventBlockNumber
      ) {
        const onchainTip = (yield* tryPromise(() =>
          args.context.client.readContract({
            abi: EVENT_CHAIN_TIP_ABI,
            address: contractAddress,
            functionName: "eventChainTip",
            blockNumber: state.lastEventBlockNumber,
          })
        )) as Hex;

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

      if (skipTipUpdateEventSet.has(args.eventName)) {
        if (args.event.block.number > state.lastEventBlockNumber) {
          yield* tryPromise(() =>
            args.context.db.update(eventChainState, { id: stateId }).set({
              lastEventBlockNumber: args.event.block.number,
            })
          );
        }

        if (afterEvent) {
          yield* afterEvent(args);
        }

        return;
      }

      const topic0 = args.event.log.topics[0];
      if (!topic0) throw new Error(`Missing topic0 for "${contractName}:${args.eventName}"`);

      const abiEvent = getAbiEvent(abi, args.eventName);
      const encodedEventData = encodeEventArgs({ abiEvent, args: args.event.args });
      const nextTip = computeNextEventChainTip({
        previousTip: state.eventChainTip,
        blockNumber: args.event.block.number,
        blockTimestamp: args.event.block.timestamp,
        eventSignature: topic0,
        encodedEventData,
      });

      const nextSequence = state.sequence + 1n;

      yield* tryPromise(() =>
        args.context.db.insert(eventChainEvent).values({
          id: makeEventId({ chainId, contractName, contractAddress, tip: nextTip }),
          tip: nextTip,
          previousTip: state.eventChainTip,
          sequence: nextSequence,
          chainId,
          contractName,
          contractAddress,
          blockNumber: args.event.block.number,
          blockTimestamp: args.event.block.timestamp,
          transactionHash: args.event.transaction.hash,
          transactionIndex: args.event.transaction.transactionIndex,
          logIndex: args.event.log.logIndex,
          eventName: args.eventName,
          eventSignature: topic0,
          encodedEventData,
          argsJson: safeJsonStringify(args.event.args),
        })
      );

      yield* tryPromise(() =>
        args.context.db.update(eventChainState, { id: stateId }).set({
          eventChainTip: nextTip,
          lastEventBlockNumber: args.event.block.number,
          sequence: nextSequence,
        })
      );

      if (onchainTipValidation === "head") {
        const head = yield* getHeadBlockNumber(args.context);
        if (head !== null && args.event.block.number === head) {
          const onchainTip = (yield* tryPromise(() =>
            args.context.client.readContract({
              abi: EVENT_CHAIN_TIP_ABI,
              address: contractAddress,
              functionName: "eventChainTip",
            })
          )) as Hex;

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

      if (afterEvent) {
        yield* afterEvent(args);
      }
    });

  for (const eventName of chainedEvents) {
    ponder.on(`${contractName}:${eventName}` as any, ({ event, context }: any) =>
      IndexerRuntime.runPromise(
        handleChainedEvent({
          eventName: eventName as AbiEventName<TAbi>,
          event,
          context,
        }).pipe(
          Effect.tapErrorCause(
            (cause): Effect.Effect<void, never, never> =>
              Effect.logError("[event_chain] handler failed").pipe(
                Effect.annotateLogs({
                  chainId: context.chain.id,
                  contractName,
                  eventName,
                  blockNumber: String(event.block.number),
                  transactionHash: event.transaction.hash,
                  logIndex: String(event.log.logIndex),
                  cause: Cause.pretty(cause),
                })
              )
          )
        )
      )
    );
  }
}
