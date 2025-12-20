import type { RelayJobHandler } from "./types";

export const handleTrc20Transfer: RelayJobHandler<"trc20_transfer"> = async ({ ctx }) => {
  if (ctx.dryRun) return;
  throw new Error('Relay job kind "trc20_transfer" not implemented');
};
