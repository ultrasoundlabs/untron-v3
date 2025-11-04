import { z } from "zod";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { createTronClients } from "@untron/tron-protocol";
import type { BlockExtention, EmptyMessage, NumberMessage } from "@untron/tron-protocol/api";
import { BlockHeader_raw } from "@untron/tron-protocol/tron";
import Long from "long";
import { sha256 } from "@noble/hashes/sha2.js";
import * as secp256k1 from "@noble/secp256k1";
import { writeFile } from "node:fs/promises";

function toLong(value: bigint | number | string): Long {
  return Long.fromValue(typeof value === "bigint" ? value.toString() : value);
}

async function getNowBlock(
  wallet: ReturnType<typeof createTronClients>["wallet"],
  metadata: ReturnType<typeof createTronClients>["callOpts"]["metadata"]
): Promise<BlockExtention> {
  return await new Promise((resolve, reject) => {
    wallet.getNowBlock2({} as EmptyMessage, metadata, (err, res) => {
      if (err) return reject(err);
      resolve(res);
    });
  });
}

async function getBlockByNumber(
  wallet: ReturnType<typeof createTronClients>["wallet"],
  metadata: ReturnType<typeof createTronClients>["callOpts"]["metadata"],
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

function encodeBlockHeaderRaw(
  raw?: { [K in keyof BlockHeader_raw]?: BlockHeader_raw[K] } | null
): Uint8Array {
  if (!raw) return new Uint8Array();
  return BlockHeader_raw.encode(raw as BlockHeader_raw).finish();
}

function recoverUncompressedPublicKey(
  hash32: Uint8Array,
  witnessSignature: Buffer
): Uint8Array | null {
  if (!witnessSignature || witnessSignature.length < 65) return null;
  const r = witnessSignature.subarray(0, 32);
  const s = witnessSignature.subarray(32, 64);
  let recovery = Number(witnessSignature[64]! & 0xff);
  if (recovery >= 27) recovery -= 27; // normalize eth-style v (27/28) -> 0/1
  if (recovery < 0 || recovery > 3) return null;
  const sig65 = new Uint8Array(65);
  sig65[0] = recovery;
  sig65.set(r, 1);
  sig65.set(s, 33);
  try {
    const pub = secp256k1.recoverPublicKey(sig65, hash32, { prehash: false });
    if (!pub || pub.length === 0) return null;
    if (pub.length === 65) return pub; // uncompressed 0x04 || X || Y
    if (pub.length === 33) {
      try {
        const hex = Buffer.from(pub).toString("hex");
        return secp256k1.Point.fromHex(hex).toBytes(false);
      } catch (e) {
        log.warn("failed to decompress pubkey", { e });
        return null;
      }
    }
    log.warn("unexpected recovered pubkey length", { len: pub.length });
    return null;
  } catch (err) {
    log.warn("failed to recover public key", { err, recovery });
    return null;
  }
}

function publicKey64FromUncompressed(pub: Uint8Array): Uint8Array | null {
  if (pub.length === 65 && pub[0] === 0x04) return pub.subarray(1);
  if (pub.length === 64) return pub;
  return null;
}

async function main() {
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
      SRSCAN_EPOCHS: z.string().optional(),
    })
  );

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  // args: depth (number) and optional flags like --verbose/-v
  const args = process.argv.slice(3);
  const verbose =
    args.includes("--verbose") || args.includes("-v") || process.env.SRSCAN_VERBOSE === "1";
  const depthToken = args.find((a) => /^\d+$/.test(a));
  const depthArg = depthToken ?? env.SRSCAN_EPOCHS ?? "5";
  const depth = BigInt(depthArg);
  if (depth <= 0n) {
    log.error("Depth must be > 0");
    process.exit(1);
  }

  const nowBlock = await getNowBlock(wallet, callOpts.metadata);
  const latestNumber = BigInt(nowBlock.blockHeader?.rawData?.number?.toString() ?? "0");

  const srCounts = new Map<string, bigint>();

  const totals = {
    fetchedBlocks: 0n,
    grpcErrors: 0n,
    missingRaw: 0n,
    missingSig: 0n,
    shortSig: 0n,
    recoverOk: 0n,
    recoverFail: 0n,
  } as const;
  // mutable copy
  const counters: Record<keyof typeof totals, bigint> = { ...totals } as any;

  const startNs = process.hrtime.bigint();
  for (let epoch = 1n; epoch < depth; epoch++) {
    const epochBlock = latestNumber - epoch * 7200n;
    const epochEndBlock = epochBlock + 26n;
    let eFetched = 0n,
      eGrpc = 0n,
      eMissingRaw = 0n,
      eMissingSig = 0n,
      eShortSig = 0n,
      eRecoverOk = 0n,
      eRecoverFail = 0n;

    log.info(
      `epoch ${epoch.toString()}: scanning blocks ${epochBlock.toString()}..${epochEndBlock.toString()}`
    );
    for (let blockOffset = 0n; blockOffset < 27n; blockOffset++) {
      const target = epochBlock + blockOffset;
      if (verbose) {
        log.debug(`block ${target.toString()}`);
      }
      try {
        const block = await getBlockByNumber(wallet, callOpts.metadata, target);
        const header = block.blockHeader;
        const raw = header?.rawData ?? null;
        const sig = header?.witnessSignature ?? Buffer.alloc(0);
        counters.fetchedBlocks++;
        eFetched++;
        if (!raw) {
          counters.missingRaw++;
          eMissingRaw++;
          if (verbose) log.debug("skip: missing raw header", { block: target.toString() });
          continue;
        }
        if (!sig || sig.length === 0) {
          counters.missingSig++;
          eMissingSig++;
          if (verbose) log.debug("skip: missing signature", { block: target.toString() });
          continue;
        }
        if (sig.length < 65) {
          counters.shortSig++;
          eShortSig++;
          if (verbose)
            log.debug("skip: short signature", { block: target.toString(), len: sig.length });
          continue;
        }

        const rawBytes = encodeBlockHeaderRaw(raw);
        const digest = sha256(rawBytes);
        const pubUncompressed = recoverUncompressedPublicKey(digest, sig);
        const pub64 = pubUncompressed ? publicKey64FromUncompressed(pubUncompressed) : null;
        if (!pub64) {
          counters.recoverFail++;
          eRecoverFail++;
          if (verbose) log.debug("recover failed", { block: target.toString() });
          continue;
        }
        counters.recoverOk++;
        eRecoverOk++;
        const hex = Buffer.from(pub64).toString("hex");
        srCounts.set(hex, (srCounts.get(hex) ?? 0n) + 1n);
      } catch (err) {
        counters.grpcErrors++;
        eGrpc++;
        log.warn("failed to fetch/process block", { block: target.toString(), err });
      }
    }

    log.info(
      `epoch ${epoch.toString()} done: fetched=${eFetched.toString()} grpcErrors=${eGrpc.toString()} missingRaw=${eMissingRaw.toString()} missingSig=${eMissingSig.toString()} shortSig=${eShortSig.toString()} recoverOk=${eRecoverOk.toString()} recoverFail=${eRecoverFail.toString()} uniqueSRs=${BigInt(srCounts.size).toString()}`
    );
  }

  const elapsedNs = process.hrtime.bigint() - startNs;
  const elapsedSec = elapsedNs / 1_000_000_000n;
  const uniqueCount = BigInt(srCounts.size);

  // compute epochs scanned and a fixed-point rate (3 decimal places) using BigInt only
  const scannedEpochs = depth - 1n;
  const scale = 1000n;
  const perSecScaled =
    elapsedSec === 0n ? scannedEpochs * scale : (scannedEpochs * scale) / elapsedSec;
  const whole = perSecScaled / scale;
  const frac = perSecScaled % scale;
  const fracStr = frac.toString().padStart(3, "0");

  const keys = Array.from(srCounts.keys()).join("\n");
  if (verbose) {
    log.info(keys);
  }
  log.info(`total ${uniqueCount.toString()} unique SRs`);
  log.info(
    `scraped in ${elapsedSec.toString()} sec, avg ${whole.toString()}.${fracStr} epochs/s (epochs=${scannedEpochs.toString()})`
  );
  log.info(
    `totals: fetched=${counters.fetchedBlocks.toString()} grpcErrors=${counters.grpcErrors.toString()} missingRaw=${counters.missingRaw.toString()} missingSig=${counters.missingSig.toString()} shortSig=${counters.shortSig.toString()} recoverOk=${counters.recoverOk.toString()} recoverFail=${counters.recoverFail.toString()}`
  );
  await writeFile("srs.txt", keys, { encoding: "utf8" });
  log.info("saved in srs.txt");
}

main().catch((err) => {
  log.error(err);
  process.exit(1);
});
