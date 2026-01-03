import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { log } from "../lib/logger.js";
import {
  createPublicClient,
  createWalletClient,
  encodeFunctionData,
  http,
  keccak256,
  parseAbiItem,
  parseEventLogs,
  parseUnits,
  toHex,
  type Address,
  type Chain,
  type Hex,
} from "viem";
import { mnemonicToAccount, privateKeyToAccount } from "viem/accounts";
import { untronControllerAbi, untronV3Abi } from "@untron/v3-contracts";

type DeployJsonHub = {
  contracts: {
    UntronV3: Address;
    StatefulTronTxReader?: Address;
    CCTPV2Bridger?: Address;
    USDT0Bridger?: Address;
    USDC?: Address;
    USDT0?: Address;
    USDT?: Address;
  };
};

type DeployJsonController = {
  contracts: {
    UntronController: Address;
    USDT: Address;
  };
};

const DEFAULT_HUB_RPC = "http://127.0.0.1:8545";
const DEFAULT_CONTROLLER_RPC = "http://127.0.0.1:8546";

const DEFAULT_ANVIL_MNEMONIC = "test test test test test test test test test test test junk";

const mockErc20Abi = [
  {
    type: "function",
    name: "mint",
    stateMutability: "nonpayable",
    inputs: [
      { name: "to", type: "address" },
      { name: "amount", type: "uint256" },
    ],
    outputs: [],
  },
  {
    type: "function",
    name: "approve",
    stateMutability: "nonpayable",
    inputs: [
      { name: "spender", type: "address" },
      { name: "amount", type: "uint256" },
    ],
    outputs: [{ name: "", type: "bool" }],
  },
] as const;

type Args = {
  hubRpc: string;
  controllerRpc: string;
  outDir: string;
  hubJson: string;
  controllerJson: string;
  receiverSalts: number;
  deposits: number;
  controllerPulls: number;
  relayBatchSize: number;
  fills: number;
  mnemonic?: string;
  privateKey?: Hex;
  privateKeyExplicit: boolean;
  pollMs: number;
  forceAutomine: boolean;
};

function repoRoot(): string {
  // `pnpm --filter @untron/research exec ...` runs with cwd at `apps/research`, so use a repo-root detector.
  let cur = path.resolve(process.cwd());
  for (let i = 0; i < 6; i++) {
    if (fs.existsSync(path.join(cur, "packages/contracts"))) return cur;
    const parent = path.dirname(cur);
    if (parent === cur) break;
    cur = parent;
  }
  throw new Error("Could not locate repo root (missing packages/contracts)");
}

function parseArgs(argv: string[]): Args {
  const get = (name: string): string | undefined => {
    const i = argv.indexOf(name);
    if (i === -1) return undefined;
    return argv[i + 1];
  };

  const outDir = get("--out-dir") ?? path.resolve(repoRoot(), "apps/research/out/mock-both-sides");
  const hubRpc = get("--hub") ?? process.env.HUB_RPC_URLS?.split(/[,\s]+/)[0] ?? DEFAULT_HUB_RPC;
  const controllerRpc =
    get("--controller") ??
    process.env.CONTROLLER_RPC_URLS?.split(/[,\s]+/)[0] ??
    DEFAULT_CONTROLLER_RPC;

  const hubJson = get("--hub-json") ?? path.join(outDir, "hub.json");
  const controllerJson = get("--controller-json") ?? path.join(outDir, "tron.json");

  const receiverSalts = Number(get("--receiver-salts") ?? "100");
  const deposits = Number(get("--deposits") ?? "200");
  const controllerPulls = Number(get("--controller-pulls") ?? "20");
  const relayBatchSize = Number(get("--relay-batch-size") ?? "50");
  const fills = Number(get("--fills") ?? "10");
  const pollMs = Number(get("--poll-ms") ?? "50");
  const forceAutomine = !argv.includes("--no-force-automine");

  if (!Number.isInteger(receiverSalts) || receiverSalts < 1) {
    throw new Error("Invalid --receiver-salts");
  }
  if (!Number.isInteger(deposits) || deposits < 0) throw new Error("Invalid --deposits");
  if (!Number.isInteger(controllerPulls) || controllerPulls < 0) {
    throw new Error("Invalid --controller-pulls");
  }
  if (!Number.isInteger(relayBatchSize) || relayBatchSize < 1) {
    throw new Error("Invalid --relay-batch-size");
  }
  if (!Number.isInteger(fills) || fills < 0) throw new Error("Invalid --fills");
  if (!Number.isFinite(pollMs) || pollMs < 1) throw new Error("Invalid --poll-ms");

  const mnemonic = get("--mnemonic") ?? process.env.ANVIL_MNEMONIC ?? undefined;
  const privateKeyExplicit = argv.includes("--pk");
  const privateKey = (get("--pk") ?? process.env.ANVIL_PRIVATE_KEY ?? undefined) as Hex | undefined;

  return {
    hubRpc,
    controllerRpc,
    outDir,
    hubJson,
    controllerJson,
    receiverSalts,
    deposits,
    controllerPulls,
    relayBatchSize,
    fills,
    mnemonic,
    privateKey,
    privateKeyExplicit,
    pollMs,
    forceAutomine,
  };
}

function loadJson<T>(p: string): T {
  return JSON.parse(fs.readFileSync(p, "utf8")) as T;
}

function contractsFromDeployJson<T extends { contracts: Record<string, Address> }>(
  json: T | Record<string, Address>
): Record<string, Address> {
  // Some forge scripts write a "flat" root object (e.g. { UntronV3: "0x..." }) rather than { contracts: { ... } }.
  return (
    "contracts" in json ? (json as T).contracts : (json as Record<string, Address>)
  ) as Record<string, Address>;
}

function requireAddress(contracts: Record<string, Address>, key: string): Address {
  const v = contracts[key];
  if (!v) throw new Error(`Deployment JSON missing ${key}`);
  return v;
}

function makeChain(id: number, rpcUrl: string): Chain {
  return {
    id,
    name: `anvil-${id}`,
    nativeCurrency: { name: "Ether", symbol: "ETH", decimals: 18 },
    rpcUrls: { default: { http: [rpcUrl] } },
  };
}

function deriveAccounts(args: Args) {
  if (args.privateKey) {
    const deployer = privateKeyToAccount(args.privateKey);
    return {
      owner: deployer,
      realtor: deployer,
      lessee: deployer,
      lp: deployer,
      relayer: deployer,
    };
  }

  const mnemonic = args.mnemonic ?? DEFAULT_ANVIL_MNEMONIC;
  return {
    owner: mnemonicToAccount(mnemonic, { accountIndex: 0 }),
    realtor: mnemonicToAccount(mnemonic, { accountIndex: 1 }),
    lessee: mnemonicToAccount(mnemonic, { accountIndex: 2 }),
    lp: mnemonicToAccount(mnemonic, { accountIndex: 3 }),
    relayer: mnemonicToAccount(mnemonic, { accountIndex: 4 }),
  };
}

function evmToTronAddress(a: Address): Hex {
  return `0x41${a.slice(2)}` as Hex;
}

function receiverSalt(i: number): Hex {
  return keccak256(toHex(`receiver-salt:${i}`));
}

async function ensureAnvilFunded(
  client: ReturnType<typeof createPublicClient>,
  address: Address,
  label: string
) {
  const bal = await client.getBalance({ address });
  if (bal > 0n) return;

  // 10_000 ETH
  const tenK = "0x21e19e0c9bab2400000";
  try {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    await (client as any).request({ method: "anvil_setBalance", params: [address, tenK] });
  } catch (e) {
    throw new Error(
      `${label} account ${address} has 0 ETH and the RPC does not support anvil_setBalance; ` +
        `use a prefunded anvil key (e.g. pass --pk for account #0) or start anvil with the mnemonic matching your keys. ` +
        `Underlying error: ${e instanceof Error ? e.message : String(e)}`
    );
  }

  const bal2 = await client.getBalance({ address });
  if (bal2 === 0n) {
    throw new Error(
      `${label} account ${address} still has 0 ETH after anvil_setBalance; check your RPC is anvil.`
    );
  }
}

async function setAnvilAutomine(client: ReturnType<typeof createPublicClient>, enabled: boolean) {
  try {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    await (client as any).request({ method: "anvil_setAutomine", params: [enabled] });
  } catch {
    // not anvil or method unavailable; ignore
  }
}

async function main() {
  const args = parseArgs(process.argv.slice(2));

  if (!fs.existsSync(args.hubJson)) throw new Error(`Missing hub deploy json: ${args.hubJson}`);
  if (!fs.existsSync(args.controllerJson)) {
    throw new Error(`Missing controller deploy json: ${args.controllerJson}`);
  }

  const hub = loadJson<DeployJsonHub>(args.hubJson);
  const controller = loadJson<DeployJsonController>(args.controllerJson);

  const hubContracts = contractsFromDeployJson(hub as unknown as any);
  const controllerContracts = contractsFromDeployJson(controller as unknown as any);

  const hubUntron = requireAddress(hubContracts, "UntronV3");
  const hubReaderAddr = requireAddress(hubContracts, "StatefulTronTxReader");
  const hubUsdcAddr = requireAddress(hubContracts, "USDC");
  const hubCctpAddr = requireAddress(hubContracts, "CCTPV2Bridger");
  const hubUsdt0BridgerAddr = requireAddress(hubContracts, "USDT0Bridger");
  const hubUsdt =
    hubContracts.USDT ?? hubContracts.USDT0 ?? (undefined as unknown as Address | undefined);
  if (!hubUsdt) throw new Error("hub.json missing USDT or USDT0");

  const controllerAddr = requireAddress(controllerContracts, "UntronController");
  const controllerUsdt = requireAddress(controllerContracts, "USDT");

  const hubChainId = await createPublicClient({ transport: http(args.hubRpc) }).getChainId();
  const controllerChainId = await createPublicClient({
    transport: http(args.controllerRpc),
  }).getChainId();

  const hubChain = makeChain(hubChainId, args.hubRpc);
  const controllerChain = makeChain(controllerChainId, args.controllerRpc);

  const hubPublic = createPublicClient({ transport: http(args.hubRpc), chain: hubChain });
  const controllerPublic = createPublicClient({
    transport: http(args.controllerRpc),
    chain: controllerChain,
  });

  let accts = deriveAccounts(args);
  // If ANVIL_PRIVATE_KEY is set in the environment but doesn't correspond to a prefunded anvil account,
  // fall back to the default mnemonic to avoid confusing "out of gas: allowance 0" errors from estimateGas.
  if (args.privateKey && !args.privateKeyExplicit) {
    const [hubBal, controllerBal] = await Promise.all([
      hubPublic.getBalance({ address: accts.owner.address }),
      controllerPublic.getBalance({ address: accts.owner.address }),
    ]);
    if (hubBal === 0n && controllerBal === 0n) {
      log.warn(
        `ANVIL_PRIVATE_KEY account ${accts.owner.address} has 0 ETH on both RPCs; falling back to mnemonic-derived anvil accounts.`
      );
      accts = deriveAccounts({ ...args, privateKey: undefined });
    }
  }

  log.info("Using role accounts:", {
    owner: accts.owner.address,
    realtor: accts.realtor.address,
    lessee: accts.lessee.address,
    lp: accts.lp.address,
    relayer: accts.relayer.address,
  });

  // Make this script resilient even if the user passed an arbitrary private key: on anvil we can top up balances.
  await Promise.all([
    ensureAnvilFunded(hubPublic, accts.owner.address, "hub.owner"),
    ensureAnvilFunded(hubPublic, accts.realtor.address, "hub.realtor"),
    ensureAnvilFunded(hubPublic, accts.lessee.address, "hub.lessee"),
    ensureAnvilFunded(hubPublic, accts.lp.address, "hub.lp"),
    ensureAnvilFunded(hubPublic, accts.relayer.address, "hub.relayer"),
    ensureAnvilFunded(controllerPublic, accts.owner.address, "controller.owner"),
    ensureAnvilFunded(controllerPublic, accts.relayer.address, "controller.relayer"),
  ]);

  if (args.forceAutomine) {
    // If the user started anvil with `--no-mining`, viem's `waitForTransactionReceipt` will poll forever.
    // Force automine back on for performance and predictability.
    await Promise.all([
      setAnvilAutomine(hubPublic, true),
      setAnvilAutomine(controllerPublic, true),
    ]);
  }

  const hubOwner = createWalletClient({
    transport: http(args.hubRpc),
    chain: hubChain,
    account: accts.owner,
  });
  const hubRealtor = createWalletClient({
    transport: http(args.hubRpc),
    chain: hubChain,
    account: accts.realtor,
  });
  const hubLessee = createWalletClient({
    transport: http(args.hubRpc),
    chain: hubChain,
    account: accts.lessee,
  });
  const hubLp = createWalletClient({
    transport: http(args.hubRpc),
    chain: hubChain,
    account: accts.lp,
  });

  const controllerOwner = createWalletClient({
    transport: http(args.controllerRpc),
    chain: controllerChain,
    account: accts.owner,
  });
  const controllerRelayer = createWalletClient({
    transport: http(args.controllerRpc),
    chain: controllerChain,
    account: accts.relayer,
  });

  async function waitTx(chain: "hub" | "controller", hash: Hex) {
    const client = chain === "hub" ? hubPublic : controllerPublic;
    return await client.waitForTransactionReceipt({
      hash,
      pollingInterval: args.pollMs,
      timeout: 120_000,
    });
  }

  const emptyBlocks = [
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
    "0x",
  ] as const;

  const mockReaderAbi = [
    {
      type: "function",
      name: "setNext",
      stateMutability: "nonpayable",
      inputs: [
        {
          name: "next_",
          type: "tuple",
          components: [
            { name: "txId", type: "bytes32" },
            { name: "tronBlockNumber", type: "uint256" },
            { name: "tronBlockTimestamp", type: "uint32" },
            { name: "senderTron", type: "bytes21" },
            { name: "toTron", type: "bytes21" },
            { name: "data", type: "bytes" },
          ],
        },
      ],
      outputs: [],
    },
    { type: "function", name: "clearNext", stateMutability: "nonpayable", inputs: [], outputs: [] },
  ] as const;

  const trc20TransferAbi = [
    {
      type: "function",
      name: "transfer",
      stateMutability: "nonpayable",
      inputs: [
        { name: "to", type: "address" },
        { name: "amount", type: "uint256" },
      ],
      outputs: [{ name: "", type: "bool" }],
    },
  ] as const;

  const isEventChainTipAbi = [
    {
      type: "function",
      name: "isEventChainTip",
      stateMutability: "nonpayable",
      inputs: [{ name: "eventChainTip_", type: "bytes32" }],
      outputs: [{ name: "", type: "bool" }],
    },
  ] as const;

  const eventAppended = parseAbiItem(
    "event EventAppended(uint256 indexed eventSeq, bytes32 indexed prevTip, bytes32 indexed newTip, bytes32 eventSignature, bytes abiEncodedEventData)"
  );

  const blockTimestampCache = new Map<string, bigint>();
  async function blockTimestamp(chain: "hub" | "controller", blockNumber: bigint): Promise<bigint> {
    const key = `${chain}:${blockNumber}`;
    const cached = blockTimestampCache.get(key);
    if (cached !== undefined) return cached;
    const client = chain === "hub" ? hubPublic : controllerPublic;
    const b = await client.getBlock({ blockNumber });
    blockTimestampCache.set(key, b.timestamp);
    return b.timestamp;
  }

  async function relayControllerLogsToHub(fromBlock: bigint, toBlock: bigint) {
    const logs = await controllerPublic.getLogs({
      address: controllerAddr,
      event: eventAppended,
      fromBlock,
      toBlock,
    });
    if (logs.length === 0) return;

    const parsed = parseEventLogs({
      abi: untronControllerAbi,
      logs,
      eventName: "EventAppended",
    }).sort((a, b) => Number(a.args.eventSeq - b.args.eventSeq));

    // Build a contiguous sequence starting from the hub's current tip/seq, to avoid EventTipMismatch if we
    // accidentally include already-relayed logs in the requested range.
    let tip = (await hubPublic.readContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "lastControllerEventTip",
      args: [],
    })) as Hex;
    let seq = (await hubPublic.readContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "lastControllerEventSeq",
      args: [],
    })) as bigint;

    const bySeq = new Map<bigint, (typeof parsed)[number]>();
    for (const ev of parsed) bySeq.set(ev.args.eventSeq as bigint, ev);

    // NOTE: Our controller's `isEventChainTip(tip)` only succeeds for the *current* tip.
    // That means we must relay the entire contiguous segment in a single hub transaction; we cannot split
    // it into multiple batches (intermediate tips would fail the tip-check).
    const segment: Array<{
      sig: Hex;
      data: Hex;
      blockNumber: bigint;
      blockTimestamp: bigint;
    }> = [];

    while (true) {
      const nextSeq = seq + 1n;
      const ev = bySeq.get(nextSeq);
      if (!ev) break;

      const prevTip = ev.args.prevTip as Hex;
      if (prevTip !== tip) break;

      const bn = ev.blockNumber!;
      const ts = await blockTimestamp("controller", bn);
      segment.push({
        sig: ev.args.eventSignature as Hex,
        data: ev.args.abiEncodedEventData as Hex,
        blockNumber: bn,
        blockTimestamp: ts,
      });

      tip = ev.args.newTip as Hex;
      seq = nextSeq;
    }

    if (segment.length === 0) return;
    const tipNew = tip;

    const controllerTipNow = (await controllerPublic.readContract({
      address: controllerAddr,
      abi: untronControllerAbi,
      functionName: "eventChainTip",
      args: [],
    })) as Hex;
    if (controllerTipNow !== tipNew) {
      throw new Error(
        `Controller tip advanced beyond relayed segment (likely missed logs): computed=${tipNew}, controller=${controllerTipNow}.`
      );
    }

    // Real controller-side tip-check tx (indexer listens for this too).
    await waitTx(
      "controller",
      await controllerRelayer.writeContract({
        address: controllerAddr,
        abi: untronControllerAbi,
        functionName: "isEventChainTip",
        args: [controllerTipNow],
      })
    );

    // Hub mock reader pretends to have proven an isEventChainTip(tipNew) tx into the controller.
    const tipCallData = encodeFunctionData({
      abi: isEventChainTipAbi,
      functionName: "isEventChainTip",
      args: [controllerTipNow],
    });

    const next = {
      txId: controllerTipNow,
      tronBlockNumber: 0n,
      tronBlockTimestamp: Number(await blockTimestamp("controller", segment[0]!.blockNumber)),
      senderTron: evmToTronAddress(accts.relayer.address),
      toTron: evmToTronAddress(controllerAddr),
      data: tipCallData,
    };

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubReaderAddr,
        abi: mockReaderAbi,
        functionName: "setNext",
        args: [next],
      })
    );

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubUntron,
        abi: untronV3Abi,
        functionName: "relayControllerEventChain",
        args: [emptyBlocks, "0x", [], 0n, segment],
      })
    );

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubReaderAddr,
        abi: mockReaderAbi,
        functionName: "clearNext",
        args: [],
      })
    );

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubUntron,
        abi: untronV3Abi,
        functionName: "processControllerEvents",
        args: [BigInt(segment.length)],
      })
    );
  }

  const remoteChainId = BigInt(hubChainId + 1000);

  log.info("Configuring hub routes/allowlists...");
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "setRealtor",
      args: [accts.realtor.address, true],
    })
  );
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "setLp",
      args: [accts.lp.address, true],
    })
  );
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "setSwapRate",
      args: [hubUsdcAddr, 1_000_000n],
    })
  );
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "setBridger",
      args: [hubUsdcAddr, remoteChainId, hubCctpAddr],
    })
  );
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "setBridger",
      args: [hubUsdt, remoteChainId, hubUsdt0BridgerAddr],
    })
  );

  log.info("Seeding hub LP liquidity...");
  await waitTx(
    "hub",
    // Minting is restricted on our MockERC20; mint from the deployer/owner, then LP approves & deposits.
    await hubOwner.writeContract({
      address: hubUsdt,
      abi: mockErc20Abi,
      functionName: "mint",
      args: [accts.lp.address, parseUnits("2000000", 6)],
    })
  );
  await waitTx(
    "hub",
    await hubLp.writeContract({
      address: hubUsdt,
      abi: mockErc20Abi,
      functionName: "approve",
      args: [hubUntron, parseUnits("2000000", 6)],
    })
  );
  await waitTx(
    "hub",
    await hubLp.writeContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "deposit",
      args: [parseUnits("1500000", 6)],
    })
  );

  log.info("Syncing controller event chain into hub (relay + process)...");
  let controllerLastRelayedBlock = 0n;
  {
    const controllerLatest = await controllerPublic.getBlockNumber();
    await relayControllerLogsToHub(0n, controllerLatest);
    controllerLastRelayedBlock = controllerLatest;
  }

  log.info("Creating leases (mixed payout targets)...");
  const salts = Array.from({ length: args.receiverSalts }, (_, i) => receiverSalt(i));
  for (let i = 0; i < salts.length; i++) {
    const salt = salts[i]!;
    const now = BigInt(Math.floor(Date.now() / 1000));
    const nukeableAfter = now + 30n * 24n * 60n * 60n;

    // 50% local USDT, 25% local USDC, 25% remote USDC
    const mode = i % 4;
    const targetToken = mode === 0 || mode === 1 ? hubUsdt : hubUsdcAddr;
    const targetChain = mode === 3 ? remoteChainId : BigInt(hubChainId);
    const leaseFeePpm = mode === 0 ? 10_000 : 0;

    const { request } = await hubPublic.simulateContract({
      account: accts.realtor,
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "createLease",
      args: [
        salt,
        accts.lessee.address,
        nukeableAfter,
        leaseFeePpm,
        0n,
        targetChain,
        targetToken,
        accts.lessee.address,
      ],
    });
    await waitTx("hub", await hubRealtor.writeContract(request));
  }

  const tronUsdt = (await hubPublic.readContract({
    address: hubUntron,
    abi: untronV3Abi,
    functionName: "tronUsdt",
    args: [],
  })) as Address;
  if (tronUsdt === ("0x0000000000000000000000000000000000000000" as Address)) {
    throw new Error(
      "hub.tronUsdt is zero; controller UsdtSet likely was not relayed. Re-run mockBothSides + generateBothSidesActivity."
    );
  }

  log.info("Simulating deposits (controller receiver balances + hub preEntitle proofs)...");
  const depositCount = Math.min(args.deposits, salts.length * 4);
  for (let i = 0; i < depositCount; i++) {
    const salt = salts[i % salts.length]!;
    const receiver = (await controllerPublic.readContract({
      address: controllerAddr,
      abi: untronControllerAbi,
      functionName: "predictReceiverAddress",
      args: [salt],
    })) as Address;

    const amount = 10_000n + BigInt(i % 10_000);

    // Controller-side "user deposit": mint to receiver address to create balance for later pull.
    await waitTx(
      "controller",
      await controllerOwner.writeContract({
        address: controllerUsdt,
        abi: mockErc20Abi,
        functionName: "mint",
        args: [receiver, amount],
      })
    );

    // Hub-side proof via mock reader: TRC-20 transfer(receiver, amount) on tronUsdt.
    const trc20Data = encodeFunctionData({
      abi: trc20TransferAbi,
      functionName: "transfer",
      args: [receiver, amount],
    });

    const txId = toHex(BigInt(i + 1), { size: 32 });
    const next = {
      txId,
      tronBlockNumber: 0n,
      tronBlockTimestamp: Number(BigInt(Math.floor(Date.now() / 1000))),
      senderTron: evmToTronAddress(accts.lessee.address),
      toTron: evmToTronAddress(tronUsdt),
      data: trc20Data,
    };

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubReaderAddr,
        abi: mockReaderAbi,
        functionName: "setNext",
        args: [next],
      })
    );

    await waitTx(
      "hub",
      await hubLessee.writeContract({
        address: hubUntron,
        abi: untronV3Abi,
        functionName: "preEntitle",
        args: [salt, emptyBlocks, "0x", [], 0n],
      })
    );
  }

  // Clearing is optional (setNext overwrites), but keep it tidy for subsequent stages.
  await waitTx(
    "hub",
    await hubOwner.writeContract({
      address: hubReaderAddr,
      abi: mockReaderAbi,
      functionName: "clearNext",
      args: [],
    })
  );

  log.info("Controller pulls -> relay+process -> backing/unbacking updates...");
  const pullChunkSize = Math.max(1, Math.floor(salts.length / Math.max(1, args.controllerPulls)));
  for (let i = 0; i < salts.length; i += pullChunkSize) {
    const chunk = salts.slice(i, i + pullChunkSize);
    const fromBlock = controllerLastRelayedBlock;

    await waitTx(
      "controller",
      await controllerRelayer.writeContract({
        address: controllerAddr,
        abi: untronControllerAbi,
        functionName: "pullFromReceivers",
        args: [controllerUsdt, chunk],
      })
    );

    const toBlock = await controllerPublic.getBlockNumber();
    // Include `fromBlock` again to avoid missing logs if multiple txs land in the same block number.
    // The relay logic is seq/tip-aware and will ignore already-processed events.
    await relayControllerLogsToHub(fromBlock, toBlock);
    controllerLastRelayedBlock = toBlock;
  }

  log.info("Filling hub claim queues (local + swap + bridge)...");
  const swapExecutor = (await hubPublic.readContract({
    address: hubUntron,
    abi: untronV3Abi,
    functionName: "SWAP_EXECUTOR",
    args: [],
  })) as Address;

  async function sumClaimsUsdt(targetToken: Address, maxClaims: bigint): Promise<bigint> {
    const head = (await hubPublic.readContract({
      address: hubUntron,
      abi: untronV3Abi,
      functionName: "nextIndexByTargetToken",
      args: [targetToken],
    })) as bigint;

    let total = 0n;
    for (let i = 0n; i < maxClaims; i++) {
      const idx = head + i;
      let c: readonly [bigint, bigint, bigint, bigint, Address];
      try {
        c = (await hubPublic.readContract({
          address: hubUntron,
          abi: untronV3Abi,
          functionName: "claimsByTargetToken",
          args: [targetToken, idx],
        })) as readonly [bigint, bigint, bigint, bigint, Address];
      } catch {
        // Out-of-bounds index on the underlying array -> stop scanning.
        break;
      }
      const amountUsdt = c[1];
      if (amountUsdt === 0n) break;
      total += amountUsdt;
    }
    return total;
  }

  for (let i = 0; i < args.fills; i++) {
    const target: Address = i % 2 === 0 ? (hubUsdt as Address) : hubUsdcAddr;
    const maxClaims = 25n;

    if (target === hubUsdt) {
      await waitTx(
        "hub",
        await hubLp.writeContract({
          address: hubUntron,
          abi: untronV3Abi,
          functionName: "fill",
          args: [target, maxClaims, []],
        })
      );
      continue;
    }

    const totalUsdt = await sumClaimsUsdt(hubUsdcAddr, maxClaims);
    if (totalUsdt === 0n) continue;

    const mintCalldata = encodeFunctionData({
      abi: mockErc20Abi,
      functionName: "mint",
      args: [swapExecutor, totalUsdt + 1n],
    });
    const calls = [{ to: hubUsdcAddr, value: 0n, data: mintCalldata }];

    await waitTx(
      "hub",
      await hubLp.writeContract({
        address: hubUntron,
        abi: untronV3Abi,
        functionName: "fill",
        args: [hubUsdcAddr, maxClaims, calls],
      })
    );
  }

  log.info("Done. This run produced coherent two-chain activity:");
  log.info(
    "- Controller: ReceiverDeployed + PulledFromReceiver + IsEventChainTipCalled + EventAppended"
  );
  log.info(
    "- Hub: LeaseCreated + PreEntitle + ControllerEventChainTipUpdated/Processed + ClaimCreated/Filled + swaps/bridges"
  );
}

main().catch((e) => {
  log.error("generateBothSidesActivity failed:", e);
  process.exit(1);
});
