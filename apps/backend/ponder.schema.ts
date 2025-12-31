import { and, desc, eq, index, onchainEnum, onchainTable, onchainView, sql } from "ponder";

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

// Relayer-side backstop for deduping `isEventChainTip` calls when Tron log indexing is missing/laggy.
// Stores the last confirmed `isEventChainTip` tx per controller contract.
export const tronIsEventChainTipSent = onchainTable(
  "tron_is_event_chain_tip_sent",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    eventChainTip: t.hex().notNull(),
    txid: t.hex().notNull(),
    confirmedAtBlockNumber: t.bigint().notNull(),
    confirmedAtBlockTimestamp: t.bigint().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

// Backstop for deduping `pullFromReceivers(token, salts)` calls.
// Stores the last confirmed receiverSalt set per (controller, token).
export const tronPullFromReceiversSent = onchainTable(
  "tron_pull_from_receivers_sent",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${tokenAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    tokenAddress: t.hex().notNull(),
    receiverSaltsHash: t.hex().notNull(), // bytes32 hash of sorted salts
    txid: t.hex().notNull(),
    confirmedAtBlockNumber: t.bigint().notNull(),
    confirmedAtBlockTimestamp: t.bigint().notNull(),
  }),
  (table) => ({
    contractTokenIdx: index().on(table.chainId, table.contractAddress, table.tokenAddress),
  })
);

// Backstop for deduping `rebalanceUsdt(rebalancer, amount)` calls.
// Stores the last confirmed rebalance decision per controller.
export const tronRebalanceUsdtSent = onchainTable(
  "tron_rebalance_usdt_sent",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    pulledUsdt: t.bigint().notNull(),
    inAmount: t.bigint().notNull(),
    txid: t.hex().notNull(),
    confirmedAtBlockNumber: t.bigint().notNull(),
    confirmedAtBlockTimestamp: t.bigint().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

// Demand-driven publish requests for storing Tron txTrieRoots in TronLightClient.
// Rows are deleted once the Tron block root is confirmed on mainnet.
export const tronLightClientPublishRequest = onchainTable(
  "tron_light_client_publish_request",
  (t) => ({
    id: t.text().primaryKey(), // `${mainnetChainId}:${tronLightClientAddress}:${tronBlockNumber}`
    chainId: t.integer().notNull(), // mainnet chain id
    tronLightClientAddress: t.hex().notNull(),
    tronBlockNumber: t.bigint().notNull(),
    requestedAtTronBlockTimestamp: t.bigint().notNull(),
    lastSentAtTronBlockNumber: t.bigint(),
    lastSentAtTronBlockTimestamp: t.bigint(),
    source: t.text(),
  }),
  (table) => ({
    contractBlockIdx: index().on(
      table.chainId,
      table.tronLightClientAddress,
      table.tronBlockNumber
    ),
    contractIdx: index().on(table.chainId, table.tronLightClientAddress),
  })
);

export const tronLightClientCheckpoint = onchainTable(
  "tron_light_client_checkpoint",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${tronBlockNumber}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    tronBlockNumber: t.bigint().notNull(),
    tronBlockId: t.hex().notNull(),
    tronTxTrieRoot: t.hex().notNull(),
    tronBlockTimestamp: t.bigint().notNull(),
    storedAtBlockNumber: t.bigint().notNull(),
    storedAtBlockTimestamp: t.bigint().notNull(),
    storedAtTransactionHash: t.hex().notNull(),
    storedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTronBlockIdx: index().on(table.chainId, table.contractAddress, table.tronBlockNumber),
    contractStoredAtBlockIdx: index().on(
      table.chainId,
      table.contractAddress,
      table.storedAtBlockNumber
    ),
  })
);

// Derived index for skipping already pre-entitled deposits without onchain reads.
export const untronV3DepositPreEntitled = onchainTable(
  "untron_v3_deposit_preentitled",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${txId}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    txId: t.hex().notNull(), // bytes32
    leaseId: t.bigint().notNull(),
    rawAmount: t.bigint().notNull(),
    netOut: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractTxIdIdx: index().on(table.chainId, table.contractAddress, table.txId),
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

// Derived state for UntronV3.lastReceiverPullTimestampByToken[receiverSalt][token], based on processed controller events.
// Used by the relayer to avoid attempting preEntitle for deposits that are known to be at/before the latest pull.
export const untronV3LastReceiverPull = onchainTable(
  "untron_v3_last_receiver_pull",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${receiverSalt}:${tokenAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    receiverSalt: t.hex().notNull(), // bytes32
    tokenAddress: t.hex().notNull(),
    lastPullTronBlockNumber: t.bigint().notNull(),
    lastPullTronBlockTimestamp: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractReceiverTokenIdx: index().on(
      table.chainId,
      table.contractAddress,
      table.receiverSalt,
      table.tokenAddress
    ),
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

// Derived state for UntronV3.tronUsdt based on emitted events.
export const untronV3TronUsdt = onchainTable(
  "untron_v3_tron_usdt",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    tronUsdt: t.hex().notNull(), // EVM-representation of Tron TRC20 address
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

// Singleton config/state row for each TronLightClient instance, derived from TronLightClientConfigured.
// Used by the publisher to derive the SR-owner -> witnessIndex mapping without onchain calls.
export const tronLightClientConfig = onchainTable(
  "tron_light_client_config",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    srDataHash: t.hex().notNull(),
    initialBlockId: t.hex().notNull(),
    srsJson: t.text().notNull(), // JSON array of bytes20 hex (length 27)
    witnessDelegateesJson: t.text().notNull(), // JSON array of bytes20 hex (length 27)
    configuredAtBlockNumber: t.bigint().notNull(),
    configuredAtBlockTimestamp: t.bigint().notNull(),
    configuredAtTransactionHash: t.hex().notNull(),
    configuredAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
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
    contractBeneficiaryIdx: index().on(table.chainId, table.contractAddress, table.beneficiary),
    contractTargetTokenIdx: index().on(table.chainId, table.contractAddress, table.targetToken),
    contractTargetChainIdx: index().on(table.chainId, table.contractAddress, table.targetChainId),
  })
);

export const untronV3Lease = onchainTable(
  "untron_v3_lease",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${leaseId}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    leaseId: t.bigint().notNull(),
    receiverSalt: t.hex().notNull(), // bytes32
    realtor: t.hex().notNull(),
    lessee: t.hex().notNull(),
    startTime: t.bigint().notNull(), // uint64
    nukeableAfter: t.bigint().notNull(), // uint64
    leaseFeePpm: t.bigint().notNull(), // uint32
    flatFee: t.bigint().notNull(), // uint64
    createdAtBlockNumber: t.bigint().notNull(),
    createdAtBlockTimestamp: t.bigint().notNull(),
    createdAtTransactionHash: t.hex().notNull(),
    createdAtLogIndex: t.integer().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractLeaseIdx: index().on(table.chainId, table.contractAddress, table.leaseId),
    contractReceiverSaltIdx: index().on(table.chainId, table.contractAddress, table.receiverSalt),
    contractRealtorIdx: index().on(table.chainId, table.contractAddress, table.realtor),
    contractLesseeIdx: index().on(table.chainId, table.contractAddress, table.lessee),
    contractNukeableAfterIdx: index().on(table.chainId, table.contractAddress, table.nukeableAfter),
  })
);

export const untronV3LeaseNonce = onchainTable(
  "untron_v3_lease_nonce",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${leaseId}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    leaseId: t.bigint().notNull(),
    nonce: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractLeaseIdx: index().on(table.chainId, table.contractAddress, table.leaseId),
  })
);

export const untronV3ProtocolLeaseRateLimit = onchainTable(
  "untron_v3_protocol_lease_rate_limit",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    maxLeases: t.bigint().notNull(),
    windowSeconds: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

export const untronV3LesseePayoutConfigRateLimit = onchainTable(
  "untron_v3_lessee_payout_config_rate_limit",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    maxUpdates: t.bigint().notNull(),
    windowSeconds: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

export const untronV3Realtor = onchainTable(
  "untron_v3_realtor",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}:${realtor}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    realtor: t.hex().notNull(),
    allowed: t.boolean().notNull(),
    minFeePpm: t.bigint().notNull(),
    leaseRateLimitMode: t.integer().notNull(), // uint8 enum in contract
    leaseRateLimitMaxLeases: t.bigint().notNull(),
    leaseRateLimitWindowSeconds: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractRealtorIdx: index().on(table.chainId, table.contractAddress, table.realtor),
    contractAllowedIdx: index().on(table.chainId, table.contractAddress, table.allowed),
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

export const untronV3ControllerEventQueue = onchainTable(
  "untron_v3_controller_event_queue",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    enqueuedCount: t.bigint().notNull(),
    processedCount: t.bigint().notNull(),
    updatedAtBlockNumber: t.bigint().notNull(),
    updatedAtBlockTimestamp: t.bigint().notNull(),
    updatedAtTransactionHash: t.hex().notNull(),
    updatedAtLogIndex: t.integer().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
  })
);

export const untronV3ProcessControllerEventsSent = onchainTable(
  "untron_v3_process_controller_events_sent",
  (t) => ({
    id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
    chainId: t.integer().notNull(),
    contractAddress: t.hex().notNull(),
    enqueuedCount: t.bigint().notNull(),
    processedCount: t.bigint().notNull(),
    sentAtBlockNumber: t.bigint().notNull(),
    sentAtBlockTimestamp: t.bigint().notNull(),
  }),
  (table) => ({
    contractIdx: index().on(table.chainId, table.contractAddress),
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
    isFilled: t.boolean(),
    filledAtBlockNumber: t.bigint(),
    filledAtBlockTimestamp: t.bigint(),
    filledAtTransactionHash: t.hex(),
    filledAtLogIndex: t.integer(),
  }),
  (table) => ({
    contractTokenClaimIdx: index().on(
      table.chainId,
      table.contractAddress,
      table.targetToken,
      table.claimIndex
    ),
    contractLeaseIdx: index().on(table.chainId, table.contractAddress, table.leaseId),
    contractFilledIdx: index().on(table.chainId, table.contractAddress, table.isFilled),
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

export const untronV3LeaseFull = onchainView("untron_v3_lease_full").as((qb) =>
  qb
    .select({
      id: untronV3Lease.id,
      chainId: untronV3Lease.chainId,
      contractAddress: untronV3Lease.contractAddress,
      leaseId: untronV3Lease.leaseId,
      receiverSalt: untronV3Lease.receiverSalt,
      realtor: untronV3Lease.realtor,
      lessee: untronV3Lease.lessee,
      startTime: untronV3Lease.startTime,
      nukeableAfter: untronV3Lease.nukeableAfter,
      leaseFeePpm: untronV3Lease.leaseFeePpm,
      flatFee: untronV3Lease.flatFee,
      leaseNonce: untronV3LeaseNonce.nonce,
      payoutTargetChainId: untronV3LeasePayoutConfig.targetChainId,
      payoutTargetToken: untronV3LeasePayoutConfig.targetToken,
      payoutBeneficiary: untronV3LeasePayoutConfig.beneficiary,
      realtorAllowed: untronV3Realtor.allowed,
      realtorMinFeePpm: untronV3Realtor.minFeePpm,
      realtorLeaseRateLimitMode: untronV3Realtor.leaseRateLimitMode,
      realtorLeaseRateLimitMaxLeases: untronV3Realtor.leaseRateLimitMaxLeases,
      realtorLeaseRateLimitWindowSeconds: untronV3Realtor.leaseRateLimitWindowSeconds,
      protocolLeaseRateLimitMaxLeases: untronV3ProtocolLeaseRateLimit.maxLeases,
      protocolLeaseRateLimitWindowSeconds:
        sql<bigint>`${untronV3ProtocolLeaseRateLimit.windowSeconds}`.as(
          "protocol_lease_rate_limit_window_seconds"
        ),
      lesseePayoutConfigRateLimitMaxUpdates: untronV3LesseePayoutConfigRateLimit.maxUpdates,
      lesseePayoutConfigRateLimitWindowSeconds:
        sql<bigint>`${untronV3LesseePayoutConfigRateLimit.windowSeconds}`.as(
          "lessee_payout_config_rate_limit_window_seconds"
        ),
      isNukeableYet:
        sql<boolean>`(${untronV3Lease.nukeableAfter} <= (EXTRACT(EPOCH FROM NOW())::bigint))`.as(
          "is_nukeable_yet"
        ),
      isActive:
        sql<boolean>`(${untronV3Lease.nukeableAfter} > (EXTRACT(EPOCH FROM NOW())::bigint))`.as(
          "is_active"
        ),
      createdAtBlockNumber: untronV3Lease.createdAtBlockNumber,
      createdAtBlockTimestamp: untronV3Lease.createdAtBlockTimestamp,
      createdAtTransactionHash: untronV3Lease.createdAtTransactionHash,
      createdAtLogIndex: untronV3Lease.createdAtLogIndex,
      updatedAtBlockNumber: untronV3Lease.updatedAtBlockNumber,
      updatedAtBlockTimestamp: untronV3Lease.updatedAtBlockTimestamp,
      updatedAtTransactionHash: untronV3Lease.updatedAtTransactionHash,
      updatedAtLogIndex: untronV3Lease.updatedAtLogIndex,
    })
    .from(untronV3Lease)
    .leftJoin(
      untronV3LeaseNonce,
      and(
        eq(untronV3LeaseNonce.chainId, untronV3Lease.chainId),
        eq(untronV3LeaseNonce.contractAddress, untronV3Lease.contractAddress),
        eq(untronV3LeaseNonce.leaseId, untronV3Lease.leaseId)
      )
    )
    .leftJoin(
      untronV3LeasePayoutConfig,
      and(
        eq(untronV3LeasePayoutConfig.chainId, untronV3Lease.chainId),
        eq(untronV3LeasePayoutConfig.contractAddress, untronV3Lease.contractAddress),
        eq(untronV3LeasePayoutConfig.leaseId, untronV3Lease.leaseId)
      )
    )
    .leftJoin(
      untronV3Realtor,
      and(
        eq(untronV3Realtor.chainId, untronV3Lease.chainId),
        eq(untronV3Realtor.contractAddress, untronV3Lease.contractAddress),
        eq(untronV3Realtor.realtor, untronV3Lease.realtor)
      )
    )
    .leftJoin(
      untronV3ProtocolLeaseRateLimit,
      and(
        eq(untronV3ProtocolLeaseRateLimit.chainId, untronV3Lease.chainId),
        eq(untronV3ProtocolLeaseRateLimit.contractAddress, untronV3Lease.contractAddress)
      )
    )
    .leftJoin(
      untronV3LesseePayoutConfigRateLimit,
      and(
        eq(untronV3LesseePayoutConfigRateLimit.chainId, untronV3Lease.chainId),
        eq(untronV3LesseePayoutConfigRateLimit.contractAddress, untronV3Lease.contractAddress)
      )
    )
);

export const untronV3ClaimFull = onchainView("untron_v3_claim_full").as((qb) =>
  qb
    .select({
      id: untronV3Claim.id,
      chainId: untronV3Claim.chainId,
      contractAddress: untronV3Claim.contractAddress,
      targetToken: untronV3Claim.targetToken,
      claimIndex: untronV3Claim.claimIndex,
      claimQueueId:
        sql<string>`concat(${untronV3Claim.targetToken}, ':', ${untronV3Claim.claimIndex})`.as(
          "claim_queue_id"
        ),
      leaseId: untronV3Claim.leaseId,
      amountUsdt: untronV3Claim.amountUsdt,
      targetChainId: untronV3Claim.targetChainId,
      beneficiary: untronV3Claim.beneficiary,
      status:
        sql<string>`(CASE WHEN ${untronV3Claim.isFilled} THEN 'filled' ELSE 'pending' END)`.as(
          "status"
        ),
      isFilled: untronV3Claim.isFilled,
      createdAtBlockNumber: untronV3Claim.createdAtBlockNumber,
      createdAtBlockTimestamp: untronV3Claim.createdAtBlockTimestamp,
      createdAtTransactionHash: untronV3Claim.createdAtTransactionHash,
      createdAtLogIndex: untronV3Claim.createdAtLogIndex,
      filledAtBlockNumber: untronV3Claim.filledAtBlockNumber,
      filledAtBlockTimestamp: untronV3Claim.filledAtBlockTimestamp,
      filledAtTransactionHash: untronV3Claim.filledAtTransactionHash,
      filledAtLogIndex: untronV3Claim.filledAtLogIndex,
      leaseReceiverSalt: untronV3Lease.receiverSalt,
      leaseRealtor: untronV3Lease.realtor,
      leaseLessee: untronV3Lease.lessee,
      leaseStartTime: untronV3Lease.startTime,
      leaseNukeableAfter: untronV3Lease.nukeableAfter,
      leaseIsActive:
        sql<boolean>`(${untronV3Lease.nukeableAfter} > (EXTRACT(EPOCH FROM NOW())::bigint))`.as(
          "lease_is_active"
        ),
      swapRatePpm: untronV3SwapRate.ratePpm,
      bridger: untronV3BridgerRoute.bridger,
      realtorAllowed: untronV3Realtor.allowed,
      realtorMinFeePpm: untronV3Realtor.minFeePpm,
      realtorLeaseRateLimitMode: untronV3Realtor.leaseRateLimitMode,
      realtorLeaseRateLimitMaxLeases: untronV3Realtor.leaseRateLimitMaxLeases,
      realtorLeaseRateLimitWindowSeconds: untronV3Realtor.leaseRateLimitWindowSeconds,
      protocolLeaseRateLimitMaxLeases: untronV3ProtocolLeaseRateLimit.maxLeases,
      protocolLeaseRateLimitWindowSeconds: untronV3ProtocolLeaseRateLimit.windowSeconds,
    })
    .from(untronV3Claim)
    .leftJoin(
      untronV3Lease,
      and(
        eq(untronV3Lease.chainId, untronV3Claim.chainId),
        eq(untronV3Lease.contractAddress, untronV3Claim.contractAddress),
        eq(untronV3Lease.leaseId, untronV3Claim.leaseId)
      )
    )
    .leftJoin(
      untronV3SwapRate,
      and(
        eq(untronV3SwapRate.chainId, untronV3Claim.chainId),
        eq(untronV3SwapRate.contractAddress, untronV3Claim.contractAddress),
        eq(untronV3SwapRate.targetToken, untronV3Claim.targetToken)
      )
    )
    .leftJoin(
      untronV3BridgerRoute,
      and(
        eq(untronV3BridgerRoute.chainId, untronV3Claim.chainId),
        eq(untronV3BridgerRoute.contractAddress, untronV3Claim.contractAddress),
        eq(untronV3BridgerRoute.targetToken, untronV3Claim.targetToken),
        eq(untronV3BridgerRoute.targetChainId, untronV3Claim.targetChainId)
      )
    )
    .leftJoin(
      untronV3Realtor,
      and(
        eq(untronV3Realtor.chainId, untronV3Lease.chainId),
        eq(untronV3Realtor.contractAddress, untronV3Lease.contractAddress),
        eq(untronV3Realtor.realtor, untronV3Lease.realtor)
      )
    )
    .leftJoin(
      untronV3ProtocolLeaseRateLimit,
      and(
        eq(untronV3ProtocolLeaseRateLimit.chainId, untronV3Claim.chainId),
        eq(untronV3ProtocolLeaseRateLimit.contractAddress, untronV3Claim.contractAddress)
      )
    )
);

export const untronV3RealtorLeaseStats = onchainView("untron_v3_realtor_lease_stats").as((qb) =>
  qb
    .select({
      chainId: untronV3Lease.chainId,
      contractAddress: untronV3Lease.contractAddress,
      realtor: untronV3Lease.realtor,
      totalLeases: sql<bigint>`count(*)::bigint`.as("total_leases"),
      activeLeases:
        sql<bigint>`count(*) FILTER (WHERE ${untronV3Lease.nukeableAfter} > (EXTRACT(EPOCH FROM NOW())::bigint))::bigint`.as(
          "active_leases"
        ),
      nukeableLeases:
        sql<bigint>`count(*) FILTER (WHERE ${untronV3Lease.nukeableAfter} <= (EXTRACT(EPOCH FROM NOW())::bigint))::bigint`.as(
          "nukeable_leases"
        ),
    })
    .from(untronV3Lease)
    .groupBy(untronV3Lease.chainId, untronV3Lease.contractAddress, untronV3Lease.realtor)
);

export const untronV3RealtorFull = onchainView("untron_v3_realtor_full").as((qb) =>
  qb
    .select({
      id: untronV3Realtor.id,
      chainId: untronV3Realtor.chainId,
      contractAddress: untronV3Realtor.contractAddress,
      realtor: untronV3Realtor.realtor,
      allowed: untronV3Realtor.allowed,
      minFeePpm: untronV3Realtor.minFeePpm,
      leaseRateLimitMode: untronV3Realtor.leaseRateLimitMode,
      leaseRateLimitMaxLeases: untronV3Realtor.leaseRateLimitMaxLeases,
      leaseRateLimitWindowSeconds: untronV3Realtor.leaseRateLimitWindowSeconds,
      totalLeases: untronV3RealtorLeaseStats.totalLeases,
      activeLeases: untronV3RealtorLeaseStats.activeLeases,
      nukeableLeases: untronV3RealtorLeaseStats.nukeableLeases,
      updatedAtBlockNumber: untronV3Realtor.updatedAtBlockNumber,
      updatedAtBlockTimestamp: untronV3Realtor.updatedAtBlockTimestamp,
      updatedAtTransactionHash: untronV3Realtor.updatedAtTransactionHash,
      updatedAtLogIndex: untronV3Realtor.updatedAtLogIndex,
    })
    .from(untronV3Realtor)
    .leftJoin(
      untronV3RealtorLeaseStats,
      and(
        eq(untronV3RealtorLeaseStats.chainId, untronV3Realtor.chainId),
        eq(untronV3RealtorLeaseStats.contractAddress, untronV3Realtor.contractAddress),
        eq(untronV3RealtorLeaseStats.realtor, untronV3Realtor.realtor)
      )
    )
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
  "relay_controller_event_chain",
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
