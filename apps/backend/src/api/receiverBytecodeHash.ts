import { concatHex, keccak256, type Address, type Hex } from "viem";

const RECEIVER_BYTECODE_PREFIX = "0x3d602d80600a3d3981f3363d3d373d3d3d363d73" as const;
const RECEIVER_BYTECODE_SUFFIX = "0x5af43d82803e903d91602b57fd5bf3" as const;

export const computeReceiverBytecodeHash = (receiverImpl: Address): Hex =>
  keccak256(concatHex([RECEIVER_BYTECODE_PREFIX, receiverImpl, RECEIVER_BYTECODE_SUFFIX]));
