import { Effect } from "effect";
import { sql } from "ponder";
import type { Address, Hex } from "viem";

import { tronLightClientConfig } from "ponder:schema";
import type { Context as PonderContext } from "ponder:registry";

import { tryPromise } from "../../effect/tryPromise";
import { MAINNET_CHAIN_ID } from "../../env";
import { getRows } from "../sqlRows";

function coerceBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) return BigInt(value);
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

function coerceHexBytes32(value: unknown, label: string): Hex {
  const hex = String(value).toLowerCase();
  if (!/^0x[0-9a-f]{64}$/.test(hex)) throw new Error(`Invalid ${label} (expected bytes32 hex)`);
  return hex as Hex;
}

function normalizeBytes20ToOwnerHex(bytes20Hex: string, label: string): string {
  const raw = bytes20Hex.toLowerCase();
  if (!/^0x[0-9a-f]{40}$/.test(raw)) throw new Error(`Invalid ${label} (expected bytes20 hex)`);
  return `41${raw.slice(2)}`;
}

function parseJsonArrayOfBytes20Hex(value: string, label: string): string[] {
  let parsed: unknown;
  try {
    parsed = JSON.parse(value) as unknown;
  } catch {
    throw new Error(`Invalid ${label} (expected JSON array)`);
  }
  if (!Array.isArray(parsed) || parsed.length !== 27) {
    throw new Error(`Invalid ${label} (expected JSON array length 27)`);
  }
  for (let i = 0; i < parsed.length; i++) {
    const v = parsed[i];
    if (typeof v !== "string" || !/^0x[0-9a-f]{40}$/i.test(v)) {
      throw new Error(`Invalid ${label}[${i}] (expected bytes20 hex)`);
    }
  }
  return parsed as string[];
}

export type WitnessIndexMap = ReadonlyMap<string, number>;

export const deleteFulfilledPublishRequests = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
}) =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      DELETE FROM "tron_light_client_publish_request" r
      USING "tron_light_client_checkpoint" c
      WHERE r.chain_id = ${MAINNET_CHAIN_ID}
        AND r.tron_light_client_address = ${args.tronLightClientAddress}
        AND c.chain_id = r.chain_id
        AND c.contract_address = r.tron_light_client_address
        AND c.tron_block_number = r.tron_block_number;
    `)
  ).pipe(Effect.asVoid);

export const getLatestCheckpoint = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
}): Effect.Effect<{ tronBlockNumber: bigint; tronBlockId: Hex } | null, Error> =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      SELECT
        tron_block_number AS "tronBlockNumber",
        tron_block_id AS "tronBlockId"
      FROM "tron_light_client_checkpoint"
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND contract_address = ${args.tronLightClientAddress}
      ORDER BY tron_block_number DESC
      LIMIT 1;
    `)
  ).pipe(
    Effect.map((result) => {
      const rows = getRows(result) as Array<{ tronBlockNumber: unknown; tronBlockId: unknown }>;
      if (rows.length === 0) return null;
      return {
        tronBlockNumber: coerceBigint(rows[0]!.tronBlockNumber, "checkpoint.tronBlockNumber"),
        tronBlockId: coerceHexBytes32(rows[0]!.tronBlockId, "checkpoint.tronBlockId"),
      };
    })
  );

export const getCheckpointAtOrAbove = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
  tronBlockNumber: bigint;
}): Effect.Effect<{ tronBlockNumber: bigint; tronBlockId: Hex } | null, Error> =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      SELECT
        tron_block_number AS "tronBlockNumber",
        tron_block_id AS "tronBlockId"
      FROM "tron_light_client_checkpoint"
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND contract_address = ${args.tronLightClientAddress}
        AND tron_block_number >= ${args.tronBlockNumber}
      ORDER BY tron_block_number ASC
      LIMIT 1;
    `)
  ).pipe(
    Effect.map((result) => {
      const rows = getRows(result) as Array<{ tronBlockNumber: unknown; tronBlockId: unknown }>;
      if (rows.length === 0) return null;
      return {
        tronBlockNumber: coerceBigint(rows[0]!.tronBlockNumber, "checkpoint.tronBlockNumber"),
        tronBlockId: coerceHexBytes32(rows[0]!.tronBlockId, "checkpoint.tronBlockId"),
      };
    })
  );

export const getEligibleRequestBlockNumbersInRange = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
  rangeStart: bigint;
  rangeEnd: bigint;
  eligibleLastSent: bigint;
  limit: number;
}): Effect.Effect<readonly bigint[], Error> =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      SELECT
        tron_block_number AS "tronBlockNumber"
      FROM "tron_light_client_publish_request"
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND tron_light_client_address = ${args.tronLightClientAddress}
        AND tron_block_number >= ${args.rangeStart}
        AND tron_block_number <= ${args.rangeEnd}
        AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${args.eligibleLastSent})
      ORDER BY tron_block_number ASC, "id" ASC
      LIMIT ${args.limit};
    `)
  ).pipe(
    Effect.map((result) => {
      const rows = getRows(result) as Array<{ tronBlockNumber: unknown }>;
      return rows.map((r) => coerceBigint(r.tronBlockNumber, "publish_request.tronBlockNumber"));
    })
  );

export const getOldestEligibleRequestBlockNumber = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
  eligibleLastSent: bigint;
}): Effect.Effect<bigint | null, Error> =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      SELECT
        tron_block_number AS "tronBlockNumber"
      FROM "tron_light_client_publish_request"
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND tron_light_client_address = ${args.tronLightClientAddress}
        AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${args.eligibleLastSent})
      ORDER BY tron_block_number ASC, "id" ASC
      LIMIT 1;
    `)
  ).pipe(
    Effect.map((result) => {
      const rows = getRows(result) as Array<{ tronBlockNumber: unknown }>;
      if (rows.length === 0) return null;
      return coerceBigint(rows[0]!.tronBlockNumber, "publish_request.tronBlockNumber");
    })
  );

export const markPublishRequestsSentInRange = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
  rangeStart: bigint;
  rangeEnd: bigint;
  eligibleLastSent: bigint;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
}) =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      UPDATE "tron_light_client_publish_request"
      SET last_sent_at_tron_block_number = ${args.headBlockNumber},
          last_sent_at_tron_block_timestamp = ${args.headBlockTimestamp}
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND tron_light_client_address = ${args.tronLightClientAddress}
        AND tron_block_number >= ${args.rangeStart}
        AND tron_block_number <= ${args.rangeEnd}
        AND (last_sent_at_tron_block_number IS NULL OR last_sent_at_tron_block_number <= ${args.eligibleLastSent});
    `)
  ).pipe(Effect.asVoid);

export const markPublishRequestSent = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
  tronBlockNumber: bigint;
  headBlockNumber: bigint;
  headBlockTimestamp: bigint;
}) =>
  tryPromise(() =>
    args.context.db.sql.execute(sql`
      UPDATE "tron_light_client_publish_request"
      SET last_sent_at_tron_block_number = ${args.headBlockNumber},
          last_sent_at_tron_block_timestamp = ${args.headBlockTimestamp}
      WHERE chain_id = ${MAINNET_CHAIN_ID}
        AND tron_light_client_address = ${args.tronLightClientAddress}
        AND tron_block_number = ${args.tronBlockNumber};
    `)
  ).pipe(Effect.asVoid);

export const loadWitnessIndexByTronOwnerAddressHex = (args: {
  context: PonderContext;
  tronLightClientAddress: Address;
}): Effect.Effect<WitnessIndexMap, Error> =>
  tryPromise(async () => {
    const key = args.tronLightClientAddress.toLowerCase();
    const config = await args.context.db.find(tronLightClientConfig, {
      id: `${MAINNET_CHAIN_ID}:${key}`,
    });
    if (!config) {
      throw new Error(
        "Missing tron_light_client_config row in DB; ensure TronLightClientConfigured is indexed"
      );
    }

    const srs = parseJsonArrayOfBytes20Hex(config.srsJson, "tron_light_client_config.srsJson");
    const entries: Array<[string, number]> = [];
    for (let i = 0; i < 27; i++) {
      entries.push([normalizeBytes20ToOwnerHex(srs[i]!, `srs[${i}]`), i]);
    }
    return new Map(entries) as WitnessIndexMap;
  });
