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
  for (let i = 0; i < 100; i++) {
    try {
      const got = await client.getChainId();
      if (got !== chainId)
        throw new Error(`RPC ${url} chainId mismatch: expected ${chainId}, got ${got}`);
      return;
    } catch {
      await sleep(100);
    }
  }
  throw new Error(`Timed out waiting for RPC: ${url}`);
}

function spawnAnvil(opts: { port: number; chainId: number; mnemonic?: string }) {
  const args = [
    "--host",
    "127.0.0.1",
    "--port",
    String(opts.port),
    "--chain-id",
    String(opts.chainId),
    "--silent",
  ];
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

async function main() {
  const args = parseArgs(process.argv.slice(2));

  const hubUrl = `http://127.0.0.1:${args.hubPort}`;
  const tronUrl = `http://127.0.0.1:${args.tronPort}`;

  fs.mkdirSync(args.outDir, { recursive: true });
  const tronOut = path.join(args.outDir, "tron.json");
  const hubOut = path.join(args.outDir, "hub.json");
  const indexerEnvOut = path.join(args.outDir, "indexer.env");

  let killAll = () => {};

  if (args.spawnAnvil) {
    log.info("Starting anvils...");
    const hubAnvil = spawnAnvil({
      port: args.hubPort,
      chainId: args.hubChainId,
      mnemonic: args.mnemonic,
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
      OUTPUT_PATH: tronOut,
      TRON_CREATE2_PREFIX: args.controllerCreate2Prefix,
    },
  });

  const tronJson = readJson<{
    contracts: { UntronController: Address; TRON_RECEIVER_IMPL: Address; USDT: Address };
  }>(tronOut);

  log.info("Deploying Hub-side contracts...");
  await runForgeScript({
    rpcUrl: hubUrl,
    scriptTarget: "script/DeployMockAnvilHubSide.s.sol:DeployMockAnvilHubSideScript",
    env: {
      PRIVATE_KEY: DEFAULT_ANVIL_PK,
      OUTPUT_PATH: hubOut,
      CONTROLLER_ADDRESS: tronJson.contracts.UntronController,
      TRON_RECEIVER_IMPL: tronJson.contracts.TRON_RECEIVER_IMPL,
      UNTRON_CREATE2_PREFIX: args.hubCreate2Prefix,
    },
  });

  const hubJson = readJson<{ contracts: { UntronV3: Address } }>(hubOut);

  const indexerEnv = [
    `HUB_RPC_URLS=${hubUrl}`,
    `HUB_CHAIN_ID=${args.hubChainId}`,
    `HUB_CONTRACT_ADDRESS=${hubJson.contracts.UntronV3}`,
    `HUB_DEPLOYMENT_BLOCK=0`,
    `CONTROLLER_RPC_URLS=${tronUrl}`,
    `CONTROLLER_CHAIN_ID=${args.tronChainId}`,
    `CONTROLLER_CONTRACT_ADDRESS=${tronJson.contracts.UntronController}`,
    `CONTROLLER_DEPLOYMENT_BLOCK=0`,
  ].join("\n");
  fs.writeFileSync(indexerEnvOut, `${indexerEnv}\n`, "utf8");

  log.info("Wrote:", path.resolve(indexerEnvOut));
  log.info("Hub UntronV3:", hubJson.contracts.UntronV3);
  log.info("Tron UntronController:", tronJson.contracts.UntronController);
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
