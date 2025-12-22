import { createSmartAccountClient } from "permissionless";
import { type SafeVersion, toSafeSmartAccount } from "permissionless/accounts";
import {
  entryPoint06Abi,
  entryPoint06Address,
  entryPoint07Abi,
  entryPoint07Address,
} from "viem/account-abstraction";
import { privateKeyToAccount } from "viem/accounts";
import { http, isAddress, type Address, type Hash, type Hex, type PublicClient } from "viem";

import { parseBigintEnv, parseNumberEnv } from "../env";
import type { EntryPointVersion, RelayerDeps, SendMainnetUserOperationResult } from "./types";

export function createMainnetRelayer({
  getPublicClient,
}: {
  getPublicClient: (chain: "mainnet") => PublicClient;
}): Pick<RelayerDeps, "getMainnetRelayerAddress" | "sendMainnetUserOperation"> {
  let mainnetSafeAccountPromise: ReturnType<typeof toSafeSmartAccount> | null = null;
  let mainnetBundlerUrls: readonly string[] | null = null;

  const getMainnetBundlerUrls = (): readonly string[] => {
    if (mainnetBundlerUrls) return mainnetBundlerUrls;

    const raw = process.env.RELAYER_MAINNET_BUNDLER_URLS;
    if (!raw) throw new Error("Missing env var RELAYER_MAINNET_BUNDLER_URLS");

    const urls = raw
      .split(",")
      .map((value) => value.trim())
      .filter(Boolean);

    if (urls.length === 0) throw new Error("Missing env var RELAYER_MAINNET_BUNDLER_URLS");

    mainnetBundlerUrls = urls;
    return mainnetBundlerUrls;
  };

  const getMainnetSafeAccount = () => {
    if (mainnetSafeAccountPromise) return mainnetSafeAccountPromise;

    mainnetSafeAccountPromise = (async () => {
      const ownerPrivateKey = process.env.RELAYER_MAINNET_OWNER_PRIVATE_KEY;
      if (!ownerPrivateKey) throw new Error("Missing env var RELAYER_MAINNET_OWNER_PRIVATE_KEY");

      const safeVersionRaw = process.env.RELAYER_MAINNET_SAFE_VERSION ?? "1.4.1";
      if (safeVersionRaw !== "1.4.1" && safeVersionRaw !== "1.5.0") {
        throw new Error('Invalid RELAYER_MAINNET_SAFE_VERSION (expected "1.4.1" or "1.5.0")');
      }
      const safeVersion = safeVersionRaw as SafeVersion;

      const entryPointVersionRaw = process.env.RELAYER_MAINNET_ENTRYPOINT_VERSION ?? "0.7";
      if (entryPointVersionRaw !== "0.6" && entryPointVersionRaw !== "0.7") {
        throw new Error('Invalid RELAYER_MAINNET_ENTRYPOINT_VERSION (expected "0.6" or "0.7")');
      }
      const entryPointVersion = entryPointVersionRaw as EntryPointVersion;

      const entryPointAddressRaw = process.env.RELAYER_MAINNET_ENTRYPOINT_ADDRESS;
      if (entryPointAddressRaw && !isAddress(entryPointAddressRaw)) {
        throw new Error("Invalid env var RELAYER_MAINNET_ENTRYPOINT_ADDRESS");
      }
      const entryPointAddress =
        (entryPointAddressRaw as Address | undefined) ??
        (entryPointVersion === "0.6" ? entryPoint06Address : entryPoint07Address);

      const safeAddressRaw = process.env.RELAYER_MAINNET_SAFE_ADDRESS;
      if (safeAddressRaw && !isAddress(safeAddressRaw)) {
        throw new Error("Invalid env var RELAYER_MAINNET_SAFE_ADDRESS");
      }

      const saltNonce = parseBigintEnv("RELAYER_MAINNET_SAFE_SALT_NONCE", 0n);

      return toSafeSmartAccount({
        client: getPublicClient("mainnet"),
        owners: [privateKeyToAccount(ownerPrivateKey as Hex)],
        version: safeVersion,
        entryPoint: { address: entryPointAddress, version: entryPointVersion },
        address: safeAddressRaw as Address | undefined,
        saltNonce,
      });
    })();

    return mainnetSafeAccountPromise;
  };

  const getMainnetRelayerAddress = async (): Promise<Address> => {
    const account = await getMainnetSafeAccount();
    return account.address;
  };

  const sendMainnetUserOperation: RelayerDeps["sendMainnetUserOperation"] = async ({
    calls,
    bundlerUrls,
    timeoutBlocks,
    pollIntervalMs,
  }) => {
    if (calls.length === 0) throw new Error("sendMainnetUserOperation: expected at least 1 call");

    const resolvedBundlerUrls =
      bundlerUrls && bundlerUrls.length > 0 ? bundlerUrls : getMainnetBundlerUrls();

    const resolvedTimeoutBlocks =
      timeoutBlocks ?? parseBigintEnv("RELAYER_MAINNET_BUNDLER_TIMEOUT_BLOCKS", 20n);
    const resolvedPollIntervalMs =
      pollIntervalMs ?? parseNumberEnv("RELAYER_MAINNET_BUNDLER_POLL_INTERVAL_MS", 3_000);

    const normalizedCalls = calls.map((call) => ({
      to: call.to,
      value: call.value ?? 0n,
      data: call.data ?? "0x",
    }));

    const account = await getMainnetSafeAccount();
    const publicClient = getPublicClient("mainnet");

    const entryPointAddress = account.entryPoint.address;
    const entryPointAbi = account.entryPoint.version === "0.6" ? entryPoint06Abi : entryPoint07Abi;

    const sent: Array<{ bundlerUrl: string; userOpHash: Hash }> = [];
    let nextFromBlock = await publicClient.getBlockNumber();

    const checkInclusionUpTo = async (
      toBlock: bigint
    ): Promise<SendMainnetUserOperationResult | null> => {
      if (sent.length === 0) return null;
      if (toBlock < nextFromBlock) return null;

      const fromBlock = nextFromBlock;

      for (const attempt of sent) {
        const logs = await publicClient.getContractEvents({
          address: entryPointAddress,
          abi: entryPointAbi,
          eventName: "UserOperationEvent",
          args: { userOpHash: attempt.userOpHash },
          fromBlock,
          toBlock,
        });

        const log = logs[0];
        if (!log) continue;

        return {
          bundlerUrl: attempt.bundlerUrl,
          userOpHash: attempt.userOpHash,
          transactionHash: log.transactionHash,
          blockNumber: log.blockNumber,
        };
      }

      nextFromBlock = toBlock + 1n;
      return null;
    };

    const errors: string[] = [];

    for (const bundlerUrl of resolvedBundlerUrls) {
      const includedBeforeSend = await checkInclusionUpTo(await publicClient.getBlockNumber());
      if (includedBeforeSend) return includedBeforeSend;

      try {
        const smartAccountClient = createSmartAccountClient({
          account,
          bundlerTransport: http(bundlerUrl),
          client: publicClient,
        });

        const userOpHash = await smartAccountClient.sendUserOperation({
          account,
          calls: normalizedCalls,
        });

        sent.push({ bundlerUrl, userOpHash });

        const startWaitBlock = await publicClient.getBlockNumber();
        const deadlineBlock = startWaitBlock + resolvedTimeoutBlocks;

        while (true) {
          const head = await publicClient.getBlockNumber();

          const included = await checkInclusionUpTo(head);
          if (included) return included;

          if (head >= deadlineBlock) break;
          await new Promise((resolve) => setTimeout(resolve, resolvedPollIntervalMs));
        }

        const includedAfterTimeout = await checkInclusionUpTo(await publicClient.getBlockNumber());
        if (includedAfterTimeout) return includedAfterTimeout;
      } catch (error) {
        const includedAfterError = await checkInclusionUpTo(await publicClient.getBlockNumber());
        if (includedAfterError) return includedAfterError;

        const errorMessage =
          error instanceof Error ? `${error.name}: ${error.message}` : String(error);
        errors.push(`${bundlerUrl}: ${errorMessage}`);
      }
    }

    const sentHashes = sent.map((s) => `${s.bundlerUrl} => ${s.userOpHash}`).join(", ");
    const errorsJoined = errors.length > 0 ? ` Errors: ${errors.join(" | ")}` : "";
    throw new Error(
      `UserOperation not included after trying ${resolvedBundlerUrls.length} bundler(s). Sent: ${sentHashes}.${errorsJoined}`
    );
  };

  return { getMainnetRelayerAddress, sendMainnetUserOperation };
}
