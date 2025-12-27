import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import Long from "long";
import { z } from "zod";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { summarizeError } from "../lib/sanitize.js";
import { createTronClients } from "@untron/tron-protocol";
import type { BlockExtention, NumberMessage } from "@untron/tron-protocol/api";
import { BlockHeader_raw } from "@untron/tron-protocol/tron";
import { sha256 } from "@noble/hashes/sha2.js";
import { keccak_256 } from "@noble/hashes/sha3.js";
import * as secp256k1 from "@noble/secp256k1";
import { tronLightClientAbi } from "@untron/v3-contracts";
import { createPublicClient, createWalletClient, http, type Address, type Hex } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { foundry } from "viem/chains";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const REPO_ROOT = resolve(__dirname, "../../../..");
const TLC_ARTIFACT_PATH = resolve(
  REPO_ROOT,
  "packages/contracts/out/TronLightClient.sol/TronLightClient.json"
);

const TRON_BLOCK_METADATA_SIZE = 69; // 32 + 32 + 4 + 1
const TRON_SIG_SIZE = 65;
const FINALITY_DISTINCT_SR_THRESHOLD = 19;
const MAX_UINT256 = (1n << 256n) - 1n;

type Tuple27<T> = readonly [
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
  T,
];

type Args = {
  start: number;
  end: number;
  batch: number;
  concurrency: number;
  srScanMax: number;
  anvilRpcUrl: string;
  privateKeyHex: Hex;
  contractAddress?: Address;
  dryRun: boolean;
  maxBatches?: number;
};

function parseArgs(argv: string[]): Args {
  const get = (name: string): string | undefined => {
    const i = argv.indexOf(name);
    if (i === -1) return undefined;
    return argv[i + 1];
  };
  const has = (name: string): boolean => argv.includes(name);

  const startStr = get("--start") ?? get("-s");
  const endStr = get("--end") ?? get("-e");
  if (!startStr || !endStr) {
    // eslint-disable-next-line no-console
    console.error(
      [
        "Usage:",
        "  pnpm research tlcStream --start <block> --end <block> [options]",
        "",
        "Options:",
        "  --batch <n>           Blocks per proveBlocks call (default: 10000, max: 65535)",
        "  --concurrency <n>     Tron RPC concurrency per batch (default: 16)",
        "  --sr-scan-max <n>     Max blocks to scan to discover 27 SR owners + delegatees (default: 1000)",
        "  --anvil <url>         Anvil RPC URL (default: http://127.0.0.1:8545)",
        "  --pk <hex>            EOA private key (0x...) to deploy/call on anvil (or set ANVIL_PRIVATE_KEY)",
        "  --contract <addr>     Use existing TronLightClient address (skip deploy)",
        "  --dry-run             Build batches but do not send transactions",
        "  --max-batches <n>     Stop after N batches (for quick checks)",
      ].join("\n")
    );
    process.exit(1);
  }

  const start = Number(startStr);
  const end = Number(endStr);
  if (!Number.isInteger(start) || !Number.isInteger(end) || start <= 0 || end < start) {
    throw new Error("Invalid --start/--end");
  }

  const batch = Number(get("--batch") ?? "10000");
  if (!Number.isInteger(batch) || batch <= 0 || batch > 65535) {
    throw new Error("Invalid --batch (must be 1..65535)");
  }

  const concurrency = Number(get("--concurrency") ?? "16");
  if (!Number.isInteger(concurrency) || concurrency <= 0) {
    throw new Error("Invalid --concurrency");
  }

  const srScanMax = Number(get("--sr-scan-max") ?? "1000");
  if (!Number.isInteger(srScanMax) || srScanMax < 27) {
    throw new Error("Invalid --sr-scan-max (must be >= 27)");
  }

  const anvilRpcUrl = get("--anvil") ?? "http://127.0.0.1:8545";

  const contractAddress = get("--contract") as Address | undefined;

  const dryRun = has("--dry-run");

  const maxBatchesStr = get("--max-batches");
  const maxBatches = maxBatchesStr ? Number(maxBatchesStr) : undefined;
  if (maxBatches != null && (!Number.isInteger(maxBatches) || maxBatches <= 0)) {
    throw new Error("Invalid --max-batches");
  }

  const pk = (get("--pk") ?? process.env.ANVIL_PRIVATE_KEY ?? "") as Hex;
  if (!/^0x[0-9a-fA-F]{64}$/.test(pk)) {
    throw new Error("Missing/invalid private key: pass --pk 0x... or set ANVIL_PRIVATE_KEY");
  }

  return {
    start,
    end,
    batch,
    concurrency,
    srScanMax,
    anvilRpcUrl,
    privateKeyHex: pk,
    contractAddress,
    dryRun,
    maxBatches,
  };
}

function toHex0x(bytes: Uint8Array | Buffer): Hex {
  return `0x${Buffer.from(bytes).toString("hex")}` as Hex;
}

function toTuple27<T>(arr: readonly T[], label: string): Tuple27<T> {
  if (arr.length !== 27) {
    throw new Error(`${label} must have length 27, got ${arr.length}`);
  }
  return arr as unknown as Tuple27<T>;
}

function blockIdToNumber(blockId: Hex): bigint {
  const x = BigInt(blockId);
  return x >> 192n;
}

function tronWitnessAddressToEvmAddress(bytes: Uint8Array | Buffer): Address | null {
  if (bytes.length !== 21 || bytes[0] !== 0x41) return null;
  return `0x${Buffer.from(bytes.subarray(1)).toString("hex")}` as Address;
}

function publicKey64FromUncompressed(pub: Uint8Array): Uint8Array | null {
  if (pub.length === 65 && pub[0] === 0x04) return pub.subarray(1);
  if (pub.length === 64) return pub;
  return null;
}

function evmAddressFromUncompressed(pub: Uint8Array): Address {
  const pub64 = publicKey64FromUncompressed(pub);
  if (!pub64) throw new Error("Invalid uncompressed public key length");
  const hash = keccak_256(pub64);
  return `0x${Buffer.from(hash.subarray(12)).toString("hex")}` as Address;
}

function recoverUncompressedPublicKey(
  hash32: Uint8Array,
  witnessSignature: Buffer
): Uint8Array | null {
  if (!witnessSignature || witnessSignature.length < 65) return null;
  const r = witnessSignature.subarray(0, 32);
  const s = witnessSignature.subarray(32, 64);
  let recovery = Number(witnessSignature[64]! & 0xff);
  if (recovery >= 27) recovery -= 27;
  if (recovery < 0 || recovery > 3) return null;
  const sig65 = new Uint8Array(65);
  sig65[0] = recovery;
  sig65.set(r, 1);
  sig65.set(s, 33);
  try {
    const pub = secp256k1.recoverPublicKey(sig65, hash32, { prehash: false });
    if (!pub || pub.length === 0) return null;
    if (pub.length === 65) return pub;
    if (pub.length === 33) {
      const hex = Buffer.from(pub).toString("hex");
      return secp256k1.Point.fromHex(hex).toBytes(false);
    }
    return null;
  } catch {
    return null;
  }
}

function sortLex(addresses: Address[]): Address[] {
  return [...addresses].sort((a, b) => a.toLowerCase().localeCompare(b.toLowerCase()));
}

async function fetchBlock(wallet: any, callOpts: any, num: number): Promise<BlockExtention> {
  const req: NumberMessage = { num: Long.fromNumber(num, true) };
  return await new Promise((resolve, reject) => {
    wallet.getBlockByNum2(req, callOpts.metadata, (err: any, res: BlockExtention | null) => {
      if (err || !res) return reject(err ?? new Error("Empty response from getBlockByNum2"));
      resolve(res);
    });
  });
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

async function discoverSrSetBySignatureRecovery(
  wallet: any,
  callOpts: any,
  startBlock: number,
  srScanMax: number
): Promise<{
  srs: Address[];
  witnessDelegatees: Address[];
  indexBySrOwner: Map<Address, number>;
}> {
  const ownerToDelegatee = new Map<Address, Address>();
  for (let n = startBlock; n < startBlock + srScanMax; n++) {
    const b = await fetchBlock(wallet, callOpts, n);
    const header = b.blockHeader;
    const raw = header?.rawData as BlockHeader_raw | undefined;
    const sig = header?.witnessSignature as Buffer | undefined;
    if (!raw || !sig || sig.length < 65 || !raw.witnessAddress) continue;

    const owner = tronWitnessAddressToEvmAddress(raw.witnessAddress);
    if (!owner) continue;

    const rawBytes = BlockHeader_raw.encode(raw).finish();
    const digest = sha256(rawBytes);
    const pub = recoverUncompressedPublicKey(digest, sig.subarray(0, 65));
    if (!pub) continue;
    const delegatee = evmAddressFromUncompressed(pub);

    const existing = ownerToDelegatee.get(owner);
    if (existing && existing.toLowerCase() !== delegatee.toLowerCase()) {
      throw new Error(
        `Conflicting delegatee for SR owner ${owner}: ${existing} vs ${delegatee} (block ${n})`
      );
    }
    ownerToDelegatee.set(owner, delegatee);

    if (ownerToDelegatee.size === 27) break;
  }

  if (ownerToDelegatee.size !== 27) {
    throw new Error(
      `Failed to discover 27 unique SR owners within ${srScanMax} blocks starting at ${startBlock} (got ${ownerToDelegatee.size})`
    );
  }

  const srs = sortLex([...ownerToDelegatee.keys()]);
  for (let i = 1; i < srs.length; i++) {
    if (srs[i - 1]!.toLowerCase() >= srs[i]!.toLowerCase()) {
      throw new Error("SR owners must be strictly increasing (lexicographic)");
    }
  }

  const witnessDelegatees = srs.map((sr) => ownerToDelegatee.get(sr)!);
  const indexBySrOwner = new Map<Address, number>(srs.map((sr, i) => [sr, i]));

  return { srs, witnessDelegatees, indexBySrOwner };
}

function latestFinalizedOffset(witnessIndices: Uint8Array): number {
  let mask = 0;
  for (let i = witnessIndices.length; i > 0; i--) {
    const idx = i - 1;
    const wi = witnessIndices[idx]!;
    mask |= 1 << wi;
    const distinct = popcount32(mask);
    if (distinct >= FINALITY_DISTINCT_SR_THRESHOLD) return idx;
  }
  throw new Error("No finalized offset in range (need >=19 distinct SRs)");
}

function popcount32(x: number): number {
  x >>>= 0;
  let c = 0;
  while (x !== 0) {
    x &= x - 1;
    c++;
  }
  return c;
}

function packStoreOnly(offset: number): bigint {
  if (offset < 0 || offset >= 0xffff) throw new Error("store offset out of uint16 range");
  let x = (1n << 256n) - 1n; // all lanes sentinel 0xFFFF
  x = (x & ~0xffffn) | BigInt(offset);
  return x;
}

async function buildBatch(
  wallet: any,
  callOpts: any,
  batchStart: number,
  batchEnd: number,
  indexBySrOwner: Map<Address, number>,
  concurrency: number
): Promise<{
  metadata: Uint8Array;
  sigs: Uint8Array;
  witnessIndices: Uint8Array;
}> {
  const count = batchEnd - batchStart + 1;
  const metadata = new Uint8Array(count * TRON_BLOCK_METADATA_SIZE);
  const sigs = new Uint8Array(count * TRON_SIG_SIZE);
  const witnessIndices = new Uint8Array(count);

  const blockNumbers = Array.from({ length: count }, (_, i) => batchStart + i);
  await mapWithConcurrency(blockNumbers, concurrency, async (n) => {
    const i = n - batchStart;
    const b = await fetchBlock(wallet, callOpts, n);
    const header = b.blockHeader;
    const raw = header?.rawData as BlockHeader_raw | undefined;
    if (!raw) throw new Error(`Block ${n} missing blockHeader.rawData`);
    if (!raw.parentHash || !raw.txTrieRoot)
      throw new Error(`Block ${n} missing parentHash/txTrieRoot`);
    if (!raw.witnessAddress) throw new Error(`Block ${n} missing witnessAddress`);

    const owner = tronWitnessAddressToEvmAddress(raw.witnessAddress);
    if (!owner) throw new Error(`Block ${n} has invalid witnessAddress`);

    const witnessIndex = indexBySrOwner.get(owner);
    if (witnessIndex == null) {
      throw new Error(`Block ${n} witness owner not in SR set: ${owner}`);
    }
    if (witnessIndex < 0 || witnessIndex > 26)
      throw new Error(`Invalid witnessIndex for block ${n}`);
    witnessIndices[i] = witnessIndex;

    const tsMs = BigInt(raw.timestamp.toString());
    const tsSec = tsMs / 1000n;
    if (tsSec > 0xffffffffn) throw new Error(`Timestamp seconds overflow uint32 for block ${n}`);

    const metaOff = i * TRON_BLOCK_METADATA_SIZE;
    metadata.set(raw.parentHash, metaOff + 0);
    metadata.set(raw.txTrieRoot, metaOff + 32);
    metadata[metaOff + 64] = Number((tsSec >> 24n) & 0xffn);
    metadata[metaOff + 65] = Number((tsSec >> 16n) & 0xffn);
    metadata[metaOff + 66] = Number((tsSec >> 8n) & 0xffn);
    metadata[metaOff + 67] = Number(tsSec & 0xffn);
    metadata[metaOff + 68] = witnessIndex;

    const sig = header?.witnessSignature as Buffer | undefined;
    if (!sig || sig.length < TRON_SIG_SIZE)
      throw new Error(`Block ${n} missing/invalid witnessSignature`);
    sigs.set(sig.subarray(0, TRON_SIG_SIZE), i * TRON_SIG_SIZE);
  });

  return { metadata, sigs, witnessIndices };
}

function artifactBytecode0x(artifact: any): Hex {
  const obj = artifact?.bytecode?.object;
  if (typeof obj !== "string" || obj.length === 0) {
    throw new Error(`Invalid artifact bytecode at ${TLC_ARTIFACT_PATH}`);
  }
  return (obj.startsWith("0x") ? obj : `0x${obj}`) as Hex;
}

async function main() {
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
      ANVIL_PRIVATE_KEY: z.string().optional(),
    })
  );

  const args = parseArgs(process.argv.slice(3));

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  log.info("Discovering SR set + delegatees from recent blocks...", {
    start: args.start,
    srScanMax: args.srScanMax,
  });
  const { srs, witnessDelegatees, indexBySrOwner } = await discoverSrSetBySignatureRecovery(
    wallet,
    callOpts,
    args.start,
    args.srScanMax
  );
  const srs27 = toTuple27(srs, "srs");
  const witnessDelegatees27 = toTuple27(witnessDelegatees, "witnessDelegatees");
  log.info("Discovered SR set", { srs: srs.length });

  const first = await fetchBlock(wallet, callOpts, args.start);
  const firstRaw = first.blockHeader?.rawData as BlockHeader_raw | undefined;
  if (!firstRaw?.parentHash) throw new Error(`Block ${args.start} missing parentHash`);
  const startingBlockId = toHex0x(firstRaw.parentHash);

  const parentNum = args.start - 1;
  const parent = await fetchBlock(wallet, callOpts, parentNum);
  const parentRaw = parent.blockHeader?.rawData as BlockHeader_raw | undefined;
  if (!parentRaw?.txTrieRoot) throw new Error(`Parent block ${parentNum} missing txTrieRoot`);
  const startingBlockTxTrieRoot = toHex0x(parentRaw.txTrieRoot);
  const startingTimestampSec = Number(BigInt(parentRaw.timestamp.toString()) / 1000n);
  if (
    !Number.isInteger(startingTimestampSec) ||
    startingTimestampSec < 0 ||
    startingTimestampSec > 0xffffffff
  ) {
    throw new Error("startingTimestampSec out of uint32 range");
  }

  const account = privateKeyToAccount(args.privateKeyHex);
  const publicClient = createPublicClient({
    chain: foundry,
    transport: http(args.anvilRpcUrl),
  });
  const walletClient = createWalletClient({
    chain: foundry,
    transport: http(args.anvilRpcUrl),
    account,
  });

  let contract = args.contractAddress;

  if (!contract) {
    const artifact = JSON.parse(readFileSync(TLC_ARTIFACT_PATH, "utf8"));
    const bytecode = artifactBytecode0x(artifact);

    log.info("Deploying TronLightClient to anvil...", { anvil: args.anvilRpcUrl });
    const hash = await walletClient.deployContract({
      abi: tronLightClientAbi,
      bytecode,
      args: [
        "0x0000000000000000000000000000000000000000", // blockRangeProver
        startingBlockId,
        startingBlockTxTrieRoot,
        startingTimestampSec,
        srs27,
        witnessDelegatees27,
        "0x0000000000000000000000000000000000000000000000000000000000000000", // srDataHash (unused by proveBlocks)
      ],
    });
    const receipt = await publicClient.waitForTransactionReceipt({ hash });
    if (!receipt.contractAddress) throw new Error("Deploy did not return contractAddress");
    contract = receipt.contractAddress;
    log.info("Deployed", { contract });
  } else {
    log.info("Using existing TronLightClient", { contract });
  }

  let anchorBlockId = startingBlockId;
  let anchorNumber = Number(blockIdToNumber(anchorBlockId));
  let nextBlock = anchorNumber + 1;

  log.info("Streaming proveBlocks batches...", {
    start: args.start,
    end: args.end,
    initialAnchor: anchorNumber,
    batch: args.batch,
    concurrency: args.concurrency,
    dryRun: args.dryRun,
  });

  let batchCount = 0;
  while (nextBlock <= args.end) {
    batchCount++;
    if (args.maxBatches != null && batchCount > args.maxBatches) {
      log.warn("Stopping early due to --max-batches", { batchCount, maxBatches: args.maxBatches });
      break;
    }

    const batchStart = nextBlock;
    const batchEnd = Math.min(args.end, batchStart + args.batch - 1);
    const count = batchEnd - batchStart + 1;

    const t0 = Date.now();
    const { metadata, sigs, witnessIndices } = await buildBatch(
      wallet,
      callOpts,
      batchStart,
      batchEnd,
      indexBySrOwner,
      args.concurrency
    );
    const metadataHex = toHex0x(metadata);
    const sigsHex = toHex0x(sigs);
    const storeOffset = latestFinalizedOffset(witnessIndices);
    const storeOffsets16 = packStoreOnly(storeOffset);
    const expectedStoredBlock = batchStart + storeOffset;

    const buildMs = Date.now() - t0;
    log.info("Built batch", {
      batch: batchCount,
      batchStart,
      batchEnd,
      count,
      buildMs,
      storeOffset,
      expectedStoredBlock,
    });

    if (args.dryRun) {
      anchorNumber = expectedStoredBlock;
      nextBlock = anchorNumber + 1;
      continue;
    }

    const txHash = await walletClient.writeContract({
      address: contract,
      abi: tronLightClientAbi,
      functionName: "proveBlocks",
      args: [anchorBlockId, metadataHex, sigsHex, MAX_UINT256, storeOffsets16],
    });

    const receipt = await publicClient.waitForTransactionReceipt({ hash: txHash });

    const latest = (await publicClient.readContract({
      address: contract,
      abi: tronLightClientAbi,
      functionName: "latestProvenBlock",
    })) as Hex;

    const latestNum = Number(blockIdToNumber(latest));
    if (latestNum !== expectedStoredBlock) {
      throw new Error(
        `Unexpected latestProvenBlock number: got ${latestNum}, expected ${expectedStoredBlock} (batch ${batchCount})`
      );
    }
    if (latestNum <= anchorNumber) {
      throw new Error(`latestProvenBlock did not advance (prev=${anchorNumber}, new=${latestNum})`);
    }

    log.info("Proved batch", {
      batch: batchCount,
      txHash,
      gasUsed: receipt.gasUsed.toString(),
      latestProvenBlock: latest,
      latestNum,
    });

    anchorBlockId = latest;
    anchorNumber = latestNum;
    nextBlock = anchorNumber + 1;
  }

  log.info("Done", {
    batches: batchCount,
    finalAnchorBlock: anchorNumber,
    finalAnchorId: anchorBlockId,
  });
}

main().catch((err) => {
  const summary = summarizeError(err);
  // eslint-disable-next-line no-console
  console.error(summary.message);
  if (process.env.RESEARCH_DEBUG_ERRORS === "1") {
    // eslint-disable-next-line no-console
    console.error(err);
  } else if (summary.data) {
    // eslint-disable-next-line no-console
    console.error(summary.data);
  }
  process.exit(1);
});
