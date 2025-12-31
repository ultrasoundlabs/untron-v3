import { Effect } from "effect";
import { Hono } from "hono";
import { db } from "ponder:api";
import { sql } from "ponder";
import { parseEventLogs, encodeFunctionData, type Address, type Hex } from "viem";

import { untronV3Abi } from "@untron/v3-contracts";

import { getUntronV3Address } from "../contracts";
import { MAINNET_CHAIN_ID } from "../env";
import { BackendRuntime } from "../effect/runtime";
import { tryPromise } from "../effect/tryPromise";
import { expectAddress, expectBigint, expectHex, expectHexBytes32, expectRecord } from "../parse";
import { PublicClients } from "../relayer/deps/publicClients";
import { MainnetRelayer } from "../relayer/deps/mainnet";
import { getRows } from "../relayer/sqlRows";

const untronV3Address = getUntronV3Address().toLowerCase() as Address;
const chainId = MAINNET_CHAIN_ID;

const jsonError = (args: { status: number; message: string; details?: unknown }) => ({
  ok: false as const,
  error: { message: args.message, details: args.details ?? null },
});

function toJsonSafe(value: unknown): unknown {
  if (typeof value === "bigint") return value.toString();
  if (Array.isArray(value)) return value.map(toJsonSafe);
  if (value && typeof value === "object") {
    const out: Record<string, unknown> = {};
    for (const [k, v] of Object.entries(value as Record<string, unknown>)) {
      out[k] = toJsonSafe(v);
    }
    return out;
  }
  return value;
}

class NotRealtorError extends Error {
  readonly relayerAddress: Address;

  constructor(args: { relayerAddress: Address }) {
    super(`Backend relayer is not a realtor (address=${args.relayerAddress})`);
    this.name = "NotRealtorError";
    this.relayerAddress = args.relayerAddress;
  }
}

function asUint32Number(value: bigint, label: string): number {
  if (value < 0n || value > 0xffff_ffffn) throw new Error(`Invalid ${label} (expected uint32)`);
  return Number(value);
}

class NoFreeReceiverSaltError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NoFreeReceiverSaltError";
  }
}

function parsePreknownReceiverSaltsFromEnv(): Hex[] {
  const raw = process.env.PREKNOWN_RECEIVER_SALTS;
  if (!raw) return [];

  const seen = new Set<string>();
  const out: Hex[] = [];

  for (const part of raw.split(",")) {
    const trimmed = part.trim();
    if (!trimmed) continue;
    const normalized = trimmed.startsWith("0x")
      ? trimmed.toLowerCase()
      : `0x${trimmed.toLowerCase()}`;
    if (!/^0x[0-9a-f]{64}$/.test(normalized)) {
      throw new Error(`Invalid PREKNOWN_RECEIVER_SALTS entry "${trimmed}" (expected bytes32 hex)`);
    }
    if (seen.has(normalized)) continue;
    seen.add(normalized);
    out.push(normalized as Hex);
  }

  return out;
}

async function getRealtorRow(args: { realtor: Address }) {
  const result = await db.execute(sql`
    SELECT *
    FROM untron_v3_realtor_full
    WHERE chain_id = ${chainId}
      AND contract_address = ${untronV3Address}
      AND realtor = ${args.realtor.toLowerCase()}
    LIMIT 1
  `);

  const rows = getRows(result);
  return rows[0] ?? null;
}

export const untronV3Api = new Hono()
  .get("/protocol", async (c) => {
    try {
      const protocolResult = await db.execute(sql`
        SELECT *
        FROM untron_v3_protocol_full
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
        LIMIT 1
      `);
      const protocol = getRows(protocolResult)[0] ?? null;

      const deprecatedResult = await db.execute(sql`
        SELECT *
        FROM untron_v3_chain_deprecated
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
        ORDER BY target_chain_id ASC
      `);
      const deprecatedChains = getRows(deprecatedResult);

      const swapRatesResult = await db.execute(sql`
        SELECT *
        FROM untron_v3_swap_rate
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
        ORDER BY target_token ASC
      `);
      const swapRates = getRows(swapRatesResult);

      const bridgerRoutesResult = await db.execute(sql`
        SELECT *
        FROM untron_v3_bridger_route
        WHERE chain_id = ${chainId}
          AND contract_address = ${untronV3Address}
        ORDER BY target_token ASC, target_chain_id ASC
      `);
      const bridgerRoutes = getRows(bridgerRoutesResult);

      return c.json(
        toJsonSafe({
          ok: true,
          chainId,
          contractAddress: untronV3Address,
          protocol,
          deprecatedChains,
          swapRates,
          bridgerRoutes,
        })
      );
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 500, message }), 500);
    }
  })
  .get("/realtors", async (c) => {
    try {
      const relayerAddress = await BackendRuntime.runPromise(MainnetRelayer.getAddress());
      const row = await getRealtorRow({ realtor: relayerAddress });
      return c.json(
        toJsonSafe({
          ok: true,
          chainId,
          contractAddress: untronV3Address,
          relayerAddress,
          realtor: row,
        })
      );
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 501, message, details: null }), 501);
    }
  })
  .get("/realtors/:address", async (c) => {
    try {
      const realtor = expectAddress(c.req.param("address"), "realtor").toLowerCase() as Address;
      const row = await getRealtorRow({ realtor });
      return c.json(
        toJsonSafe({ ok: true, chainId, contractAddress: untronV3Address, realtor, result: row })
      );
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 400, message }), 400);
    }
  })
  .post("/leases", async (c) => {
    let body: unknown;
    try {
      body = await c.req.json();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 400, message: `Invalid JSON body. ${message}` }), 400);
    }

    try {
      const record = expectRecord(body, "body");
      const receiverSalt =
        record.receiverSalt === undefined ||
        record.receiverSalt === null ||
        record.receiverSalt === ""
          ? null
          : expectHexBytes32(record.receiverSalt, "receiverSalt");
      const lessee = expectAddress(record.lessee, "lessee");
      const nukeableAfter = expectBigint(record.nukeableAfter, "nukeableAfter");
      const leaseFeePpm = asUint32Number(
        expectBigint(record.leaseFeePpm, "leaseFeePpm"),
        "leaseFeePpm"
      );
      const flatFee = expectBigint(record.flatFee, "flatFee");
      const targetChainId = expectBigint(record.targetChainId, "targetChainId");
      const targetToken = expectAddress(record.targetToken, "targetToken");
      const beneficiary = expectAddress(record.beneficiary, "beneficiary");

      const result = await BackendRuntime.runPromise(
        Effect.gen(function* () {
          const publicClients = yield* PublicClients;
          const mainnetClient = yield* publicClients.get("mainnet");
          const relayerAddress = yield* MainnetRelayer.getAddress();

          const resolvedReceiverSalt: Hex = receiverSalt
            ? receiverSalt
            : yield* Effect.gen(function* () {
                const salts = parsePreknownReceiverSaltsFromEnv();
                if (salts.length === 0) {
                  return yield* Effect.fail(
                    new Error(
                      "Missing receiverSalt and PREKNOWN_RECEIVER_SALTS is empty (cannot auto-select receiverSalt)"
                    )
                  );
                }

                const head = yield* tryPromise(() => mainnetClient.getBlock());
                const nowSeconds = head.timestamp;

                const inList = sql.join(
                  salts.map((s) => sql`${s}`),
                  sql`, `
                );
                const latestResult = yield* tryPromise(() =>
                  db.execute(sql`
                    SELECT DISTINCT ON (receiver_salt)
                      receiver_salt,
                      lease_id,
                      nukeable_after
                    FROM untron_v3_lease
                    WHERE chain_id = ${chainId}
                      AND contract_address = ${untronV3Address}
                      AND receiver_salt IN (${inList})
                    ORDER BY receiver_salt ASC, lease_id DESC
                  `)
                );
                const rows = getRows(latestResult);

                const bySalt = new Map<string, bigint>();
                for (const row of rows) {
                  if (!row || typeof row !== "object") continue;
                  const rec = row as Record<string, unknown>;
                  const saltRaw = rec.receiver_salt;
                  if (typeof saltRaw !== "string") continue;
                  const salt = saltRaw.toLowerCase();
                  try {
                    bySalt.set(salt, expectBigint(rec.nukeable_after, "lease.nukeable_after"));
                  } catch {
                    // ignore malformed
                  }
                }

                for (const salt of salts) {
                  const lastNukeableAfter = bySalt.get(salt.toLowerCase());
                  if (lastNukeableAfter === undefined) return salt;
                  if (lastNukeableAfter <= nowSeconds) return salt;
                }

                return yield* Effect.fail(
                  new NoFreeReceiverSaltError(
                    "No free receiverSalt available among PREKNOWN_RECEIVER_SALTS (all have active non-nukeable leases)."
                  )
                );
              });

          const args = [
            resolvedReceiverSalt,
            lessee,
            nukeableAfter,
            leaseFeePpm,
            flatFee,
            targetChainId,
            targetToken,
            beneficiary,
          ] as const;

          const data = encodeFunctionData({
            abi: untronV3Abi,
            functionName: "createLease",
            args,
          });

          const isRealtor = yield* tryPromise(() =>
            mainnetClient.readContract({
              address: untronV3Address,
              abi: untronV3Abi,
              functionName: "isRealtor",
              args: [relayerAddress],
            })
          );
          if (!isRealtor) {
            return yield* Effect.fail(new NotRealtorError({ relayerAddress }));
          }

          // Fail fast on obvious reverts before we pay bundler fees.
          yield* tryPromise(() =>
            mainnetClient.simulateContract({
              address: untronV3Address,
              abi: untronV3Abi,
              functionName: "createLease",
              args,
              account: relayerAddress,
            })
          );

          const sent = yield* MainnetRelayer.sendUserOperation({
            calls: [{ to: untronV3Address, data }],
          });

          const receipt = yield* tryPromise(() =>
            mainnetClient.getTransactionReceipt({ hash: sent.transactionHash })
          );

          const logs = receipt.logs.filter(
            (log) => log.address.toLowerCase() === untronV3Address.toLowerCase()
          );

          const parsed = parseEventLogs({
            abi: untronV3Abi,
            logs,
            eventName: "LeaseCreated",
            strict: false,
          });

          const leaseId =
            (parsed[0]?.args as unknown as { leaseId?: bigint } | undefined)?.leaseId ?? null;

          return { sent, leaseId, receiverSalt: resolvedReceiverSalt };
        })
      );

      return c.json(
        toJsonSafe({
          ok: true,
          chainId,
          contractAddress: untronV3Address,
          receiverSalt: result.receiverSalt,
          leaseId: result.leaseId?.toString() ?? null,
          userOperation: result.sent,
        })
      );
    } catch (error) {
      if (error instanceof NoFreeReceiverSaltError) {
        return c.json(jsonError({ status: 409, message: error.message }), 409);
      }
      if (error instanceof NotRealtorError) {
        return c.json(
          jsonError({
            status: 403,
            message: error.message,
            details: { relayerAddress: error.relayerAddress },
          }),
          403
        );
      }
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 400, message }), 400);
    }
  })
  .put("/leases/:leaseId", async (c) => {
    let body: unknown;
    try {
      body = await c.req.json();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 400, message: `Invalid JSON body. ${message}` }), 400);
    }

    try {
      const record = expectRecord(body, "body");
      const leaseId = expectBigint(c.req.param("leaseId"), "leaseId");

      const targetChainId = expectBigint(record.targetChainId, "targetChainId");
      const targetToken = expectAddress(record.targetToken, "targetToken");
      const beneficiary = expectAddress(record.beneficiary, "beneficiary");
      const deadline = expectBigint(record.deadline, "deadline");
      const signature = expectHex(record.signature, "signature") as Hex;

      const args = [
        leaseId,
        { targetChainId, targetToken, beneficiary },
        deadline,
        signature,
      ] as const;

      const data = encodeFunctionData({
        abi: untronV3Abi,
        functionName: "setPayoutConfigWithSig",
        args,
      });

      const result = await BackendRuntime.runPromise(
        Effect.gen(function* () {
          const publicClients = yield* PublicClients;
          const mainnetClient = yield* publicClients.get("mainnet");
          const relayerAddress = yield* MainnetRelayer.getAddress();

          yield* tryPromise(() =>
            mainnetClient.simulateContract({
              address: untronV3Address,
              abi: untronV3Abi,
              functionName: "setPayoutConfigWithSig",
              args,
              account: relayerAddress,
            })
          );

          const sent = yield* MainnetRelayer.sendUserOperation({
            calls: [{ to: untronV3Address, data }],
          });

          const receipt = yield* tryPromise(() =>
            mainnetClient.getTransactionReceipt({ hash: sent.transactionHash })
          );

          const logs = receipt.logs.filter(
            (log) => log.address.toLowerCase() === untronV3Address.toLowerCase()
          );

          const parsed = parseEventLogs({
            abi: untronV3Abi,
            logs,
            eventName: "PayoutConfigUpdated",
            strict: false,
          });

          const updated =
            (parsed[0]?.args as unknown as { leaseId?: bigint } | undefined)?.leaseId ?? null;

          return { sent, updated };
        })
      );

      return c.json(
        toJsonSafe({
          ok: true,
          chainId,
          contractAddress: untronV3Address,
          leaseId: leaseId.toString(),
          updated: Boolean(result.updated),
          userOperation: result.sent,
        })
      );
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return c.json(jsonError({ status: 400, message }), 400);
    }
  });
