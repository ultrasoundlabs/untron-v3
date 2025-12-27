import { Effect, Option, Redacted } from "effect";
import { type SafeVersion, toSafeSmartAccount } from "permissionless/accounts";
import { privateKeyToAccount } from "viem/accounts";
import { type Address, type Hex, type PublicClient } from "viem";
import { entryPoint06Address } from "viem/account-abstraction";

const requireSome = <A>(opt: Option.Option<A>, message: string): Effect.Effect<A, Error> =>
  Option.match(opt, {
    onNone: () => Effect.fail(new Error(message)),
    onSome: Effect.succeed,
  });

export type SafeAccountConfig = Readonly<{
  ownerPrivateKey: Option.Option<Redacted.Redacted<string>>;
  safeVersion: "1.4.1" | "1.5.0";
  entryPointAddress: Option.Option<Address>;
  safeAddress: Option.Option<Address>;
  saltNonce: bigint;
}>;

const parseMainnetOwnerPrivateKey = (value: Redacted.Redacted<string>): Hex => {
  const raw = Redacted.value(value).trim();
  if (!/^0x[0-9a-fA-F]{64}$/.test(raw)) {
    throw new Error('Invalid RELAYER_MAINNET_OWNER_PRIVATE_KEY (expected "0x" + 64 hex chars)');
  }
  return raw as Hex;
};

export const makeSafeAccount = (args: { config: SafeAccountConfig; publicClient: PublicClient }) =>
  Effect.gen(function* () {
    const ownerPrivateKey = yield* requireSome(
      args.config.ownerPrivateKey,
      "Missing env var RELAYER_MAINNET_OWNER_PRIVATE_KEY"
    );

    const entryPointAddress =
      Option.getOrUndefined(args.config.entryPointAddress) ?? entryPoint06Address;

    const safeAddress = Option.getOrUndefined(args.config.safeAddress);

    return yield* Effect.tryPromise({
      try: () =>
        toSafeSmartAccount({
          client: args.publicClient,
          owners: [privateKeyToAccount(parseMainnetOwnerPrivateKey(ownerPrivateKey))],
          version: args.config.safeVersion as SafeVersion,
          entryPoint: { address: entryPointAddress, version: "0.6" },
          address: safeAddress,
          saltNonce: args.config.saltNonce,
        }),
      catch: (error) => (error instanceof Error ? error : new Error(String(error))),
    });
  });
