import { Effect } from "effect";

import type { Return } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import {
  TransactionInfo_code,
  Transaction_raw,
  type TransactionInfo,
  type Transaction,
} from "@untron/tron-protocol/tron";

import type { UnaryCall } from "./grpcHelpers";
import { signTronTransaction } from "./protocol";

export const getTxRefBlockNumber = (tx: Transaction): bigint => {
  const rawData = tx.rawData;
  if (!rawData) throw new Error("Tron tx missing rawData");
  return BigInt(rawData.refBlockNum.toString());
};

export const getTxTimestamp = (tx: Transaction): bigint => {
  const rawData = tx.rawData;
  if (!rawData) throw new Error("Tron tx missing rawData");
  return BigInt(rawData.timestamp.toString());
};

export const setTxTriggerSmartContractData = (args: { tx: Transaction; data: `0x${string}` }) => {
  const rawData = args.tx.rawData;
  if (!rawData) throw new Error("Tron tx missing rawData");

  const contract = rawData.contract?.[0];
  if (!contract?.parameter?.value?.length) {
    throw new Error("Tron tx missing contract parameter value");
  }

  const trigger = TriggerSmartContract.decode(contract.parameter.value);
  trigger.data = Buffer.from(args.data.slice(2), "hex");
  contract.parameter.value = Buffer.from(TriggerSmartContract.encode(trigger).finish());
};

export const waitForTronTransaction = <E>(args: {
  txid: string;
  pollTimes: number;
  pollIntervalMs: number;
  getTransactionInfoById: (txidHex: string) => Effect.Effect<TransactionInfo | null, E>;
}): Effect.Effect<void, E | Error> =>
  Effect.gen(function* () {
    for (let i = 0; i < args.pollTimes; i++) {
      const info = yield* args.getTransactionInfoById(args.txid);
      if (!info) {
        yield* Effect.sleep(args.pollIntervalMs);
        continue;
      }

      if (info.result === TransactionInfo_code.FAILED) {
        const message = info.resMessage?.length
          ? info.resMessage.toString("utf8")
          : "Tron transaction execution failed";
        return yield* Effect.fail(new Error(message));
      }

      return;
    }

    return yield* Effect.fail(new Error(`Timed out waiting for Tron tx receipt: ${args.txid}`));
  });

export const broadcastTronTx = <E>(args: {
  tx: Transaction;
  feeLimit: number;
  privateKeyRaw: string;
  pollTimes: number;
  pollIntervalMs: number;
  grpcUnary: <Req, Res>(call: UnaryCall<Req, Res>, req: Req) => Effect.Effect<Res, E>;
  wallet: any;
  getTransactionInfoById: (txidHex: string) => Effect.Effect<TransactionInfo | null, E>;
}): Effect.Effect<string, E | Error> =>
  Effect.gen(function* () {
    if (!args.tx.rawData) return yield* Effect.fail(new Error("Tron tx missing rawData"));
    args.tx.rawData = Transaction_raw.fromPartial({
      ...args.tx.rawData,
      feeLimit: args.feeLimit,
    });

    const { txidHex, signed } = signTronTransaction(args.tx, args.privateKeyRaw);

    const broadcast = yield* args.grpcUnary(
      args.wallet.broadcastTransaction.bind(args.wallet) as unknown as UnaryCall<
        Transaction,
        Return
      >,
      signed
    );

    if (!broadcast.result) {
      const msg = broadcast.message?.length ? broadcast.message.toString("utf8") : "unknown";
      return yield* Effect.fail(new Error(`Tron broadcast rejected: ${msg}`));
    }

    yield* waitForTronTransaction({
      txid: txidHex,
      pollTimes: args.pollTimes,
      pollIntervalMs: args.pollIntervalMs,
      getTransactionInfoById: args.getTransactionInfoById,
    });

    return txidHex;
  }).pipe(Effect.withLogSpan("tron.tx"));
