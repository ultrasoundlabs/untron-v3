import type { Hex } from "viem";

export const safeUrlForLogs = (rawUrl: string): string => {
  try {
    const url = new URL(rawUrl);
    const path = url.pathname === "/" ? "" : url.pathname;
    return `${url.protocol}//${url.host}${path}`;
  } catch {
    return "<invalid-url>";
  }
};

const isPlainObject = (value: unknown): value is Record<string, unknown> => {
  if (!value || typeof value !== "object") return false;
  const proto = Object.getPrototypeOf(value);
  return proto === Object.prototype || proto === null;
};

const normalizeForJson = (value: unknown): unknown => {
  if (typeof value === "bigint") return value.toString();
  if (Array.isArray(value)) return value.map(normalizeForJson);
  if (isPlainObject(value)) {
    const out: Record<string, unknown> = {};
    for (const key of Object.keys(value).sort()) {
      out[key] = normalizeForJson(value[key]);
    }
    return out;
  }
  return value;
};

export const canonicalJsonStringify = (value: unknown): string =>
  JSON.stringify(normalizeForJson(value));

export const toPaymasterAndDataV06 = (value: unknown): { paymasterAndData: Hex } => {
  if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    const paymasterAndData = record.paymasterAndData;
    if (typeof paymasterAndData === "string" && paymasterAndData.startsWith("0x")) {
      return { paymasterAndData: paymasterAndData as Hex };
    }

    const paymaster = record.paymaster;
    const paymasterData = record.paymasterData;
    if (
      typeof paymaster === "string" &&
      paymaster.startsWith("0x") &&
      typeof paymasterData === "string" &&
      paymasterData.startsWith("0x")
    ) {
      return { paymasterAndData: `0x${paymaster.slice(2)}${paymasterData.slice(2)}` as Hex };
    }
  }
  throw new Error("Paymaster response missing paymasterAndData for EntryPoint v0.6");
};

export const isProbablyBundler429 = (error: unknown): boolean => {
  const seen = new Set<unknown>();
  const queue: unknown[] = [error];

  while (queue.length > 0) {
    const current = queue.shift();
    if (!current || seen.has(current)) continue;
    seen.add(current);

    const message =
      current instanceof Error
        ? current.message
        : typeof current === "object" && current && "message" in current
          ? (current as { message?: unknown }).message
          : null;

    if (typeof message === "string") {
      const lower = message.toLowerCase();
      if (
        lower.includes("too many requests") ||
        lower.includes("rate limit") ||
        lower.includes("ratelimit") ||
        lower.includes("http 429") ||
        lower.includes("status code 429") ||
        lower.includes("429")
      ) {
        return true;
      }
    }

    const cause =
      current instanceof Error
        ? (current as { cause?: unknown }).cause
        : typeof current === "object"
          ? (current as { cause?: unknown }).cause
          : undefined;

    if (cause) queue.push(cause);
  }

  return false;
};
