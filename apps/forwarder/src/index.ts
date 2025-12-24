import "dotenv/config";
import {
  createPublicClient,
  createWalletClient,
  defineChain,
  fallback,
  formatUnits,
  http,
  parseAbi,
  parseUnits,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import pino from "pino";

const log = pino({
  level: process.env.LOG_LEVEL ?? "info",
  timestamp: pino.stdTimeFunctions.isoTime,
});

process.on("unhandledRejection", (err) => log.error({ err }, "unhandledRejection"));
process.on("uncaughtException", (err) => {
  log.fatal({ err }, "uncaughtException");
  setTimeout(() => process.exit(1), 50).unref();
});

type Env = {
  RPC_URLS: string[];
  FORWARDER_CONTRACT_ADDRESS: `0x${string}`;
  PRIVATE_KEY: `0x${string}`;
  POLL_INTERVAL_MS: number;
  MIN_USDT: string;
};

function parseRpcUrls(raw: string): string[] {
  const urls = raw
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);

  if (urls.length === 0) return [];

  for (const url of urls) {
    try {
      // Validate only; do not keep parsed object (preserve original string).
      // eslint-disable-next-line no-new
      new URL(url);
    } catch {
      throw new Error(`Invalid RPC url: ${url}`);
    }
  }

  return urls;
}

function mustGetEnv(name: string): string {
  const value = process.env[name];
  if (!value) throw new Error(`Missing required env var: ${name}`);
  return value;
}

function getEnv(): Env {
  const rpcUrls = parseRpcUrls(mustGetEnv("RPC_URLS"));
  if (rpcUrls.length === 0) throw new Error("RPC_URLS must contain at least one URL");
  const forwarderAddress = mustGetEnv(
    "FORWARDER_CONTRACT_ADDRESS"
  ) as Env["FORWARDER_CONTRACT_ADDRESS"];
  const privateKey = mustGetEnv("PRIVATE_KEY") as Env["PRIVATE_KEY"];

  const pollIntervalMsRaw = process.env.POLL_INTERVAL_MS ?? "15000";
  const pollIntervalMs = Number(pollIntervalMsRaw);
  if (!Number.isFinite(pollIntervalMs) || pollIntervalMs < 1000) {
    throw new Error(`Invalid POLL_INTERVAL_MS: ${pollIntervalMsRaw}`);
  }

  const minUsdt = process.env.MIN_USDT ?? "1";

  return {
    RPC_URLS: rpcUrls,
    FORWARDER_CONTRACT_ADDRESS: forwarderAddress,
    PRIVATE_KEY: privateKey,
    POLL_INTERVAL_MS: pollIntervalMs,
    MIN_USDT: minUsdt,
  };
}

function safeRpcLabel(url: string): string {
  try {
    const u = new URL(url);
    return `${u.protocol}//${u.host}${u.pathname === "/" ? "" : u.pathname}`;
  } catch {
    return "<invalid>";
  }
}

const forwarderAbi = parseAbi([
  "function TOKEN() view returns (address)",
  "function OFT() view returns (address)",
  "function DST_EID() view returns (uint32)",
  "function BENEFICIARY() view returns (bytes32)",
  "function forward(uint256 amountLD) payable",
]);

const erc20Abi = parseAbi([
  "function balanceOf(address) view returns (uint256)",
  "function decimals() view returns (uint8)",
  "function symbol() view returns (string)",
]);

const oftAbi = parseAbi([
  "function quoteSend((uint32,bytes32,uint256,uint256,bytes,bytes,bytes),bool) view returns ((uint256,uint256))",
]);

async function main(): Promise<void> {
  const env = getEnv();
  const account = privateKeyToAccount(env.PRIVATE_KEY);

  const chainIdRaw = process.env.CHAIN_ID;
  const chain =
    chainIdRaw && Number.isFinite(Number(chainIdRaw))
      ? defineChain({
          id: Number(chainIdRaw),
          name: process.env.CHAIN_NAME ?? "custom",
          nativeCurrency: { name: "ETH", symbol: "ETH", decimals: 18 },
          rpcUrls: { default: { http: env.RPC_URLS } },
        })
      : undefined;

  const transport = fallback(env.RPC_URLS.map((url) => http(url)));

  const publicClient = createPublicClient({
    chain,
    transport,
  });

  const walletClient = createWalletClient({
    account,
    chain,
    transport,
  });

  log.info(
    {
      signer: account.address,
      forwarder: env.FORWARDER_CONTRACT_ADDRESS,
      rpcCount: env.RPC_URLS.length,
      rpcs: env.RPC_URLS.map(safeRpcLabel),
    },
    "forwarder service starting"
  );

  const tokenAddress = (await publicClient.readContract({
    address: env.FORWARDER_CONTRACT_ADDRESS,
    abi: forwarderAbi,
    functionName: "TOKEN",
  })) as `0x${string}`;

  const [decimals, symbol, oftAddress, dstEidRaw, beneficiary] = await Promise.all([
    publicClient.readContract({ address: tokenAddress, abi: erc20Abi, functionName: "decimals" }),
    publicClient.readContract({ address: tokenAddress, abi: erc20Abi, functionName: "symbol" }),
    publicClient.readContract({
      address: env.FORWARDER_CONTRACT_ADDRESS,
      abi: forwarderAbi,
      functionName: "OFT",
    }),
    publicClient.readContract({
      address: env.FORWARDER_CONTRACT_ADDRESS,
      abi: forwarderAbi,
      functionName: "DST_EID",
    }),
    publicClient.readContract({
      address: env.FORWARDER_CONTRACT_ADDRESS,
      abi: forwarderAbi,
      functionName: "BENEFICIARY",
    }),
  ]);

  const dstEid = typeof dstEidRaw === "bigint" ? Number(dstEidRaw) : dstEidRaw;
  const minUsdtThreshold = parseUnits(env.MIN_USDT, decimals);

  log.info(
    { token: tokenAddress, symbol, decimals, minThreshold: env.MIN_USDT },
    "loaded token config"
  );
  log.info({ oft: oftAddress, dstEid, beneficiary }, "loaded forwarder config");

  let stopping = false;
  let inFlight = false;

  const stop = (signal: string) => {
    if (stopping) return;
    stopping = true;
    log.info({ signal }, "stopping");
  };
  process.on("SIGINT", () => stop("SIGINT"));
  process.on("SIGTERM", () => stop("SIGTERM"));

  const tick = async () => {
    if (stopping || inFlight) return;
    inFlight = true;
    try {
      const tokenBalance = (await publicClient.readContract({
        address: tokenAddress,
        abi: erc20Abi,
        functionName: "balanceOf",
        args: [env.FORWARDER_CONTRACT_ADDRESS],
      })) as bigint;

      if (tokenBalance <= minUsdtThreshold) {
        return;
      }

      const signerEthBalance = await publicClient.getBalance({ address: account.address });
      if (signerEthBalance === 0n) {
        log.warn("skip: signer has 0 ETH for fees");
        return;
      }

      const sendParam = [
        dstEid,
        beneficiary,
        tokenBalance,
        tokenBalance,
        "0x",
        "0x",
        "0x",
      ] as const;

      const [fee, feesPerGas] = await Promise.all([
        publicClient.readContract({
          address: oftAddress as `0x${string}`,
          abi: oftAbi,
          functionName: "quoteSend",
          args: [sendParam, false],
        }),
        publicClient.estimateFeesPerGas(),
      ]);

      const feeNative = (fee as readonly [bigint, bigint])[0];
      if (feeNative === 0n) {
        log.warn("skip: quoteSend returned 0 nativeFee");
        return;
      }

      const gas = await publicClient.estimateContractGas({
        address: env.FORWARDER_CONTRACT_ADDRESS,
        abi: forwarderAbi,
        functionName: "forward",
        args: [tokenBalance],
        account: account.address,
        value: feeNative,
      });

      const maxFeePerGas = feesPerGas.maxFeePerGas ?? feesPerGas.gasPrice;
      if (!maxFeePerGas) {
        log.warn("skip: unable to estimate gas price");
        return;
      }

      const gasCostUpperBound = gas * maxFeePerGas;
      if (signerEthBalance <= gasCostUpperBound) {
        log.warn(
          {
            signerEthBalance: signerEthBalance.toString(),
            gasCostUpperBound: gasCostUpperBound.toString(),
          },
          "skip: signer ETH too low for gas"
        );
        return;
      }

      const valueWanted = signerEthBalance / 2n;
      const valueMax = signerEthBalance - gasCostUpperBound;
      const value = valueWanted < valueMax ? valueWanted : valueMax;

      if (value < feeNative) {
        log.warn(
          {
            valueWanted: valueWanted.toString(),
            feeNative: feeNative.toString(),
            signerEthBalance: signerEthBalance.toString(),
          },
          "skip: half ETH < required fee"
        );
        return;
      }

      log.info(
        {
          amount: formatUnits(tokenBalance, decimals),
          symbol,
          valueEth: formatUnits(value, 18),
          tokenBalance: tokenBalance.toString(),
          msgValueWei: value.toString(),
        },
        "forwarding"
      );

      const hash = await walletClient.writeContract({
        address: env.FORWARDER_CONTRACT_ADDRESS,
        abi: forwarderAbi,
        functionName: "forward",
        args: [tokenBalance],
        value,
      });

      log.info({ hash }, "tx sent");
      const receipt = await publicClient.waitForTransactionReceipt({ hash });
      log.info(
        { hash, status: receipt.status, blockNumber: receipt.blockNumber?.toString() },
        "tx confirmed"
      );
    } catch (error) {
      log.error({ err: error }, "tick error");
    } finally {
      inFlight = false;
    }
  };

  for (;;) {
    if (stopping) break;
    await tick();
    await new Promise((resolve) => setTimeout(resolve, env.POLL_INTERVAL_MS));
  }
}

main().catch((error) => {
  log.fatal({ err: error }, "fatal");
  process.exitCode = 1;
});
