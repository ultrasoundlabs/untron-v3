import { Effect } from "effect";
import { decodeAbiParameters, type Hex } from "viem";

import type { Return } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import {
  TransactionInfo_code,
  Transaction_raw,
  type TransactionInfo,
  type Transaction,
} from "@untron/tron-protocol/tron";

import type { UnaryCall } from "./grpcClient";
import { signTronTransaction } from "./protocol";

const bufferToHex = (buf: Buffer): Hex => `0x${buf.toString("hex")}` as Hex;

const decodeEvmRevertData = (data: Hex): string | null => {
  if (data === "0x") return null;

  const selector = data.slice(0, 10).toLowerCase();

  // EVM standard errors.
  if (selector === "0x08c379a0") {
    // Error(string)
    try {
      const [reason] = decodeAbiParameters([{ type: "string" }], `0x${data.slice(10)}` as Hex);
      return typeof reason === "string" ? `Error(${JSON.stringify(reason)})` : "Error(<unknown>)";
    } catch {
      return "Error(<decode failed>)";
    }
  }
  if (selector === "0x4e487b71") {
    // Panic(uint256)
    try {
      const [code] = decodeAbiParameters([{ type: "uint256" }], `0x${data.slice(10)}` as Hex);
      return typeof code === "bigint" ? `Panic(${code})` : "Panic(<unknown>)";
    } catch {
      return "Panic(<decode failed>)";
    }
  }

  // Solady SafeTransferLib custom errors (common in our Tron controller path).
  switch (selector) {
    case "0xb12d13eb":
      return "SafeTransferLib.ETHTransferFailed()";
    case "0x7939f424":
      return "SafeTransferLib.TransferFromFailed()";
    case "0x90b8ec18":
      return "SafeTransferLib.TransferFailed()";
    case "0x3e3f8f73":
      return "SafeTransferLib.ApproveFailed()";
    case "0x54cd9435":
      return "SafeTransferLib.TotalSupplyQueryFailed()";
    default:
      return null;
  }
};

const formatTronTxFailure = (info: TransactionInfo): string => {
  const resMessage = info.resMessage?.length
    ? info.resMessage.toString("utf8")
    : "Tron transaction execution failed";

  const contractResult0 = info.contractResult?.[0];
  if (!contractResult0?.length) return resMessage;

  const revertData = bufferToHex(contractResult0);
  const decoded = decodeEvmRevertData(revertData);

  if (decoded) return `${resMessage} (${decoded})`;

  const dataLenBytes = (revertData.length - 2) / 2;
  return `${resMessage} (revertData=${revertData.slice(0, 10)}, len=${dataLenBytes} bytes)`;
};

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
        return yield* Effect.fail(new Error(`${formatTronTxFailure(info)} (txid=${args.txid})`));
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
