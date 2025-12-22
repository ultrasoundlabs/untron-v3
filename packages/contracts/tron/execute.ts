import { Effect } from "effect";
import * as fs from "node:fs/promises";
import * as path from "node:path";
import * as readline from "node:readline";
import { TronWeb } from "tronweb";

type Artifact = {
  abi: any[];
  bytecode: string;
};

type AbiInput = {
  name?: string;
  type: string;
  components?: readonly AbiInput[];
};

function usageAndExit(message?: string): never {
  if (message) console.error(`Error: ${message}\n`);
  console.error(
    [
      "Usage:",
      "  npx tsx execute.ts ContractName FunctionNameOrSignature --private-key <64-hex-no-0x> [--full-host https://api.trongrid.io] [--api-key ...] [--fee-limit 100000000] [--call-value 0] [--contract <T...|41...>]",
      "",
      "Examples:",
      "  npx tsx execute.ts MyContract transfer --private-key deadbeef... --contract T... --fee-limit 150000000",
      "  npx tsx execute.ts MyContract 'setStruct((address,uint256,address))' --private-key deadbeef... --full-host https://api.shasta.trongrid.io",
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
  const positionals = argv.filter((a) => !a.startsWith("-"));
  const contractName = positionals[0];
  const functionNameOrSig = positionals[1];

  const privateKey = getFlag(argv, "--private-key") ?? getFlag(argv, "-k");
  const fullHost =
    getFlag(argv, "--full-host") ?? getFlag(argv, "--node") ?? "https://api.trongrid.io";
  const apiKey = getFlag(argv, "--api-key");
  const feeLimit = parseIntFlag("--fee-limit", getFlag(argv, "--fee-limit")) ?? 100_000_000; // SUN
  const callValue = parseIntFlag("--call-value", getFlag(argv, "--call-value")) ?? 0; // SUN
  const contract = getFlag(argv, "--contract") ?? getFlag(argv, "--address");

  if (!contractName) usageAndExit("Missing ContractName positional argument.");
  if (!functionNameOrSig) usageAndExit("Missing FunctionNameOrSignature positional argument.");
  if (!privateKey) usageAndExit("Missing --private-key <64-hex-no-0x> argument.");
  if (privateKey.startsWith("0x")) usageAndExit("TRON private keys must NOT be 0x-prefixed.");
  if (!/^[0-9a-fA-F]{64}$/.test(privateKey))
    usageAndExit("--private-key must be 64 hex characters.");
  if (feeLimit <= 0) usageAndExit("--fee-limit must be > 0 (SUN).");
  if (callValue < 0) usageAndExit("--call-value must be >= 0 (SUN).");

  return {
    contractName,
    functionNameOrSig,
    privateKey,
    fullHost,
    apiKey,
    feeLimit,
    callValue,
    contract,
  };
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
  return v === "y" || v === "yes" || v === "go" || v === "send" || v === "execute";
}

function parseValueBySolType(solType: string, raw: string): any {
  const t = solType.trim();
  const s = raw.trim();

  // Arrays: accept JSON or comma-separated.
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
  if (t === "string") return raw;

  if (t === "bytes" || /^bytes[0-9]+$/.test(t)) {
    const hex = s.startsWith("0x") || s.startsWith("0X") ? s : `0x${s}`;
    ensureHexLike("bytes", hex);
    return hex;
  }

  if (t.startsWith("uint") || t.startsWith("int")) {
    const n = normalizeBigIntString(s);
    return BigInt(n);
  }

  if (s.startsWith("{") || s.startsWith("[")) return JSON.parse(s);
  return s;
}

// TRON "41...." is 21 bytes; ABI address is 20 bytes -> strip the leading "41".
function tronToAbiAddress20(addr: string): string {
  const a = addr.trim();
  let hex41: string;
  if (a.startsWith("T")) hex41 = TronWeb.address.toHex(a);
  else if (/^41[0-9a-fA-F]{40}$/.test(a)) hex41 = a;
  else throw new Error(`Invalid TRON address: "${addr}"`);
  return `0x${hex41.slice(2).toLowerCase()}`;
}

function normalizeByAbiInput(input: AbiInput, value: any): any {
  const t = input.type.trim();

  // arrays
  if (/\[[0-9]*\]$/.test(t)) {
    if (!Array.isArray(value)) throw new Error(`Expected array for type ${t}`);
    const baseType = t.replace(/\[[0-9]*\]$/, "");
    const inner: AbiInput = { ...input, type: baseType };
    return value.map((v) => normalizeByAbiInput(inner, v));
  }

  // tuples
  if (t === "tuple" || t.startsWith("tuple")) {
    const comps = input.components ?? [];
    if (!Array.isArray(comps)) throw new Error(`Tuple missing components for type ${t}`);

    if (Array.isArray(value)) {
      if (value.length !== comps.length) {
        throw new Error(`Tuple arity mismatch: expected ${comps.length}, got ${value.length}`);
      }
      return comps.map((c, i) => normalizeByAbiInput(c, value[i]));
    }

    if (value && typeof value === "object") {
      return comps.map((c) => {
        const key = c.name ?? "";
        if (!key) throw new Error(`Tuple component missing name; provide tuple as JSON array.`);
        return normalizeByAbiInput(c, (value as any)[key]);
      });
    }

    throw new Error(`Expected tuple as JSON array or object for type ${t}`);
  }

  if (t === "address") return tronToAbiAddress20(String(value));

  if (t === "bytes" || /^bytes[0-9]+$/.test(t)) {
    const s = String(value);
    const hex = s.startsWith("0x") || s.startsWith("0X") ? s : `0x${s}`;
    ensureHexLike("bytes", hex);
    return hex.toLowerCase();
  }

  return value;
}

function isViewOrPure(funAbi: any): boolean {
  const sm = String(funAbi?.stateMutability ?? "").toLowerCase();
  const constant = funAbi?.constant === true;
  return constant || sm === "view" || sm === "pure";
}

// Canonical ABI type for function selector (tuple expansion)
function canonicalType(input: AbiInput): string {
  const t = input.type.trim();

  // tuple, tuple[], tuple[2], ...
  if (t === "tuple" || t.startsWith("tuple")) {
    const comps = input.components ?? [];
    if (!Array.isArray(comps)) throw new Error(`Tuple missing components for selector.`);
    const tupleBody = `(${comps.map(canonicalType).join(",")})`;
    const suffix = t.slice("tuple".length); // "", "[]", "[2]", "[][3]" etc
    return `${tupleBody}${suffix}`;
  }
  return t;
}

function signatureOf(funAbi: any): string {
  const name = String(funAbi?.name ?? "");
  const inputs = Array.isArray(funAbi?.inputs) ? (funAbi.inputs as AbiInput[]) : [];
  return `${name}(${inputs.map(canonicalType).join(",")})`;
}

function normalizeContractAddress(addr: string): { base58: string; hex41: string } {
  const a = addr.trim();
  if (a.startsWith("T")) {
    const hex41 = TronWeb.address.toHex(a);
    if (!/^41[0-9a-fA-F]{40}$/.test(hex41))
      throw new Error(`Invalid base58 TRON address: "${addr}"`);
    return { base58: a, hex41 };
  }
  if (/^41[0-9a-fA-F]{40}$/.test(a)) {
    const base58 = TronWeb.address.fromHex(a);
    return { base58, hex41: a };
  }
  throw new Error(`Invalid contract address: "${addr}" (expected T... or 41...)`);
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
  const {
    contractName,
    functionNameOrSig,
    privateKey,
    fullHost,
    apiKey,
    feeLimit,
    callValue,
    contract,
  } = parseCli(argv);

  const tronWeb = new TronWeb({
    fullHost,
    headers: apiKey ? { "TRON-PRO-API-KEY": apiKey } : undefined,
    privateKey,
  });

  const ownerAddress = TronWeb.address.fromPrivateKey(privateKey);
  if (!ownerAddress) throw new Error("Failed to derive owner address from private key.");

  const p = artifactPath(contractName);
  const raw = yield* Effect.tryPromise({
    try: () => fs.readFile(p, "utf8"),
    catch: (e) => new Error(`Failed to read artifact at ${p}: ${String(e)}`),
  });

  const artifact = ensureArtifact(safeJsonParse(raw));

  // Find functions
  const funs = artifact.abi.filter((x) => x?.type === "function");
  if (funs.length === 0) throw new Error(`No functions found in ABI for ${contractName}.`);

  const wantSig = functionNameOrSig.includes("(");
  let candidates = funs.filter((f: any) => {
    if (!f?.name) return false;
    if (wantSig) return signatureOf(f) === functionNameOrSig;
    return f.name === functionNameOrSig;
  });

  if (!wantSig && candidates.length === 0) {
    // allow "foo(address,uint256)" typed as FunctionNameOrSignature but with spaces etc
    const normalized = functionNameOrSig.replace(/\s+/g, "");
    candidates = funs.filter((f: any) => signatureOf(f) === normalized);
  }

  if (candidates.length === 0) {
    const sample = funs
      .slice(0, 10)
      .map((f: any) => `  - ${signatureOf(f)}`)
      .join("\n");
    throw new Error(
      `Function not found: "${functionNameOrSig}"\n\nSome ABI functions:\n${sample}\n...`
    );
  }

  // Choose overload if needed
  const funAbi = yield* Effect.scoped(
    Effect.gen(function* () {
      if (candidates.length === 1) return candidates[0];

      console.log(`Multiple overloads found for "${functionNameOrSig}". Select one:`);
      candidates.forEach((f: any, i: number) => {
        const sm = String(f?.stateMutability ?? "");
        console.log(`  [${i}] ${signatureOf(f)}  (${sm || "unknown"})`);
      });

      const rl = yield* makeReadline;
      const ans = yield* question(rl, `Choose overload index (0-${candidates.length - 1}) [0]: `);
      const idx = ans === "" ? 0 : Number(ans);
      if (!Number.isInteger(idx) || idx < 0 || idx >= candidates.length) {
        throw new Error(`Invalid overload index: "${ans}"`);
      }
      return candidates[idx];
    })
  );

  const functionSelector = signatureOf(funAbi);
  const inputs = Array.isArray(funAbi?.inputs) ? (funAbi.inputs as AbiInput[]) : [];

  // Contract address prompt
  const contractAddr = yield* Effect.scoped(
    Effect.gen(function* () {
      if (contract && contract.trim().length > 0) return normalizeContractAddress(contract);
      const rl = yield* makeReadline;
      const ans = yield* question(rl, `Contract address (T... or 41...): `);
      return normalizeContractAddress(ans);
    })
  );

  // Prompt args
  const argsRaw = yield* Effect.gen(function* () {
    if (inputs.length === 0) return [] as any[];
    console.log(`Function inputs (${inputs.length}). For arrays/tuples, paste JSON.`);
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

  const normalizedArgs = inputs.map((inp, i) => normalizeByAbiInput(inp, argsRaw[i]));

  console.log("\n---");
  console.log(`Network fullHost: ${fullHost}`);
  console.log(`Caller: ${ownerAddress}`);
  console.log(`Contract (base58): ${contractAddr.base58}`);
  console.log(`Contract (hex41):  ${contractAddr.hex41}`);
  console.log(`Function: ${functionSelector}`);
  console.log(`callValue (SUN): ${callValue}  (~${callValue / 1_000_000} TRX)`);
  console.log(`feeLimit (SUN):  ${feeLimit}  (~${feeLimit / 1_000_000} TRX max burn)`);

  // View/pure => constant call and decode outputs
  if (isViewOrPure(funAbi)) {
    console.log("\nFunction is view/pure. Running constant call (no broadcast)...");
    const sim = yield* Effect.tryPromise({
      try: () =>
        tronWeb.transactionBuilder.triggerConstantContract(
          contractAddr.base58,
          functionSelector,
          { callValue, funcABIV2: funAbi, parametersV2: normalizedArgs },
          [] // ignored when funcABIV2 exists
        ),
      catch: (e) => new Error(`triggerConstantContract failed: ${String(e)}`),
    });

    const ok = (sim as any)?.result?.result === true;
    const energyUsed = (sim as any)?.energy_used;
    console.log(`Result: ${ok ? "OK" : "FAILED"}`);
    if (typeof energyUsed === "number") console.log(`energy_used: ${energyUsed}`);

    const constantResult: string[] | undefined = (sim as any)?.constant_result;
    const outputs = Array.isArray(funAbi?.outputs) ? funAbi.outputs : [];
    if (Array.isArray(constantResult) && constantResult.length > 0) {
      const data = `0x${constantResult[0]}`;
      try {
        const decoded = tronWeb.utils.abi.decodeParamsV2ByABI({ ...funAbi, outputs }, data);
        console.log("Decoded output:", decoded);
      } catch {
        console.log("Raw constant_result[0]:", constantResult[0]);
      }
    } else {
      console.log("No constant_result returned.");
    }
    return;
  }

  // --- Estimate energy ---
  console.log("\nEstimating energy...");
  let energyRequired: number | undefined;

  const estimateEnergyResult = yield* Effect.tryPromise({
    try: async () => {
      const est = await tronWeb.transactionBuilder.estimateEnergy(
        contractAddr.base58,
        functionSelector,
        { callValue, funcABIV2: funAbi, parametersV2: normalizedArgs },
        [] // ignored when funcABIV2 exists
      );
      const ok = (est as any)?.result?.result === true;
      const er = (est as any)?.energy_required;
      if (!ok) throw new Error(`estimateEnergy returned result=false: ${JSON.stringify(est)}`);
      if (typeof er !== "number")
        throw new Error(`estimateEnergy missing energy_required: ${JSON.stringify(est)}`);
      return { est, energyRequired: er };
    },
    catch: (e) => (e instanceof Error ? e : new Error(String(e))),
  }).pipe(
    Effect.map((r) => ({ _tag: "ok" as const, ...r })),
    Effect.catchAll((error) => Effect.succeed({ _tag: "err" as const, error }))
  );

  if (estimateEnergyResult._tag === "ok") {
    energyRequired = estimateEnergyResult.energyRequired;
    console.log(`energy_required: ${energyRequired}`);
  } else {
    // Fallback: triggerConstantContract can also simulate & report energy_used
    const msg =
      estimateEnergyResult.error instanceof Error
        ? estimateEnergyResult.error.message
        : String(estimateEnergyResult.error);
    console.log(
      `estimateEnergy unavailable/failed (${msg}). Falling back to triggerConstantContract...`
    );

    const sim = yield* Effect.tryPromise({
      try: () =>
        tronWeb.transactionBuilder.triggerConstantContract(
          contractAddr.base58,
          functionSelector,
          { callValue, funcABIV2: funAbi, parametersV2: normalizedArgs },
          []
        ),
      catch: (err) => new Error(`Energy simulation failed: ${String(err)}`),
    });

    const ok = (sim as any)?.result?.result === true;
    const energyUsed = (sim as any)?.energy_used;
    const energyPenalty = (sim as any)?.energy_penalty;

    if (!ok) throw new Error(`Simulation returned result=false: ${JSON.stringify(sim)}`);
    if (typeof energyUsed !== "number")
      throw new Error(`Simulation missing energy_used: ${JSON.stringify(sim)}`);

    energyRequired = typeof energyPenalty === "number" ? energyUsed + energyPenalty : energyUsed;

    console.log(`energy_used: ${energyUsed}`);
    if (typeof energyPenalty === "number") console.log(`energy_penalty: ${energyPenalty}`);
    console.log(`estimated total energy: ${energyRequired}`);
  }

  // Confirm
  const proceed = yield* Effect.scoped(
    Effect.gen(function* () {
      const rl = yield* makeReadline;
      const ans = yield* question(
        rl,
        `\nReady to execute ${functionSelector} on ${contractAddr.base58} from ${ownerAddress}? Type "yes" to continue: `
      );
      return parseProceedConfirmation(ans);
    })
  );

  if (!proceed) {
    console.log("Aborted before execution.");
    return;
  }

  // --- Execute ---
  console.log("\nBuilding transaction...");
  const txWrap = yield* Effect.tryPromise({
    try: () =>
      tronWeb.transactionBuilder.triggerSmartContract(
        contractAddr.base58,
        functionSelector,
        { feeLimit, callValue, funcABIV2: funAbi, parametersV2: normalizedArgs },
        [] // ignored when funcABIV2 exists
      ),
    catch: (e) => new Error(`Failed to build triggerSmartContract tx: ${String(e)}`),
  });

  const tx = (txWrap as any)?.transaction;
  const txID = (tx as any)?.txID ?? (txWrap as any)?.txID;

  if (!tx)
    throw new Error(`triggerSmartContract returned no transaction: ${JSON.stringify(txWrap)}`);

  console.log("Unsigned txID:", txID ?? "(unknown)");

  const signed = yield* Effect.tryPromise({
    try: () => tronWeb.trx.sign(tx),
    catch: (e) => new Error(`Failed to sign transaction: ${String(e)}`),
  });

  const broadcast = yield* Effect.tryPromise({
    try: () => tronWeb.trx.sendRawTransaction(signed),
    catch: (e) => new Error(`Broadcast failed: ${String(e)}`),
  });

  const ok = (broadcast as any)?.result === true;
  if (!ok) throw new Error(`Broadcast rejected: ${JSON.stringify(broadcast)}`);

  console.log("\n---");
  console.log("Broadcast result: OK");
  console.log("Transaction:", (broadcast as any)?.txid ?? txID ?? "(unknown)");
});

Effect.runPromise(program).catch((e) => {
  console.error(e instanceof Error ? e.message : e);
  process.exitCode = 1;
});
