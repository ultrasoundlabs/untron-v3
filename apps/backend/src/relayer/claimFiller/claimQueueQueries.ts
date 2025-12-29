import { sql } from "ponder";
import type { Address } from "viem";

export const selectNonEmptyClaimQueuesSql = (args: { chainId: number; contractAddress: Address }) =>
  sql`
    SELECT
      target_token AS "targetToken",
      queue_length AS "queueLength"
    FROM "untron_v3_claim_queue"
    WHERE chain_id = ${args.chainId}
      AND contract_address = ${args.contractAddress}
      AND queue_length > 0;
  `;
