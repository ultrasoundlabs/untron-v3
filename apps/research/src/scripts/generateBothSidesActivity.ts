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

const DEFAULT_OUT_DIR = "apps/research/out/mock-both-sides";
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
};

function parseArgs(argv: string[]): Args {
  const get = (name: string): string | undefined => {
    const i = argv.indexOf(name);
    if (i === -1) return undefined;
    return argv[i + 1];
  };

  const outDir = get("--out-dir") ?? path.resolve(process.cwd(), DEFAULT_OUT_DIR);
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

  const mnemonic = get("--mnemonic") ?? process.env.ANVIL_MNEMONIC ?? undefined;
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
  };
}

function loadJson<T>(p: string): T {
  return JSON.parse(fs.readFileSync(p, "utf8")) as T;
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

async function main() {
  const args = parseArgs(process.argv.slice(2));

  if (!fs.existsSync(args.hubJson)) throw new Error(`Missing hub deploy json: ${args.hubJson}`);
  if (!fs.existsSync(args.controllerJson)) {
    throw new Error(`Missing controller deploy json: ${args.controllerJson}`);
  }

  const hub = loadJson<DeployJsonHub>(args.hubJson);
  const controller = loadJson<DeployJsonController>(args.controllerJson);

  const hubUntron = hub.contracts.UntronV3;
  const hubReader = hub.contracts.StatefulTronTxReader;
  const hubUsdc = hub.contracts.USDC;
  const hubCctp = hub.contracts.CCTPV2Bridger;
  const hubUsdt0Bridger = hub.contracts.USDT0Bridger;
  const hubUsdt = (hub.contracts.USDT ?? hub.contracts.USDT0) as Address | undefined;
  if (!hubUsdt) throw new Error("hub.json missing contracts.USDT or contracts.USDT0");
  if (!hubReader || !hubUsdc || !hubCctp || !hubUsdt0Bridger) {
    throw new Error(
      "hub.json missing contracts.StatefulTronTxReader / USDC / CCTPV2Bridger / USDT0Bridger"
    );
  }
  const hubReaderAddr = hubReader as Address;
  const hubUsdcAddr = hubUsdc as Address;
  const hubCctpAddr = hubCctp as Address;
  const hubUsdt0BridgerAddr = hubUsdt0Bridger as Address;

  const controllerAddr = controller.contracts.UntronController;
  const controllerUsdt = controller.contracts.USDT;

  const accts = deriveAccounts(args);

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
    return await client.waitForTransactionReceipt({ hash });
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

    for (let i = 0; i < parsed.length; i += args.relayBatchSize) {
      const chunk = parsed.slice(i, i + args.relayBatchSize);
      const tipNew = chunk[chunk.length - 1]!.args.newTip as Hex;

      const events = await Promise.all(
        chunk.map(async (ev) => {
          const bn = ev.blockNumber!;
          const ts = await blockTimestamp("controller", bn);
          return {
            sig: ev.args.eventSignature as Hex,
            data: ev.args.abiEncodedEventData as Hex,
            blockNumber: bn,
            blockTimestamp: ts,
          };
        })
      );

      // Real controller-side tip-check tx (indexer listens for this too).
      await waitTx(
        "controller",
        await controllerRelayer.writeContract({
          address: controllerAddr,
          abi: untronControllerAbi,
          functionName: "isEventChainTip",
          args: [tipNew],
        })
      );

      // Hub mock reader pretends to have proven an isEventChainTip(tipNew) tx into the controller.
      const tipCallData = encodeFunctionData({
        abi: isEventChainTipAbi,
        functionName: "isEventChainTip",
        args: [tipNew],
      });

      const next = {
        txId: tipNew,
        tronBlockNumber: 0n,
        tronBlockTimestamp: Number(await blockTimestamp("controller", events[0]!.blockNumber)),
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
          args: [emptyBlocks, "0x", [], 0n, events],
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
          args: [BigInt(events.length)],
        })
      );
    }
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
    await hubLp.writeContract({
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
  const controllerLatest = await controllerPublic.getBlockNumber();
  await relayControllerLogsToHub(0n, controllerLatest);

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

    await waitTx(
      "hub",
      await hubOwner.writeContract({
        address: hubReaderAddr,
        abi: mockReaderAbi,
        functionName: "clearNext",
        args: [],
      })
    );
  }

  log.info("Controller pulls -> relay+process -> backing/unbacking updates...");
  const pullChunkSize = Math.max(1, Math.floor(salts.length / Math.max(1, args.controllerPulls)));
  for (let i = 0; i < salts.length; i += pullChunkSize) {
    const chunk = salts.slice(i, i + pullChunkSize);
    const fromBlock = await controllerPublic.getBlockNumber();

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
    await relayControllerLogsToHub(fromBlock, toBlock);
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
      const c = (await hubPublic.readContract({
        address: hubUntron,
        abi: untronV3Abi,
        functionName: "claimsByTargetToken",
        args: [targetToken, idx],
      })) as readonly [bigint, bigint, bigint, bigint, Address];
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
