import { index, onchainTable } from "ponder";

export const untronV3State = onchainTable("untron_v3_state", (t) => ({
  id: t.text().primaryKey(), // `${chainId}:${contractAddress}`
  chainId: t.integer().notNull(),
  contractAddress: t.hex().notNull(),
  eventChainTip: t.hex().notNull(),
  lastEventBlockNumber: t.bigint().notNull(),
  sequence: t.bigint().notNull(),
}));

export const untronV3Event = onchainTable(
  "untron_v3_event",
  (t) => ({
    tip: t.hex().primaryKey(),
    previousTip: t.hex().notNull(),
    sequence: t.bigint().notNull(),
    chainId: t.integer().notNull(),
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
    sequenceIdx: index().on(table.sequence),
    blockNumberIdx: index().on(table.blockNumber),
    transactionHashIdx: index().on(table.transactionHash),
  })
);
