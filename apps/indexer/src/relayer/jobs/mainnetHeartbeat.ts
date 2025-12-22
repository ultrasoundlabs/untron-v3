import { Effect } from "effect";

import type { RelayJobRow } from "../types";
import type { RelayJobHandlerContext } from "./types";

export const handleMainnetHeartbeat = (_args: {
  job: RelayJobRow & { kind: "mainnet_heartbeat" };
  ctx: RelayJobHandlerContext;
}) => Effect.void;
