import { describe, expect, it } from "@effect/vitest";
import type { Address } from "viem";

import { selectNonEmptyClaimQueuesSql } from "../src/relayer/claimFiller/claimQueueQueries";

describe("claimQueueQueries", () => {
  it("quotes camelCase aliases to avoid Postgres folding", () => {
    const query = selectNonEmptyClaimQueuesSql({
      chainId: 1,
      contractAddress: "0x0000000000000000000000000000000000000001" as Address,
    });

    const queryChunks = (query as unknown as { queryChunks?: unknown[] }).queryChunks ?? [];
    const sqlText = queryChunks
      .map((chunk) => {
        if (!chunk || typeof chunk !== "object") return "";
        if (!("value" in chunk)) return "";
        const value = (chunk as { value?: unknown }).value;
        return Array.isArray(value) ? value.join("") : "";
      })
      .join("");

    expect(sqlText).toContain('AS "targetToken"');
    expect(sqlText).toContain('AS "queueLength"');
  });
});
