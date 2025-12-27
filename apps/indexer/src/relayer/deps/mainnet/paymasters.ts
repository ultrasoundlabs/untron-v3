import { Effect } from "effect";
import { createPaymasterClient } from "viem/account-abstraction";
import { http } from "viem";

import { safeUrlForLogs } from "./utils";

export type PaymasterConfig = Readonly<{
  name: string;
  url: string;
  context?: unknown;
  timeoutMs?: number;
}>;

export type ResolvedPaymaster = Readonly<{
  name: string;
  url: string;
  urlForLogs: string;
  context?: unknown;
  timeoutMs: number;
  client: ReturnType<typeof createPaymasterClient>;
}>;

const parsePositiveMs = (value: unknown, label: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
    throw new Error(`Invalid ${label} (expected a positive number)`);
  }
  return value;
};

export const parsePaymastersJson = (raw: string): readonly PaymasterConfig[] => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(`Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected JSON array): ${message}`);
  }

  if (!Array.isArray(parsed)) {
    throw new Error("Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected JSON array)");
  }
  if (parsed.length === 0) {
    throw new Error("Invalid RELAYER_MAINNET_PAYMASTERS_JSON (expected non-empty array)");
  }

  return parsed.map((item, index): PaymasterConfig => {
    if (!item || typeof item !== "object" || Array.isArray(item)) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}] (expected object with { name, url, ... })`
      );
    }
    const record = item as Record<string, unknown>;

    const nameRaw = record.name;
    const urlRaw = record.url;
    if (typeof nameRaw !== "string" || nameRaw.trim().length === 0) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].name (expected non-empty string)`
      );
    }
    if (typeof urlRaw !== "string" || urlRaw.trim().length === 0) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected non-empty string)`
      );
    }

    const name = nameRaw.trim();
    const url = urlRaw.trim();
    if (/\s/.test(url)) {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (must not contain whitespace)`
      );
    }

    let parsedUrl: URL;
    try {
      parsedUrl = new URL(url);
    } catch {
      throw new Error(`Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected valid URL)`);
    }
    if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") {
      throw new Error(
        `Invalid RELAYER_MAINNET_PAYMASTERS_JSON[${index}].url (expected http(s) URL)`
      );
    }

    const timeoutMs =
      typeof record.timeoutMs === "undefined"
        ? undefined
        : parsePositiveMs(record.timeoutMs, `RELAYER_MAINNET_PAYMASTERS_JSON[${index}].timeoutMs`);

    return {
      name,
      url,
      context: record.context,
      timeoutMs,
    };
  });
};

export const resolvePaymasters = (
  rawJson: string
): Effect.Effect<readonly ResolvedPaymaster[], Error> =>
  Effect.try({
    try: () => {
      const parsed = parsePaymastersJson(rawJson);
      return parsed.map(
        (pm): ResolvedPaymaster => ({
          name: pm.name,
          url: pm.url,
          urlForLogs: safeUrlForLogs(pm.url),
          context: pm.context,
          timeoutMs: pm.timeoutMs ?? 10_000,
          client: createPaymasterClient({
            transport: http(pm.url, { timeout: pm.timeoutMs ?? 10_000, retryCount: 0 }),
          }),
        })
      );
    },
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });
