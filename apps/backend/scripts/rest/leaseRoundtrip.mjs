import { randomBytes } from "node:crypto";

import { createPublicClient, defineChain, http, isAddress } from "viem";
import { privateKeyToAccount } from "viem/accounts";

import { untronV3Abi } from "@untron/v3-contracts";

import { loadDotEnvLocal } from "../graphql/_client.mjs";

loadDotEnvLocal();

const API_BASE_URL = process.env.UNTRON_API_URL ?? "http://localhost:42069";
const chainId = Number(process.env.UNTRON_V3_CHAIN_ID ?? "0");
const rpcUrl = process.env.UNTRON_V3_CHAIN_RPC_URL ?? "";
const untronV3Address = (process.env.UNTRON_V3_ADDRESS ?? "").toLowerCase();

function requireAddress(value, label) {
  if (typeof value !== "string" || !isAddress(value)) {
    throw new Error(`Missing/invalid ${label} (expected 0x-address)`);
  }
  return value.toLowerCase();
}

function requireString(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Missing/invalid ${label}`);
  }
  return value;
}

function randomBytes32Hex() {
  return `0x${randomBytes(32).toString("hex")}`;
}

function isTruthyEnv(value) {
  if (value == null) return false;
  const s = String(value).trim().toLowerCase();
  return s === "1" || s === "true" || s === "yes" || s === "y" || s === "on";
}

async function httpJson(method, url, body) {
  const res = await fetch(url, {
    method,
    headers: { "content-type": "application/json" },
    body: body ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  let json;
  try {
    json = JSON.parse(text);
  } catch {
    throw new Error(`[api] ${method} ${url} -> HTTP ${res.status} (non-JSON): ${text}`);
  }
  if (!res.ok || (json && json.ok === false)) {
    throw new Error(`[api] ${method} ${url} -> ${JSON.stringify(json)}`);
  }
  return json;
}

function toBigInt(value, label) {
  try {
    if (typeof value === "bigint") return value;
    if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
    if (typeof value === "string" && value.length > 0) return BigInt(value);
  } catch {}
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

async function main() {
  if (!chainId) throw new Error("Missing UNTRON_V3_CHAIN_ID");
  requireString(rpcUrl, "UNTRON_V3_CHAIN_RPC_URL");
  requireAddress(untronV3Address, "UNTRON_V3_ADDRESS");

  const chain = defineChain({
    id: chainId,
    name: "mainnet",
    nativeCurrency: { name: "Native", symbol: "NATIVE", decimals: 18 },
    rpcUrls: { default: { http: [rpcUrl] } },
  });
  const publicClient = createPublicClient({ chain, transport: http(rpcUrl) });

  // 1) Create a fresh lessee EOA.
  const lesseeAccount = privateKeyToAccount(`0x${randomBytes(32).toString("hex")}`);
  const lessee = lesseeAccount.address.toLowerCase();

  // Use a different beneficiary for the update to prove the flow.
  const beneficiaryAccount = privateKeyToAccount(`0x${randomBytes(32).toString("hex")}`);
  const newBeneficiary = beneficiaryAccount.address.toLowerCase();

  console.log("[toy] API:", API_BASE_URL);
  console.log("[toy] chainId:", chainId);
  console.log("[toy] UntronV3:", untronV3Address);
  console.log("[toy] lessee:", lessee);
  console.log("[toy] newBeneficiary:", newBeneficiary);

  // Read protocol/realtor info from the backend (DB-derived).
  const protocol = await httpJson("GET", `${API_BASE_URL}/protocol`);
  const realtorInfo = await httpJson("GET", `${API_BASE_URL}/realtors`);

  const realtorRow = realtorInfo.realtor;
  if (!realtorRow) {
    throw new Error(
      `[toy] backend relayer has no realtor row (not indexed or not a realtor). relayerAddress=${realtorInfo.relayerAddress}`
    );
  }
  if (realtorRow.allowed !== true) {
    throw new Error(
      `[toy] backend relayer realtor is not allowed. relayerAddress=${realtorInfo.relayerAddress}`
    );
  }

  const effectiveMinFeePpm = (() => {
    const v = realtorRow.effective_min_fee_ppm ?? realtorRow.min_fee_ppm ?? "0";
    const ppm = toBigInt(v, "realtor.effective_min_fee_ppm");
    // createLease expects uint32
    if (ppm < 0n || ppm > 0xffff_ffffn) {
      throw new Error(`[toy] effective_min_fee_ppm out of uint32 range: ${ppm.toString()}`);
    }
    return ppm;
  })();

  const isChainDeprecated = (() => {
    const rows = Array.isArray(protocol.deprecatedChains) ? protocol.deprecatedChains : [];
    const target = BigInt(chainId);
    for (const row of rows) {
      try {
        const rowChainId = toBigInt(row.target_chain_id, "deprecated.target_chain_id");
        if (rowChainId === target) return row.deprecated === true;
      } catch {}
    }
    return false;
  })();
  if (isChainDeprecated) {
    throw new Error(
      `[toy] chainId ${chainId} is marked deprecated by protocol; refusing to proceed`
    );
  }

  // Use USDT (always routable without swap rate requirements).
  const usdt = (
    await publicClient.readContract({
      address: untronV3Address,
      abi: untronV3Abi,
      functionName: "usdt",
    })
  ).toLowerCase();

  // Create a lease that stays "active" (non-nukeable) for ~60 seconds.
  const now = BigInt(Math.floor(Date.now() / 1000));
  const nukeableAfter = now + 60n;

  const omitReceiverSalt = isTruthyEnv(process.env.TOY_OMIT_RECEIVER_SALT);
  const receiverSalt = omitReceiverSalt ? null : randomBytes32Hex();
  const leaseFeePpm = effectiveMinFeePpm > 0n ? effectiveMinFeePpm : 100n;
  const leaseCreateBody = {
    lessee,
    nukeableAfter: nukeableAfter.toString(),
    leaseFeePpm: leaseFeePpm.toString(),
    flatFee: "0",
    targetChainId: String(chainId),
    targetToken: usdt,
    beneficiary: lessee, // initial beneficiary
  };
  if (receiverSalt) leaseCreateBody.receiverSalt = receiverSalt;

  console.log("\n[toy] POST /leases body:", leaseCreateBody);
  const created = await httpJson("POST", `${API_BASE_URL}/leases`, leaseCreateBody);

  const leaseId = created.leaseId;
  if (!leaseId) {
    throw new Error(
      `[toy] createLease succeeded but API returned leaseId=null. Response: ${JSON.stringify(created)}`
    );
  }

  console.log("[toy] lease created:", {
    leaseId,
    receiverSalt: created.receiverSalt ?? receiverSalt,
    userOperation: created.userOperation,
  });

  // 2) Sign a gasless payout config update as the lessee (EIP-712).
  const nonce = await publicClient.readContract({
    address: untronV3Address,
    abi: untronV3Abi,
    functionName: "leaseNonces",
    args: [BigInt(leaseId)],
  });

  const deadline = now + 10n * 60n; // 10 minutes

  const typedData = {
    domain: {
      name: "Untron",
      version: "1",
      chainId,
      verifyingContract: untronV3Address,
    },
    types: {
      PayoutConfigUpdate: [
        { name: "leaseId", type: "uint256" },
        { name: "targetChainId", type: "uint256" },
        { name: "targetToken", type: "address" },
        { name: "beneficiary", type: "address" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ],
    },
    primaryType: "PayoutConfigUpdate",
    message: {
      leaseId: BigInt(leaseId),
      targetChainId: BigInt(chainId),
      targetToken: usdt,
      beneficiary: newBeneficiary,
      nonce,
      deadline,
    },
  };

  const signature = await lesseeAccount.signTypedData(typedData);

  const updateBody = {
    targetChainId: String(chainId),
    targetToken: usdt,
    beneficiary: newBeneficiary,
    deadline: deadline.toString(),
    signature,
  };

  console.log("\n[toy] PUT /leases/:leaseId body:", updateBody);
  const updated = await httpJson("PUT", `${API_BASE_URL}/leases/${leaseId}`, updateBody);

  console.log("[toy] payout config updated:", {
    leaseId,
    updated: updated.updated,
    userOperation: updated.userOperation,
  });

  console.log("\n[toy] done");
}

main().catch((err) => {
  console.error(err?.stack ?? String(err));
  process.exitCode = 1;
});
