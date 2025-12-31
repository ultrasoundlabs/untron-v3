import { loadDotEnvLocal } from "../graphql/_client.mjs";
import { sqlDbRequest } from "./_client.mjs";

loadDotEnvLocal();

const SQL_URL = process.env.PONDER_SQL_URL ?? "http://localhost:42069/sql/db";
const chainId = Number(process.env.UNTRON_V3_CHAIN_ID ?? "0");
const contractAddress = (process.env.UNTRON_V3_ADDRESS ?? "").toLowerCase();

function hexEnv(key) {
  const value = (process.env[key] ?? "").toLowerCase();
  return value || undefined;
}

async function main() {
  if (!chainId || !contractAddress) {
    console.error(
      "Missing UNTRON_V3_CHAIN_ID/UNTRON_V3_ADDRESS. Set them or create apps/backend/.env.local."
    );
    console.error("Also ensure the backend is running (pnpm dev).");
    process.exitCode = 1;
    return;
  }

  console.log(`[demo] SQL endpoint: ${SQL_URL}`);
  console.log(`[demo] chainId=${chainId} contractAddress=${contractAddress}`);

  const leaseId = process.env.DEMO_LEASE_ID ? BigInt(String(process.env.DEMO_LEASE_ID)) : undefined;
  const lessee = hexEnv("DEMO_LESSEE");
  const beneficiary = hexEnv("DEMO_BENEFICIARY");

  {
    const where = ["chain_id = $1", "contract_address = $2"];
    const params = [chainId, contractAddress];

    if (leaseId !== undefined) {
      where.push(`lease_id = $${params.length + 1}`);
      params.push(leaseId.toString());
    } else {
      where.push("is_active = true");
    }

    if (lessee && beneficiary) {
      where.push(`(lessee = $${params.length + 1} OR payout_beneficiary = $${params.length + 2})`);
      params.push(lessee, beneficiary);
    } else if (lessee) {
      where.push(`lessee = $${params.length + 1}`);
      params.push(lessee);
    } else if (beneficiary) {
      where.push(`payout_beneficiary = $${params.length + 1}`);
      params.push(beneficiary);
    }

    const sql = `
      SELECT *
      FROM untron_v3_lease_full
      WHERE ${where.join(" AND ")}
      ORDER BY lease_id DESC
      LIMIT 10
    `;

    const result = await sqlDbRequest({ url: SQL_URL, sql, params });
    console.log("\n[leases] sql:", sql.trim().replace(/\s+/g, " "));
    console.log("[leases] params:", params);
    console.log("[leases] rows:", (result?.rows ?? []).length);
    console.log(JSON.stringify((result?.rows ?? []).slice(0, 3), null, 2));
  }

  const targetToken = hexEnv("DEMO_TARGET_TOKEN");
  const claimIndex = process.env.DEMO_CLAIM_INDEX
    ? BigInt(String(process.env.DEMO_CLAIM_INDEX))
    : undefined;
  const claimLeaseId = process.env.DEMO_CLAIM_LEASE_ID
    ? BigInt(String(process.env.DEMO_CLAIM_LEASE_ID))
    : undefined;
  const claimId =
    process.env.DEMO_CLAIM_ID ??
    (targetToken && claimIndex !== undefined
      ? `${chainId}:${contractAddress}:${targetToken}:${claimIndex.toString()}`
      : undefined);
  const claimStatus = process.env.DEMO_CLAIM_STATUS ?? "pending"; // pending|filled

  {
    const where = ["chain_id = $1", "contract_address = $2"];
    const params = [chainId, contractAddress];

    if (claimId) {
      where.push(`id = $${params.length + 1}`);
      params.push(claimId);
    }
    if (targetToken) {
      where.push(`target_token = $${params.length + 1}`);
      params.push(targetToken);
    }
    if (claimIndex !== undefined) {
      where.push(`claim_index = $${params.length + 1}`);
      params.push(claimIndex.toString());
    }
    if (claimLeaseId !== undefined) {
      where.push(`lease_id = $${params.length + 1}`);
      params.push(claimLeaseId.toString());
    }

    if (claimStatus === "pending" || claimStatus === "filled") {
      where.push(`status = $${params.length + 1}`);
      params.push(claimStatus);
    }

    const sql = `
      SELECT *
      FROM untron_v3_claim_full
      WHERE ${where.join(" AND ")}
      ORDER BY target_token ASC, claim_index DESC
      LIMIT 10
    `;

    const result = await sqlDbRequest({ url: SQL_URL, sql, params });
    console.log("\n[claims] sql:", sql.trim().replace(/\s+/g, " "));
    console.log("[claims] params:", params);
    console.log("[claims] rows:", (result?.rows ?? []).length);
    console.log(JSON.stringify((result?.rows ?? []).slice(0, 3), null, 2));
  }

  const realtor = hexEnv("DEMO_REALTOR");
  {
    const where = ["chain_id = $1", "contract_address = $2"];
    const params = [chainId, contractAddress];

    if (realtor) {
      where.push(`realtor = $${params.length + 1}`);
      params.push(realtor);
    }

    const sql = `
      SELECT *
      FROM untron_v3_realtor_full
      WHERE ${where.join(" AND ")}
      ORDER BY realtor ASC
      LIMIT 10
    `;

    const result = await sqlDbRequest({ url: SQL_URL, sql, params });
    console.log("\n[realtors] sql:", sql.trim().replace(/\s+/g, " "));
    console.log("[realtors] params:", params);
    console.log("[realtors] rows:", (result?.rows ?? []).length);
    console.log(JSON.stringify((result?.rows ?? []).slice(0, 3), null, 2));
  }
}

main().catch((err) => {
  console.error(err?.stack ?? String(err));
  process.exitCode = 1;
});
