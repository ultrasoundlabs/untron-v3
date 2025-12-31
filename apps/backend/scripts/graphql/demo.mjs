import { graphqlRequest, loadDotEnvLocal } from "./_client.mjs";

loadDotEnvLocal();

const GRAPHQL_URL = process.env.PONDER_GRAPHQL_URL ?? "http://localhost:42069/graphql";
const chainId = Number(process.env.UNTRON_V3_CHAIN_ID ?? "0");
const contractAddress = (process.env.UNTRON_V3_ADDRESS ?? "").toLowerCase();

const nowSeconds = BigInt(Math.floor(Date.now() / 1000)).toString();

async function main() {
  if (!chainId || !contractAddress) {
    console.error(
      `Missing UNTRON_V3_CHAIN_ID/UNTRON_V3_ADDRESS. Set them or create apps/backend/.env.local.`
    );
    console.error(`Also ensure the backend is running (pnpm dev).`);
    process.exitCode = 1;
    return;
  }

  console.log(`[demo] GraphQL endpoint: ${GRAPHQL_URL}`);
  console.log(`[demo] chainId=${chainId} contractAddress=${contractAddress}`);

  // 1) Leases: filter by (lessee OR beneficiary) + active (nukeableAfter > now)
  const lessee = (process.env.DEMO_LESSEE ?? "").toLowerCase();
  const beneficiary = (process.env.DEMO_BENEFICIARY ?? "").toLowerCase();
  const leaseId = process.env.DEMO_LEASE_ID ? String(process.env.DEMO_LEASE_ID) : "";

  const leasesQuery = `
    query Leases($where: untronV3LeaseFullFilter) {
      untronV3LeaseFulls(where: $where, limit: 10) {
        totalCount
        items {
          leaseId
          receiverSalt
          realtor
          lessee
          startTime
          nukeableAfter
          leaseFeePpm
          flatFee
          payoutTargetToken
          payoutTargetChainId
          payoutBeneficiary
          realtorAllowed
          realtorMinFeePpm
          protocolLeaseRateLimitMaxLeases
          lesseePayoutConfigRateLimitMaxUpdates
        }
      }
    }
  `;

  const leasesQueryMinimal = `
    query LeasesMinimal($where: untronV3LeaseFullFilter) {
      untronV3LeaseFulls(where: $where, limit: 10) {
        totalCount
        items {
          leaseId
          receiverSalt
          realtor
          lessee
          startTime
          nukeableAfter
        }
      }
    }
  `;

  const leaseWhere = {
    chainId,
    contractAddress,
    ...(leaseId ? { leaseId } : { nukeableAfter_gt: nowSeconds }),
    ...(lessee && beneficiary
      ? { OR: [{ lessee }, { payoutBeneficiary: beneficiary }] }
      : {
          ...(lessee ? { lessee } : {}),
          ...(beneficiary ? { payoutBeneficiary: beneficiary } : {}),
        }),
  };

  let leases;
  try {
    leases = await graphqlRequest({
      url: GRAPHQL_URL,
      query: leasesQuery,
      variables: { where: leaseWhere },
    });
  } catch (err) {
    console.warn(
      "\n[leases] full query failed; retrying with minimal selection (this is usually due to ordering or a non-null field resolving to NULL in a view row)."
    );
    console.warn(err?.message ?? String(err));
    leases = await graphqlRequest({
      url: GRAPHQL_URL,
      query: leasesQueryMinimal,
      variables: { where: leaseWhere },
    });
  }
  console.log("\n[leases] filter:", leaseWhere);
  console.log(JSON.stringify(leases.untronV3LeaseFulls, null, 2));

  // 2) Claims: pending (isFilled=false) optionally scoped to a token
  const targetToken = (process.env.DEMO_TARGET_TOKEN ?? "").toLowerCase();
  const claimIndex = process.env.DEMO_CLAIM_INDEX ? String(process.env.DEMO_CLAIM_INDEX) : "";
  const claimId =
    process.env.DEMO_CLAIM_ID ??
    (targetToken && claimIndex ? `${chainId}:${contractAddress}:${targetToken}:${claimIndex}` : "");
  const claimLeaseId = process.env.DEMO_CLAIM_LEASE_ID
    ? String(process.env.DEMO_CLAIM_LEASE_ID)
    : "";
  const claimsQuery = `
    query Claims($where: untronV3ClaimFullFilter) {
      untronV3ClaimFulls(where: $where, limit: 10) {
        totalCount
        items {
          targetToken
          claimIndex
          leaseId
          amountUsdt
          beneficiary
          isFilled
          leaseRealtor
          leaseLessee
          leaseNukeableAfter
          swapRatePpm
          bridger
        }
      }
    }
  `;
  const claimsQueryMinimal = `
    query ClaimsMinimal($where: untronV3ClaimFullFilter) {
      untronV3ClaimFulls(where: $where, limit: 10) {
        totalCount
        items {
          targetToken
          claimIndex
          leaseId
          amountUsdt
          beneficiary
          isFilled
        }
      }
    }
  `;
  const claimWhere = {
    chainId,
    contractAddress,
    ...(claimId ? { id: claimId } : {}),
    ...(claimLeaseId ? { leaseId: claimLeaseId } : {}),
    ...(targetToken ? { targetToken } : {}),
    ...(claimIndex ? { claimIndex } : {}),
    isFilled: false,
  };
  let claims;
  try {
    claims = await graphqlRequest({
      url: GRAPHQL_URL,
      query: claimsQuery,
      variables: { where: claimWhere },
    });
  } catch (err) {
    console.warn(
      "\n[claims] full query failed; retrying with minimal selection (this is usually a non-null field resolving to NULL in a view row)."
    );
    console.warn(err?.message ?? String(err));
    claims = await graphqlRequest({
      url: GRAPHQL_URL,
      query: claimsQueryMinimal,
      variables: { where: claimWhere },
    });
  }
  console.log("\n[claims] filter:", claimWhere);
  console.log(JSON.stringify(claims.untronV3ClaimFulls, null, 2));

  // 3) Realtors: by address (or just list allowed ones)
  const realtor = (process.env.DEMO_REALTOR ?? "").toLowerCase();
  const realtorsQuery = `
    query Realtors($where: untronV3RealtorFullFilter) {
      untronV3RealtorFulls(where: $where, limit: 10) {
        totalCount
        items {
          realtor
          allowed
          minFeePpm
          leaseRateLimitMode
          leaseRateLimitMaxLeases
          leaseRateLimitWindowSeconds
          totalLeases
          activeLeases
          nukeableLeases
        }
      }
    }
  `;
  const realtorWhere = {
    chainId,
    contractAddress,
    ...(realtor ? { realtor } : { allowed: true }),
  };
  const realtors = await graphqlRequest({
    url: GRAPHQL_URL,
    query: realtorsQuery,
    variables: { where: realtorWhere },
  });
  console.log("\n[realtors] filter:", realtorWhere);
  console.log(JSON.stringify(realtors.untronV3RealtorFulls, null, 2));
}

main().catch((err) => {
  console.error(err?.stack ?? String(err));
  process.exitCode = 1;
});
