import type { RelayJobHandler } from "./types";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { UntronControllerAbi } from "../../../abis/tron/UntronControllerAbi";
import { decodeFunctionResult, encodeFunctionData, type Address, type Hex } from "viem";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import { tronEvmAddressToBytes21 } from "../deps/tronProtocol";

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000" as const;

export const handleTronHeartbeat: RelayJobHandler<"tron_heartbeat"> = async ({ ctx }) => {
  if (ctx.dryRun) return;

  const controllerAddress = ctx.deps.getTronControllerEvmAddress();
  const controllerBytes21 = tronEvmAddressToBytes21(controllerAddress);
  const receiverMap = await ctx.deps.getTronReceiverMap();
  if (receiverMap.size === 0) return;

  const { wallet, callOpts } = ctx.deps.getTronGrpcClients();

  const grpcUnary = async (call: any, req: any) =>
    await new Promise<any>((resolve, reject) => {
      call(req, callOpts.metadata, (err: any, res: any) => {
        if (err) return reject(err);
        if (!res) return reject(new Error("Empty gRPC response"));
        resolve(res);
      });
    });

  const tronReadContract = async <T>({
    contractAddressBytes21,
    abi,
    functionName,
    args,
  }: {
    contractAddressBytes21: Buffer;
    abi: any;
    functionName: string;
    args?: readonly unknown[];
  }): Promise<T> => {
    const data = encodeFunctionData({
      abi,
      functionName: functionName as never,
      args: (args ?? []) as never,
    });

    const request = TriggerSmartContract.fromPartial({
      ownerAddress: controllerBytes21,
      contractAddress: contractAddressBytes21,
      callValue: 0,
      data: Buffer.from(data.slice(2), "hex"),
    });

    const res = await grpcUnary(wallet.triggerConstantContract.bind(wallet), request);
    if (!res.result?.result) {
      const msg = res.result?.message?.length ? res.result.message.toString("utf8") : "unknown";
      throw new Error(`Tron triggerConstantContract failed: ${msg}`);
    }

    const buf = res.constantResult?.[0];
    if (!buf || buf.length === 0) {
      throw new Error("Tron triggerConstantContract returned empty constantResult");
    }

    return decodeFunctionResult({
      abi,
      functionName: functionName as never,
      data: `0x${buf.toString("hex")}` as Hex,
    }) as T;
  };

  const usdtAddress = await tronReadContract<Address>({
    contractAddressBytes21: controllerBytes21,
    abi: UntronControllerAbi,
    functionName: "usdt",
  });
  if (usdtAddress.toLowerCase() === ZERO_ADDRESS) return;
  const usdtBytes21 = tronEvmAddressToBytes21(usdtAddress);

  const entries = Array.from(receiverMap.values());
  const balances = await Promise.all(
    entries.map(async (entry) => {
      const balance = await tronReadContract<bigint>({
        contractAddressBytes21: usdtBytes21,
        abi: ERC20Abi,
        functionName: "balanceOf",
        args: [entry.receiverAddress],
      });

      const sweepAmount = balance > 0n ? balance - 1n : 0n;
      return { entry, sweepAmount };
    })
  );

  const receiverSalts = balances.filter((r) => r.sweepAmount > 0n).map((r) => r.entry.receiverSalt);

  if (receiverSalts.length === 0) return;

  await ctx.deps.sendTronControllerPullFromReceivers({
    tokenAddress: usdtAddress,
    receiverSalts,
  });
};
