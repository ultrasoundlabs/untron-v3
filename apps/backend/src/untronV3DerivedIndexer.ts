import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext, Event as PonderEvent } from "ponder:registry";

import {
  untronV3BridgerRoute,
  untronV3Claim,
  untronV3ClaimQueue,
  untronV3DepositPreEntitled,
  untronV3LastReceiverPull,
  untronV3Lease,
  untronV3LeaseNonce,
  untronV3LeasePayoutConfig,
  untronV3LesseePayoutConfigRateLimit,
  untronV3ProtocolFloor,
  untronV3ProtocolLeaseRateLimit,
  untronV3ChainDeprecated,
  untronV3Realtor,
  untronV3SwapRate,
  untronV3TronUsdt,
} from "ponder:schema";
import { decodeAbiParameters, keccak256, stringToHex, type Hex } from "viem";

import { tryPromise } from "./effect/tryPromise";
import {
  expectBigint,
  expectBoolean,
  expectHex,
  expectHexAddress,
  expectRecord,
  getArgValue,
} from "./parse";

type PonderLogEvent = Extract<PonderEvent, { log: unknown }>;

type UntronV3DerivedEventName =
  | "LeaseCreated"
  | "LeaseNonceUpdated"
  | "PayoutConfigUpdated"
  | "LesseePayoutConfigRateLimitSet"
  | "SwapRateSet"
  | "ClaimCreated"
  | "ClaimFilled"
  | "BridgerSet"
  | "DepositPreEntitled"
  | "TronUsdtSet"
  | "ProtocolFloorSet"
  | "ChainDeprecatedSet"
  | "ProtocolLeaseRateLimitSet"
  | "RealtorLeaseRateLimitSet"
  | "RealtorMinFeeSet"
  | "RealtorSet"
  | "ControllerEventChainTipUpdated"
  | "ControllerEventProcessed";

function makeLeaseId(args: { chainId: number; contractAddress: string; leaseId: bigint }): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.leaseId.toString()}`;
}

function makeLeaseConfigId(args: {
  chainId: number;
  contractAddress: string;
  leaseId: bigint;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.leaseId.toString()}`;
}

function makeTokenStateId(args: {
  chainId: number;
  contractAddress: string;
  targetToken: string;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.targetToken.toLowerCase()}`;
}

function makeClaimId(args: {
  chainId: number;
  contractAddress: string;
  targetToken: string;
  claimIndex: bigint;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.targetToken.toLowerCase()}:${args.claimIndex.toString()}`;
}

function makeBridgerRouteId(args: {
  chainId: number;
  contractAddress: string;
  targetToken: string;
  targetChainId: bigint;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.targetToken.toLowerCase()}:${args.targetChainId.toString()}`;
}

function makeControllerEventQueueId(args: { chainId: number; contractAddress: string }): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}`;
}

function makeDepositPreEntitledId(args: {
  chainId: number;
  contractAddress: string;
  txId: string;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.txId.toLowerCase()}`;
}

function makeTronUsdtId(args: { chainId: number; contractAddress: string }): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}`;
}

function makeRealtorId(args: {
  chainId: number;
  contractAddress: string;
  realtor: string;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.realtor.toLowerCase()}`;
}

function makeSingletonConfigId(args: { chainId: number; contractAddress: string }): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}`;
}

function makeChainDeprecatedId(args: {
  chainId: number;
  contractAddress: string;
  targetChainId: bigint;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.targetChainId.toString()}`;
}

function makeLastReceiverPullId(args: {
  chainId: number;
  contractAddress: string;
  receiverSalt: string;
  tokenAddress: string;
}): string {
  return `${args.chainId}:${args.contractAddress.toLowerCase()}:${args.receiverSalt.toLowerCase()}:${args.tokenAddress.toLowerCase()}`;
}

const EVENT_SIG_PULLED_FROM_RECEIVER = keccak256(
  stringToHex("PulledFromReceiver(bytes32,address,uint256,uint256,uint256)")
) as Hex;

const handleLeaseCreated = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const leaseId = expectBigint(getArgValue(parsedArgs, 0, "leaseId"), "LeaseCreated.leaseId");
    const receiverSalt = expectHex(
      getArgValue(parsedArgs, 1, "receiverSalt"),
      "LeaseCreated.receiverSalt"
    );
    const realtor = expectHexAddress(getArgValue(parsedArgs, 2, "realtor"), "LeaseCreated.realtor");
    const lessee = expectHexAddress(getArgValue(parsedArgs, 3, "lessee"), "LeaseCreated.lessee");
    const startTime = expectBigint(
      getArgValue(parsedArgs, 4, "startTime"),
      "LeaseCreated.startTime"
    );
    const nukeableAfter = expectBigint(
      getArgValue(parsedArgs, 5, "nukeableAfter"),
      "LeaseCreated.nukeableAfter"
    );
    const leaseFeePpm = expectBigint(
      getArgValue(parsedArgs, 6, "leaseFeePpm"),
      "LeaseCreated.leaseFeePpm"
    );
    const flatFee = expectBigint(getArgValue(parsedArgs, 7, "flatFee"), "LeaseCreated.flatFee");

    const id = makeLeaseId({ chainId, contractAddress, leaseId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3Lease)
        .values({
          id,
          chainId,
          contractAddress,
          leaseId,
          receiverSalt,
          realtor,
          lessee,
          startTime,
          nukeableAfter,
          leaseFeePpm,
          flatFee,
          createdAtBlockNumber: event.block.number,
          createdAtBlockTimestamp: event.block.timestamp,
          createdAtTransactionHash: event.transaction.hash,
          createdAtLogIndex: event.log.logIndex,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          receiverSalt,
          realtor,
          lessee,
          startTime,
          nukeableAfter,
          leaseFeePpm,
          flatFee,
          createdAtBlockNumber: event.block.number,
          createdAtBlockTimestamp: event.block.timestamp,
          createdAtTransactionHash: event.transaction.hash,
          createdAtLogIndex: event.log.logIndex,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleLeaseNonceUpdated = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const leaseId = expectBigint(
      getArgValue(parsedArgs, 0, "leaseId"),
      "LeaseNonceUpdated.leaseId"
    );
    const nonce = expectBigint(getArgValue(parsedArgs, 1, "nonce"), "LeaseNonceUpdated.nonce");

    const id = makeLeaseId({ chainId, contractAddress, leaseId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3LeaseNonce)
        .values({
          id,
          chainId,
          contractAddress,
          leaseId,
          nonce,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          nonce,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handlePayoutConfigUpdated = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const leaseId = expectBigint(
      getArgValue(parsedArgs, 0, "leaseId"),
      "PayoutConfigUpdated.leaseId"
    );
    const targetChainId = expectBigint(
      getArgValue(parsedArgs, 1, "targetChainId"),
      "PayoutConfigUpdated.targetChainId"
    );
    const targetToken = expectHexAddress(
      getArgValue(parsedArgs, 2, "targetToken"),
      "PayoutConfigUpdated.targetToken"
    );
    const beneficiary = expectHexAddress(
      getArgValue(parsedArgs, 3, "beneficiary"),
      "PayoutConfigUpdated.beneficiary"
    );

    const id = makeLeaseConfigId({ chainId, contractAddress, leaseId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3LeasePayoutConfig)
        .values({
          id,
          chainId,
          contractAddress,
          leaseId,
          targetChainId,
          targetToken,
          beneficiary,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          targetChainId,
          targetToken,
          beneficiary,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleSwapRateSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const targetToken = expectHexAddress(
      getArgValue(parsedArgs, 0, "targetToken"),
      "SwapRateSet.targetToken"
    );
    const ratePpm = expectBigint(getArgValue(parsedArgs, 1, "ratePpm"), "SwapRateSet.ratePpm");

    const id = makeTokenStateId({ chainId, contractAddress, targetToken });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3SwapRate)
        .values({
          id,
          chainId,
          contractAddress,
          targetToken,
          ratePpm,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          ratePpm,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleBridgerSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const targetToken = expectHexAddress(
      getArgValue(parsedArgs, 0, "targetToken"),
      "BridgerSet.targetToken"
    );
    const targetChainId = expectBigint(
      getArgValue(parsedArgs, 1, "targetChainId"),
      "BridgerSet.targetChainId"
    );
    const bridger = expectHexAddress(getArgValue(parsedArgs, 2, "bridger"), "BridgerSet.bridger");

    const id = makeBridgerRouteId({ chainId, contractAddress, targetToken, targetChainId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3BridgerRoute)
        .values({
          id,
          chainId,
          contractAddress,
          targetToken,
          targetChainId,
          bridger,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          bridger,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleClaimCreated = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const claimIndex = expectBigint(
      getArgValue(parsedArgs, 0, "claimIndex"),
      "ClaimCreated.claimIndex"
    );
    const leaseId = expectBigint(getArgValue(parsedArgs, 1, "leaseId"), "ClaimCreated.leaseId");
    const amountUsdt = expectBigint(
      getArgValue(parsedArgs, 2, "amountUsdt"),
      "ClaimCreated.amountUsdt"
    );

    const leaseConfigId = makeLeaseConfigId({ chainId, contractAddress, leaseId });
    const leaseConfig = yield* tryPromise(() =>
      context.db.find(untronV3LeasePayoutConfig, { id: leaseConfigId })
    );
    if (!leaseConfig) {
      return yield* Effect.fail(
        new Error(`Missing lease payout config for leaseId=${leaseId.toString()}`)
      );
    }

    const targetToken = expectHexAddress(leaseConfig.targetToken, "leaseConfig.targetToken");
    const targetChainId = expectBigint(leaseConfig.targetChainId, "leaseConfig.targetChainId");
    const beneficiary = expectHexAddress(leaseConfig.beneficiary, "leaseConfig.beneficiary");

    const claimId = makeClaimId({ chainId, contractAddress, targetToken, claimIndex });
    yield* tryPromise(() =>
      context.db
        .insert(untronV3Claim)
        .values({
          id: claimId,
          chainId,
          contractAddress,
          targetToken,
          claimIndex,
          leaseId,
          amountUsdt,
          targetChainId,
          beneficiary,
          createdAtBlockNumber: event.block.number,
          createdAtBlockTimestamp: event.block.timestamp,
          createdAtTransactionHash: event.transaction.hash,
          createdAtLogIndex: event.log.logIndex,
          isFilled: false,
          filledAtBlockNumber: null,
          filledAtBlockTimestamp: null,
          filledAtTransactionHash: null,
          filledAtLogIndex: null,
        })
        .onConflictDoNothing()
    );

    const queueId = makeTokenStateId({ chainId, contractAddress, targetToken });

    const existing = yield* tryPromise(() => context.db.find(untronV3ClaimQueue, { id: queueId }));

    const nextMaxClaimIndex =
      existing && existing.maxClaimIndex > claimIndex ? existing.maxClaimIndex : claimIndex;
    const queueLength = nextMaxClaimIndex + 1n;

    yield* tryPromise(() =>
      context.db
        .insert(untronV3ClaimQueue)
        .values({
          id: queueId,
          chainId,
          contractAddress,
          targetToken,
          maxClaimIndex: nextMaxClaimIndex,
          queueLength,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          maxClaimIndex: nextMaxClaimIndex,
          queueLength,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleClaimFilled = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const claimIndex = expectBigint(
      getArgValue(parsedArgs, 0, "claimIndex"),
      "ClaimFilled.claimIndex"
    );
    const leaseId = expectBigint(getArgValue(parsedArgs, 1, "leaseId"), "ClaimFilled.leaseId");
    const amountUsdt = expectBigint(
      getArgValue(parsedArgs, 2, "amountUsdt"),
      "ClaimFilled.amountUsdt"
    );

    const leaseConfigId = makeLeaseConfigId({ chainId, contractAddress, leaseId });
    const leaseConfig = yield* tryPromise(() =>
      context.db.find(untronV3LeasePayoutConfig, { id: leaseConfigId })
    );
    if (!leaseConfig) {
      return yield* Effect.fail(
        new Error(`Missing lease payout config for leaseId=${leaseId.toString()}`)
      );
    }

    const targetToken = expectHexAddress(leaseConfig.targetToken, "leaseConfig.targetToken");
    const targetChainId = expectBigint(leaseConfig.targetChainId, "leaseConfig.targetChainId");
    const beneficiary = expectHexAddress(leaseConfig.beneficiary, "leaseConfig.beneficiary");

    const claimId = makeClaimId({ chainId, contractAddress, targetToken, claimIndex });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3Claim)
        .values({
          id: claimId,
          chainId,
          contractAddress,
          targetToken,
          claimIndex,
          leaseId,
          amountUsdt,
          targetChainId,
          beneficiary,
          createdAtBlockNumber: event.block.number,
          createdAtBlockTimestamp: event.block.timestamp,
          createdAtTransactionHash: event.transaction.hash,
          createdAtLogIndex: event.log.logIndex,
          isFilled: true,
          filledAtBlockNumber: event.block.number,
          filledAtBlockTimestamp: event.block.timestamp,
          filledAtTransactionHash: event.transaction.hash,
          filledAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          isFilled: true,
          filledAtBlockNumber: event.block.number,
          filledAtBlockTimestamp: event.block.timestamp,
          filledAtTransactionHash: event.transaction.hash,
          filledAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleDepositPreEntitled = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const txId = expectHex(getArgValue(parsedArgs, 0, "txId"), "DepositPreEntitled.txId");
    const leaseId = expectBigint(
      getArgValue(parsedArgs, 1, "leaseId"),
      "DepositPreEntitled.leaseId"
    );
    const rawAmount = expectBigint(
      getArgValue(parsedArgs, 2, "rawAmount"),
      "DepositPreEntitled.rawAmount"
    );
    const netOut = expectBigint(getArgValue(parsedArgs, 3, "netOut"), "DepositPreEntitled.netOut");

    const id = makeDepositPreEntitledId({ chainId, contractAddress, txId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3DepositPreEntitled)
        .values({
          id,
          chainId,
          contractAddress,
          txId,
          leaseId,
          rawAmount,
          netOut,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          leaseId,
          rawAmount,
          netOut,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleTronUsdtSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const tronUsdt = expectHexAddress(
      getArgValue(parsedArgs, 0, "tronUsdt"),
      "TronUsdtSet.tronUsdt"
    );
    const id = makeTronUsdtId({ chainId, contractAddress });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3TronUsdt)
        .values({
          id,
          chainId,
          contractAddress,
          tronUsdt,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          tronUsdt,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleProtocolFloorSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const floorPpm = expectBigint(
      getArgValue(parsedArgs, 0, "floorPpm"),
      "ProtocolFloorSet.floorPpm"
    );

    const id = makeSingletonConfigId({ chainId, contractAddress });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3ProtocolFloor)
        .values({
          id,
          chainId,
          contractAddress,
          floorPpm,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          floorPpm,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleChainDeprecatedSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const targetChainId = expectBigint(
      getArgValue(parsedArgs, 0, "targetChainId"),
      "ChainDeprecatedSet.targetChainId"
    );
    const deprecated = expectBoolean(
      getArgValue(parsedArgs, 1, "deprecated"),
      "ChainDeprecatedSet.deprecated"
    );

    const id = makeChainDeprecatedId({ chainId, contractAddress, targetChainId });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3ChainDeprecated)
        .values({
          id,
          chainId,
          contractAddress,
          targetChainId,
          deprecated,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          deprecated,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleControllerEventChainTipUpdated = (args: {
  event: PonderLogEvent;
  context: PonderContext;
}) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const id = makeControllerEventQueueId({ chainId, contractAddress });

    yield* tryPromise(() =>
      context.db.sql.execute(sql`
        INSERT INTO "untron_v3_controller_event_queue" (
          id,
          chain_id,
          contract_address,
          enqueued_count,
          processed_count,
          updated_at_block_number,
          updated_at_block_timestamp,
          updated_at_transaction_hash,
          updated_at_log_index
        )
        VALUES (
          ${id},
          ${chainId},
          ${contractAddress},
          1,
          0,
          ${event.block.number},
          ${event.block.timestamp},
          ${event.transaction.hash},
          ${event.log.logIndex}
        )
        ON CONFLICT (id) DO UPDATE SET
          enqueued_count = "untron_v3_controller_event_queue".enqueued_count + 1,
          updated_at_block_number = EXCLUDED.updated_at_block_number,
          updated_at_block_timestamp = EXCLUDED.updated_at_block_timestamp,
          updated_at_transaction_hash = EXCLUDED.updated_at_transaction_hash,
          updated_at_log_index = EXCLUDED.updated_at_log_index;
      `)
    );
  });

const handleControllerEventProcessed = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const eventIndex = expectBigint(
      getArgValue(parsedArgs, 0, "eventIndex"),
      "ControllerEventProcessed.eventIndex"
    );
    const tronBlockNumber = expectBigint(
      getArgValue(parsedArgs, 1, "blockNumber"),
      "ControllerEventProcessed.blockNumber"
    );
    const tronBlockTimestamp = expectBigint(
      getArgValue(parsedArgs, 2, "blockTimestamp"),
      "ControllerEventProcessed.blockTimestamp"
    );
    const eventSignature = expectHex(
      getArgValue(parsedArgs, 3, "eventSignature"),
      "ControllerEventProcessed.eventSignature"
    ).toLowerCase() as Hex;
    const abiEncodedEventData = expectHex(
      getArgValue(parsedArgs, 4, "abiEncodedEventData"),
      "ControllerEventProcessed.abiEncodedEventData"
    ).toLowerCase() as Hex;

    const processedCount = eventIndex + 1n;
    const id = makeControllerEventQueueId({ chainId, contractAddress });

    if (eventSignature === EVENT_SIG_PULLED_FROM_RECEIVER) {
      const [receiverSalt, tokenAddress] = decodeAbiParameters(
        [
          { type: "bytes32" },
          { type: "address" },
          { type: "uint256" },
          { type: "uint256" },
          { type: "uint256" },
        ],
        abiEncodedEventData
      ) as readonly [Hex, Hex, unknown, unknown, unknown];

      const receiverId = makeLastReceiverPullId({
        chainId,
        contractAddress,
        receiverSalt,
        tokenAddress,
      });

      const existing = yield* tryPromise(() =>
        context.db.find(untronV3LastReceiverPull, { id: receiverId })
      );
      const existingTs =
        existing?.lastPullTronBlockTimestamp != null
          ? expectBigint(existing.lastPullTronBlockTimestamp, "lastPull.lastPullTronBlockTimestamp")
          : null;

      // Only move forward (onchain state is monotonic; avoid regressing on reorgs/out-of-order indexing).
      if (existingTs === null || tronBlockTimestamp > existingTs) {
        yield* tryPromise(() =>
          context.db
            .insert(untronV3LastReceiverPull)
            .values({
              id: receiverId,
              chainId,
              contractAddress,
              receiverSalt,
              tokenAddress,
              lastPullTronBlockNumber: tronBlockNumber,
              lastPullTronBlockTimestamp: tronBlockTimestamp,
              updatedAtBlockNumber: event.block.number,
              updatedAtBlockTimestamp: event.block.timestamp,
              updatedAtTransactionHash: event.transaction.hash,
              updatedAtLogIndex: event.log.logIndex,
            })
            .onConflictDoUpdate({
              receiverSalt,
              tokenAddress,
              lastPullTronBlockNumber: tronBlockNumber,
              lastPullTronBlockTimestamp: tronBlockTimestamp,
              updatedAtBlockNumber: event.block.number,
              updatedAtBlockTimestamp: event.block.timestamp,
              updatedAtTransactionHash: event.transaction.hash,
              updatedAtLogIndex: event.log.logIndex,
            })
        );
      }
    }

    yield* tryPromise(() =>
      context.db.sql.execute(sql`
        INSERT INTO "untron_v3_controller_event_queue" (
          id,
          chain_id,
          contract_address,
          enqueued_count,
          processed_count,
          updated_at_block_number,
          updated_at_block_timestamp,
          updated_at_transaction_hash,
          updated_at_log_index
        )
        VALUES (
          ${id},
          ${chainId},
          ${contractAddress},
          ${processedCount},
          ${processedCount},
          ${event.block.number},
          ${event.block.timestamp},
          ${event.transaction.hash},
          ${event.log.logIndex}
        )
        ON CONFLICT (id) DO UPDATE SET
          processed_count = GREATEST("untron_v3_controller_event_queue".processed_count, ${processedCount}),
          enqueued_count = GREATEST("untron_v3_controller_event_queue".enqueued_count, ${processedCount}),
          updated_at_block_number = EXCLUDED.updated_at_block_number,
          updated_at_block_timestamp = EXCLUDED.updated_at_block_timestamp,
          updated_at_transaction_hash = EXCLUDED.updated_at_transaction_hash,
          updated_at_log_index = EXCLUDED.updated_at_log_index;
      `)
    );
  });

const DEFAULT_REALTOR_ROW = {
  allowed: false,
  minFeePpm: 0n,
  leaseRateLimitMode: 0,
  leaseRateLimitMaxLeases: 0n,
  leaseRateLimitWindowSeconds: 0n,
} as const;

const handleRealtorSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const realtor = expectHexAddress(getArgValue(parsedArgs, 0, "realtor"), "RealtorSet.realtor");
    const allowed = expectBoolean(getArgValue(parsedArgs, 1, "allowed"), "RealtorSet.allowed");

    const id = makeRealtorId({ chainId, contractAddress, realtor });
    const existing = yield* tryPromise(() => context.db.find(untronV3Realtor, { id }));

    const previous = existing
      ? {
          allowed: expectBoolean(existing.allowed, "realtor.allowed"),
          minFeePpm: expectBigint(existing.minFeePpm, "realtor.minFeePpm"),
          leaseRateLimitMode: Number(existing.leaseRateLimitMode),
          leaseRateLimitMaxLeases: expectBigint(
            existing.leaseRateLimitMaxLeases,
            "realtor.leaseRateLimitMaxLeases"
          ),
          leaseRateLimitWindowSeconds: expectBigint(
            existing.leaseRateLimitWindowSeconds,
            "realtor.leaseRateLimitWindowSeconds"
          ),
        }
      : DEFAULT_REALTOR_ROW;

    yield* tryPromise(() =>
      context.db
        .insert(untronV3Realtor)
        .values({
          id,
          chainId,
          contractAddress,
          realtor,
          allowed,
          minFeePpm: previous.minFeePpm,
          leaseRateLimitMode: previous.leaseRateLimitMode,
          leaseRateLimitMaxLeases: previous.leaseRateLimitMaxLeases,
          leaseRateLimitWindowSeconds: previous.leaseRateLimitWindowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          allowed,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleRealtorMinFeeSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const realtor = expectHexAddress(
      getArgValue(parsedArgs, 0, "realtor"),
      "RealtorMinFeeSet.realtor"
    );
    const minFeePpm = expectBigint(
      getArgValue(parsedArgs, 1, "minFeePpm"),
      "RealtorMinFeeSet.minFeePpm"
    );

    const id = makeRealtorId({ chainId, contractAddress, realtor });
    const existing = yield* tryPromise(() => context.db.find(untronV3Realtor, { id }));

    const previous = existing
      ? {
          allowed: expectBoolean(existing.allowed, "realtor.allowed"),
          leaseRateLimitMode: Number(existing.leaseRateLimitMode),
          leaseRateLimitMaxLeases: expectBigint(
            existing.leaseRateLimitMaxLeases,
            "realtor.leaseRateLimitMaxLeases"
          ),
          leaseRateLimitWindowSeconds: expectBigint(
            existing.leaseRateLimitWindowSeconds,
            "realtor.leaseRateLimitWindowSeconds"
          ),
        }
      : {
          allowed: DEFAULT_REALTOR_ROW.allowed,
          leaseRateLimitMode: DEFAULT_REALTOR_ROW.leaseRateLimitMode,
          leaseRateLimitMaxLeases: DEFAULT_REALTOR_ROW.leaseRateLimitMaxLeases,
          leaseRateLimitWindowSeconds: DEFAULT_REALTOR_ROW.leaseRateLimitWindowSeconds,
        };

    yield* tryPromise(() =>
      context.db
        .insert(untronV3Realtor)
        .values({
          id,
          chainId,
          contractAddress,
          realtor,
          allowed: previous.allowed,
          minFeePpm,
          leaseRateLimitMode: previous.leaseRateLimitMode,
          leaseRateLimitMaxLeases: previous.leaseRateLimitMaxLeases,
          leaseRateLimitWindowSeconds: previous.leaseRateLimitWindowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          minFeePpm,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleRealtorLeaseRateLimitSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const realtor = expectHexAddress(
      getArgValue(parsedArgs, 0, "realtor"),
      "RealtorLeaseRateLimitSet.realtor"
    );
    const mode = expectBigint(getArgValue(parsedArgs, 1, "mode"), "RealtorLeaseRateLimitSet.mode");
    const maxLeases = expectBigint(
      getArgValue(parsedArgs, 2, "maxLeases"),
      "RealtorLeaseRateLimitSet.maxLeases"
    );
    const windowSeconds = expectBigint(
      getArgValue(parsedArgs, 3, "windowSeconds"),
      "RealtorLeaseRateLimitSet.windowSeconds"
    );

    const id = makeRealtorId({ chainId, contractAddress, realtor });
    const existing = yield* tryPromise(() => context.db.find(untronV3Realtor, { id }));

    const previous = existing
      ? {
          allowed: expectBoolean(existing.allowed, "realtor.allowed"),
          minFeePpm: expectBigint(existing.minFeePpm, "realtor.minFeePpm"),
        }
      : { allowed: DEFAULT_REALTOR_ROW.allowed, minFeePpm: DEFAULT_REALTOR_ROW.minFeePpm };

    yield* tryPromise(() =>
      context.db
        .insert(untronV3Realtor)
        .values({
          id,
          chainId,
          contractAddress,
          realtor,
          allowed: previous.allowed,
          minFeePpm: previous.minFeePpm,
          leaseRateLimitMode: Number(mode),
          leaseRateLimitMaxLeases: maxLeases,
          leaseRateLimitWindowSeconds: windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          leaseRateLimitMode: Number(mode),
          leaseRateLimitMaxLeases: maxLeases,
          leaseRateLimitWindowSeconds: windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleProtocolLeaseRateLimitSet = (args: { event: PonderLogEvent; context: PonderContext }) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const maxLeases = expectBigint(
      getArgValue(parsedArgs, 0, "maxLeases"),
      "ProtocolLeaseRateLimitSet.maxLeases"
    );
    const windowSeconds = expectBigint(
      getArgValue(parsedArgs, 1, "windowSeconds"),
      "ProtocolLeaseRateLimitSet.windowSeconds"
    );

    const id = makeSingletonConfigId({ chainId, contractAddress });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3ProtocolLeaseRateLimit)
        .values({
          id,
          chainId,
          contractAddress,
          maxLeases,
          windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          maxLeases,
          windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

const handleLesseePayoutConfigRateLimitSet = (args: {
  event: PonderLogEvent;
  context: PonderContext;
}) =>
  Effect.gen(function* () {
    const { event, context } = args;
    const parsedArgs = expectRecord(event.args, "event.args");
    const chainId = context.chain.id;
    const contractAddress = expectHexAddress(event.log.address, "event.log.address");

    const maxUpdates = expectBigint(
      getArgValue(parsedArgs, 0, "maxUpdates"),
      "LesseePayoutConfigRateLimitSet.maxUpdates"
    );
    const windowSeconds = expectBigint(
      getArgValue(parsedArgs, 1, "windowSeconds"),
      "LesseePayoutConfigRateLimitSet.windowSeconds"
    );

    const id = makeSingletonConfigId({ chainId, contractAddress });

    yield* tryPromise(() =>
      context.db
        .insert(untronV3LesseePayoutConfigRateLimit)
        .values({
          id,
          chainId,
          contractAddress,
          maxUpdates,
          windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
        .onConflictDoUpdate({
          maxUpdates,
          windowSeconds,
          updatedAtBlockNumber: event.block.number,
          updatedAtBlockTimestamp: event.block.timestamp,
          updatedAtTransactionHash: event.transaction.hash,
          updatedAtLogIndex: event.log.logIndex,
        })
    );
  });

export const handleUntronV3DerivedEvent = (args: {
  eventName: UntronV3DerivedEventName | (string & {});
  event: PonderLogEvent;
  context: PonderContext;
}): Effect.Effect<void, Error> => {
  switch (args.eventName) {
    case "LeaseCreated":
      return handleLeaseCreated({ event: args.event, context: args.context });
    case "LeaseNonceUpdated":
      return handleLeaseNonceUpdated({ event: args.event, context: args.context });
    case "PayoutConfigUpdated":
      return handlePayoutConfigUpdated({ event: args.event, context: args.context });
    case "LesseePayoutConfigRateLimitSet":
      return handleLesseePayoutConfigRateLimitSet({ event: args.event, context: args.context });
    case "SwapRateSet":
      return handleSwapRateSet({ event: args.event, context: args.context });
    case "BridgerSet":
      return handleBridgerSet({ event: args.event, context: args.context });
    case "ClaimCreated":
      return handleClaimCreated({ event: args.event, context: args.context });
    case "ClaimFilled":
      return handleClaimFilled({ event: args.event, context: args.context });
    case "DepositPreEntitled":
      return handleDepositPreEntitled({ event: args.event, context: args.context });
    case "TronUsdtSet":
      return handleTronUsdtSet({ event: args.event, context: args.context });
    case "ProtocolFloorSet":
      return handleProtocolFloorSet({ event: args.event, context: args.context });
    case "ChainDeprecatedSet":
      return handleChainDeprecatedSet({ event: args.event, context: args.context });
    case "ProtocolLeaseRateLimitSet":
      return handleProtocolLeaseRateLimitSet({ event: args.event, context: args.context });
    case "RealtorLeaseRateLimitSet":
      return handleRealtorLeaseRateLimitSet({ event: args.event, context: args.context });
    case "RealtorMinFeeSet":
      return handleRealtorMinFeeSet({ event: args.event, context: args.context });
    case "RealtorSet":
      return handleRealtorSet({ event: args.event, context: args.context });
    case "ControllerEventChainTipUpdated":
      return handleControllerEventChainTipUpdated({ event: args.event, context: args.context });
    case "ControllerEventProcessed":
      return handleControllerEventProcessed({ event: args.event, context: args.context });
    default:
      return Effect.void;
  }
};
