import { eq, index, onchainTable, onchainView } from "ponder";

export const eventChainState = onchainTable("event_chain_state", (t) => ({
  id: t.text().primaryKey(), // `${chainId}:${contractName}:${contractAddress}`
  chainId: t.integer().notNull(),
  contractName: t.text().notNull(),
  contractAddress: t.hex().notNull(),
  eventChainTip: t.hex().notNull(),
  lastEventBlockNumber: t.bigint().notNull(),
  sequence: t.bigint().notNull(),
}));

export const eventChainEvent = onchainTable(
  "event_chain_event",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractName}:${contractAddress}:${tip}`
    tip: t.hex().notNull(),
    previousTip: t.hex().notNull(),
    sequence: t.bigint().notNull(),
    chainId: t.integer().notNull(),
    contractName: t.text().notNull(),
    contractAddress: t.hex().notNull(),
    blockNumber: t.bigint().notNull(),
    blockTimestamp: t.bigint().notNull(),
    transactionHash: t.hex().notNull(),
    transactionIndex: t.integer().notNull(),
    logIndex: t.integer().notNull(),
    eventName: t.text().notNull(),
    eventSignature: t.hex().notNull(),
    encodedEventData: t.hex().notNull(),
    argsJson: t.text(),
  }),
  (table) => ({
    contractAddressSequenceIdx: index().on(
      table.contractName,
      table.contractAddress,
      table.sequence
    ),
    contractBlockNumberIdx: index().on(
      table.contractName,
      table.contractAddress,
      table.blockNumber
    ),
    contractTipIdx: index().on(table.contractName, table.contractAddress, table.tip),
    transactionHashIdx: index().on(table.transactionHash),
  })
);

export const untronV3State = onchainView("untron_v3_state").as((qb) =>
  qb.select().from(eventChainState).where(eq(eventChainState.contractName, "UntronV3"))
);
export const untronV3Event = onchainView("untron_v3_event").as((qb) =>
  qb.select().from(eventChainEvent).where(eq(eventChainEvent.contractName, "UntronV3"))
);

export const tronLightClientState = onchainView("tron_light_client_state").as((qb) =>
  qb.select().from(eventChainState).where(eq(eventChainState.contractName, "TronLightClient"))
);
export const tronLightClientEvent = onchainView("tron_light_client_event").as((qb) =>
  qb.select().from(eventChainEvent).where(eq(eventChainEvent.contractName, "TronLightClient"))
);

export const untronControllerState = onchainView("untron_controller_state").as((qb) =>
  qb.select().from(eventChainState).where(eq(eventChainState.contractName, "UntronController"))
);
export const untronControllerEvent = onchainView("untron_controller_event").as((qb) =>
  qb.select().from(eventChainEvent).where(eq(eventChainEvent.contractName, "UntronController"))
);

export const relayerStatus = onchainTable("relayer_status", (t) => ({
  chainId: t.integer().primaryKey(),
  isLive: t.boolean().notNull(),
  headBlockNumber: t.bigint().notNull(),
  headBlockTimestamp: t.bigint().notNull(),
}));

export const relayJob = onchainTable(
  "relay_job",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${kind}:${dedupeKey}`
    chainId: t.integer().notNull(),
    createdAtBlockNumber: t.bigint().notNull(),
    createdAtBlockTimestamp: t.bigint().notNull(),
    kind: t.text().notNull(),
    status: t.text().notNull(), // "pending" | "sent" | "failed"
    attempts: t.integer().notNull(),
    payloadJson: t.jsonb().notNull(),
  }),
  (table) => ({
    chainStatusIdx: index().on(table.chainId, table.status),
  })
);

export const trc20Transfer = onchainTable("trc20_transfer", (t) => ({
  id: t.text().primaryKey(), // `${chainId}:${transactionHash}:${logIndex}`
  chainId: t.integer().notNull(),
  tokenAddress: t.hex().notNull(),
  from: t.hex().notNull(),
  to: t.hex().notNull(),
  value: t.bigint().notNull(),
  blockNumber: t.bigint().notNull(),
  blockTimestamp: t.bigint().notNull(),
  transactionHash: t.hex().notNull(),
  logIndex: t.integer().notNull(),
}));
