import { describe, expect, it } from "vitest";
import type { Address } from "viem";

import { computeReceiverBytecodeHash } from "../src/api/receiverBytecodeHash";

describe("computeReceiverBytecodeHash", () => {
  it("matches expected keccak256 for EIP-1167 bytecode", () => {
    const receiverImpl = "0x000000000000000000000000000000000000beef" as Address;
    expect(computeReceiverBytecodeHash(receiverImpl)).toBe(
      "0x5697cd44146094e8f128ebba44afd22c351a9eec57978bac3d000b3db70fca01"
    );
  });
});
