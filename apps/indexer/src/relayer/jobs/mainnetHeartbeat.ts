import type { RelayJobHandler } from "./types";

export const handleMainnetHeartbeat: RelayJobHandler<"mainnet_heartbeat"> = async ({ ctx }) => {
  if (ctx.dryRun) return;
  throw new Error('Relay job kind "mainnet_heartbeat" not implemented');
};
