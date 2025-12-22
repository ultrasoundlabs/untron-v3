import { Effect, Layer } from "effect";
import { encodeFunctionData, type Address } from "viem";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import type { MainnetUserOperationCall } from "../deps/types";

import type { SwapExecutorCall } from "./types";

export type SwapPlanningInput = Readonly<{
  usdt: Address;
  targetToken: Address;
  amountInUsdt: bigint;
  minAmountOut: bigint;
  swapExecutor: Address;
  maxTopUpUsdt: bigint;
}>;

export type SwapPlan = Readonly<{
  safePreCalls: readonly MainnetUserOperationCall[];
  swapExecutorCalls: readonly SwapExecutorCall[];
}>;

export class SwapPlanUnavailableError extends Error {
  readonly _tag = "SwapPlanUnavailableError";
  constructor(message: string) {
    super(message);
    this.name = "SwapPlanUnavailableError";
  }
}

export type SwapProvider = Readonly<{
  name: string;
  planSwap: (input: SwapPlanningInput) => Effect.Effect<SwapPlan, Error>;
}>;

export const makeSafeTopUpUsdtCall = (args: {
  usdt: Address;
  to: Address;
  amount: bigint;
}): MainnetUserOperationCall => ({
  to: args.usdt,
  value: 0n,
  data: encodeFunctionData({
    abi: ERC20Abi,
    functionName: "transfer",
    args: [args.to, args.amount],
  }),
});

export class SwapPlanner extends Effect.Tag("SwapPlanner")<
  SwapPlanner,
  {
    readonly providerCount: number;
    readonly planSwap: (
      input: SwapPlanningInput
    ) => Effect.Effect<SwapPlan, SwapPlanUnavailableError | Error>;
  }
>() {
  static readonly Live = (providers: readonly SwapProvider[] = []) =>
    Layer.succeed(
      this,
      SwapPlanner.of({
        providerCount: providers.length,
        planSwap: (input) => {
          if (providers.length === 0) {
            return Effect.fail(
              new SwapPlanUnavailableError(
                `No swap providers configured for targetToken=${input.targetToken}`
              )
            );
          }
          return Effect.firstSuccessOf(
            providers.map((p) =>
              p.planSwap(input).pipe(
                Effect.timeout("5 seconds"),
                Effect.mapError((e) => (e instanceof Error ? e : new Error(String(e))))
              )
            )
          );
        },
      })
    );
}
