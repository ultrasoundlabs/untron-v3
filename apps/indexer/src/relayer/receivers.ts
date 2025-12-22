import { ConfigError, Effect } from "effect";
import type { Address } from "viem";

import { TronRelayer } from "./deps/tron";
import type { TronReceiverMapEntry } from "./deps/types";

export const getKnownTronReceiver = (
  receiverAddress: Address
): Effect.Effect<TronReceiverMapEntry, ConfigError.ConfigError | Error, TronRelayer> =>
  TronRelayer.getReceiverMap().pipe(
    Effect.flatMap((receiverMap) => {
      const receiver = receiverMap.get(receiverAddress.toLowerCase());
      if (!receiver) {
        return Effect.fail(
          new Error(
            `Unknown receiver address (not in PREKNOWN_RECEIVER_SALTS mapping): ${receiverAddress}`
          )
        );
      }
      return Effect.succeed(receiver);
    })
  );
