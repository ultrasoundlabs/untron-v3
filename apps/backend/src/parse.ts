import { isAddress, type Address, type Hex } from "viem";

export function getArgValue(args: unknown, index: number, name?: string): unknown {
  if (args && typeof args === "object" && !Array.isArray(args)) {
    if (name && name in args) return (args as Record<string, unknown>)[name];
  }
  if (Array.isArray(args)) return args[index];
  return undefined;
}

export function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

export function expectString(value: unknown, label: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`Invalid ${label}`);
  return value;
}

export function expectHex(value: unknown, label: string): Hex {
  const raw = expectString(value, label).toLowerCase();
  if (!/^0x[0-9a-f]+$/.test(raw)) throw new Error(`Invalid ${label} (expected 0x-hex)`);
  return raw as Hex;
}

export function expectHexAddress(value: unknown, label: string): `0x${string}` {
  const raw = expectString(value, label).toLowerCase();
  if (!/^0x[0-9a-f]{40}$/.test(raw))
    throw new Error(`Invalid ${label} (expected 0x + 40 hex chars)`);
  return raw as `0x${string}`;
}

export function expectAddress(value: unknown, label: string): Address {
  const raw = expectString(value, label);
  if (!isAddress(raw)) throw new Error(`Invalid ${label} (expected EVM address)`);
  return raw.toLowerCase() as Address;
}

export function expectBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      // fall through
    }
  }
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

export function expectBoolean(value: unknown, label: string): boolean {
  if (typeof value === "boolean") return value;
  throw new Error(`Invalid ${label} (expected boolean)`);
}
