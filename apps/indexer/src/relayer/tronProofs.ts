import { sha256 as sha256Noble } from "@noble/hashes/sha2.js";
import type { BlockExtention } from "@untron/tron-protocol/api";
import { Transaction } from "@untron/tron-protocol/tron";
import type { Hex } from "viem";
import { Transaction_raw } from "@untron/tron-protocol/tron";

function toHex0x(bytes: Uint8Array | Buffer): Hex {
  return `0x${Buffer.from(bytes).toString("hex")}` as Hex;
}

function sha256(bytes: Uint8Array | Buffer): Buffer {
  return Buffer.from(sha256Noble(bytes));
}

class MerkleNode {
  digest: Buffer;
  left?: MerkleNode;
  right?: MerkleNode;
  parent?: MerkleNode;

  constructor(digest: Buffer, left?: MerkleNode, right?: MerkleNode) {
    this.digest = digest;
    this.left = left;
    this.right = right;
    if (left) left.parent = this;
    if (right) right.parent = this;
  }

  isRoot() {
    return !this.parent;
  }

  isLeftChild() {
    return this.parent?.left === this;
  }

  getAncestor(degree: number) {
    let curr: MerkleNode = this;
    while (degree-- > 0 && curr.parent) curr = curr.parent;
    return curr;
  }
}

class MerkleLeaf extends MerkleNode {
  constructor(
    public data: Buffer,
    digest: Buffer
  ) {
    super(digest);
  }
}

function decompose(n: number): number[] {
  const exponents: number[] = [];
  for (let i = 0; i < 32; i++) {
    if ((n >> i) & 1) exponents.push(i);
  }
  return exponents;
}

class InMemoryMerkleTree {
  root?: MerkleNode;
  leaves: MerkleLeaf[] = [];

  appendEntry(data: Buffer) {
    const digest = sha256(data);
    const tail = new MerkleLeaf(data, digest);

    if (this.leaves.length === 0) {
      this.leaves.push(tail);
      this.root = tail;
      return;
    }

    const node = this.leaves[this.leaves.length - 1]!.getAncestor(
      decompose(this.leaves.length)[0]!
    );
    this.leaves.push(tail);
    const parentDigest = sha256(Buffer.concat([node.digest, tail.digest]));

    if (node.isRoot()) {
      this.root = new MerkleNode(parentDigest, node, tail);
      return;
    }

    const curr = node.parent!;
    const subRoot = new MerkleNode(parentDigest, node, tail);
    curr.right = subRoot;
    subRoot.parent = curr;

    let ptr = curr;
    while (true) {
      ptr.digest = sha256(Buffer.concat([ptr.left!.digest, ptr.right!.digest]));
      if (!ptr.parent) break;
      ptr = ptr.parent;
    }
  }

  getProof(index: number) {
    if (index < 0 || index >= this.leaves.length) throw new Error("Index out of bounds");

    const leafNode = this.leaves[index]!;
    const proof: Buffer[] = [];
    let curr: MerkleNode = leafNode;
    let pathBits = 0;
    let depth = 0;

    while (curr.parent) {
      const parent = curr.parent;
      if (curr.isLeftChild()) {
        proof.push(parent.right!.digest);
      } else {
        proof.push(parent.left!.digest);
        pathBits |= 1 << depth;
      }
      curr = parent;
      depth++;
    }

    return {
      leaf: leafNode.digest,
      proof,
      root: this.root!.digest,
      index: pathBits,
      totalLeaves: this.leaves.length,
    };
  }
}

export type TronTxMerkleProof = {
  encodedTx: Hex;
  proof: readonly Hex[];
  index: bigint;
};

export function computeTronTxIdFromEncodedTx(encodedTx: Hex): Hex {
  const bytes = Buffer.from(encodedTx.slice(2), "hex");
  const tx = Transaction.decode(bytes);
  if (!tx.rawData) throw new Error("Tron tx missing rawData");

  const rawBytes = Buffer.from(Transaction_raw.encode(tx.rawData).finish());
  const txId = sha256(rawBytes);
  return toHex0x(txId);
}

export function computeTronTxMerkleProof(args: {
  block: BlockExtention;
  txidHex: string;
}): TronTxMerkleProof {
  const txidHexLower = args.txidHex.replace(/^0x/i, "").toLowerCase();

  const txs = args.block.transactions ?? [];
  if (txs.length === 0) throw new Error("Block has no transactions");

  const tree = new InMemoryMerkleTree();
  let targetIndex = -1;
  let encodedTx: Buffer | null = null;

  for (let i = 0; i < txs.length; i++) {
    const txExt = txs[i]!;
    const tx: Transaction | undefined = txExt.transaction;
    if (!tx) throw new Error("Missing transaction in block transaction extension");

    const encoded = Buffer.from(Transaction.encode(tx).finish());
    tree.appendEntry(encoded);

    const txid = Buffer.from(txExt.txid ?? Buffer.alloc(0))
      .toString("hex")
      .toLowerCase();
    if (txid === txidHexLower) {
      targetIndex = i;
      encodedTx = encoded;
    }
  }

  if (targetIndex === -1 || !encodedTx) throw new Error("Transaction not found in block");

  const headerRoot = args.block.blockHeader?.rawData?.txTrieRoot;
  if (!headerRoot || headerRoot.length !== 32) {
    throw new Error("Block header does not contain txTrieRoot");
  }

  const result = tree.getProof(targetIndex);

  const calculatedRootHex = toHex0x(result.root);
  const headerRootHex = toHex0x(headerRoot);
  if (calculatedRootHex !== headerRootHex) {
    throw new Error(
      `Tron txTrieRoot mismatch (calculated=${calculatedRootHex}, header=${headerRootHex})`
    );
  }

  return {
    encodedTx: toHex0x(encodedTx),
    proof: result.proof.map((p) => toHex0x(p)),
    index: BigInt(result.index),
  };
}

export type TronBlockForLightClient = {
  parentHash: Buffer;
  txTrieRoot: Buffer;
  timestampSec: number;
  witnessAddress: Buffer;
  witnessSignature: Buffer;
};

export function parseTronBlockForLightClient(block: BlockExtention): TronBlockForLightClient {
  const header = block.blockHeader;
  const raw = header?.rawData;
  if (!header || !raw) throw new Error("Tron block missing header/rawData");

  const parentHash = raw.parentHash as Buffer | undefined;
  const txTrieRoot = raw.txTrieRoot as Buffer | undefined;
  const witnessAddress = raw.witnessAddress as Buffer | undefined;
  const witnessSignature = header.witnessSignature as Buffer | undefined;

  if (!parentHash || parentHash.length !== 32) throw new Error("Tron block missing parentHash");
  if (!txTrieRoot || txTrieRoot.length !== 32) throw new Error("Tron block missing txTrieRoot");
  if (!witnessAddress || witnessAddress.length !== 21)
    throw new Error("Tron block missing witnessAddress");
  if (!witnessSignature || witnessSignature.length < 65)
    throw new Error("Tron block missing witnessSignature");

  const timestampMs = BigInt(raw.timestamp.toString());
  const timestampSecBig = timestampMs / 1000n;
  if (timestampSecBig < 0n || timestampSecBig > 0xffff_ffffn) {
    throw new Error("Tron block timestamp out of uint32 range");
  }

  return {
    parentHash,
    txTrieRoot,
    timestampSec: Number(timestampSecBig),
    witnessAddress,
    witnessSignature: witnessSignature.subarray(0, 65),
  };
}

export function encodeTronLightClientMetadataAndSignatures(args: {
  blocks: readonly TronBlockForLightClient[];
  witnessIndexByTronOwnerAddressHex: ReadonlyMap<string, number>;
}): { compressedTronBlockMetadata: Hex; compressedSignatures: Hex } {
  const TRON_BLOCK_METADATA_SIZE = 69;
  const SIGNATURE_SIZE = 65;

  const metadataBuf = Buffer.alloc(args.blocks.length * TRON_BLOCK_METADATA_SIZE);
  const sigsBuf = Buffer.alloc(args.blocks.length * SIGNATURE_SIZE);

  let metaOffset = 0;
  let sigOffset = 0;

  for (const block of args.blocks) {
    const ownerHex = toHex0x(block.witnessAddress).slice(2).toLowerCase();
    const idx = args.witnessIndexByTronOwnerAddressHex.get(ownerHex);
    if (idx === undefined) {
      throw new Error(`Unknown witnessAddress (not in TronLightClient srs[]): 0x${ownerHex}`);
    }
    if (!Number.isInteger(idx) || idx < 0 || idx > 26) {
      throw new Error(`Invalid witness index for 0x${ownerHex}: ${idx}`);
    }

    block.parentHash.copy(metadataBuf, metaOffset);
    metaOffset += 32;

    block.txTrieRoot.copy(metadataBuf, metaOffset);
    metaOffset += 32;

    metadataBuf.writeUInt32BE(block.timestampSec, metaOffset);
    metaOffset += 4;

    metadataBuf.writeUInt8(idx, metaOffset);
    metaOffset += 1;

    block.witnessSignature.copy(sigsBuf, sigOffset, 0, SIGNATURE_SIZE);
    sigOffset += SIGNATURE_SIZE;
  }

  return {
    compressedTronBlockMetadata: toHex0x(metadataBuf),
    compressedSignatures: toHex0x(sigsBuf),
  };
}

export function encodeStoreOffsets16(offsets: readonly number[]): bigint {
  if (offsets.length > 16) throw new Error("storeOffsets16 supports up to 16 offsets");

  const sorted = [...offsets].sort((a, b) => a - b);
  for (let i = 0; i < sorted.length; i++) {
    const off = sorted[i]!;
    if (!Number.isInteger(off) || off < 0 || off > 0xffff - 1) {
      throw new Error(`Invalid store offset: ${off}`);
    }
    if (i > 0 && off <= sorted[i - 1]!) {
      throw new Error(
        `storeOffsets16 offsets must be strictly increasing: ${sorted[i - 1]} -> ${off}`
      );
    }
  }

  const SENTINEL = 0xffffn;
  let packed = 0n;

  for (let lane = 0; lane < 16; lane++) {
    const v = lane < sorted.length ? BigInt(sorted[lane]!) : SENTINEL;
    packed |= (v & 0xffffn) << (16n * BigInt(lane));
  }

  return packed;
}
