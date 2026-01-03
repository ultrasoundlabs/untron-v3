import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { setTimeout as sleep } from "node:timers/promises";
import { log } from "../lib/logger.js";
import { createPublicClient, http, type Address } from "viem";

type Args = {
  hubPort: number;
  tronPort: number;
  hubChainId: number;
  tronChainId: number;
  mnemonic?: string;
  controllerCreate2Prefix: string;
  hubCreate2Prefix: string;
  spawnAnvil: boolean;
  keepAlive: boolean;
  outDir: string;
};

const DEFAULT_ANVIL_PK = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

class ChainIdMismatchError extends Error {
  public readonly url: string;
  public readonly expected: number;
  public readonly got: number;

  constructor(opts: { url: string; expected: number; got: number }) {
    super(`RPC ${opts.url} chainId mismatch: expected ${opts.expected}, got ${opts.got}`);
    this.url = opts.url;
    this.expected = opts.expected;
    this.got = opts.got;
  }
}

function parseArgs(argv: string[]): Args {
  const get = (name: string): string | undefined => {
    const i = argv.indexOf(name);
    if (i === -1) return undefined;
    return argv[i + 1];
  };
  const has = (name: string): boolean => argv.includes(name);

  const hubPort = Number(get("--hub-port") ?? "8545");
  const tronPort = Number(get("--tron-port") ?? "8546");
  const hubChainId = Number(get("--hub-chain-id") ?? "31337");
  const tronChainId = Number(get("--tron-chain-id") ?? "31338");

  if (!Number.isInteger(hubPort) || hubPort <= 0) throw new Error("Invalid --hub-port");
  if (!Number.isInteger(tronPort) || tronPort <= 0) throw new Error("Invalid --tron-port");
  if (!Number.isInteger(hubChainId) || hubChainId <= 0) throw new Error("Invalid --hub-chain-id");
  if (!Number.isInteger(tronChainId) || tronChainId <= 0)
    throw new Error("Invalid --tron-chain-id");

  const mnemonic = get("--mnemonic");
  const controllerCreate2Prefix = get("--controller-create2-prefix") ?? "0xff";
  const hubCreate2Prefix = get("--hub-create2-prefix") ?? "0xff";
  const spawnAnvil = has("--spawn-anvil");
  const keepAlive = !has("--no-keep");
  const outDir = get("--out-dir") ?? path.resolve(repoRoot(), "apps/research/out/mock-both-sides");

  return {
    hubPort,
    tronPort,
    hubChainId,
    tronChainId,
    mnemonic,
    controllerCreate2Prefix,
    hubCreate2Prefix,
    spawnAnvil,
    keepAlive,
    outDir,
  };
}

function repoRoot(): string {
  // `pnpm research ...` runs from the monorepo root; if not, walk upwards until we find packages/contracts.
  let cur = path.resolve(process.cwd());
  for (let i = 0; i < 6; i++) {
    if (fs.existsSync(path.join(cur, "packages/contracts"))) return cur;
    const parent = path.dirname(cur);
    if (parent === cur) break;
    cur = parent;
  }
  throw new Error("Could not locate repo root (missing packages/contracts)");
}

function contractsRoot(): string {
  return path.resolve(repoRoot(), "packages/contracts");
}

async function waitForRpc(url: string, chainId: number) {
  const client = createPublicClient({ transport: http(url) });
  let lastError: unknown;
  for (let i = 0; i < 100; i++) {
    try {
      const got = await client.getChainId();
      if (got !== chainId) throw new ChainIdMismatchError({ url, expected: chainId, got });
      return;
    } catch (err) {
      if (err instanceof ChainIdMismatchError) {
        const hint =
          `Start the node with the expected chainId (e.g. \`anvil --host 127.0.0.1 --port ${
            new URL(url).port
          } --chain-id ${chainId}\`) ` +
          `or pass the matching \`--hub-chain-id\` / \`--tron-chain-id\` to \`pnpm research mockBothSides\`.`;
        throw new Error(`${err.message}\n${hint}`);
      }
      lastError = err;
      await sleep(100);
    }
  }
  const extra = lastError instanceof Error ? ` (last error: ${lastError.message})` : "";
  throw new Error(`Timed out waiting for RPC: ${url}${extra}`);
}

function spawnAnvil(opts: {
  port: number;
  chainId: number;
  mnemonic?: string;
  disableCodeSizeLimit?: boolean;
}) {
  const args = [
    "--host",
    "127.0.0.1",
    "--port",
    String(opts.port),
    "--chain-id",
    String(opts.chainId),
    "--silent",
  ];
  if (opts.disableCodeSizeLimit) args.push("--disable-code-size-limit");
  if (opts.mnemonic) {
    args.push("--mnemonic", opts.mnemonic);
  }

  const child = spawn("anvil", args, { stdio: ["ignore", "pipe", "pipe"] });
  child.stdout.on("data", (d) => {
    const s = d.toString();
    if (s.trim().length > 0) log.info(`[anvil:${opts.port}] ${s.trimEnd()}`);
  });
  child.stderr.on("data", (d) => {
    const s = d.toString();
    if (s.trim().length > 0) log.warn(`[anvil:${opts.port}] ${s.trimEnd()}`);
  });
  return child;
}

async function runForgeScript(opts: {
  rpcUrl: string;
  scriptTarget: string;
  env: Record<string, string>;
}) {
  await new Promise<void>((resolve, reject) => {
    const child = spawn(
      "forge",
      ["script", opts.scriptTarget, "--rpc-url", opts.rpcUrl, "--broadcast", "-vvv"],
      { cwd: contractsRoot(), env: { ...process.env, ...opts.env }, stdio: "inherit" }
    );
    child.on("exit", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`forge script failed (${opts.scriptTarget}) with code ${code}`));
    });
    child.on("error", reject);
  });
}

function readJson<T>(p: string): T {
  return JSON.parse(fs.readFileSync(p, "utf8")) as T;
}

function readDeploymentAddress(json: any, key: string): Address {
  const fromNested = json?.contracts?.[key];
  const fromFlat = json?.[key];
  const v = (fromNested ?? fromFlat) as Address | undefined;
  if (!v) throw new Error(`Deployment JSON missing ${key}`);
  return v;
}

async function main() {
  const args = parseArgs(process.argv.slice(2));

  const hubUrl = `http://127.0.0.1:${args.hubPort}`;
  const tronUrl = `http://127.0.0.1:${args.tronPort}`;

  fs.mkdirSync(args.outDir, { recursive: true });
  const tronOut = path.join(args.outDir, "tron.json");
  const hubOut = path.join(args.outDir, "hub.json");
  const indexerEnvOut = path.join(args.outDir, "indexer.env");

  // Foundry's `fs_permissions` are scoped to `packages/contracts`, so forge scripts cannot write directly to
  // `apps/research/out/...`. We write JSON outputs under `packages/contracts/out/...` and then copy them out.
  const forgeOutDir = path.join(contractsRoot(), "out", "mock-both-sides");
  fs.mkdirSync(forgeOutDir, { recursive: true });
  const tronForgeOut = path.join(forgeOutDir, "tron.json");
  const hubForgeOut = path.join(forgeOutDir, "hub.json");

  let killAll = () => {};

  if (args.spawnAnvil) {
    log.info("Starting anvils...");
    const hubAnvil = spawnAnvil({
      port: args.hubPort,
      chainId: args.hubChainId,
      mnemonic: args.mnemonic,
      // UntronV3 is above the EIP-170 24KB code size limit; for local/mock anvils we disable the limit.
      disableCodeSizeLimit: true,
    });
    const tronAnvil = spawnAnvil({
      port: args.tronPort,
      chainId: args.tronChainId,
      mnemonic: args.mnemonic,
    });

    killAll = () => {
      for (const p of [hubAnvil, tronAnvil]) {
        try {
          p.kill("SIGKILL");
        } catch {}
      }
    };
    process.on("SIGINT", () => {
      killAll();
      process.exit(130);
    });
    process.on("SIGTERM", () => {
      killAll();
      process.exit(143);
    });
  } else {
    log.info("Using existing RPCs (not spawning anvil).");
    log.info("Hub RPC:", hubUrl);
    log.info("Controller RPC:", tronUrl);
    log.info(
      "Note: hub deployment requires an RPC with code size limit disabled (UntronV3 is > 24KB); for anvil use `--disable-code-size-limit`."
    );
  }

  try {
    await Promise.all([waitForRpc(hubUrl, args.hubChainId), waitForRpc(tronUrl, args.tronChainId)]);
  } catch (e) {
    killAll();
    throw e;
  }

  log.info("Deploying Tron-side contracts...");
  await runForgeScript({
    rpcUrl: tronUrl,
    scriptTarget: "script/DeployMockAnvilControllerSide.s.sol:DeployMockAnvilControllerSideScript",
    env: {
      PRIVATE_KEY: DEFAULT_ANVIL_PK,
      OUTPUT_PATH: tronForgeOut,
      TRON_CREATE2_PREFIX: args.controllerCreate2Prefix,
    },
  });

  fs.copyFileSync(tronForgeOut, tronOut);
  const tronJson = readJson<any>(tronOut);
  const tronController = readDeploymentAddress(tronJson, "UntronController");
  const tronReceiverImpl = readDeploymentAddress(tronJson, "TRON_RECEIVER_IMPL");

  log.info("Deploying Hub-side contracts...");
  await runForgeScript({
    rpcUrl: hubUrl,
    scriptTarget: "script/DeployMockAnvilHubSide.s.sol:DeployMockAnvilHubSideScript",
    env: {
      PRIVATE_KEY: DEFAULT_ANVIL_PK,
      OUTPUT_PATH: hubForgeOut,
      CONTROLLER_ADDRESS: tronController,
      TRON_RECEIVER_IMPL: tronReceiverImpl,
      UNTRON_CREATE2_PREFIX: args.hubCreate2Prefix,
    },
  });

  fs.copyFileSync(hubForgeOut, hubOut);
  const hubJson = readJson<any>(hubOut);
  const hubUntronV3 = readDeploymentAddress(hubJson, "UntronV3");

  const indexerEnv = [
    `HUB_RPC_URLS=${hubUrl}`,
    `HUB_CHAIN_ID=${args.hubChainId}`,
    `HUB_CONTRACT_ADDRESS=${hubUntronV3}`,
    `HUB_DEPLOYMENT_BLOCK=0`,
    `CONTROLLER_RPC_URLS=${tronUrl}`,
    `CONTROLLER_CHAIN_ID=${args.tronChainId}`,
    `CONTROLLER_CONTRACT_ADDRESS=${tronController}`,
    `CONTROLLER_DEPLOYMENT_BLOCK=0`,
  ].join("\n");
  fs.writeFileSync(indexerEnvOut, `${indexerEnv}\n`, "utf8");

  log.info("Wrote:", path.resolve(indexerEnvOut));
  log.info("Hub UntronV3:", hubUntronV3);
  log.info("Tron UntronController:", tronController);
  log.info(
    "Next: start the indexer with `source apps/research/out/mock-both-sides/indexer.env` (plus DATABASE_URL)."
  );

  if (!args.keepAlive || !args.spawnAnvil) {
    killAll();
    return;
  }

  log.info("Anvils are running. Ctrl-C to stop.");
  // eslint-disable-next-line no-constant-condition
  while (true) {
    // keep process alive without busy loop
    await sleep(60_000);
  }
}

main().catch((e) => {
  log.error("mockBothSides failed:", e);
  process.exit(1);
});
