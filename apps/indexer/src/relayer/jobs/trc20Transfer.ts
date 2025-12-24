import { Effect } from "effect";
import { encodeFunctionData, isAddress, type Address, type Hex, type PublicClient } from "viem";

import { NumberMessage, type BlockExtention } from "@untron/tron-protocol/api";

import { TronLightClientAbi } from "../../../abis/evm/TronLightClientAbi";
import { UntronV3Abi } from "../../../abis/evm/UntronV3Abi";
import { tryPromise } from "../../effect/tryPromise";
import { MainnetRelayer } from "../deps/mainnet";
import { PublicClients } from "../deps/publicClients";
import { TronGrpc } from "../deps/tronGrpc";
import { buildTronLightClientProveBlocksCallToCheckpointBlock } from "../tronLightClientPublisher";
import { computeTronTxIdFromEncodedTx, computeTronTxMerkleProof } from "../tronProofs";

import { TronRelayer } from "../deps/tron";
import { getKnownTronReceiver } from "../receivers";
import { resolveContractAddress } from "../resolveContractAddress";
import type { RelayJobRow } from "../types";
import type { RelayJobHandlerContext } from "./types";
import { tronLightClientProveBlocksSent } from "ponder:schema";

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

function expectString(value: unknown, label: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`Invalid ${label}`);
  return value;
}

function expectHex(value: unknown, label: string): Hex {
  const raw = expectString(value, label).toLowerCase();
  if (!/^0x[0-9a-f]+$/.test(raw)) throw new Error(`Invalid ${label} (expected 0x-hex)`);
  return raw as Hex;
}

function expectBigint(value: unknown, label: string): bigint {
  if (typeof value === "bigint") return value;
  if (typeof value === "number" && Number.isSafeInteger(value)) return BigInt(value);
  if (typeof value === "string" && value.length > 0) {
    try {
      return BigInt(value);
    } catch {
      // fall through
    }
  }
  throw new Error(`Invalid ${label} (expected bigint-compatible value)`);
}

async function fetchTronBlockByNum(args: {
  wallet: any;
  metadata: unknown;
  blockNumber: bigint;
  timeoutMs?: number;
}): Promise<BlockExtention> {
  const req = NumberMessage.fromPartial({ num: args.blockNumber.toString() });
  return await new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      reject(new Error(`Timeout in getBlockByNum2(${args.blockNumber.toString()})`));
    }, args.timeoutMs ?? 15_000);

    args.wallet.getBlockByNum2(req, args.metadata, (err: unknown, res: BlockExtention | null) =>
      err || !res
        ? (clearTimeout(timeout), reject(err ?? new Error("Empty response from getBlockByNum2")))
        : (clearTimeout(timeout), resolve(res))
    );
  });
}

function isIgnorablePreEntitleFailure(error: unknown): boolean {
  const msg = error instanceof Error ? `${error.name}: ${error.message}` : String(error);
  return (
    msg.includes("DepositAlreadyProcessed") ||
    msg.includes("DepositNotAfterLastReceiverPull") ||
    msg.includes("NoActiveLease") ||
    msg.includes("InvalidReceiverForSalt") ||
    msg.includes("NotTronUsdt")
  );
}

const tronUsdtCache = new Map<string, Promise<Address>>();

async function loadTronUsdt(args: {
  mainnetClient: PublicClient;
  untronV3Address: Address;
}): Promise<Address> {
  const key = args.untronV3Address.toLowerCase();
  const existing = tronUsdtCache.get(key);
  if (existing) return existing;

  const promise = (async () => {
    const tronUsdt = (await args.mainnetClient.readContract({
      address: args.untronV3Address,
      abi: UntronV3Abi,
      functionName: "tronUsdt",
    })) as Address;
    return tronUsdt.toLowerCase() as Address;
  })();

  tronUsdtCache.set(key, promise);
  return promise;
}

export const handleTrc20Transfer = ({
  ctx,
  job,
}: {
  job: RelayJobRow & { kind: "trc20_transfer" };
  ctx: RelayJobHandlerContext;
}) =>
  Effect.gen(function* () {
    if (ctx.dryRun) return;

    const payload = expectRecord(job.payloadJson, "payloadJson");
    const tokenAddress = expectAddress(payload.tokenAddress, "payload.tokenAddress");
    const receiverAddress = expectAddress(payload.to, "payload.to");
    const tronBlockNumber = expectBigint(payload.blockNumber, "payload.blockNumber");
    const transactionHash = expectHex(payload.transactionHash, "payload.transactionHash");

    yield* Effect.logDebug("[trc20_transfer] handle").pipe(
      Effect.annotateLogs({
        tokenAddress,
        receiverAddress,
        tronBlockNumber: tronBlockNumber.toString(),
        transactionHash,
      })
    );

    const receiver = yield* getKnownTronReceiver(receiverAddress);

    const sweepReceiver = () =>
      Effect.gen(function* () {
        const balance = yield* TronRelayer.getErc20BalanceOf({
          tokenAddress,
          account: receiver.receiverAddress,
        });

        const sweepAmount = balance > 0n ? balance - 1n : 0n;
        if (sweepAmount === 0n) return;

        yield* TronRelayer.sendTronControllerPullFromReceivers({
          tokenAddress,
          receiverSalts: [receiver.receiverSalt],
        });
      });

    const controllerUsdt = (
      (yield* TronRelayer.getControllerUsdt()) as Address
    ).toLowerCase() as Address;
    const isControllerUsdt = tokenAddress.toLowerCase() === controllerUsdt;

    if (!isControllerUsdt) {
      yield* sweepReceiver();
      return;
    }

    const publicClients = yield* PublicClients;
    const tronGrpc = yield* TronGrpc;
    const mainnetClient = yield* publicClients.get("mainnet");

    const tronLightClientAddress = resolveContractAddress({
      ponderContracts: ctx.ponderContext.contracts,
      contractName: "TronLightClient",
      envVar: "TRON_LIGHT_CLIENT_ADDRESS",
    });
    const untronV3Address = resolveContractAddress({
      ponderContracts: ctx.ponderContext.contracts,
      contractName: "UntronV3",
      envVar: "UNTRON_V3_ADDRESS",
    });

    const tronUsdt = yield* tryPromise(() => loadTronUsdt({ mainnetClient, untronV3Address }));
    if (tokenAddress.toLowerCase() !== tronUsdt) {
      yield* sweepReceiver();
      return;
    }

    {
      const { wallet, callOpts } = yield* tronGrpc.get();

      const block = yield* tryPromise(() =>
        fetchTronBlockByNum({
          wallet,
          metadata: callOpts.metadata,
          blockNumber: tronBlockNumber,
          timeoutMs: 15_000,
        })
      );

      const headerRaw = block.blockHeader?.rawData;
      if (!headerRaw) throw new Error("Tron block missing header/rawData");

      const tronBlockTimestampSec = BigInt(headerRaw.timestamp.toString()) / 1000n;

      const lastPullTs = (yield* tryPromise(() =>
        mainnetClient.readContract({
          address: untronV3Address,
          abi: UntronV3Abi,
          functionName: "lastReceiverPullTimestamp",
          args: [receiver.receiverSalt],
        })
      )) as bigint;

      if (lastPullTs !== 0n && tronBlockTimestampSec <= lastPullTs) return;

      const proof = computeTronTxMerkleProof({ block, txidHex: transactionHash });
      const txId = computeTronTxIdFromEncodedTx(proof.encodedTx);

      const alreadyProcessed = (yield* tryPromise(() =>
        mainnetClient.readContract({
          address: untronV3Address,
          abi: UntronV3Abi,
          functionName: "depositProcessed",
          args: [txId],
        })
      )) as boolean;
      if (alreadyProcessed) return;

      const tronBlockPublished = yield* tryPromise(() =>
        mainnetClient.readContract({
          address: tronLightClientAddress,
          abi: TronLightClientAbi,
          functionName: "getTxTrieRoot",
          args: [tronBlockNumber],
        })
      ).pipe(
        Effect.as(true as const),
        Effect.catchAll(() => Effect.succeed(false as const))
      );

      if (!tronBlockPublished) {
        const PROVE_BLOCKS_COOLDOWN_BLOCKS = 5n;
        const proveId = `${job.chainId}:${tronLightClientAddress.toLowerCase()}:${tronBlockNumber.toString()}`;

        const lastAttempt = yield* tryPromise(() =>
          ctx.ponderContext.db.find(tronLightClientProveBlocksSent, { id: proveId })
        );

        if (
          lastAttempt &&
          lastAttempt.lastAttemptAtBlockNumber >= ctx.headBlockNumber - PROVE_BLOCKS_COOLDOWN_BLOCKS
        ) {
          // Another job recently attempted to prove this block; avoid spamming userOps.
        } else {
          yield* tryPromise(() =>
            ctx.ponderContext.db
              .insert(tronLightClientProveBlocksSent)
              .values({
                id: proveId,
                chainId: job.chainId,
                tronLightClientAddress: tronLightClientAddress.toLowerCase() as `0x${string}`,
                tronBlockNumber,
                lastAttemptAtBlockNumber: ctx.headBlockNumber,
                lastAttemptAtBlockTimestamp: ctx.headBlockTimestamp,
                lockedBy: job.id,
                includedTransactionHash: null,
                includedAtBlockNumber: null,
              })
              .onConflictDoUpdate({
                lastAttemptAtBlockNumber: ctx.headBlockNumber,
                lastAttemptAtBlockTimestamp: ctx.headBlockTimestamp,
                lockedBy: job.id,
              })
          );

          const proveCall = yield* buildTronLightClientProveBlocksCallToCheckpointBlock({
            context: ctx.ponderContext,
            mainnetClient,
            tronLightClientAddress,
            tronBlockNumber,
            fetchTronBlockByNum: (blockNumber) =>
              tryPromise(() =>
                fetchTronBlockByNum({
                  wallet,
                  metadata: callOpts.metadata,
                  blockNumber,
                  timeoutMs: 15_000,
                })
              ),
          });

          if (!proveCall) {
            return yield* Effect.fail(
              new Error(
                "Tron block not yet published in TronLightClient; no proveBlocks call planned."
              )
            );
          }

          const included = yield* MainnetRelayer.sendUserOperation({ calls: [proveCall] });

          yield* tryPromise(() =>
            ctx.ponderContext.db.update(tronLightClientProveBlocksSent, { id: proveId }).set({
              includedTransactionHash: included.transactionHash,
              includedAtBlockNumber: included.blockNumber,
              lockedBy: null,
              lastAttemptAtBlockNumber: ctx.headBlockNumber,
              lastAttemptAtBlockTimestamp: ctx.headBlockTimestamp,
            })
          );

          return yield* Effect.fail(
            new Error(
              "Tron block not yet published in TronLightClient; proveBlocks submitted (possibly chunked), retry later."
            )
          );
        }
      }

      const args = [
        receiver.receiverSalt,
        tronBlockNumber,
        proof.encodedTx,
        [...proof.proof] as readonly Hex[],
        proof.index,
      ] as const;

      const relayerAddress = yield* MainnetRelayer.getAddress();

      const simulation = yield* tryPromise(() =>
        mainnetClient.simulateContract({
          address: untronV3Address,
          abi: UntronV3Abi,
          functionName: "preEntitle",
          args,
          account: relayerAddress,
        })
      ).pipe(
        Effect.map(() => "ok" as const),
        Effect.catchAll((error) =>
          isIgnorablePreEntitleFailure(error) ? Effect.succeed("skip" as const) : Effect.fail(error)
        )
      );

      if (simulation === "skip") return;

      const data = encodeFunctionData({
        abi: UntronV3Abi,
        functionName: "preEntitle",
        args,
      });

      yield* MainnetRelayer.sendUserOperation({ calls: [{ to: untronV3Address, data }] });
      return;
    }
  });
