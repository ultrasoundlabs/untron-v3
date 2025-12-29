import { Effect } from "effect";
import { decodeAbiParameters, type Address, type Hex } from "viem";

import type { Return } from "@untron/tron-protocol/api";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import {
  TransactionInfo_code,
  Transaction_raw,
  type TransactionInfo,
  type Transaction,
} from "@untron/tron-protocol/tron";

import type { UnaryCall } from "./grpcClient";
import { signTronTransaction, tronBytes21ToBase58 } from "./protocol";

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

const tryDecodeErc20TransferCalldata = (data: string): { to: Address; value: bigint } | null => {
  const normalized = data.trim().toLowerCase();
  const hex = normalized.startsWith("0x") ? (normalized as Hex) : (`0x${normalized}` as Hex);
  if (!hex.startsWith("0xa9059cbb")) return null;
  if (hex.length < 10 + 64 + 64) return null;

  try {
    const [to, value] = decodeAbiParameters(
      [{ type: "address" }, { type: "uint256" }],
      `0x${hex.slice(10)}` as Hex
    );
    if (typeof to !== "string" || typeof value !== "bigint") return null;
    return { to: to as Address, value };
  } catch {
    return null;
  }
};

const summarizeRejectedInternalTx = (info: TransactionInfo): string | null => {
  const internalTxs = info.internalTransactions ?? [];
  if (internalTxs.length === 0) return null;

  const rejected = internalTxs.find((t) => t.rejected) ?? null;
  const tx = rejected ?? internalTxs[internalTxs.length - 1] ?? null;
  if (!tx) return null;

  let fromTo = null as string | null;
  try {
    const from = tronBytes21ToBase58(tx.callerAddress);
    const to = tronBytes21ToBase58(tx.transferToAddress);
    fromTo = `${from}->${to}`;
  } catch {
    // ignore
  }

  const note = tx.note?.length ? tx.note.toString("utf8") : "";
  const extra = tx.extra?.length ? tx.extra : "";

  const maybeTransferHex = (() => {
    if (!extra) return null;
    const match = extra.match(/0x?a9059cbb[0-9a-fA-F]{1,512}/);
    return match?.[0] ?? null;
  })();
  const decodedTransfer = maybeTransferHex
    ? tryDecodeErc20TransferCalldata(maybeTransferHex)
    : null;

  const parts: string[] = [];
  parts.push(`internalTxs=${internalTxs.length}`);
  if (tx.rejected) parts.push("rejected=true");
  if (fromTo) parts.push(fromTo);
  if (note) parts.push(`note=${JSON.stringify(note)}`);
  if (decodedTransfer)
    parts.push(`erc20.transfer(to=${decodedTransfer.to}, value=${decodedTransfer.value})`);

  // Keep logfmt-ish and short; extra is sometimes huge.
  if (extra && !decodedTransfer) parts.push(`extra=${JSON.stringify(extra.slice(0, 160))}`);

  return parts.join(" ");
};

const formatTronTxFailure = (info: TransactionInfo): string => {
  const resMessage = info.resMessage?.length
    ? info.resMessage.toString("utf8")
    : "Tron transaction execution failed";

  const internalSummary = summarizeRejectedInternalTx(info);

  const contractResult0 = info.contractResult?.[0];
  if (!contractResult0?.length) {
    return internalSummary ? `${resMessage} (${internalSummary})` : resMessage;
  }

  const revertData = bufferToHex(contractResult0);
  const decoded = decodeEvmRevertData(revertData);

  if (decoded) {
    const suffix = internalSummary ? `${decoded}; ${internalSummary}` : decoded;
    return `${resMessage} (${suffix})`;
  }

  const dataLenBytes = (revertData.length - 2) / 2;
  const suffix = `revertData=${revertData.slice(0, 10)}, len=${dataLenBytes} bytes`;
  return internalSummary
    ? `${resMessage} (${suffix}; ${internalSummary})`
    : `${resMessage} (${suffix})`;
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
