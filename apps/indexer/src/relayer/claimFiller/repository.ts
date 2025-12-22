import { Effect } from "effect";
import { sql } from "ponder";
import {
  untronV3BridgerRoute,
  untronV3Claim,
  untronV3ClaimQueue,
  untronV3SwapRate,
} from "ponder:schema";
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
          ${untronV3ClaimQueue.targetToken} AS targetToken,
          ${untronV3ClaimQueue.queueLength} AS queueLength
        FROM ${untronV3ClaimQueue}
        WHERE ${untronV3ClaimQueue.chainId} = ${args.chainId}
          AND ${untronV3ClaimQueue.contractAddress} = ${args.contractAddress}
          AND ${untronV3ClaimQueue.queueLength} > 0;
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
          ${untronV3Claim.claimIndex} AS claimIndex,
          ${untronV3Claim.leaseId} AS leaseId,
          ${untronV3Claim.amountUsdt} AS amountUsdt,
          ${untronV3Claim.targetChainId} AS targetChainId,
          ${untronV3Claim.beneficiary} AS beneficiary
        FROM ${untronV3Claim}
        WHERE ${untronV3Claim.chainId} = ${args.chainId}
          AND ${untronV3Claim.contractAddress} = ${args.contractAddress}
          AND ${untronV3Claim.targetToken} = ${args.targetToken}
          AND ${untronV3Claim.claimIndex} = ${args.claimIndex}
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
          ${untronV3Claim.claimIndex} AS claimIndex,
          ${untronV3Claim.leaseId} AS leaseId,
          ${untronV3Claim.amountUsdt} AS amountUsdt,
          ${untronV3Claim.targetChainId} AS targetChainId,
          ${untronV3Claim.beneficiary} AS beneficiary
        FROM ${untronV3Claim}
        WHERE ${untronV3Claim.chainId} = ${args.chainId}
          AND ${untronV3Claim.contractAddress} = ${args.contractAddress}
          AND ${untronV3Claim.targetToken} = ${args.targetToken}
          AND ${untronV3Claim.claimIndex} >= ${args.startIndex}
        ORDER BY ${untronV3Claim.claimIndex} ASC
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
          ${untronV3SwapRate.ratePpm} AS ratePpm
        FROM ${untronV3SwapRate}
        WHERE ${untronV3SwapRate.chainId} = ${args.chainId}
          AND ${untronV3SwapRate.contractAddress} = ${args.contractAddress}
          AND ${untronV3SwapRate.targetToken} = ${args.targetToken}
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
          ${untronV3BridgerRoute.targetChainId} AS targetChainId,
          ${untronV3BridgerRoute.bridger} AS bridger
        FROM ${untronV3BridgerRoute}
        WHERE ${untronV3BridgerRoute.chainId} = ${args.chainId}
          AND ${untronV3BridgerRoute.contractAddress} = ${args.contractAddress}
          AND ${untronV3BridgerRoute.targetToken} = ${args.targetToken};
      `)
    ).pipe(
      Effect.map((result) => getRows(result) as Array<{ targetChainId: bigint; bridger: Address }>)
    ),
} as const;
