import Long from "long";
import { z } from "zod";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { createTronClients } from "@untron/tron-protocol";
import type {
  BlockExtention,
  NumberMessage,
  TransactionExtention,
} from "@untron/tron-protocol/api";
import { Transaction } from "@untron/tron-protocol/tron";
import { BlockHeader_raw } from "@untron/tron-protocol/tron";
import { sha256 } from "@noble/hashes/sha2.js";

type MerkleProof = {
  leaf: Buffer;
  proof: Buffer[];
  root: Buffer;
  index: number;
  totalLeaves: number;
};

function toHex0x(buf: Uint8Array | Buffer): string {
  return `0x${Buffer.from(buf).toString("hex")}`;
}

function parseArgs(): { blockNumber: Long; txIdHex: string } {
  const argv = process.argv.slice(2);

  // Support both:
  // - tsx tronTxMerkleProof.ts <blockNumber> <txId>
  // - tsx src/run.ts tronTxMerkleProof <blockNumber> <txId>
  let argOffset = 0;
  if (argv.length > 0 && !/^[0-9]+$/.test(argv[0]!)) {
    argOffset = 1;
  }

  const args = argv.slice(argOffset);
  if (args.length < 2) {
    // eslint-disable-next-line no-console
    console.error(
      "Usage: tsx tronTxMerkleProof.ts <blockNumber> <txId>\n" +
        "   or: tsx src/run.ts tronTxMerkleProof <blockNumber> <txId>\n" +
        "Example: tsx tronTxMerkleProof.ts 55000000 0xabcdef..."
    );
    process.exit(1);
  }

  const blockNumberStr = args[0]!;
  if (!/^[0-9]+$/.test(blockNumberStr)) {
    throw new Error(`Invalid blockNumber '${blockNumberStr}' – expected decimal integer`);
  }

  let txIdHex = args[1]!;
  if (txIdHex.startsWith("0x") || txIdHex.startsWith("0X")) {
    txIdHex = txIdHex.slice(2);
  }
  if (!/^[0-9a-fA-F]{64}$/.test(txIdHex)) {
    throw new Error(
      `Invalid txId '${args[1]}' – expected 32-byte hex string (optionally 0x-prefixed)`
    );
  }

  return {
    blockNumber: Long.fromString(blockNumberStr, true),
    txIdHex: txIdHex.toLowerCase(),
  };
}

async function fetchBlockByNumber(
  wallet: ReturnType<typeof createTronClients>["wallet"],
  metadata: ReturnType<typeof createTronClients>["callOpts"]["metadata"],
  blockNumber: Long
): Promise<BlockExtention> {
  const req: NumberMessage = { num: blockNumber };
  return await new Promise((resolve, reject) => {
    wallet.getBlockByNum2(req, metadata, (err, res) => {
      if (err || !res) return reject(err ?? new Error("Empty response from getBlockByNum2"));
      resolve(res);
    });
  });
}

// ---- Merkle tree helpers (ported to mirror pymerkle InmemoryTree behaviour) ----

class MerkleNode {
  digest: Buffer;
  left?: MerkleNode;
  right?: MerkleNode;
  parent?: MerkleNode;

  constructor(digest: Buffer, left?: MerkleNode, right?: MerkleNode) {
    this.digest = digest;
    this.left = left;
    if (left) {
      left.parent = this;
    }
    this.right = right;
    if (right) {
      right.parent = this;
    }
    this.parent = undefined;
  }

  isRoot(): boolean {
    return !this.parent;
  }

  isLeaf(): boolean {
    return !this.left && !this.right;
  }

  isLeftChild(): boolean {
    const parent = this.parent;
    if (!parent) return false;
    return parent.left === this;
  }

  isRightChild(): boolean {
    const parent = this.parent;
    if (!parent) return false;
    return parent.right === this;
  }

  getAncestor(degree: number): MerkleNode {
    let curr: MerkleNode = this;
    let remaining = degree;
    while (remaining > 0 && curr.parent) {
      curr = curr.parent;
      remaining -= 1;
    }
    return curr;
  }
}

class MerkleLeaf extends MerkleNode {
  data: Buffer;

  constructor(data: Buffer, digest: Buffer) {
    super(digest);
    this.data = data;
  }
}

function log2Int(n: number): number {
  // Exponent of the largest power of two <= n (n > 0).
  let k = 0;
  let x = n;
  while (x >> 1) {
    k += 1;
    x >>= 1;
  }
  return k;
}

function decompose(n: number): number[] {
  const exponents: number[] = [];
  let i = 1;
  while (i < n + 1) {
    if (i & n) {
      let p = -1;
      let j = i;
      while (j) {
        j >>= 1;
        p += 1;
      }
      exponents.push(p);
    }
    i <<= 1;
  }
  return exponents;
}

class InMemoryMerkleTree {
  root?: MerkleNode;
  private readonly leaves: MerkleLeaf[] = [];

  private hashEntry(data: Buffer): Buffer {
    // disable_security=True in pymerkle → plain sha256(data)
    return Buffer.from(sha256(data));
  }

  private hashNodes(left: Buffer, right: Buffer): Buffer {
    // disable_security=True in pymerkle → plain sha256(left + right)
    return Buffer.from(sha256(Buffer.concat([left, right])));
  }

  private getLastMaximalSubroot(): MerkleNode {
    const leafCount = this.leaves.length;
    const exponents = decompose(leafCount);
    const degree = exponents[0]!;
    return this.leaves[leafCount - 1]!.getAncestor(degree);
  }

  appendEntry(data: Buffer): void {
    const digest = this.hashEntry(data);
    const tail = new MerkleLeaf(data, digest);

    if (this.leaves.length === 0) {
      this.leaves.push(tail);
      this.root = tail;
      return;
    }

    const node = this.getLastMaximalSubroot();
    this.leaves.push(tail);
    const parentDigest = this.hashNodes(node.digest, tail.digest);

    if (node.isRoot()) {
      this.root = new MerkleNode(parentDigest, node, tail);
      return;
    }

    let curr = node.parent!;
    curr.right = new MerkleNode(parentDigest, node, tail);

    // Recompute digests up to the root
    // (mirrors InmemoryTree._store_leaf in pymerkle)
    // while curr:
    //   curr.digest = hash_nodes(curr.left.digest, curr.right.digest)
    //   curr = curr.parent
    // We keep the same semantics but in TypeScript.
    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (!curr.left || !curr.right) {
        throw new Error("Merkle tree node missing child during recomputation");
      }
      curr.digest = this.hashNodes(curr.left.digest, curr.right.digest);
      if (!curr.parent) {
        break;
      }
      curr = curr.parent;
    }
  }

  getLeafCount(): number {
    return this.leaves.length;
  }

  private getLeafDigest(indexOneBased: number): Buffer {
    if (indexOneBased < 1 || indexOneBased > this.leaves.length) {
      throw new Error(`Leaf index ${indexOneBased} out of range`);
    }
    return this.leaves[indexOneBased - 1]!.digest;
  }

  private getLeavesSlice(offset: number, width: number): Buffer[] {
    // offset: starting position counting from zero
    // width: number of leaves to consider
    return this.leaves.slice(offset, offset + width).map((l) => l.digest);
  }
  inclusionPath(offset: number): { rule: number[]; path: Buffer[] } {
    // Backwards-compatible wrapper: offset is 0-based index.
    return this.proveInclusion(offset + 1);
  }

  // ---- Naive algorithms ported from BaseMerkleTree._get_root_naive and
  // _inclusion_path_naive. These are simpler and guaranteed to match the
  // reference behaviour of pymerkle (disable_security=True) for any tree
  // size, including non-powers-of-two.

  private getRootDigestForRangeNaive(start: number, limit: number): Buffer {
    // Computes the Merkle root for the leaf range [start, limit),
    // where start is 0-based and limit is a 1-based upper bound.

    // Debug: mirror py_root_naive_* logs when computing:
    // - the full-tree root [0, size)
    // - any subtree entirely inside the left half [0,256)
    const fullSize = this.leaves.length;
    const isFullRange = start === 0 && limit === fullSize;
    const isLeftSubtree = start >= 0 && limit <= 256;
    if (isFullRange || isLeftSubtree) {
      // eslint-disable-next-line no-console
      console.log(
        JSON.stringify({
          tag: "ts_root_naive_call",
          range: [start, limit],
        })
      );
    }

    if (limit === start) {
      // Hash of empty; should not occur for valid non-empty ranges,
      // but keep it for completeness.
      const h = Buffer.from(sha256(new Uint8Array()));
      if (isLeftSubtree) {
        // eslint-disable-next-line no-console
        console.log(
          JSON.stringify({
            tag: "ts_root_naive_base_empty_left",
            range: [start, limit],
            hash: h.toString("hex"),
          })
        );
      }
      return h;
    }

    if (limit === start + 1) {
      // Base case: single leaf.
      const leaf = this.getLeafDigest(limit);
      if (isLeftSubtree) {
        // eslint-disable-next-line no-console
        console.log(
          JSON.stringify({
            tag: "ts_root_naive_base_leaf_left",
            range: [start, limit],
            leafIndexOneBased: limit,
            leaf: leaf.toString("hex"),
          })
        );
      }
      return leaf;
    }

    let k = 1 << log2Int(limit - start);
    if (k === limit - start) {
      k >>= 1;
    }

    const left = this.getRootDigestForRangeNaive(start, start + k);
    const right = this.getRootDigestForRangeNaive(start + k, limit);
    const node = this.hashNodes(left, right);

    if (isLeftSubtree) {
      // eslint-disable-next-line no-console
      console.log(
        JSON.stringify({
          tag: "ts_root_naive_combine_left",
          range: [start, limit],
          k,
          lnode: left.toString("hex"),
          rnode: right.toString("hex"),
          node: node.toString("hex"),
        })
      );
    }

    if (isFullRange) {
      // eslint-disable-next-line no-console
      console.log(
        JSON.stringify({
          tag: "ts_root_naive_combine_full",
          range: [start, limit],
          k,
          lnode: left.toString("hex"),
          rnode: right.toString("hex"),
          node: node.toString("hex"),
        })
      );
    }

    return node;
  }

  getRootDigest(): Buffer {
    // Public entry point: full-tree root, equivalent to
    // BaseMerkleTree._get_root_naive(0, size).
    const size = this.leaves.length;
    if (size === 0) {
      return Buffer.from(sha256(new Uint8Array()));
    }
    return this.getRootDigestForRangeNaive(0, size);
  }

  private inclusionPathNaive(
    start: number,
    offset: number,
    limit: number,
    bit: number
  ): { rule: number[]; path: Buffer[] } {
    // Port of BaseMerkleTree._inclusion_path_naive.
    // start: leftmost leaf index (0-based)
    // offset: target leaf index (0-based)
    // limit: one-past-right leaf index (1-based upper bound)

    const fullSize = this.leaves.length;
    const isFullRangeLeaf0 = start === 0 && offset === 0 && limit === fullSize;
    if (isFullRangeLeaf0) {
      // eslint-disable-next-line no-console
      console.log(
        JSON.stringify({
          tag: "ts_inclusion_naive_call",
          start,
          offset,
          limit,
          bit,
        })
      );
    }

    if (offset === start && start === limit - 1) {
      const node = this.getLeafDigest(offset + 1);
      if (isFullRangeLeaf0) {
        // eslint-disable-next-line no-console
        console.log(
          JSON.stringify({
            tag: "ts_inclusion_naive_base",
            start,
            offset,
            limit,
            bit,
            node: node.toString("hex"),
          })
        );
      }
      return { rule: [bit], path: [node] };
    }

    let k = 1 << log2Int(limit - start);
    if (k === limit - start) {
      k >>= 1;
    }

    if (offset < start + k) {
      const left = this.inclusionPathNaive(start, offset, start + k, 0);
      const node = this.getRootDigestForRangeNaive(start + k, limit);
      if (isFullRangeLeaf0) {
        // eslint-disable-next-line no-console
        console.log(
          JSON.stringify({
            tag: "ts_inclusion_naive_step",
            branch: "left",
            start,
            offset,
            limit,
            k,
            bitIn: bit,
            subroot: node.toString("hex"),
          })
        );
      }
      return {
        rule: [...left.rule, bit],
        path: [...left.path, node],
      };
    }

    const right = this.inclusionPathNaive(start + k, offset, limit, 1);
    const node = this.getRootDigestForRangeNaive(start, start + k);
    if (isFullRangeLeaf0) {
      // eslint-disable-next-line no-console
      console.log(
        JSON.stringify({
          tag: "ts_inclusion_naive_step",
          branch: "right",
          start,
          offset,
          limit,
          k,
          bitIn: bit,
          subroot: node.toString("hex"),
        })
      );
    }
    return {
      rule: [...right.rule, bit],
      path: [...right.path, node],
    };
  }

  // Direct port of InmemoryTree._inclusion_path_fallback from pymerkle,
  // using the explicit node graph (parent/left/right pointers).
  private inclusionPathFallback(offset: number): { rule: number[]; path: Buffer[] } {
    const base = this.leaves[offset]!;
    let bit = base.isRightChild() ? 1 : 0;
    const path: Buffer[] = [base.digest];
    const rule: number[] = [bit];

    let curr: MerkleNode = base;

    while (curr.parent) {
      const parent = curr.parent;
      let digest: Buffer;

      if (curr.isLeftChild()) {
        if (!parent.right) {
          throw new Error("Missing right child while building fallback inclusion path");
        }
        digest = parent.right.digest;
        bit = parent.isLeftChild() ? 0 : 1;
      } else {
        if (!parent.left) {
          throw new Error("Missing left child while building fallback inclusion path");
        }
        digest = parent.left.digest;
        bit = parent.isRightChild() ? 1 : 0;
      }

      rule.push(bit);
      path.push(digest);
      curr = parent;
    }

    // Last bit is insignificant; set to zero to match pymerkle exactly.
    if (rule.length > 0) {
      rule[rule.length - 1] = 0;
    }

    return { rule, path };
  }

  proveInclusion(indexOneBased: number): { rule: number[]; path: Buffer[] } {
    const size = this.getLeafCount();
    if (size === 0) {
      throw new Error("Cannot prove inclusion on an empty Merkle tree");
    }
    if (!(indexOneBased > 0 && indexOneBased <= size)) {
      throw new Error(`Provided index ${indexOneBased} is out of bounds`);
    }

    const offset = indexOneBased - 1;
    // Use the same path-computation strategy as InmemoryTree._inclusion_path_fallback
    // when querying the full tree, to mirror the Python implementation exactly.
    if (offset >= 0 && offset < size) {
      return this.inclusionPathFallback(offset);
    }

    return this.inclusionPathNaive(0, offset, size, 0);
  }
}

function soliditifyFromRuleAndPath(
  rule: number[],
  path: Buffer[]
): {
  leaf: Buffer;
  proof: Buffer[];
  index: number;
} {
  if (path.length === 0) {
    throw new Error("Merkle path is empty");
  }

  const leaf = path[0]!;
  const proof = path.slice(1);

  // index = int("".join(str(x) for x in rule)[::-1], 2)
  let index = 0;
  for (let i = 0; i < rule.length; i++) {
    const bit = rule[i];
    if (bit !== 0 && bit !== 1) {
      throw new Error(`Invalid Merkle rule bit '${bit}' – expected 0 or 1`);
    }
    if (bit === 1) {
      index |= 1 << i;
    }
  }

  return { leaf, proof, index };
}

function computeMerkleProofFromTxs(
  txs: TransactionExtention[],
  targetTxIdHex: string
): MerkleProof {
  if (txs.length === 0) {
    throw new Error("Block has no transactions");
  }

  // Reconstruct per-tx "leaf hash" as sha256(Transaction.encode(transaction)).
  // The Merkle tree order follows the transaction order returned by the API.
  const items = txs.map((txExt) => {
    const txid = txExt.txid as Buffer | undefined;
    if (!txid || txid.length === 0) {
      throw new Error("Encountered TransactionExtention without txid");
    }
    const txidHex = Buffer.from(txid).toString("hex").toLowerCase();

    const tx = txExt.transaction;
    if (!tx) {
      throw new Error("Encountered TransactionExtention without transaction");
    }

    const encoded = Transaction.encode(tx).finish();
    return {
      txidHex,
      encoded: Buffer.from(encoded),
    };
  });

  const targetIndex = items.findIndex((item) => item.txidHex === targetTxIdHex);

  if (targetIndex === -1) {
    throw new Error(
      "Target transaction not found in block's transactions (by txid). " +
        "Make sure you passed the txid for this block."
    );
  }

  // Debug: inspect 1-based leaf index 90 (0-based offset 89) to compare with Python.
  const debugIdx = 89;
  if (debugIdx >= 0 && debugIdx < items.length) {
    const debugItem = items[debugIdx]!;
    const debugLeaf = Buffer.from(sha256(debugItem.encoded));
    // eslint-disable-next-line no-console
    console.log(
      JSON.stringify({
        tag: "ts_debug_leaf_90",
        indexOneBased: debugIdx + 1,
        txId: debugItem.txidHex,
        rawTx: debugItem.encoded.toString("hex"),
        leaf: debugLeaf.toString("hex"),
      })
    );
  }

  const tree = new InMemoryMerkleTree();
  for (const item of items) {
    tree.appendEntry(item.encoded);
  }

  if (!tree.root) {
    throw new Error("Merkle tree root is undefined after building from transactions");
  }

  // Debug: dump the first 256 leaf digests to compare with Python.
  const prefixCount = Math.min(256, tree.getLeafCount());
  const leafDigests: string[] = [];
  for (let i = 1; i <= prefixCount; i++) {
    const leaf = (tree as any).getLeafDigest(i) as Buffer;
    leafDigests.push(leaf.toString("hex"));
  }
  // eslint-disable-next-line no-console
  console.log(
    JSON.stringify({
      tag: "ts_leaves_prefix",
      count: prefixCount,
      leaves: leafDigests,
    })
  );

  const { rule, path } = tree.inclusionPath(targetIndex);
  // Also compute the naive algorithmic path for debugging, to compare with
  // pymerkle's BaseMerkleTree._inclusion_path behaviour.
  const { rule: naiveRule, path: naivePath } = (tree as any).inclusionPathNaive(
    0,
    targetIndex,
    tree.getLeafCount(),
    0
  ) as { rule: number[]; path: Buffer[] };
  const solid = soliditifyFromRuleAndPath(rule, path);

  // Debug logging to compare with the original Python script.
  // This logs the exact Merkle inputs/outputs for the selected transaction.
  const targetItem = items[targetIndex]!;
  // eslint-disable-next-line no-console
  console.log(
    JSON.stringify({
      tag: "ts_merkle",
      txIndex: targetIndex,
      txId: targetItem.txidHex,
      leaf: solid.leaf.toString("hex"),
      root: tree.getRootDigest().toString("hex"),
      rule,
      path: path.map((d: Buffer) => d.toString("hex")),
      debug: {
        naiveRule,
        naivePath: naivePath.map((d: Buffer) => d.toString("hex")),
      },
      index: solid.index,
      totalLeaves: tree.getLeafCount(),
      rawTx: targetItem.encoded.toString("hex"),
    })
  );

  return {
    leaf: solid.leaf,
    proof: solid.proof,
    root: tree.getRootDigest(),
    index: solid.index,
    totalLeaves: tree.getLeafCount(),
  };
}

function verifyProofLocal(merkle: MerkleProof): boolean {
  // Port of the Python verify_proof(proof, root, leaf, index)
  let hash = Buffer.from(merkle.leaf);
  let index = merkle.index;

  for (let i = 0; i < merkle.proof.length; i++) {
    const proofElement = merkle.proof[i]!;

    if (index % 2 === 0) {
      hash = Buffer.from(sha256(Buffer.concat([hash, proofElement])));
    } else {
      hash = Buffer.from(sha256(Buffer.concat([proofElement, hash])));
    }

    index = Math.floor(index / 2);
  }

  return hash.equals(merkle.root);
}

function soliditify(merkle: MerkleProof): {
  root: string;
  leaf: string;
  proof: string[];
  index: string;
  totalLeaves: string;
} {
  return {
    root: toHex0x(merkle.root),
    leaf: toHex0x(merkle.leaf),
    proof: merkle.proof.map((p) => toHex0x(p)),
    // Solidity-side verification for this Tron-style tree will also need the
    // total number of leaves to reconstruct when a node is "promoted" without
    // a sibling. We expose both the leaf index and leaf count.
    index: merkle.index.toString(10),
    totalLeaves: merkle.totalLeaves.toString(10),
  };
}

async function main() {
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
    })
  );

  const { blockNumber, txIdHex } = parseArgs();

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  log.info("Fetching Tron block for Merkle proof", {
    blockNumber: blockNumber.toString(),
    txId: `0x${txIdHex}`,
  });

  const block = await fetchBlockByNumber(wallet, callOpts.metadata, blockNumber);
  const header = block.blockHeader;
  const raw = header?.rawData as BlockHeader_raw | undefined;

  if (!header || !raw) {
    throw new Error("Block header or rawData is missing on the fetched block");
  }

  const txTrieRoot = raw.txTrieRoot as Buffer | undefined;
  const merkle = computeMerkleProofFromTxs(block.transactions, txIdHex);
  const ok = verifyProofLocal(merkle);
  const solidity = soliditify(merkle);

  log.info("Tron transaction Merkle proof (research script)", {
    block: {
      number: raw.number.toString(),
      txTrieRoot: txTrieRoot ? toHex0x(txTrieRoot) : undefined,
      computedRoot: solidity.root,
      rootsMatch: txTrieRoot ? toHex0x(txTrieRoot) === solidity.root : undefined,
      txCount: block.transactions.length.toString(),
    },
    merkle: {
      verifiedLocally: ok,
      leafTxId: solidity.leaf,
      index: solidity.index,
      proofLength: merkle.proof.length.toString(),
    },
    solidityArgs: {
      // bytes32 root, bytes32 leaf, bytes32[] calldata proof, uint256 index
      root: solidity.root,
      leaf: solidity.leaf,
      proof: solidity.proof,
      index: solidity.index,
      totalLeaves: solidity.totalLeaves,
    },
  });
}

main().catch((err) => {
  log.error(err);
  process.exit(1);
});
