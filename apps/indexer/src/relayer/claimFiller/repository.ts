import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import type { Address } from "viem";

import { tryPromise } from "../../effect/tryPromise";
import { getRows } from "../sqlRows";
import type { Claim } from "./types";

export const ClaimFillerRepository = {
  getNonEmptyClaimQueues: (args: {
    context: PonderContext;
    chainId: number;
    contractAddress: Address;
  }) =>
    tryPromise(() =>
      args.context.db.sql.execute(sql`
        SELECT
          target_token AS targetToken,
          queue_length AS queueLength
        FROM "untron_v3_claim_queue"
        WHERE chain_id = ${args.chainId}
          AND contract_address = ${args.contractAddress}
          AND queue_length > 0;
      `)
    ).pipe(
      Effect.map(
        (result) => getRows(result) as Array<{ targetToken: Address; queueLength: bigint }>
      )
    ),

  getClaimAtIndex: (args: {
    context: PonderContext;
    chainId: number;
    contractAddress: Address;
    targetToken: Address;
    claimIndex: bigint;
  }) =>
    tryPromise(() =>
      args.context.db.sql.execute(sql`
        SELECT
          claim_index AS claimIndex,
          lease_id AS leaseId,
          amount_usdt AS amountUsdt,
          target_chain_id AS targetChainId,
          beneficiary AS beneficiary
        FROM "untron_v3_claim"
        WHERE chain_id = ${args.chainId}
          AND contract_address = ${args.contractAddress}
          AND target_token = ${args.targetToken}
          AND claim_index = ${args.claimIndex}
        LIMIT 1;
      `)
    ).pipe(
      Effect.map((result) => {
        const row = (getRows(result)[0] ?? null) as null | {
          claimIndex: bigint;
          leaseId: bigint;
          amountUsdt: bigint;
          targetChainId: bigint;
          beneficiary: Address;
        };
        return row;
      })
    ),

  getClaimsFromIndex: (args: {
    context: PonderContext;
    chainId: number;
    contractAddress: Address;
    targetToken: Address;
    startIndex: bigint;
    limit: bigint;
  }) =>
    tryPromise(() =>
      args.context.db.sql.execute(sql`
        SELECT
          claim_index AS claimIndex,
          lease_id AS leaseId,
          amount_usdt AS amountUsdt,
          target_chain_id AS targetChainId,
          beneficiary AS beneficiary
        FROM "untron_v3_claim"
        WHERE chain_id = ${args.chainId}
          AND contract_address = ${args.contractAddress}
          AND target_token = ${args.targetToken}
          AND claim_index >= ${args.startIndex}
        ORDER BY claim_index ASC
        LIMIT ${args.limit};
      `)
    ).pipe(Effect.map((result) => getRows(result) as Claim[])),

  getSwapRatePpm: (args: {
    context: PonderContext;
    chainId: number;
    contractAddress: Address;
    targetToken: Address;
  }) =>
    tryPromise(() =>
      args.context.db.sql.execute(sql`
        SELECT
          rate_ppm AS ratePpm
        FROM "untron_v3_swap_rate"
        WHERE chain_id = ${args.chainId}
          AND contract_address = ${args.contractAddress}
          AND target_token = ${args.targetToken}
        LIMIT 1;
      `)
    ).pipe(
      Effect.map((result) => {
        const row = (getRows(result)[0] ?? null) as null | { ratePpm: bigint };
        return row?.ratePpm ?? null;
      })
    ),

  getBridgerRoutesForToken: (args: {
    context: PonderContext;
    chainId: number;
    contractAddress: Address;
    targetToken: Address;
  }) =>
    tryPromise(() =>
      args.context.db.sql.execute(sql`
        SELECT
          target_chain_id AS targetChainId,
          bridger AS bridger
        FROM "untron_v3_bridger_route"
        WHERE chain_id = ${args.chainId}
          AND contract_address = ${args.contractAddress}
          AND target_token = ${args.targetToken};
      `)
    ).pipe(
      Effect.map((result) => getRows(result) as Array<{ targetChainId: bigint; bridger: Address }>)
    ),
} as const;
