import { desc, eq, index, onchainEnum, onchainTable, onchainView } from "ponder";

export const eventChainState = onchainTable(
  "event_chain_state",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractName}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractName: t.text().notNull(),
    contractAddress: t.hex().notNull(),
    eventChainTip: t.hex().notNull(),
    lastEventBlockNumber: t.bigint().notNull(),
    sequence: t.bigint().notNull(),
  }),
  (table) => ({
    contractAddressIdx: index().on(table.chainId, table.contractName, table.contractAddress),
    contractLastBlockIdx: index().on(table.chainId, table.contractName, table.lastEventBlockNumber),
  })
);

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
    contractEventNameBlockIdx: index().on(
      table.contractName,
      table.contractAddress,
      table.eventName,
      table.blockNumber
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

export const untronControllerLatestEventChainTip = onchainView(
  "untron_controller_latest_event_chain_tip"
).as((qb) =>
  qb
    .select()
    .from(eventChainState)
    .where(eq(eventChainState.contractName, "UntronController"))
    .orderBy(desc(eventChainState.lastEventBlockNumber))
    .limit(1)
);

export const untronControllerIsEventChainTipCalled = onchainTable(
  "untron_controller_is_event_chain_tip_called",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${transactionHash}:${logIndex}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    caller: t.hex().notNull(),
    eventChainTip: t.hex().notNull(),
    blockNumber: t.bigint().notNull(),
    blockTimestamp: t.bigint().notNull(),
    transactionHash: t.hex().notNull(),
    logIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractBlockIdx: index().on(table.chainId, table.contractAddress, table.blockNumber),
    contractTxLogIdx: index().on(table.chainId, table.contractAddress, table.transactionHash),
  })
);

export const untronControllerLatestIsEventChainTipCalled = onchainView(
  "untron_controller_latest_is_event_chain_tip_called"
).as((qb) =>
  qb
    .select()
    .from(untronControllerIsEventChainTipCalled)
    .orderBy(
      desc(untronControllerIsEventChainTipCalled.blockNumber),
      desc(untronControllerIsEventChainTipCalled.logIndex)
    )
    .limit(1)
);

export const untronV3LeasePayoutConfig = onchainTable(
  "untron_v3_lease_payout_config",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${leaseId}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    leaseId: t.bigint().notNull(),
    targetChainId: t.bigint().notNull(),
    targetToken: t.hex().notNull(),
    beneficiary: t.hex().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractLeaseIdx: index().on(table.chainId, table.contractAddress, table.leaseId),
  })
);

export const untronV3SwapRate = onchainTable(
  "untron_v3_swap_rate",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${targetToken}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    targetToken: t.hex().notNull(),
    ratePpm: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTokenIdx: index().on(table.chainId, table.contractAddress, table.targetToken),
  })
);

export const untronV3ClaimQueue = onchainTable(
  "untron_v3_claim_queue",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${targetToken}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    targetToken: t.hex().notNull(),
    maxClaimIndex: t.bigint().notNull(),
    queueLength: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTokenIdx: index().on(table.chainId, table.contractAddress, table.targetToken),
    contractQueueLengthIdx: index().on(table.chainId, table.contractAddress, table.queueLength),
  })
);

export const untronV3Claim = onchainTable(
  "untron_v3_claim",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${targetToken}:${claimIndex}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    targetToken: t.hex().notNull(),
    claimIndex: t.bigint().notNull(),
    leaseId: t.bigint().notNull(),
    amountUsdt: t.bigint().notNull(),
    targetChainId: t.bigint().notNull(),
    beneficiary: t.hex().notNull(),
    createdAtBlockNumber: t.bigint().notNull(),
    createdAtBlockTimestamp: t.bigint().notNull(),
    createdAtTransactionHash: t.hex().notNull(),
    createdAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTokenClaimIdx: index().on(
      table.chainId,
      table.contractAddress,
      table.targetToken,
      table.claimIndex
    ),
    contractLeaseIdx: index().on(table.chainId, table.contractAddress, table.leaseId),
  })
);

export const untronV3BridgerRoute = onchainTable(
  "untron_v3_bridger_route",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${targetToken}:${targetChainId}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    targetToken: t.hex().notNull(),
    targetChainId: t.bigint().notNull(),
    bridger: t.hex().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTokenChainIdx: index().on(
      table.chainId,
      table.contractAddress,
      table.targetToken,
      table.targetChainId
    ),
  })
);

export const relayerStatus = onchainTable("relayer_status", (t) => ({
  chainId: t.integer().primaryKey(),
  isLive: t.boolean().notNull(),
  headBlockNumber: t.bigint().notNull(),
  headBlockTimestamp: t.bigint().notNull(),
}));

export const relayJobKindEnum = onchainEnum("relay_job_kind", [
  "mainnet_heartbeat",
  "tron_heartbeat",
  "trc20_transfer",
] as const);

export const relayJobStatusEnum = onchainEnum("relay_job_status", [
  "pending",
  "processing",
  "sent",
  "failed",
] as const);

export const relayJob = onchainTable(
  "relay_job",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${kind}:${dedupeKey}`
    chainId: t.integer().notNull(),
    createdAtBlockNumber: t.bigint().notNull(),
    createdAtBlockTimestamp: t.bigint().notNull(),
    kind: relayJobKindEnum("kind").notNull(),
    status: relayJobStatusEnum("status").notNull(),
    attempts: t.integer().notNull(),
    lockedAtBlockNumber: t.bigint(),
    lockedAtBlockTimestamp: t.bigint(),
    lockedBy: t.text(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    lastError: t.text(),
    nextRetryBlockNumber: t.bigint(),
    payloadJson: t.jsonb().notNull(),
  }),
  (table) => ({
    chainStatusIdx: index().on(table.chainId, table.status),
    chainKindStatusCreatedAtIdx: index().on(
      table.chainId,
      table.kind,
      table.status,
      table.createdAtBlockNumber
    ),
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
