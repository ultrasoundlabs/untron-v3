import type { Effect } from "effect";

export type HeartbeatHandler<R = any> = {
  readonly name: string;
  readonly effect: Effect.Effect<void, unknown, R>;
};
