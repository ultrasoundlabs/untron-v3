import { writeFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import Long from "long";
import { z } from "zod";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { createTronClients } from "@untron/tron-protocol";
import { BlockHeader_raw } from "@untron/tron-protocol/tron";
import type { BlockExtention, NumberMessage } from "@untron/tron-protocol/api";
import { sha256 } from "@noble/hashes/sha2.js";
import type { Address, Hex } from "viem";

// Resolve paths relative to this file so we can reliably drop fixtures
// into the contracts package, regardless of the current working directory.
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// From: apps/research/src/scripts
// To:   packages/contracts/test/evm/TronLightClient/fixtures
const TRON_FIXTURES_DIR = resolve(
  __dirname,
  "../../../../packages/contracts/test/evm/TronLightClient/fixtures"
);

function toHex(bytes: Uint8Array | Buffer): string {
  return Buffer.from(bytes).toString("hex");
}

function toHex0x(bytes: Uint8Array | Buffer): Hex {
  return `0x${Buffer.from(bytes).toString("hex")}` as Hex;
}

function tronWitnessAddressToEvmAddress(bytes: Uint8Array | Buffer): Address | null {
  // Tron witness address is 21 bytes: 0x41 prefix + 20-byte EVM address
  if (bytes.length !== 21 || bytes[0] !== 0x41) return null;
  return `0x${Buffer.from(bytes.subarray(1)).toString("hex")}` as Address;
}

// Compute Tron-style blockId and related reference values for testing:
// - blockHash: sha256(BlockHeader_raw)
// - blockId:   uint64(blockNumber) || sha256(BlockHeader_raw)[8:]
function computeBlockId(raw: BlockHeader_raw): {
  blockId: Hex;
  blockNumber: bigint;
  blockHash: Hex;
  rawHeaderBytes: Hex;
} {
  const rawBytes = BlockHeader_raw.encode(raw).finish();
  const digest = sha256(rawBytes); // Uint8Array, 32 bytes

  const digestHex = toHex(digest);
  const digestBig = BigInt("0x" + digestHex);

  const numberBig = BigInt(raw.number.toString()); // Long (uint64) -> string -> bigint

  // For Tron blockId, the upper 8 bytes are the block number (uint64),
  // and the lower 24 bytes are the tail of sha256(BlockHeader_raw).
  const tailMask = (1n << 192n) - 1n;
  const tail = digestBig & tailMask;

  const blockIdBig = (numberBig << 192n) | tail;
  const blockIdHex = blockIdBig.toString(16).padStart(64, "0");

  return {
    blockId: ("0x" + blockIdHex) as Hex,
    blockNumber: numberBig,
    blockHash: toHex0x(digest),
    rawHeaderBytes: toHex0x(rawBytes),
  };
}

type Fixture = {
  network: "tron-mainnet";
  startBlock: string;
  endBlock: string;
  startingBlockId: Hex;
  endingBlockId: Hex;
  srs: Address[]; // always length 27
  compressedTronBlockMetadata: Hex;
  compressedSignatures: Hex;
  blockNumbers: string[];
  blockIds: Hex[];
  // Extra reference data for Layer 0 (encoding / hashing) tests.
  // These are per-block arrays aligned with blockNumbers/blockIds.
  blockHashes: Hex[]; // sha256(BlockHeader_raw)
  blockHeaderRawBytes: Hex[]; // raw BlockHeader_raw.encode(raw) bytes
  witnessEvmAddresses: Address[]; // derived from raw.witnessAddress
  witnessIndices: number[]; // witness index written into compressedTronBlockMetadata
  witnessSignatures: Hex[]; // 65-byte [r|s|v] signatures as hex
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
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
    })
  );

  const argv = process.argv.slice(2);

  // Support both direct invocation:
  //   tsx genTronFixture.ts <startBlock> <endBlock> [outPath]
  // and runner style:
  //   tsx src/run.ts genTronFixture <startBlock> <endBlock> [outPath]
  let argOffset = 0;
  if (argv.length > 0 && !/^[0-9]+$/.test(argv[0]!)) {
    // First arg is script name/path, not a block number.
    argOffset = 1;
  }

  const args = argv.slice(argOffset);
  if (args.length < 2 || args.length > 3) {
    // eslint-disable-next-line no-console
    console.error(
      "Usage: tsx genTronFixture.ts <startBlock> <endBlock> [outPath]\n" +
        "Or: tsx src/run.ts genTronFixture <startBlock> <endBlock> [outPath]\n" +
        "Example: tsx genTronFixture.ts 55000000 55000099 test/fixtures/tron_55000000_55000099.json"
    );
    process.exit(1);
  }

  const startBlock = Number(args[0]!);
  const endBlock = Number(args[1]!);
  if (!Number.isInteger(startBlock) || !Number.isInteger(endBlock) || startBlock > endBlock) {
    throw new Error("Invalid startBlock/endBlock");
  }

  const outPath = args[2]
    ? resolve(args[2])
    : resolve(TRON_FIXTURES_DIR, `tron_${startBlock}_${endBlock}.json`);

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  log.info("Generating Tron fixture", { startBlock, endBlock, outPath });

  const numBlocks = endBlock - startBlock + 1;
  const blocks: BlockExtention[] = [];

  for (let n = startBlock; n <= endBlock; n++) {
    const block = await fetchBlock(wallet, callOpts, n);
    if (!block.blockHeader || !block.blockHeader.rawData) {
      throw new Error(`Block ${n} missing header/rawData`);
    }
    blocks.push(block);
  }

  const firstRaw = blocks[0]!.blockHeader!.rawData!;
  // Tron header's parentHash field is the parent blockId (32 bytes).
  // We'll use this as startingBlockId in the fixture.
  const startingBlockId = toHex0x(firstRaw.parentHash);

  // SRS construction (unique SR addresses seen in this range).
  const srsMap = new Map<Address, number>();
  const srsList: Address[] = [];

  // Pre-allocate buffers
  const TRON_BLOCK_METADATA_SIZE = 69; // 32 + 32 + 4 + 1
  const SIGNATURE_SIZE = 65;

  const metadataBuf = Buffer.alloc(numBlocks * TRON_BLOCK_METADATA_SIZE);
  const sigsBuf = Buffer.alloc(numBlocks * SIGNATURE_SIZE);

  const blockNumbers: string[] = [];
  const blockIds: Hex[] = [];
  const blockHashes: Hex[] = [];
  const blockHeaderRawBytes: Hex[] = [];
  const witnessEvmAddresses: Address[] = [];
  const witnessIndices: number[] = [];
  const witnessSignatures: Hex[] = [];

  let metaOffset = 0;
  let sigOffset = 0;

  for (let i = 0; i < blocks.length; i++) {
    const header = blocks[i]!.blockHeader!;
    const raw = header.rawData!;

    const { blockId, blockNumber, blockHash, rawHeaderBytes } = computeBlockId(
      raw as BlockHeader_raw
    );
    blockNumbers.push(blockNumber.toString());
    blockIds.push(blockId);
    blockHashes.push(blockHash);
    blockHeaderRawBytes.push(rawHeaderBytes);

    // Witness address -> EVM address, derive index into srs[]
    const tronWitness = raw.witnessAddress;
    if (!tronWitness) {
      throw new Error(`Block ${blockNumber} missing witnessAddress`);
    }
    const evmAddr = tronWitnessAddressToEvmAddress(tronWitness);
    if (!evmAddr) {
      throw new Error(`Block ${blockNumber} has invalid witnessAddress bytes`);
    }

    let idx = srsMap.get(evmAddr);
    if (idx === undefined) {
      idx = srsList.length;
      if (idx >= 27) {
        throw new Error(
          `More than 27 unique SR addresses encountered in block range; cannot fit into bytes20[27]`
        );
      }
      srsMap.set(evmAddr, idx);
      srsList.push(evmAddr);
    }

    // CompressedTronBlockMetadata layout per block:
    // [0..31]  parentHash (bytes32)  -> raw.parentHash
    // [32..63] txTrieRoot (bytes32)  -> raw.txTrieRoot
    // [64..67] timestamp (uint32, big-endian seconds)
    // [68]     witnessAddressIndex (uint8)
    const parentHash = raw.parentHash;
    const txTrieRoot = raw.txTrieRoot;

    if (!parentHash || !txTrieRoot) {
      throw new Error(`Block ${blockNumber} missing parentHash or txTrieRoot`);
    }

    parentHash.copy(metadataBuf, metaOffset);
    metaOffset += 32;

    txTrieRoot.copy(metadataBuf, metaOffset);
    metaOffset += 32;

    const tsMs = BigInt(raw.timestamp.toString()); // ms
    const tsSec = tsMs / 1000n;
    if (tsSec > BigInt(0xffffffff)) {
      throw new Error(`Timestamp seconds overflow uint32 for block ${blockNumber}`);
    }
    metadataBuf.writeUInt32BE(Number(tsSec), metaOffset);
    metaOffset += 4;

    metadataBuf.writeUInt8(idx, metaOffset);
    metaOffset += 1;
    witnessEvmAddresses.push(evmAddr);
    witnessIndices.push(idx);

    // Signatures: Tron stores [r(32) | s(32) | v(1)].
    const witnessSig = header.witnessSignature as Buffer | undefined;
    if (!witnessSig || witnessSig.length < SIGNATURE_SIZE) {
      throw new Error(`Block ${blockNumber} missing or invalid witnessSignature`);
    }

    witnessSig.copy(sigsBuf, sigOffset, 0, SIGNATURE_SIZE);
    sigOffset += SIGNATURE_SIZE;
    witnessSignatures.push(toHex0x(witnessSig.subarray(0, SIGNATURE_SIZE)));

    log.info("Generated block", {
      blockNumber,
      blockId,
      blockHash,
      rawHeaderBytes,
      witnessEvmAddress: evmAddr,
      witnessIndex: idx,
    });
  }

  if (metaOffset !== metadataBuf.length) {
    throw new Error("Metadata buffer offset mismatch");
  }
  if (sigOffset !== sigsBuf.length) {
    throw new Error("Signature buffer offset mismatch");
  }

  // Pad SRS to 27 entries with zero addresses.
  const srsFixed: Address[] = new Array(27).fill(
    "0x0000000000000000000000000000000000000000"
  ) as Address[];
  for (const [addr, idx] of srsMap.entries()) {
    srsFixed[idx] = addr;
  }

  const endingBlockId = blockIds[blockIds.length - 1]!;

  const fixture: Fixture = {
    network: "tron-mainnet",
    startBlock: String(startBlock),
    endBlock: String(endBlock),
    startingBlockId,
    endingBlockId,
    srs: srsFixed,
    compressedTronBlockMetadata: toHex0x(metadataBuf),
    compressedSignatures: toHex0x(sigsBuf),
    blockNumbers,
    blockIds,
    blockHashes,
    blockHeaderRawBytes,
    witnessEvmAddresses,
    witnessIndices,
    witnessSignatures,
  };

  writeFileSync(outPath, JSON.stringify(fixture, null, 2));
  log.info("Wrote Tron fixture", { outPath });
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});
