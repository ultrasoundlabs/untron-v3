import { describe, expect, it } from "vitest";
import type { Hash } from "viem";

import { toApiUserOperation } from "../src/api/userOperation";
import type { SendMainnetUserOperationResult } from "../src/relayer/deps/types";
import { safeUrlForLogs } from "../src/relayer/deps/mainnet/utils";

describe("bundlerUrl redaction", () => {
  it("safeUrlForLogs strips query params", () => {
    expect(safeUrlForLogs("https://api.pimlico.io/v2/polygon/rpc?apikey=secret")).toBe(
      "https://api.pimlico.io/v2/polygon/rpc"
    );
  });

  it("API userOperation omits bundlerUrl entirely", () => {
    const sent: SendMainnetUserOperationResult = {
      bundlerUrl: "https://api.pimlico.io/v2/polygon/rpc?apikey=secret",
      userOpHash: ("0x" + "3".repeat(64)) as Hash,
      transactionHash: ("0x" + "4".repeat(64)) as Hash,
      blockNumber: 123n,
      success: true,
    };

    const apiUserOperation = toApiUserOperation(sent) as unknown as Record<string, unknown>;
    expect("bundlerUrl" in apiUserOperation).toBe(false);
  });
});
