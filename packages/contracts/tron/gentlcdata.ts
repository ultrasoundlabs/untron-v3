import { z } from "zod";

import { createTronClients } from "@untron/tron-protocol";
import type {
  BlockExtention,
  EmptyMessage,
  NumberMessage,
  WitnessList,
} from "@untron/tron-protocol/api";
import { Account } from "@untron/tron-protocol/tron";

import Long from "long";
import { writeFile } from "node:fs/promises";

type TronWallet = ReturnType<typeof createTronClients>["wallet"];
type TronMetadata = ReturnType<typeof createTronClients>["callOpts"]["metadata"];
type Witness = NonNullable<WitnessList["witnesses"]>[number];

function toLong(value: bigint | number | string): Long {
  return Long.fromValue(typeof value === "bigint" ? value.toString() : value);
}

function strip0x(hex: string): string {
  return hex.startsWith("0x") ? hex.slice(2) : hex;
}

function bytesTo0xHex(bytes: Uint8Array | Buffer): string {
  return `0x${Buffer.from(bytes).toString("hex")}`;
}

function assertLen(name: string, bytes: Uint8Array | Buffer, len: number) {
  if (bytes.length !== len) {
    throw new Error(`${name} must be ${len} bytes, got ${bytes.length}`);
  }
}

function parseEnv<TSchema extends z.ZodTypeAny>(schema: TSchema): z.infer<TSchema> {
  const parsed = schema.safeParse(process.env);
  if (parsed.success) return parsed.data;

  const formatted = parsed.error.issues
    .map((i) => `${i.path.join(".") || "(root)"}: ${i.message}`)
    .join("\n");
  throw new Error(`Invalid environment variables:\n${formatted}`);
}

function tron21ToEvm20(tronAddr: Uint8Array | Buffer): string {
  const b = Buffer.from(tronAddr);
  if (b.length !== 21) throw new Error(`Tron address must be 21 bytes, got ${b.length}`);
  if (b[0] !== 0x41) {
    // still strip first byte, but warn
    console.warn("unexpected Tron address prefix (expected 0x41)", { prefix: b[0] });
  }
  const evm20 = b.subarray(1); // last 20 bytes
  return `0x${evm20.toString("hex")}`;
}

// abi.encode(bytes20[27] a, bytes20[27] b)
// Each bytes20 element occupies one 32-byte slot: value (20) + right padding (12 zeros)
function abiEncodeBytes20x27Pair(srs: string[], delegates: string[]): Uint8Array {
  if (srs.length !== 27 || delegates.length !== 27) {
    throw new Error("srs and witnessDelegatees must both have length 27");
  }

  const slots: Buffer[] = [];

  const pushBytes20Slot = (addr20: string) => {
    const raw = Buffer.from(strip0x(addr20), "hex");
    if (raw.length !== 20) throw new Error(`expected 20-byte hex address, got ${raw.length}`);
    const slot = Buffer.alloc(32, 0);
    // bytes20 is right-padded in ABI (bytesN types)
    raw.copy(slot, 0);
    slots.push(slot);
  };

  for (const a of srs) pushBytes20Slot(a);
  for (const d of delegates) pushBytes20Slot(d);

  return Buffer.concat(slots);
}

function computeSrDataHash(srs: string[], delegates: string[]): string {
  return "0x0000000000000000000000000000000000000000000000000000000000000000"; // TODO: implement
}

function withTimeout<T>(promise: Promise<T>, timeoutMs: number, label: string): Promise<T> {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) return promise;
  return new Promise<T>((resolve, reject) => {
    const t = setTimeout(
      () => reject(new Error(`${label} timed out after ${timeoutMs}ms`)),
      timeoutMs
    );
    promise.then(
      (v) => {
        clearTimeout(t);
        resolve(v);
      },
      (err) => {
        clearTimeout(t);
        reject(err);
      }
    );
  });
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function mapWithConcurrency<T, R>(
  items: readonly T[],
  concurrency: number,
  fn: (item: T, index: number) => Promise<R>
): Promise<R[]> {
  const results: R[] = new Array(items.length);
  const workerCount = Math.max(1, Math.min(concurrency, items.length));
  let nextIndex = 0;

  const workers = Array.from({ length: workerCount }, async () => {
    while (true) {
      const i = nextIndex++;
      if (i >= items.length) return;
      results[i] = await fn(items[i]!, i);
    }
  });

  await Promise.all(workers);
  return results;
}

async function getNowBlock(wallet: TronWallet, metadata: TronMetadata): Promise<BlockExtention> {
  return await new Promise((resolve, reject) => {
    wallet.getNowBlock2({} as EmptyMessage, metadata, (err, res) => {
      if (err) return reject(err);
      resolve(res);
    });
  });
}

async function getBlockByNumber(
  wallet: TronWallet,
  metadata: TronMetadata,
  blockNumber: bigint
): Promise<BlockExtention> {
  const req: NumberMessage = { num: toLong(blockNumber) };
  return await new Promise((resolve, reject) => {
    wallet.getBlockByNum2(req, metadata, (err, res) => {
      if (err) return reject(err);
      resolve(res);
    });
  });
}

async function getBlockByNumberEnsuringHeader(
  wallet: TronWallet,
  metadata: TronMetadata,
  blockNumber: bigint,
  opts?: { retries?: number; retryDelayMs?: number }
): Promise<BlockExtention> {
  const retries = Math.max(0, opts?.retries ?? 2);
  const retryDelayMs = Math.max(0, opts?.retryDelayMs ?? 200);

  for (let attempt = 0; attempt <= retries; attempt++) {
    const b = await getBlockByNumber(wallet, metadata, blockNumber);
    if (b.blockHeader?.rawData) return b;

    if (attempt < retries) {
      await sleep(retryDelayMs * (attempt + 1));
      continue;
    }

    const now = await getNowBlock(wallet, metadata);
    const latestNumber = BigInt(now.blockHeader?.rawData?.number?.toString() ?? "0");
    throw new Error(
      `missing blockHeader.rawData for block ${blockNumber.toString()} (node latest=${latestNumber.toString()})`
    );
  }

  throw new Error("unreachable");
}

async function listWitnesses(wallet: TronWallet, metadata: TronMetadata): Promise<WitnessList> {
  return await new Promise((resolve, reject) => {
    wallet.listWitnesses({} as EmptyMessage, metadata, (err, res) => {
      if (err || !res) return reject(err ?? new Error("Empty response from listWitnesses"));
      resolve(res);
    });
  });
}

async function getAccount(
  wallet: TronWallet,
  metadata: TronMetadata,
  address21: Buffer,
  timeoutMs: number
): Promise<Account> {
  const accountReq = Account.create({ address: address21 });
  const p = new Promise<Account>((resolve, reject) => {
    wallet.getAccount(accountReq, metadata, (err, res) => {
      if (err || !res) return reject(err ?? new Error("Empty response from getAccount"));
      resolve(res);
    });
  });
  return await withTimeout(p, timeoutMs, `getAccount(${address21.toString("hex")})`);
}

async function getWitnessDelegateHex21(
  wallet: TronWallet,
  metadata: TronMetadata,
  witnessOwnerHex21: string,
  timeoutMs: number
): Promise<string | null> {
  const witnessAddrBuf = Buffer.from(witnessOwnerHex21, "hex");
  if (witnessAddrBuf.length !== 21) {
    throw new Error(`witness address must be 21 bytes, got ${witnessAddrBuf.length}`);
  }

  const account = await getAccount(wallet, metadata, witnessAddrBuf, timeoutMs);
  const perm = account.witnessPermission;
  if (!perm) return null;

  const firstKey = perm.keys?.[0];
  const keyAddrBuf = firstKey?.address as Buffer | undefined;
  if (!keyAddrBuf || keyAddrBuf.length === 0) return null;

  return Buffer.from(keyAddrBuf).toString("hex");
}

function longToBigInt(v: any): bigint {
  if (v == null) return 0n;
  const s = typeof v === "string" ? v : (v.toString?.() ?? String(v));
  return BigInt(s);
}

function witnessVoteCount(w: Witness): bigint {
  // voteCount field naming may vary; try common shapes
  const anyW: any = w as any;
  return longToBigInt(anyW.voteCount ?? anyW.vote_count ?? anyW.votes ?? 0);
}

function sortBySrOwnerLex(
  srs: string[],
  witnessDelegatees: string[]
): {
  srs: string[];
  witnessDelegatees: string[];
} {
  if (srs.length !== 27 || witnessDelegatees.length !== 27) {
    throw new Error("srs and witnessDelegatees must both have length 27");
  }

  const pairs = srs.map((sr, i) => ({
    sr: sr.toLowerCase(),
    delegate: witnessDelegatees[i]!.toLowerCase(),
  }));

  pairs.sort((a, b) => strip0x(a.sr).localeCompare(strip0x(b.sr)));

  for (let i = 1; i < pairs.length; i++) {
    if (pairs[i - 1]!.sr >= pairs[i]!.sr) {
      throw new Error(`srs must be strictly increasing (index=${i})`);
    }
  }

  return {
    srs: pairs.map((p) => p.sr),
    witnessDelegatees: pairs.map((p) => p.delegate),
  };
}

async function main() {
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
      BLOCK_RANGE_PROVER: z.string().optional(),
      TLC_BLOCK: z.string().optional(), // optional override
      TRON_RPC_TIMEOUT_MS: z.string().optional(), // getAccount timeout
    })
  );

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  // tsx argv layout: [node, tsx, script, ...args] => args start at index 3
  const args = process.argv.slice(3);

  const outPath = (args.find((a) => !a.startsWith("-")) ?? "").trim() || "script/tlc.json";

  const blockFlagIdx = args.findIndex((a) => a === "--block" || a === "-b");
  const blockArg = (blockFlagIdx >= 0 ? args[blockFlagIdx + 1] : undefined) ?? env.TLC_BLOCK;

  const now = await getNowBlock(wallet, callOpts.metadata);
  const latestNumber = BigInt(now.blockHeader?.rawData?.number?.toString() ?? "0");
  if (latestNumber <= 0n) {
    throw new Error("Failed to read latest block number from getNowBlock2()");
  }

  // end-of-round blocks satisfy `block % 27 == 26`; choose the most recent one <= latest
  const safeRoundEnd = latestNumber - ((latestNumber + 1n) % 27n);
  if (safeRoundEnd < 26n)
    throw new Error(`Chain height too low (latest=${latestNumber.toString()})`);

  let chosen: bigint;
  let roundStart: bigint;
  let roundEnd: bigint;

  if (blockArg) {
    chosen = BigInt(blockArg);
    if (chosen > latestNumber) {
      throw new Error(
        `Chosen block ${chosen.toString()} is > latest block ${latestNumber.toString()}`
      );
    }

    roundStart = chosen - (chosen % 27n);
    roundEnd = roundStart + 26n;
    if (roundEnd > latestNumber) {
      throw new Error(
        `Chosen block ${chosen.toString()} is in an incomplete round (roundEnd=${roundEnd.toString()} > latest=${latestNumber.toString()}); use --block ${safeRoundEnd.toString()} or wait until block ${roundEnd.toString()} is produced`
      );
    }
  } else {
    // pick a safe “recent completed round”
    chosen = safeRoundEnd;
    roundEnd = safeRoundEnd;
    roundStart = roundEnd - 26n;
  }

  console.info(
    `using round blocks ${roundStart.toString()}..${roundEnd.toString()} (chosen=${chosen.toString()}, latest=${latestNumber.toString()})`
  );

  // collect SR owners from round headers
  console.info("deriving SR owners from 27-block round headers...");
  const roundWitnessHex21: string[] = [];
  for (let i = 0n; i < 27n; i++) {
    const n = roundStart + i;
    const b = await getBlockByNumberEnsuringHeader(wallet, callOpts.metadata, n);
    const raw = b.blockHeader?.rawData ?? null;
    if (!raw) throw new Error(`missing blockHeader.rawData for block ${n.toString()}`);
    const wAddr = raw.witnessAddress as Buffer | undefined;
    if (!wAddr || wAddr.length === 0)
      throw new Error(`missing witnessAddress for block ${n.toString()}`);
    roundWitnessHex21.push(Buffer.from(wAddr).toString("hex"));
  }

  const uniqueRound = new Set(roundWitnessHex21);
  let srOwnersHex21: string[];

  if (uniqueRound.size === 27) {
    srOwnersHex21 = roundWitnessHex21;
    console.info("derived SR owners from 27-block round headers");
  } else {
    console.warn(
      "round headers did not yield 27 unique witnesses; falling back to top-27 by votes",
      {
        unique: uniqueRound.size,
      }
    );

    const wl = await listWitnesses(wallet, callOpts.metadata);
    const sorted = [...wl.witnesses].sort((a, b) => {
      const va = witnessVoteCount(a);
      const vb = witnessVoteCount(b);
      return va === vb ? 0 : va > vb ? -1 : 1;
    });

    const top = sorted.slice(0, 27).map((w) => {
      const addr = w.address as Buffer | undefined;
      if (!addr || addr.length === 0) throw new Error("witness missing address in listWitnesses");
      return Buffer.from(addr).toString("hex");
    });

    if (top.length !== 27) throw new Error(`top witnesses length != 27 (got ${top.length})`);
    srOwnersHex21 = top;
  }

  // map to EVM 20-byte addresses (strip 0x41 prefix)
  const srs = srOwnersHex21.map((hex21) => tron21ToEvm20(Buffer.from(hex21, "hex")));

  // delegates (first key if present; else owner) — only for selected 27 to keep this fast
  const timeoutMs = Math.max(0, Number(env.TRON_RPC_TIMEOUT_MS ?? "10000"));
  console.info("loading witness delegate permissions for selected SRs...");
  let delegatesDone = 0;
  const delegatesHex21 = await mapWithConcurrency(srOwnersHex21, 6, async (wHex21, i) => {
    try {
      const del = await getWitnessDelegateHex21(wallet, callOpts.metadata, wHex21, timeoutMs);
      return del ?? wHex21;
    } catch (err) {
      console.warn("failed to load witness delegate; falling back to owner", {
        i: i + 1,
        witness: wHex21,
        err,
      });
      return wHex21;
    } finally {
      delegatesDone++;
      if (delegatesDone === 27 || delegatesDone % 5 === 0) {
        console.info(`delegate lookups: ${delegatesDone}/27`);
      }
    }
  });

  const witnessDelegatees = delegatesHex21.map((hex21) => tron21ToEvm20(Buffer.from(hex21, "hex")));
  const canonical = sortBySrOwnerLex(srs, witnessDelegatees);

  // choose initial block as end of the round (so the SR schedule we derived matches this round)
  const initBlockNum = roundEnd;
  const initBlock = await getBlockByNumberEnsuringHeader(wallet, callOpts.metadata, initBlockNum);

  const blockId = initBlock.blockid as Buffer | undefined;
  if (!blockId) throw new Error("blockid missing on BlockExtention");
  assertLen("blockid", blockId, 32);

  const raw = initBlock.blockHeader?.rawData ?? null;
  if (!raw) throw new Error("blockHeader.rawData missing on initial block");

  const txTrieRoot = raw.txTrieRoot as Buffer | undefined;
  if (!txTrieRoot) throw new Error("txTrieRoot missing on initial block rawData");
  assertLen("txTrieRoot", txTrieRoot, 32);

  const tsMs = BigInt(raw.timestamp?.toString?.() ?? "0");
  const initialTimestamp = Number(tsMs / 1000n);

  const srDataHash = computeSrDataHash(canonical.srs, canonical.witnessDelegatees);

  // basic prover address sanity
  const prover = env.BLOCK_RANGE_PROVER ?? "0x0000000000000000000000000000000000000000";
  if (!/^0x[0-9a-fA-F]{40}$/.test(prover)) {
    throw new Error(`BLOCK_RANGE_PROVER must be a 20-byte hex address (0x...), got: ${prover}`);
  }

  const out = {
    blockRangeProver: prover,
    initialBlockNumber: initBlockNum.toString(), // extra (harmless) for humans
    initialBlockHash: bytesTo0xHex(blockId),
    initialTxTrieRoot: bytesTo0xHex(txTrieRoot),
    initialTimestamp,
    srs: canonical.srs,
    witnessDelegatees: canonical.witnessDelegatees,
    srDataHash,
  };

  await writeFile(outPath, JSON.stringify(out, null, 2) + "\n", { encoding: "utf8" });
  console.info(`Saved to ${outPath}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
