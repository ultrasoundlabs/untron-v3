import { Cause, Effect, Exit } from "effect";

import type { HeartbeatHandler } from "./types";

export const runHeartbeatHandlers = <R>(args: {
  readonly jobName: string;
  readonly handlers: ReadonlyArray<HeartbeatHandler<R>>;
}): Effect.Effect<void, Error, R> =>
  Effect.gen(function* () {
    const results = yield* Effect.forEach(args.handlers, (handler) =>
      Effect.exit(handler.effect).pipe(Effect.map((exit) => ({ handler, exit })))
    );

    const failures: Array<string> = [];
    for (const { handler, exit } of results) {
      if (Exit.isFailure(exit)) {
        failures.push(`${handler.name}: ${Cause.pretty(exit.cause)}`);
      }
    }
    if (failures.length === 0) return;

    yield* Effect.fail(new Error(`${args.jobName} handlers failed:\n${failures.join("\n")}`));
  });
