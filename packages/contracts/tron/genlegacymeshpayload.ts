import * as fs from "node:fs/promises";
import * as path from "node:path";
import * as readline from "node:readline";
import { TronWeb } from "tronweb";

function usageAndExit(message?: string): never {
  if (message) console.error(`Error: ${message}\n`);
  console.error(
    [
      "Generate the UntronController payload for LegacyMeshRebalancer.",
      "",
      "Payload ABI:",
      "  abi.encode(address oft, uint32 dstEid, bytes32 to)",
      "",
      "Usage:",
      "  npx tsx genlegacymeshpayload.ts [--oft <T...|41...|0x...>] [--dst-eid <uint32>] [--to <0xbytes32|0xaddress|T...|41...>] [--out payload.json]",
      "",
      "Examples:",
      "  npx tsx genlegacymeshpayload.ts --oft T... --dst-eid 110 --to 0xB0B... --out legacy-mesh.json",
      "  npx tsx genlegacymeshpayload.ts   # interactive prompts",
    ].join("\n")
  );
  process.exit(1);
}

function getFlag(argv: string[], name: string): string | undefined {
  const i = argv.indexOf(name);
  if (i === -1) return undefined;
  return argv[i + 1];
}

function strip0x(x: string): string {
  return x.startsWith("0x") || x.startsWith("0X") ? x.slice(2) : x;
}

function normalizeAbiAddress20(input: string): string {
  const a = input.trim();
  if (a.startsWith("T")) {
    const hex41 = TronWeb.address.toHex(a);
    if (!/^41[0-9a-fA-F]{40}$/.test(hex41)) throw new Error(`Invalid base58 TRON address: "${a}"`);
    return `0x${hex41.slice(2).toLowerCase()}`;
  }
  if (/^41[0-9a-fA-F]{40}$/.test(a)) return `0x${a.slice(2).toLowerCase()}`;
  if (/^0x[0-9a-fA-F]{40}$/.test(a)) return a.toLowerCase();
  if (/^[0-9a-fA-F]{40}$/.test(a)) return `0x${a.toLowerCase()}`;
  throw new Error(`Invalid address: "${input}" (expected T..., 41..., or 0x + 20 bytes)`);
}

function normalizeBytes32FromInput(input: string): {
  bytes32: string;
  inferredFrom: "bytes32" | "address";
} {
  const s = input.trim();
  if (/^0x[0-9a-fA-F]{64}$/.test(s)) return { bytes32: s.toLowerCase(), inferredFrom: "bytes32" };
  if (/^[0-9a-fA-F]{64}$/.test(s))
    return { bytes32: `0x${s.toLowerCase()}`, inferredFrom: "bytes32" };

  const addr20 = normalizeAbiAddress20(s);
  const padded = `0x${"0".repeat(24)}${strip0x(addr20)}`; // bytes32(uint256(uint160(addr)))
  return { bytes32: padded, inferredFrom: "address" };
}

function parseUint32(raw: string): number {
  const s = raw.trim().replace(/_/g, "");
  if (s.length === 0) throw new Error("Empty uint32");
  const n = BigInt(s);
  if (n < 0n || n > 0xffff_ffffn) throw new Error(`dstEid must fit uint32, got ${n.toString()}`);
  return Number(n);
}

function question(rl: readline.Interface, q: string): Promise<string> {
  return new Promise((resolve) => rl.question(q, (ans) => resolve(ans.trim())));
}

async function promptMissing(opts: {
  oft?: string;
  dstEid?: string;
  to?: string;
  out?: string;
}): Promise<Required<typeof opts>> {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: true,
  });
  try {
    const oft = opts.oft ?? (await question(rl, "OFT address (T... / 41... / 0x...): "));
    const dstEid = opts.dstEid ?? (await question(rl, "dstEid (uint32): "));
    const to =
      opts.to ??
      (await question(
        rl,
        'to (bytes32 "0x..." or address "0x..."/"T..."/"41..."; address auto-converts to bytes32): '
      ));
    const out = opts.out ?? (await question(rl, "Output file (optional, blank to skip): "));

    return { oft, dstEid, to, out };
  } finally {
    rl.close();
  }
}

async function main() {
  const argv = process.argv.slice(2);
  if (argv.includes("--help") || argv.includes("-h")) usageAndExit();

  const oft = getFlag(argv, "--oft") ?? getFlag(argv, "--oft-address");
  const dstEid = getFlag(argv, "--dst-eid") ?? getFlag(argv, "--dstEid");
  const to = getFlag(argv, "--to");
  const out = getFlag(argv, "--out");

  const needsPrompt = !oft || !dstEid || !to;
  const prompted = needsPrompt
    ? await promptMissing({ oft, dstEid, to, out })
    : { oft: oft!, dstEid: dstEid!, to: to!, out: out ?? "" };

  const oftAddr20 = normalizeAbiAddress20(prompted.oft);
  const dstEidU32 = parseUint32(prompted.dstEid);
  const toNorm = normalizeBytes32FromInput(prompted.to);

  const tronWeb = new TronWeb({ fullHost: "http://127.0.0.1" });
  const payloadAbi = {
    inputs: [
      { name: "oft", type: "address" },
      { name: "dstEid", type: "uint32" },
      { name: "to", type: "bytes32" },
    ],
  };

  const payload = (tronWeb as any).utils.abi.encodeParamsV2ByABI(payloadAbi, [
    oftAddr20,
    dstEidU32,
    toNorm.bytes32,
  ]) as string;

  const payloadBytes = Math.floor(strip0x(payload).length / 2);

  console.log("\n---");
  console.log("LegacyMeshRebalancer payload (abi.encode(address,uint32,bytes32))");
  console.log(`oft:    ${oftAddr20}`);
  console.log(`dstEid: ${dstEidU32}`);
  console.log(`to:     ${toNorm.bytes32}  (from ${toNorm.inferredFrom})`);
  console.log(`bytes:  ${payloadBytes}`);
  console.log(`payload: ${payload.toLowerCase()}`);

  const outPath = prompted.out.trim();
  if (outPath.length > 0) {
    const resolved = path.isAbsolute(outPath) ? outPath : path.join(process.cwd(), outPath);
    const json = {
      oft: oftAddr20,
      dstEid: dstEidU32,
      to: toNorm.bytes32,
      payload: payload.toLowerCase(),
    };
    await fs.writeFile(resolved, JSON.stringify(json, null, 2) + "\n", "utf8");
    console.log(`Saved: ${outPath}`);
  }
}

main().catch((err) => {
  console.error(err instanceof Error ? err.message : err);
  process.exit(1);
});
