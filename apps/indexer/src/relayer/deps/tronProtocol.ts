import { createBase58check } from "@scure/base";
import { secp256k1 } from "@noble/curves/secp256k1.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { keccak_256 } from "@noble/hashes/sha3.js";
import { Transaction, Transaction_raw } from "@untron/tron-protocol/tron";
import type { Address } from "viem";

const b58c = createBase58check(sha256);

export function normalizeTronPrivateKey(value: string): string {
  const trimmed = value.trim();
  const raw = trimmed.startsWith("0x") || trimmed.startsWith("0X") ? trimmed.slice(2) : trimmed;
  if (!/^[0-9a-fA-F]{64}$/.test(raw)) {
    throw new Error("Invalid env var RELAYER_TRON_PRIVATE_KEY (expected 64 hex chars)");
  }
  return raw;
}

export function tronBase58ToBytes21(base58: string): Buffer {
  const decoded = b58c.decode(base58);
  if (decoded.length !== 21 || decoded[0] !== 0x41) {
    throw new Error(`Invalid Tron address "${base58}" (expected base58check with 0x41 prefix)`);
  }
  return Buffer.from(decoded);
}

export function tronHex41ToBytes21(hex41: string): Buffer {
  const normalized = hex41.trim();
  if (!/^41[0-9a-fA-F]{40}$/.test(normalized)) {
    throw new Error(`Invalid Tron hex address "${hex41}" (expected 41 + 40 hex chars)`);
  }
  return Buffer.from(normalized, "hex");
}

export function tronBytes21ToBase58(bytes21: Uint8Array): string {
  if (bytes21.length !== 21 || bytes21[0] !== 0x41) {
    throw new Error("Invalid Tron address bytes (expected 21 bytes, 0x41 prefix)");
  }
  return b58c.encode(bytes21);
}

export function tronBase58ToEvmAddress(base58: string): Address {
  const bytes21 = tronBase58ToBytes21(base58);
  return `0x${bytes21.subarray(1).toString("hex")}` as Address;
}

export function tronEvmAddressToBytes21(evmAddress: Address): Buffer {
  const normalized = evmAddress.trim().toLowerCase();
  if (!/^0x[0-9a-f]{40}$/.test(normalized)) {
    throw new Error(`Invalid EVM address "${evmAddress}" (expected 0x + 40 hex chars)`);
  }
  return Buffer.from(`41${normalized.slice(2)}`, "hex");
}

export function tronPrivateKeyToAddressBytes21(privateKeyHex: string): Buffer {
  const normalized = normalizeTronPrivateKey(privateKeyHex);
  const privateKeyBytes = Buffer.from(normalized, "hex");

  const publicKeyUncompressed = secp256k1.getPublicKey(privateKeyBytes, false);
  const hash = keccak_256(publicKeyUncompressed.slice(1));
  const address20 = Buffer.from(hash.slice(-20));

  return Buffer.concat([Buffer.from([0x41]), address20]);
}

export function tronPrivateKeyToAddressBase58(privateKeyHex: string): string {
  return tronBytes21ToBase58(tronPrivateKeyToAddressBytes21(privateKeyHex));
}

export function signTronTransaction(
  tx: Transaction,
  privateKeyHex: string
): { txidHex: string; signed: Transaction } {
  if (!tx.rawData) throw new Error("Transaction is missing rawData");

  const rawDataBytes = Transaction_raw.encode(tx.rawData).finish();
  const txidHex = Buffer.from(sha256(rawDataBytes)).toString("hex");

  const privateKeyBytes = Buffer.from(normalizeTronPrivateKey(privateKeyHex), "hex");
  const recovered = secp256k1.sign(rawDataBytes, privateKeyBytes, { format: "recovered" });

  const r = recovered.slice(1, 33);
  const s = recovered.slice(33, 65);
  const v = recovered[0]! & 1; // Tron stores v as 0/1 (y-parity) in [r|s|v]
  const signature = Buffer.concat([Buffer.from(r), Buffer.from(s), Buffer.from([v])]);

  tx.signature = [signature];
  return { txidHex, signed: tx };
}
