export const MAINNET_CHAIN_ID = (() => {
  const raw = process.env.UNTRON_V3_CHAIN_ID;
  if (!raw) throw new Error("Missing UNTRON_V3_CHAIN_ID");
  const parsed = Number.parseInt(raw, 10);
  if (!Number.isFinite(parsed)) throw new Error("Invalid UNTRON_V3_CHAIN_ID");
  return parsed;
})();
