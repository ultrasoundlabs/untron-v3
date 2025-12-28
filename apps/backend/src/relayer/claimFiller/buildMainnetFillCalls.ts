import { Effect } from "effect";
import { sql } from "ponder";
import type { Context as PonderContext } from "ponder:registry";
import { encodeFunctionData, type Address } from "viem";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { untronV3Abi } from "@untron/v3-contracts";
import { AppConfig } from "../../effect/config";
import { tryPromise } from "../../effect/tryPromise";
import { MainnetRelayer } from "../deps/mainnet";
import type { MainnetUserOperationCall } from "../deps/types";
import { expectBigint, type RelayJobHandlerContext } from "../jobs/types";
import { getRows } from "../sqlRows";
import { SwapPlanner, SwapPlanUnavailableError } from "./swapPlanner";
import type { Claim } from "./types";

const lower = (address: Address) => address.toLowerCase();

const minBigint = (a: bigint, b: bigint) => (a < b ? a : b);

export const buildMainnetFillCalls = (ctx: RelayJobHandlerContext) =>
  Effect.gen(function* () {
    const chainId = ctx.ponderContext.chain.id;
    const untronV3Address = ctx.ponderContext.contracts.UntronV3.address as Address;

    const runtime = yield* AppConfig.relayerRuntime();
    const maxClaimsPerQueue = BigInt(runtime.fillMaxClaimsPerQueue);

    const usdt = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: untronV3Abi,
        functionName: "usdt",
      })
    )) as Address;

    const untronUsdtBalance = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: untronV3Abi,
        functionName: "usdtBalance",
      })
    )) as bigint;

    if (untronUsdtBalance === 0n) return [] as const;

    const swapExecutor = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: untronV3Abi,
        functionName: "SWAP_EXECUTOR",
      })
    )) as Address;

    const queues = yield* ClaimFillerRepository.getNonEmptyClaimQueues({
      context: ctx.ponderContext,
      chainId,
      contractAddress: untronV3Address,
    });
    if (queues.length === 0) return [] as const;

    const withHeads = yield* Effect.forEach(queues, (queue) =>
      Effect.gen(function* () {
        const nextIndex = (yield* tryPromise(() =>
          ctx.ponderContext.client.readContract({
            address: untronV3Address,
            abi: untronV3Abi,
            functionName: "nextIndexByTargetToken",
            args: [queue.targetToken],
          })
        )) as bigint;

        const pendingCount = queue.queueLength > nextIndex ? queue.queueLength - nextIndex : 0n;
        if (pendingCount === 0n) return null;

        const headClaim = yield* ClaimFillerRepository.getClaimAtIndex({
          context: ctx.ponderContext,
          chainId,
          contractAddress: untronV3Address,
          targetToken: queue.targetToken,
          claimIndex: nextIndex,
        });
        if (!headClaim) {
          return yield* Effect.fail(
            new Error(
              `Missing claim row for targetToken=${queue.targetToken} claimIndex=${nextIndex.toString()}`
            )
          );
        }

        return {
          targetToken: queue.targetToken,
          queueLength: queue.queueLength,
          nextIndex,
          pendingCount,
          headClaimAmountUsdt: headClaim.amountUsdt,
        } as const;
      })
    ).pipe(
      Effect.map((rows) => rows.filter((row): row is NonNullable<typeof row> => row !== null))
    );

    if (withHeads.length === 0) return [] as const;

    const sortedQueues = withHeads.sort((a, b) => {
      if (a.headClaimAmountUsdt === b.headClaimAmountUsdt) {
        return lower(a.targetToken).localeCompare(lower(b.targetToken));
      }
      return a.headClaimAmountUsdt < b.headClaimAmountUsdt ? -1 : 1;
    });

    const swapPlanner = yield* SwapPlanner;
    const hasSwapProviders = swapPlanner.providerCount > 0;

    let relayerUsdtBalance: bigint | null = null;
    const getRelayerUsdtBalance = hasSwapProviders
      ? Effect.gen(function* () {
          if (relayerUsdtBalance !== null) return relayerUsdtBalance;

          const relayer = yield* MainnetRelayer;
          const address = yield* relayer.getAddress();
          const balance = (yield* tryPromise(() =>
            ctx.ponderContext.client.readContract({
              address: usdt,
              abi: ERC20Abi,
              functionName: "balanceOf",
              args: [address],
            })
          )) as bigint;

          relayerUsdtBalance = balance;
          return relayerUsdtBalance;
        })
      : Effect.succeed(0n);

    const calls: MainnetUserOperationCall[] = [];
    let remainingUsdt = untronUsdtBalance;

    for (const queue of sortedQueues) {
      if (remainingUsdt === 0n) break;

      const maxClaims = minBigint(maxClaimsPerQueue, queue.pendingCount);
      if (maxClaims === 0n) continue;

      const claims = yield* ClaimFillerRepository.getClaimsFromIndex({
        context: ctx.ponderContext,
        chainId,
        contractAddress: untronV3Address,
        targetToken: queue.targetToken,
        startIndex: queue.nextIndex,
        limit: maxClaims,
      });

      const bridgerRoutes = yield* ClaimFillerRepository.getBridgerRoutesForToken({
        context: ctx.ponderContext,
        chainId,
        contractAddress: untronV3Address,
        targetToken: queue.targetToken,
      }).pipe(Effect.map((rows) => new Map(rows.map((r) => [r.targetChainId, r.bridger]))));

      const isUsdtQueue = lower(queue.targetToken) === lower(usdt);
      const ratePpm = isUsdtQueue
        ? null
        : yield* ClaimFillerRepository.getSwapRatePpm({
            context: ctx.ponderContext,
            chainId,
            contractAddress: untronV3Address,
            targetToken: queue.targetToken,
          });

      const { fillCount, totalUsdt, expectedOutTotal } = planQueueFill({
        chainId,
        usdt,
        targetToken: queue.targetToken,
        ratePpm,
        maxClaims,
        availableUsdt: remainingUsdt,
        claims,
        bridgerRoutes,
      });

      if (fillCount === 0n) continue;

      const swapPlan = isUsdtQueue
        ? { safePreCalls: [] as const, swapExecutorCalls: [] as const }
        : hasSwapProviders
          ? yield* Effect.catchIf(
              swapPlanner.planSwap({
                usdt,
                targetToken: queue.targetToken,
                amountInUsdt: totalUsdt,
                minAmountOut: expectedOutTotal,
                swapExecutor,
                maxTopUpUsdt: yield* getRelayerUsdtBalance,
              }),
              (e) => e instanceof SwapPlanUnavailableError,
              () => Effect.succeed(null)
            ).pipe(
              Effect.map((plan) => plan ?? null),
              Effect.catchAll(() => Effect.succeed(null))
            )
          : null;

      if (!swapPlan) continue;

      calls.push(...swapPlan.safePreCalls);
      calls.push({
        to: untronV3Address,
        value: 0n,
        data: encodeFunctionData({
          abi: untronV3Abi,
          functionName: "fill",
          args: [queue.targetToken, fillCount, swapPlan.swapExecutorCalls],
        }),
      });

      remainingUsdt -= totalUsdt;
    }

    return calls;
  });

const ClaimFillerRepository = {
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
      Effect.map((result) => {
        const rows = getRows(result) as Array<{ targetToken: unknown; queueLength: unknown }>;
        return rows.map((row) => ({
          targetToken: String(row.targetToken).toLowerCase() as Address,
          queueLength: expectBigint(row.queueLength, "queueLength"),
        }));
      })
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
          claimIndex: unknown;
          leaseId: unknown;
          amountUsdt: unknown;
          targetChainId: unknown;
          beneficiary: unknown;
        };
        if (!row) return null;
        return {
          claimIndex: expectBigint(row.claimIndex, "claimIndex"),
          leaseId: expectBigint(row.leaseId, "leaseId"),
          amountUsdt: expectBigint(row.amountUsdt, "amountUsdt"),
          targetChainId: expectBigint(row.targetChainId, "targetChainId"),
          beneficiary: String(row.beneficiary).toLowerCase() as Address,
        } satisfies Claim;
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
    ).pipe(
      Effect.map((result) => {
        const rows = getRows(result) as Array<{
          claimIndex: unknown;
          leaseId: unknown;
          amountUsdt: unknown;
          targetChainId: unknown;
          beneficiary: unknown;
        }>;
        return rows.map(
          (row) =>
            ({
              claimIndex: expectBigint(row.claimIndex, "claimIndex"),
              leaseId: expectBigint(row.leaseId, "leaseId"),
              amountUsdt: expectBigint(row.amountUsdt, "amountUsdt"),
              targetChainId: expectBigint(row.targetChainId, "targetChainId"),
              beneficiary: String(row.beneficiary).toLowerCase() as Address,
            }) satisfies Claim
        );
      })
    ),

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
        const row = (getRows(result)[0] ?? null) as null | { ratePpm: unknown };
        return row ? expectBigint(row.ratePpm, "ratePpm") : null;
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
      Effect.map((result) => {
        const rows = getRows(result) as Array<{ targetChainId: unknown; bridger: unknown }>;
        return rows.map((row) => ({
          targetChainId: expectBigint(row.targetChainId, "targetChainId"),
          bridger: String(row.bridger).toLowerCase() as Address,
        }));
      })
    ),
} as const;

const RATE_DENOMINATOR = 1_000_000n;
const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;

const planQueueFill = (args: {
  chainId: number;
  usdt: Address;
  targetToken: Address;
  ratePpm: bigint | null;
  maxClaims: bigint;
  availableUsdt: bigint;
  claims: readonly Claim[];
  bridgerRoutes: ReadonlyMap<bigint, Address>;
}) => {
  let remainingUsdt = args.availableUsdt;
  let fillCount = 0n;
  let totalUsdt = 0n;
  let expectedOutTotal = 0n;

  const isUsdt = args.targetToken.toLowerCase() === args.usdt.toLowerCase();
  if (!isUsdt && (!args.ratePpm || args.ratePpm === 0n)) {
    return { fillCount: 0n, totalUsdt: 0n, expectedOutTotal: 0n } as const;
  }

  const max = Number(
    args.maxClaims < BigInt(args.claims.length) ? args.maxClaims : BigInt(args.claims.length)
  );

  for (let i = 0; i < max; i++) {
    const claim = args.claims[i]!;
    const amountUsdt = claim.amountUsdt;
    if (remainingUsdt < amountUsdt) break;

    const needsBridge = claim.targetChainId !== BigInt(args.chainId);
    if (needsBridge) {
      const bridger = args.bridgerRoutes.get(claim.targetChainId);
      if (!bridger || bridger.toLowerCase() === ZERO_ADDRESS) break;
    }

    totalUsdt += amountUsdt;
    remainingUsdt -= amountUsdt;
    fillCount += 1n;

    if (!isUsdt) {
      expectedOutTotal += (amountUsdt * (args.ratePpm as bigint)) / RATE_DENOMINATOR;
    }
  }

  return { fillCount, totalUsdt, expectedOutTotal } as const;
};
