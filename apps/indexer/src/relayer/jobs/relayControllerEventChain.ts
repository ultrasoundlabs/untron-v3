import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import { decodeFunctionData, encodeFunctionData, type Address, type Hex } from "viem";

import type { BlockExtention } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import type { Transaction } from "@untron/tron-protocol/tron";

import { untronControllerAbi, untronV3Abi } from "@untron/v3-contracts";
import { computeNextEventChainTip } from "../../eventChain/tip";
import { tryPromise } from "../../effect/tryPromise";
import { MAINNET_CHAIN_ID } from "../../env";
import type { RelayJobRow } from "../types";
import { getRows } from "../sqlRows";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc, fetchTronBlockByNum } from "../deps/tronGrpc";
import { computeTronTxMerkleProof } from "../tronProofs";
import { RetryLaterError } from "../errors";
import { tronLightClientCheckpoint } from "ponder:schema";

import {
  expectAddress,
  expectBigint,
  expectHex,
  expectRecord,
  type RelayJobHandlerContext,
} from "./types";

function hasIsEventChainTipCallInMulticall(calls: readonly Hex[], selectorIsEventChainTip: Hex) {
  for (const call of calls) {
    if (call.length !== 2 + (4 + 32) * 2) continue; // selector + one bytes32 arg
    if (call.slice(0, 10).toLowerCase() === selectorIsEventChainTip.toLowerCase()) return true;
  }
  return false;
}

function findTransactionInBlock(args: { block: BlockExtention; txid: Hex }): Transaction {
  const txidHex = args.txid.replace(/^0x/i, "").toLowerCase();
  const txs = args.block.transactions ?? [];

  for (const txExt of txs) {
    const got = Buffer.from(txExt?.txid ?? Buffer.alloc(0))
      .toString("hex")
      .toLowerCase();
    if (got !== txidHex) continue;
    const tx = txExt?.transaction;
    if (!tx) throw new Error("Transaction extension missing transaction");
    return tx;
  }

  throw new Error("Transaction not found in block");
}

function parseTriggerSmartContract(tx: Transaction): { toEvm: Address; data: Hex } {
  const rawData = tx.rawData;
  if (!rawData) throw new Error("Tron tx missing rawData");
  const contract = rawData.contract?.[0];
  if (!contract?.parameter?.value?.length) throw new Error("Tron tx missing contract parameter");

  const trigger = TriggerSmartContract.decode(contract.parameter.value);
  const contractAddressBytes21 = trigger.contractAddress as Buffer | undefined;
  if (
    !contractAddressBytes21 ||
    contractAddressBytes21.length !== 21 ||
    contractAddressBytes21[0] !== 0x41
  ) {
    throw new Error("Tron tx TriggerSmartContract.contractAddress is not a canonical Tron address");
  }

  const dataBuf = trigger.data as Buffer | undefined;
  const data = dataBuf?.length ? (`0x${dataBuf.toString("hex")}` as Hex) : ("0x" as Hex);

  return {
    toEvm: (
      `0x${contractAddressBytes21.subarray(1).toString("hex")}` as Address
    ).toLowerCase() as Address,
    data,
  };
}

async function loadControllerEventsToRelay(args: {
  context: PonderContext;
  tronChainId: number;
  controllerAddress: Address;
  startTip: Hex;
  endTip: Hex;
}): Promise<
  Array<{
    sig: Hex;
    data: Hex;
    blockNumber: bigint;
    blockTimestamp: bigint;
    tip: Hex;
    previousTip: Hex;
  }>
> {
  const endTipLower = args.endTip.toLowerCase() as Hex;
  const startTipLower = args.startTip.toLowerCase() as Hex;

  const endRowResult = await args.context.db.sql.execute(sql`
    SELECT
      sequence AS "sequence"
    FROM "event_chain_event"
    WHERE chain_id = ${args.tronChainId}
      AND contract_name = 'UntronController'
      AND contract_address = ${args.controllerAddress}
      AND tip = ${endTipLower}
    LIMIT 1;
  `);
  const endRows = getRows(endRowResult) as Array<{ sequence: bigint }>;
  const endSequence = endRows[0]?.sequence;
  if (typeof endSequence !== "bigint") {
    throw new Error(`Missing UntronController event chain row for end tip ${endTipLower}`);
  }

  let startSequence = 0n;
  if (startTipLower !== endTipLower) {
    const startRowResult = await args.context.db.sql.execute(sql`
      SELECT
        sequence AS "sequence"
      FROM "event_chain_event"
      WHERE chain_id = ${args.tronChainId}
        AND contract_name = 'UntronController'
        AND contract_address = ${args.controllerAddress}
        AND tip = ${startTipLower}
      LIMIT 1;
    `);
    const startRows = getRows(startRowResult) as Array<{ sequence: bigint }>;
    startSequence = startRows[0]?.sequence ?? 0n;
  }

  const eventsResult = await args.context.db.sql.execute(sql`
    SELECT
      tip AS "tip",
      previous_tip AS "previousTip",
      block_number AS "blockNumber",
      block_timestamp AS "blockTimestamp",
      event_signature AS "eventSignature",
      encoded_event_data AS "encodedEventData",
      sequence AS "sequence"
    FROM "event_chain_event"
    WHERE chain_id = ${args.tronChainId}
      AND contract_name = 'UntronController'
      AND contract_address = ${args.controllerAddress}
      AND sequence > ${startSequence}
      AND sequence <= ${endSequence}
    ORDER BY sequence ASC;
  `);

  const rows = getRows(eventsResult) as Array<{
    tip: Hex;
    previousTip: Hex;
    blockNumber: bigint;
    blockTimestamp: bigint;
    eventSignature: Hex;
    encodedEventData: Hex;
    sequence: bigint;
  }>;

  return rows.map((row) => ({
    sig: row.eventSignature,
    data: row.encodedEventData,
    blockNumber: row.blockNumber,
    blockTimestamp: row.blockTimestamp,
    tip: row.tip,
    previousTip: row.previousTip,
  }));
}

export const handleRelayControllerEventChain = ({
  ctx,
  job,
}: {
  job: RelayJobRow & { kind: "relay_controller_event_chain" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const payload = expectRecord(job.payloadJson, "payloadJson");
    const controllerAddress = expectAddress(payload.controllerAddress, "payload.controllerAddress");
    const tronBlockNumber = expectBigint(payload.tronBlockNumber, "payload.tronBlockNumber");
    const transactionHash = expectHex(payload.transactionHash, "payload.transactionHash");
    const eventChainTip = expectHex(payload.eventChainTip, "payload.eventChainTip");

    yield* Effect.logDebug("[relay_controller_event_chain] handle").pipe(
      Effect.annotateLogs({
        controllerAddress,
        tronBlockNumber: tronBlockNumber.toString(),
        transactionHash,
        eventChainTip,
      })
    );

    const publicClients = yield* PublicClients;
    const tronGrpc = yield* TronGrpc;

    const mainnetClient = yield* publicClients.get("mainnet");

    const tronLightClientAddress = (
      ctx.ponderContext.contracts.TronLightClient.address as Address
    ).toLowerCase() as Address;
    const untronV3Address = (
      ctx.ponderContext.contracts.UntronV3.address as Address
    ).toLowerCase() as Address;

    yield* Effect.logInfo("[relay_controller_event_chain] load lastControllerEventTip");
    const lastControllerEventTip = (yield* tryPromise(() =>
      mainnetClient.readContract({
        address: untronV3Address,
        abi: untronV3Abi,
        functionName: "lastControllerEventTip",
      })
    )) as Hex;

    if (lastControllerEventTip.toLowerCase() === eventChainTip.toLowerCase()) {
      yield* Effect.logDebug("[relay_controller_event_chain] already relayed").pipe(
        Effect.annotateLogs({ lastControllerEventTip })
      );
      return;
    }

    const { wallet, callOpts } = yield* tronGrpc.get();

    yield* Effect.logInfo("[relay_controller_event_chain] fetch tron block").pipe(
      Effect.annotateLogs({ tronBlockNumber: tronBlockNumber.toString() })
    );
    const block = yield* tryPromise(() =>
      fetchTronBlockByNum({
        wallet,
        metadata: callOpts.metadata,
        blockNumber: tronBlockNumber,
        timeoutMs: 15_000,
        retries: 2,
      })
    );
    const tx = findTransactionInBlock({ block, txid: transactionHash });

    const trigger = parseTriggerSmartContract(tx);
    if (trigger.toEvm.toLowerCase() !== controllerAddress.toLowerCase()) return;

    const selectorIsEventChainTip = encodeFunctionData({
      abi: untronControllerAbi,
      functionName: "isEventChainTip",
      args: [eventChainTip],
    }).slice(0, 10) as Hex;

    const selectorMulticall = encodeFunctionData({
      abi: untronControllerAbi,
      functionName: "multicall",
      args: [[] as Hex[]],
    }).slice(0, 10) as Hex;

    const topSelector = trigger.data.slice(0, 10).toLowerCase();
    if (topSelector === selectorIsEventChainTip.toLowerCase()) {
      // ok
    } else if (topSelector === selectorMulticall.toLowerCase()) {
      const decoded = decodeFunctionData({
        abi: untronControllerAbi,
        data: trigger.data,
      });
      if (decoded.functionName !== "multicall") return;
      const calls = (decoded.args?.[0] ?? []) as readonly Hex[];
      if (!hasIsEventChainTipCallInMulticall(calls, selectorIsEventChainTip)) return;
    } else {
      return;
    }

    const proof = computeTronTxMerkleProof({ block, txidHex: transactionHash });

    yield* Effect.logInfo("[relay_controller_event_chain] check tron block published").pipe(
      Effect.annotateLogs({ tronBlockNumber: tronBlockNumber.toString() })
    );

    const plannedCalls: Array<{ to: Address; data: Hex }> = [];

    const checkpoint = yield* tryPromise(() =>
      ctx.ponderContext.db.find(tronLightClientCheckpoint, {
        id: `${MAINNET_CHAIN_ID}:${tronLightClientAddress}:${tronBlockNumber.toString()}`,
      })
    );

    if (!checkpoint) {
      return yield* Effect.fail(
        new RetryLaterError(
          `Tron block ${tronBlockNumber.toString()} not yet published in TronLightClient`
        )
      );
    }

    yield* Effect.logInfo("[relay_controller_event_chain] load controller events").pipe(
      Effect.annotateLogs({ startTip: lastControllerEventTip, endTip: eventChainTip })
    );
    const controllerEvents = yield* tryPromise(() =>
      loadControllerEventsToRelay({
        context: ctx.ponderContext,
        tronChainId: job.chainId,
        controllerAddress,
        startTip: lastControllerEventTip,
        endTip: eventChainTip,
      })
    );

    let computed = lastControllerEventTip as Hex;
    for (const ev of controllerEvents) {
      computed = computeNextEventChainTip({
        previousTip: computed,
        blockNumber: ev.blockNumber,
        blockTimestamp: ev.blockTimestamp,
        eventSignature: ev.sig,
        encodedEventData: ev.data,
      });
    }

    if (computed.toLowerCase() !== eventChainTip.toLowerCase()) {
      return yield* Effect.fail(
        new Error(
          `Controller event chain mismatch (computed=${computed}, expected=${eventChainTip}). Indexer may be behind.`
        )
      );
    }

    const relayCall = encodeFunctionData({
      abi: untronV3Abi,
      functionName: "relayControllerEventChain",
      args: [
        tronBlockNumber,
        proof.encodedTx,
        [...proof.proof] as readonly Hex[],
        proof.index,
        controllerEvents.map((ev) => ({
          sig: ev.sig,
          data: ev.data,
          blockNumber: ev.blockNumber,
          blockTimestamp: ev.blockTimestamp,
        })),
      ],
    });

    plannedCalls.push({ to: untronV3Address, data: relayCall });

    yield* Effect.logInfo("[relay_controller_event_chain] send UserOperation").pipe(
      Effect.annotateLogs({ callCount: plannedCalls.length })
    );
    yield* MainnetRelayer.sendUserOperation({
      calls: plannedCalls,
    });
  });
