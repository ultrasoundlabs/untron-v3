import { Effect } from "effect";
import { encodeFunctionData, type Address } from "viem";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { UntronV3Abi } from "../../../abis/evm/UntronV3Abi";
import { AppConfig } from "../../effect/config";
import { tryPromise } from "../../effect/tryPromise";
import { MainnetRelayer } from "../deps/mainnet";
import type { MainnetUserOperationCall } from "../deps/types";
import type { RelayJobHandlerContext } from "../jobs/types";
import { ClaimFillerRepository } from "./repository";
import { planQueueFill } from "./planner";
import { SwapPlanner, SwapPlanUnavailableError } from "./swapPlanner";

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
        abi: UntronV3Abi,
        functionName: "usdt",
      })
    )) as Address;

    const untronUsdtBalance = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: UntronV3Abi,
        functionName: "usdtBalance",
      })
    )) as bigint;

    if (untronUsdtBalance === 0n) return [] as const;

    const swapExecutor = (yield* tryPromise(() =>
      ctx.ponderContext.client.readContract({
        address: untronV3Address,
        abi: UntronV3Abi,
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
            abi: UntronV3Abi,
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
          abi: UntronV3Abi,
          functionName: "fill",
          args: [queue.targetToken, fillCount, swapPlan.swapExecutorCalls],
        }),
      });

      remainingUsdt -= totalUsdt;
    }

    return calls;
  });
