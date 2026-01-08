/**
 * Compare a relayer txTrieRoot mismatch dump (from Rust) against a fresh fetch via TRON gRPC.
 *
 * Usage:
 *   pnpm research compareRelayerTxTrieRootDump <dumpJsonPath> [blockNumber]
 *
 * The dump JSON is the object printed by `crates/tron/src/proof.rs` on mismatch.
 * This script:
 * - fetches the block via getBlockByNum2
 * - recomputes txTrieRoot from sha256(Transaction.encode(tx).finish()) leaves (carry-up rule)
 * - diffs per-tx leaf + encodedTx bytes against the dump (first mismatch details)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import Long from "long";
import { z } from "zod";
import { sha256 } from "@noble/hashes/sha2.js";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { createTronClients } from "@untron/tron-protocol";
import type { BlockExtention, NumberMessage } from "@untron/tron-protocol/api";
import { Transaction } from "@untron/tron-protocol/tron";

function toHex0x(bytes: Uint8Array | Buffer): string {
  return `0x${Buffer.from(bytes).toString("hex")}`;
}

function decodeHex0x(hex: string): Buffer {
  const cleaned = hex.replace(/^0x/i, "");
  return Buffer.from(cleaned, "hex");
}

function sha256Buf(bytes: Uint8Array | Buffer): Buffer {
  return Buffer.from(sha256(bytes));
}

function sha256Concat(a: Buffer, b: Buffer): Buffer {
  return sha256Buf(Buffer.concat([a, b]));
}

function computeMerkleRootCarryUp(leaves: Buffer[]): Buffer {
  if (leaves.length === 0) throw new Error("empty merkle tree");
  let level = leaves.slice();
  while (level.length > 1) {
    const next: Buffer[] = [];
    for (let i = 0; i < level.length; i += 2) {
      const left = level[i]!;
      const right = level[i + 1];
      if (!right) {
        // Tron txTrieRoot uses a carry-up rule (no self-duplication).
        next.push(left);
      } else {
        next.push(sha256Concat(left, right));
      }
    }
    level = next;
  }
  return level[0]!;
}

function readVarint(buf: Uint8Array, startOffset: number): { value: bigint; offset: number } {
  let result = 0n;
  let shift = 0n;
  let offset = startOffset;

  while (offset < buf.length) {
    const byteValue = buf[offset];
    if (byteValue === undefined) throw new Error("unexpected end of buffer while reading varint");
    const byte = BigInt(byteValue);
    result |= (byte & 0x7fn) << shift;
    offset += 1;
    if ((byte & 0x80n) === 0n) break;
    shift += 7n;
  }

  return { value: result, offset };
}

function splitProtoFields(bytes: Uint8Array): Map<number, Uint8Array[]> {
  const out = new Map<number, Uint8Array[]>();
  let offset = 0;

  while (offset < bytes.length) {
    const keyRes = readVarint(bytes, offset);
    const key = keyRes.value;
    offset = keyRes.offset;

    const fieldNumber = Number(key >> 3n);
    const wireType = Number(key & 0x7n);

    if (wireType === 0) {
      // varint: skip value
      const v = readVarint(bytes, offset);
      offset = v.offset;
      continue;
    }
    if (wireType === 1) {
      offset += 8;
      continue;
    }
    if (wireType === 5) {
      offset += 4;
      continue;
    }
    if (wireType !== 2) {
      throw new Error(`unsupported protobuf wire type ${wireType} at offset ${offset}`);
    }

    const lenRes = readVarint(bytes, offset);
    const len = Number(lenRes.value);
    offset = lenRes.offset;

    const end = offset + len;
    const fieldBytes = bytes.slice(offset, end);
    offset = end;

    const arr = out.get(fieldNumber) ?? [];
    arr.push(fieldBytes);
    out.set(fieldNumber, arr);
  }

  return out;
}

type DumpTx = {
  index: number;
  txid_ext: string;
  txid_from_raw_data: string | null;
  encoded_tx: string;
  encoded_tx_len: number;
  tx_leaf: string;
};

type Dump = {
  tron_block_number: number;
  txid: string;
  tx_index: number;
  tx_count: number;
  header_tx_trie_root: string;
  computed_tx_trie_root: string;
  computed_tx_trie_root_duplicate_last: string;
  proof: string[];
  index_bits: string;
  encoded_tx: string;
  encoded_tx_len: number;
  leaves: string[];
  transactions: DumpTx[];
};

async function fetchBlock(wallet: any, callOpts: any, num: number): Promise<BlockExtention> {
  const req: NumberMessage = { num: Long.fromNumber(num, true) };
  return await new Promise((resolve, reject) => {
    wallet.getBlockByNum2(req, callOpts.metadata, (err: any, res: BlockExtention | null) => {
      if (err || !res) return reject(err ?? new Error("Empty response from getBlockByNum2"));
      resolve(res);
    });
  });
}

async function main() {
  // Support both:
  // - direct: `tsx compareRelayerTxTrieRootDump.ts <dumpJsonPath> [blockNumber]`
  // - runner: `tsx src/run.ts compareRelayerTxTrieRootDump <dumpJsonPath> [blockNumber]`
  const rawArgs = process.argv.slice(2);
  let args = rawArgs.slice();
  if (args[0]?.endsWith("run.ts")) args = args.slice(1);
  if (args[0] === "compareRelayerTxTrieRootDump") args = args.slice(1);

  if (args.length < 1 || args.length > 2) {
    // eslint-disable-next-line no-console
    console.error("Usage: pnpm research compareRelayerTxTrieRootDump <dumpJsonPath> [blockNumber]");
    process.exit(1);
  }

  const dumpPath = resolve(args[0]!);
  const dump: Dump = JSON.parse(readFileSync(dumpPath, "utf8"));
  const blockNumber = args[1] ? Number(args[1]!) : dump.tron_block_number;
  if (!Number.isInteger(blockNumber) || blockNumber <= 0) throw new Error("invalid blockNumber");

  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
    })
  );
  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  log.info("Fetching block via getBlockByNum2", { blockNumber, dumpPath });
  const block = await fetchBlock(wallet, callOpts, blockNumber);

  const headerRoot = block.blockHeader?.rawData?.txTrieRoot;
  if (!headerRoot) throw new Error("missing block.blockHeader.rawData.txTrieRoot");

  const dumpHeaderRoot = dump.header_tx_trie_root.toLowerCase();
  const liveHeaderRoot = toHex0x(headerRoot).toLowerCase();
  if (dumpHeaderRoot !== liveHeaderRoot) {
    log.warn("Header root differs between dump and live fetch", {
      dumpHeaderRoot,
      liveHeaderRoot,
    });
  }

  const txExts = block.transactions ?? [];
  log.info("Tx counts", { dump: dump.tx_count, live: txExts.length });

  const liveEncoded: Buffer[] = [];
  const liveLeaves: Buffer[] = [];
  const liveTxids: string[] = [];

  for (let i = 0; i < txExts.length; i++) {
    const txExt = txExts[i]!;
    const tx = txExt.transaction as Transaction | undefined;
    if (!tx) throw new Error(`missing tx at index ${i}`);
    const enc = Buffer.from(Transaction.encode(tx).finish());
    liveEncoded.push(enc);
    liveLeaves.push(sha256Buf(enc));
    liveTxids.push(toHex0x(txExt.txid).toLowerCase());
  }

  const liveRoot = toHex0x(computeMerkleRootCarryUp(liveLeaves));
  log.info("Merkle roots", {
    header: toHex0x(headerRoot),
    computedFromLive: liveRoot,
  });

  const mismatches: Array<{
    index: number;
    kind: "txid" | "leaf" | "encoded";
    dump: string;
    live: string;
  }> = [];

  const n = Math.min(dump.transactions.length, txExts.length);
  for (let i = 0; i < n; i++) {
    const dumpTx = dump.transactions[i]!;
    const liveTxid = liveTxids[i]!;
    const dumpTxid = dumpTx.txid_ext.toLowerCase();
    if (dumpTxid !== liveTxid) {
      mismatches.push({ index: i, kind: "txid", dump: dumpTxid, live: liveTxid });
      continue;
    }

    const dumpLeaf = dumpTx.tx_leaf.toLowerCase();
    const liveLeaf = toHex0x(liveLeaves[i]!).toLowerCase();
    if (dumpLeaf !== liveLeaf) {
      mismatches.push({ index: i, kind: "leaf", dump: dumpLeaf, live: liveLeaf });
      continue;
    }

    const dumpEnc = dumpTx.encoded_tx.toLowerCase();
    const liveEnc = toHex0x(liveEncoded[i]!).toLowerCase();
    if (dumpEnc !== liveEnc) {
      mismatches.push({ index: i, kind: "encoded", dump: dumpEnc, live: liveEnc });
      continue;
    }
  }

  const first = mismatches[0];
  if (first) {
    const i = first.index;
    const dumpTx = dump.transactions[i]!;
    const liveEnc = liveEncoded[i]!;
    const dumpEnc = decodeHex0x(dumpTx.encoded_tx);

    const liveFields = splitProtoFields(liveEnc);
    const dumpFields = splitProtoFields(dumpEnc);

    const report = {
      blockNumber,
      mismatch: first,
      dumpHeaderRoot,
      liveHeaderRoot,
      liveComputedRoot: liveRoot,
      details: {
        index: i,
        txid: dumpTx.txid_ext,
        dump: {
          encodedTxLen: dumpEnc.length,
          sigCount: (dumpFields.get(2) ?? []).length,
          retCount: (dumpFields.get(5) ?? []).length,
          rawDataLen: (dumpFields.get(1)?.[0] ?? new Uint8Array()).length,
          hasRawData: (dumpFields.get(1) ?? []).length === 1,
          hasRet: (dumpFields.get(5) ?? []).length >= 1,
        },
        live: {
          encodedTxLen: liveEnc.length,
          sigCount: (liveFields.get(2) ?? []).length,
          retCount: (liveFields.get(5) ?? []).length,
          rawDataLen: (liveFields.get(1)?.[0] ?? new Uint8Array()).length,
          hasRawData: (liveFields.get(1) ?? []).length === 1,
          hasRet: (liveFields.get(5) ?? []).length >= 1,
        },
        rawDataEqual:
          Buffer.compare(
            Buffer.from(dumpFields.get(1)?.[0] ?? new Uint8Array()),
            Buffer.from(liveFields.get(1)?.[0] ?? new Uint8Array())
          ) === 0,
        retEqual:
          Buffer.compare(
            Buffer.from(dumpFields.get(5)?.[0] ?? new Uint8Array()),
            Buffer.from(liveFields.get(5)?.[0] ?? new Uint8Array())
          ) === 0,
        signaturesEqual:
          JSON.stringify((dumpFields.get(2) ?? []).map(toHex0x)) ===
          JSON.stringify((liveFields.get(2) ?? []).map(toHex0x)),
      },
    };

    const outPath = resolve("compareRelayerTxTrieRootDump.report.json");
    writeFileSync(outPath, JSON.stringify(report, null, 2));
    log.warn("Found mismatches; wrote report", {
      outPath,
      firstMismatch: first,
      total: mismatches.length,
    });
  } else {
    log.info("No per-tx mismatches found between dump and live fetch", {
      liveComputedRoot: liveRoot,
      header: toHex0x(headerRoot),
    });
  }
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});
