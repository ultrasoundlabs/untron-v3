import { Effect } from "effect";

export const tryPromise = <A>(evaluate: () => PromiseLike<A>) =>
  Effect.tryPromise({
    try: () => evaluate(),
    catch: (error) => (error instanceof Error ? error : new Error(String(error))),
  });
