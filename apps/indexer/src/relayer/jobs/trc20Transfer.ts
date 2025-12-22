import type { RelayJobHandler } from "./types";

import { ERC20Abi } from "../../../abis/ERC20Abi";
import { decodeFunctionResult, encodeFunctionData, isAddress, type Address, type Hex } from "viem";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";
import { tronEvmAddressToBytes21 } from "../deps/tronProtocol";

function expectRecord(value: unknown, label: string): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid ${label} (expected object)`);
  }
  return value as Record<string, unknown>;
}

function expectAddress(value: unknown, label: string): Address {
  if (typeof value !== "string" || !isAddress(value)) throw new Error(`Invalid ${label} address`);
  return value as Address;
}

export const handleTrc20Transfer: RelayJobHandler<"trc20_transfer"> = async ({ ctx, job }) => {
  if (ctx.dryRun) return;

  const payload = expectRecord(job.payloadJson, "payloadJson");
  const tokenAddress = expectAddress(payload.tokenAddress, "payload.tokenAddress");
  const receiverAddress = expectAddress(payload.to, "payload.to");

  const receiverMap = await ctx.deps.getTronReceiverMap();
  const receiver = receiverMap.get(receiverAddress.toLowerCase());
  if (!receiver) {
    throw new Error(
      `Unknown receiver address (not in PREKNOWN_RECEIVER_SALTS mapping): ${receiverAddress}`
    );
  }

  const { wallet, callOpts } = ctx.deps.getTronGrpcClients();
  const controllerAddress = ctx.deps.getTronControllerEvmAddress();
  const controllerBytes21 = tronEvmAddressToBytes21(controllerAddress);

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

  const balance = await tronReadContract<bigint>({
    contractAddressBytes21: tronEvmAddressToBytes21(tokenAddress),
    abi: ERC20Abi,
    functionName: "balanceOf",
    args: [receiver.receiverAddress],
  });

  const sweepAmount = balance > 0n ? balance - 1n : 0n;
  if (sweepAmount === 0n) return;

  await ctx.deps.sendTronControllerPullFromReceivers({
    tokenAddress,
    receiverSalts: [receiver.receiverSalt],
  });
};
