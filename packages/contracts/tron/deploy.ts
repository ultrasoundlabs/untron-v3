import { Effect } from "effect";
import * as fs from "node:fs/promises";
import * as path from "node:path";
import * as readline from "node:readline";

import { TronWeb } from "tronweb";

type Artifact = {
  abi: any[]; // TRON/solc JSON ABI
  bytecode: string; // hex string, with or without 0x
};

type AbiInput = {
  name?: string;
  type: string;
  components?: readonly AbiInput[]; // tuple support
};

function usageAndExit(message?: string): never {
  if (message) console.error(`Error: ${message}\n`);
  console.error(
    [
      "Usage:",
      "  npx tsx deploy-tron.ts ContractName --private-key <64-hex-no-0x> [--full-host https://api.trongrid.io] [--api-key ...] [--fee-limit 1000000000] [--call-value 0]",
      "",
      "Examples:",
      "  npx tsx deploy-tron.ts MyContract --private-key deadbeef... --full-host https://api.trongrid.io --api-key <TRON-PRO-API-KEY>",
      "  npx tsx deploy-tron.ts MyContract --private-key deadbeef... --full-host https://api.shasta.trongrid.io",
    ].join("\n")
  );
  process.exit(1);
}

function getFlag(argv: string[], name: string): string | undefined {
  const i = argv.indexOf(name);
  if (i === -1) return undefined;
  return argv[i + 1];
}

function parseIntFlag(name: string, raw?: string): number | undefined {
  if (raw == null) return undefined;
  const s = raw.replace(/_/g, "").trim();
  if (!/^-?\d+$/.test(s)) throw new Error(`Invalid integer for ${name}: "${raw}"`);
  const n = Number(s);
  if (!Number.isSafeInteger(n)) throw new Error(`Invalid integer for ${name}: "${raw}"`);
  return n;
}

function parseCli(argv: string[]) {
  const [contractName] = argv.filter((a) => !a.startsWith("-"));
  const privateKey = getFlag(argv, "--private-key") ?? getFlag(argv, "-k");
  const fullHost =
    getFlag(argv, "--full-host") ?? getFlag(argv, "--node") ?? "https://api.trongrid.io";
  const apiKey = getFlag(argv, "--api-key");
  const feeLimit = parseIntFlag("--fee-limit", getFlag(argv, "--fee-limit")) ?? 100_000_000; // SUN
  const callValue = parseIntFlag("--call-value", getFlag(argv, "--call-value")) ?? 0; // SUN

  if (!contractName) usageAndExit("Missing ContractName positional argument.");
  if (!privateKey) usageAndExit("Missing --private-key <64-hex-no-0x> argument.");
  if (privateKey.startsWith("0x")) usageAndExit("TRON private keys must NOT be 0x-prefixed.");
  if (!/^[0-9a-fA-F]{64}$/.test(privateKey))
    usageAndExit("--private-key must be 64 hex characters.");

  if (feeLimit <= 0) usageAndExit("--fee-limit must be > 0 (SUN).");
  if (callValue < 0) usageAndExit("--call-value must be >= 0 (SUN).");

  return { contractName, privateKey, fullHost, apiKey, feeLimit, callValue };
}

function artifactPath(contractName: string) {
  return path.join(process.cwd(), "out", `${contractName}.sol`, `${contractName}.json`);
}

function safeJsonParse(text: string): unknown {
  try {
    return JSON.parse(text);
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    throw new Error(`Invalid JSON artifact: ${msg}`);
  }
}

function strip0x(x: string) {
  return x.startsWith("0x") || x.startsWith("0X") ? x.slice(2) : x;
}

function ensureHexLike(label: string, x: string) {
  const s = strip0x(x);
  if (s.length === 0 || s.length % 2 !== 0) throw new Error(`${label} must be even-length hex.`);
  if (!/^[0-9a-fA-F]+$/.test(s)) throw new Error(`${label} is not valid hex.`);
}
function ensureArtifact(x: unknown): Artifact {
  if (!x || typeof x !== "object") throw new Error("Artifact JSON is not an object.");
  const a = x as any;
  if (!Array.isArray(a.abi)) throw new Error('Artifact missing "abi" array.');
  if (typeof a.bytecode !== "object" || typeof a.bytecode.object !== "string")
    throw new Error('Artifact missing "bytecode.object" string.');
  ensureHexLike("bytecode.object", a.bytecode.object);
  return { abi: a.abi as any[], bytecode: a.bytecode.object as string };
}

function findConstructorAbi(abi: any[]): { inputs: readonly AbiInput[]; ctorAbi?: any } {
  const ctor = abi.find((x) => x?.type === "constructor");
  const inputs = Array.isArray(ctor?.inputs) ? (ctor.inputs as AbiInput[]) : [];
  return { inputs, ctorAbi: ctor };
}

function normalizeBigIntString(s: string) {
  return s.replace(/_/g, "").trim();
}

function parseBool(s: string): boolean {
  const v = s.trim().toLowerCase();
  if (v === "true" || v === "1" || v === "yes" || v === "y") return true;
  if (v === "false" || v === "0" || v === "no" || v === "n") return false;
  throw new Error(`Invalid bool: "${s}" (use true/false)`);
}

function parseProceedConfirmation(s: string): boolean {
  const v = s.trim().toLowerCase();
  if (v === "y" || v === "yes" || v === "deploy" || v === "funded") return true;
  return false;
}

function parseValueBySolType(solType: string, raw: string): any {
  const t = solType.trim();
  const s = raw.trim();

  // Arrays (best-effort): accept JSON or comma-separated.
  if (/\[[0-9]*\]$/.test(t)) {
    if (s.startsWith("[")) return JSON.parse(s);
    if (s === "") return [];
    return s.split(",").map((x) => x.trim());
  }

  // Tuples/structs: ask user for JSON array/object.
  if (t === "tuple" || t.startsWith("tuple")) {
    if (!s.startsWith("{") && !s.startsWith("[")) {
      throw new Error(`Tuple input must be JSON (starts with "{" or "[")`);
    }
    return JSON.parse(s);
  }

  // TRON address input: Base58 ("T...") or TRON hex ("41...").
  if (t === "address") {
    if (s.startsWith("T")) return s;
    if (/^41[0-9a-fA-F]{40}$/.test(s)) return s;
    throw new Error(
      `Invalid TRON address: "${raw}" (expected Base58 starting with 'T' or hex starting with '41')`
    );
  }

  if (t === "bool") return parseBool(s);
  if (t === "string") return raw; // keep original spacing

  if (t === "bytes" || /^bytes[0-9]+$/.test(t)) {
    const hex = s.startsWith("0x") || s.startsWith("0X") ? s : `0x${s}`;
    ensureHexLike("bytes", hex);
    return hex;
  }

  if (t.startsWith("uint") || t.startsWith("int")) {
    const n = normalizeBigIntString(s);
    return BigInt(n);
  }

  // Fallback: try JSON (for complex/nested cases), else return string.
  if (s.startsWith("{") || s.startsWith("[")) return JSON.parse(s);
  return s;
}

// Convert TRON address (base58 or 41-hex) into ABI 20-byte address (0x + 40 hex chars).
// TRON "41...." is 21 bytes; ABI address is 20 bytes -> strip the leading "41".
function tronToAbiAddress20(tronWeb: TronWeb, addr: string): string {
  const a = addr.trim();
  let hex41: string;
  if (a.startsWith("T")) {
    hex41 = TronWeb.address.toHex(a);
  } else if (/^41[0-9a-fA-F]{40}$/.test(a)) {
    hex41 = a;
  } else {
    throw new Error(`Invalid TRON address: "${addr}"`);
  }
  return `0x${hex41.slice(2).toLowerCase()}`;
}

function normalizeByAbiInput(tronWeb: TronWeb, input: AbiInput, value: any): any {
  const t = input.type.trim();

  // Multi-dim arrays handled by recursion
  if (/\[[0-9]*\]$/.test(t)) {
    if (!Array.isArray(value)) throw new Error(`Expected array for type ${t}`);
    const baseType = t.replace(/\[[0-9]*\]$/, "");
    const inner: AbiInput = { ...input, type: baseType };
    return value.map((v) => normalizeByAbiInput(tronWeb, inner, v));
  }

  if (t === "tuple" || t.startsWith("tuple")) {
    const comps = input.components ?? [];
    if (!Array.isArray(comps)) throw new Error(`Tuple missing components for type ${t}`);

    if (Array.isArray(value)) {
      if (value.length !== comps.length) {
        throw new Error(`Tuple arity mismatch: expected ${comps.length}, got ${value.length}`);
      }
      return comps.map((c, i) => normalizeByAbiInput(tronWeb, c, value[i]));
    }

    if (value && typeof value === "object") {
      return comps.map((c) => {
        const key = c.name ?? "";
        if (!key) throw new Error(`Tuple component missing name; provide tuple as JSON array.`);
        return normalizeByAbiInput(tronWeb, c, (value as any)[key]);
      });
    }

    throw new Error(`Expected tuple as JSON array or object for type ${t}`);
  }

  if (t === "address") return tronToAbiAddress20(tronWeb, String(value));

  if (t === "bytes" || /^bytes[0-9]+$/.test(t)) {
    const s = String(value);
    const hex = s.startsWith("0x") || s.startsWith("0X") ? s : `0x${s}`;
    ensureHexLike("bytes", hex);
    return hex.toLowerCase();
  }

  // ethers abi coder accepts bigint for ints; booleans, strings as-is
  return value;
}

const makeReadline = Effect.acquireRelease(
  Effect.sync(() =>
    readline.createInterface({
      input: process.stdin,
      output: process.stdout,
      terminal: true,
    })
  ),
  (rl) => Effect.sync(() => rl.close())
);

function question(rl: readline.Interface, q: string) {
  return Effect.async<string>((resume) => {
    rl.question(q, (answer) => resume(Effect.succeed(answer.trim())));
  });
}

const program = Effect.gen(function* () {
  const argv = process.argv.slice(2);
  const { contractName, privateKey, fullHost, apiKey, feeLimit, callValue } = parseCli(argv);

  // TronWeb v6 instantiation (named export).
  const tronWeb = new TronWeb({
    fullHost,
    headers: apiKey ? { "TRON-PRO-API-KEY": apiKey } : undefined,
    privateKey,
  });

  const ownerAddress = TronWeb.address.fromPrivateKey(privateKey); // Base58
  if (!ownerAddress)
    throw new Error("Failed to derive owner address from private key (check key format).");

  const p = artifactPath(contractName);
  const raw = yield* Effect.tryPromise({
    try: () => fs.readFile(p, "utf8"),
    catch: (e) => new Error(`Failed to read artifact at ${p}: ${String(e)}`),
  });

  const artifact = ensureArtifact(safeJsonParse(raw));
  const { inputs, ctorAbi } = findConstructorAbi(artifact.abi);

  const argsRaw = yield* Effect.gen(function* () {
    if (inputs.length === 0) return [] as any[];
    console.log(`Constructor inputs (${inputs.length}). For arrays/tuples, paste JSON.`);

    return yield* Effect.scoped(
      Effect.gen(function* () {
        const rl = yield* makeReadline;
        const out: any[] = [];

        for (let i = 0; i < inputs.length; i++) {
          const input = inputs[i];
          const label = input.name && input.name.length > 0 ? input.name : `arg${i}`;
          const prompt = `  - ${label} (${input.type}): `;
          const ans = yield* question(rl, prompt);

          const value = yield* Effect.try({
            try: () => parseValueBySolType(input.type, ans),
            catch: (e) => {
              const msg = e instanceof Error ? e.message : String(e);
              return new Error(`Failed to parse ${label} (${input.type}): ${msg}`);
            },
          });

          out.push(value);
        }
        return out;
      })
    );
  });

  const normalizedArgs = inputs.map((inp, i) => normalizeByAbiInput(tronWeb, inp, argsRaw[i]));

  // Build creation bytecode for deploy energy estimation:
  // input = 0x + bytecode + ABI-encoded-constructor-args
  let encodedCtor = "0x";
  if (ctorAbi && normalizedArgs.length > 0) {
    encodedCtor = (tronWeb as any).utils.abi.encodeParamsV2ByABI(ctorAbi, normalizedArgs) as string;
  }
  const creationInput = `0x${strip0x(artifact.bytecode)}${strip0x(encodedCtor)}`;

  // --- Call #1: Estimate deploy energy ---
  console.log("\n---");
  console.log(`Network fullHost: ${fullHost}`);
  console.log(`Deployer: ${ownerAddress}`);
  console.log("Estimating deployment energy...");

  // Prefer triggerconstantcontract for deploy estimation:
  // POST wallet/triggerconstantcontract with data=<bytecode + ctor-args> (no 0x).
  const data = strip0x(creationInput);
  const estimate = yield* Effect.tryPromise({
    try: () =>
      (tronWeb as any).fullNode.request(
        "wallet/triggerconstantcontract",
        {
          owner_address: ownerAddress, // base58 when visible=true
          data,
          call_value: callValue,
          visible: true,
        },
        "post"
      ),
    catch: (e) => new Error(`Energy estimation failed: ${String(e)}`),
  });

  const energyUsed = (estimate as any)?.energy_used;
  const energyPenalty = (estimate as any)?.energy_penalty;
  if (typeof energyUsed !== "number") {
    throw new Error(
      `Energy estimation returned unexpected payload (missing energy_used): ${JSON.stringify(
        estimate
      )}`
    );
  }

  const energyRequired =
    typeof energyPenalty === "number" ? energyUsed + energyPenalty : energyUsed;

  console.log(`Estimated energy_used: ${energyUsed}`);
  if (typeof energyPenalty === "number") console.log(`Estimated energy_penalty: ${energyPenalty}`);
  console.log(`Estimated total energy: ${energyRequired}`);
  console.log(`feeLimit (SUN): ${feeLimit}  (~${feeLimit / 1_000_000} TRX max burn)`);
  console.log(`callValue (SUN): ${callValue}  (~${callValue / 1_000_000} TRX sent to constructor)`);

  const proceed = yield* Effect.scoped(
    Effect.gen(function* () {
      const rl = yield* makeReadline;
      const ans = yield* question(
        rl,
        `\nReady to deploy from ${ownerAddress}? Ensure this wallet has enough resources, then type "yes" to continue: `
      );
      return parseProceedConfirmation(ans);
    })
  );
  if (!proceed) {
    console.log("Aborted before deployment.");
    return;
  }

  // --- Call #2: Deploy ---
  console.log("\nDeploying contract...");

  const bytecodeNo0x = strip0x(artifact.bytecode);

  // Use parametersV2 + funcABIV2 (works for tuple constructors; harmless otherwise).
  const tx = yield* Effect.tryPromise({
    try: () =>
      tronWeb.transactionBuilder.createSmartContract(
        {
          feeLimit,
          callValue,
          name: contractName,
          abi: artifact.abi,
          bytecode: bytecodeNo0x,
          ...(ctorAbi && normalizedArgs.length > 0
            ? { funcABIV2: ctorAbi, parametersV2: normalizedArgs }
            : {}),
        },
        ownerAddress
      ),
    catch: (e) => new Error(`Failed to build deploy transaction: ${String(e)}`),
  });

  const txID = (tx as any)?.txID;
  const contractHex = (tx as any)?.contract_address;

  console.log("Unsigned txID:", txID ?? "(unknown)");
  console.log("Predicted contract hex:", contractHex ?? "(unknown)");

  const signed = yield* Effect.tryPromise({
    try: () => tronWeb.trx.sign(tx),
    catch: (e) => new Error(`Failed to sign transaction: ${String(e)}`),
  });

  const broadcast = yield* Effect.tryPromise({
    try: () => tronWeb.trx.sendRawTransaction(signed),
    catch: (e) => new Error(`Broadcast failed: ${String(e)}`),
  });

  const ok = (broadcast as any)?.result === true;
  if (!ok) {
    throw new Error(`Deployment broadcast rejected: ${JSON.stringify(broadcast)}`);
  }

  let contractBase58: string | undefined;
  if (typeof contractHex === "string" && contractHex.length > 0) {
    try {
      contractBase58 = TronWeb.address.fromHex(contractHex);
    } catch {
      contractBase58 = undefined;
    }
  }

  console.log("\n---");
  console.log("Broadcast result: OK");
  console.log("Transaction:", (broadcast as any)?.txid ?? txID ?? "(unknown)");
  if (contractHex) console.log("Contract hex:", contractHex);
  if (contractBase58) console.log("Contract base58:", contractBase58);
});

Effect.runPromise(program).catch((e) => {
  console.error(e instanceof Error ? e.message : e);
  process.exitCode = 1;
});
