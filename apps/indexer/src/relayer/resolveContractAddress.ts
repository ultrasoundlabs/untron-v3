import { isAddress, type Address } from "viem";

function getAddressFromPonderContracts(
  ponderContracts: unknown,
  contractName: string
): string | null {
  if (!ponderContracts || typeof ponderContracts !== "object") return null;
  const contract = (ponderContracts as Record<string, unknown>)[contractName];
  if (!contract || typeof contract !== "object") return null;
  const address = (contract as Record<string, unknown>).address;
  return typeof address === "string" ? address : null;
}

export function resolveContractAddress(args: {
  ponderContracts: unknown;
  contractName: string;
  envVar: string;
}): Address {
  const fromPonder = getAddressFromPonderContracts(args.ponderContracts, args.contractName);
  if (fromPonder && isAddress(fromPonder)) return fromPonder.toLowerCase() as Address;

  const fromEnv = process.env[args.envVar];
  if (fromEnv && isAddress(fromEnv)) return fromEnv.toLowerCase() as Address;

  throw new Error(
    `Missing ${args.contractName} address (expected contracts.${args.contractName}.address or env ${args.envVar})`
  );
}
