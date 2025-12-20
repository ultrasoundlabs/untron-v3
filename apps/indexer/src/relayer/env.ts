export function parseBigintEnv(name: string, fallback: bigint): bigint {
  const raw = process.env[name];
  if (!raw) return fallback;
  return BigInt(raw);
}

export function parseBooleanEnv(name: string, fallback: boolean): boolean {
  const raw = process.env[name];
  if (raw === undefined) return fallback;
  return raw === "true";
}

export function parseNumberEnv(name: string, fallback: number): number {
  const raw = process.env[name];
  if (!raw) return fallback;
  const value = Number(raw);
  return Number.isFinite(value) ? value : fallback;
}

export function getRelayerRuntimeConfig() {
  const enabled = parseBooleanEnv("RELAYER_ENABLED", false);
  const embeddedExecutorEnabled = parseBooleanEnv("RELAYER_EMBEDDED_EXECUTOR_ENABLED", false);
  const dryRun = process.env.RELAYER_DRY_RUN !== "false";

  const workerId = process.env.RELAYER_WORKER_ID ?? `embedded:${process.pid}`;

  const mainnetConfirmations = parseBigintEnv("RELAYER_MAINNET_CONFIRMATIONS", 0n);
  const tronConfirmations = parseBigintEnv("RELAYER_TRON_CONFIRMATIONS", 0n);

  const claimLimit = parseNumberEnv("RELAYER_CLAIM_LIMIT", 10);
  const maxAttempts = parseNumberEnv("RELAYER_MAX_ATTEMPTS", 5);
  const retryDelayBlocks = parseBigintEnv("RELAYER_RETRY_DELAY_BLOCKS", 5n);

  return {
    enabled,
    embeddedExecutorEnabled,
    dryRun,
    workerId,
    mainnetConfirmations,
    tronConfirmations,
    claimLimit,
    maxAttempts,
    retryDelayBlocks,
  };
}
