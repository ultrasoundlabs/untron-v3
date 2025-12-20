import type { RelayJobHandler } from "./types";

export const handleTronHeartbeat: RelayJobHandler<"tron_heartbeat"> = async ({ ctx }) => {
  if (ctx.dryRun) return;
  throw new Error('Relay job kind "tron_heartbeat" not implemented');
};
