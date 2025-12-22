import { encodePacked, sha256, type Hex } from "viem";

export const computeNextEventChainTip = (args: {
  previousTip: Hex;
  blockNumber: bigint;
  blockTimestamp: bigint;
  eventSignature: Hex;
  encodedEventData: Hex;
}): Hex =>
  sha256(
    encodePacked(
      ["bytes32", "uint256", "uint256", "bytes32", "bytes"],
      [
        args.previousTip,
        args.blockNumber,
        args.blockTimestamp,
        args.eventSignature,
        args.encodedEventData,
      ]
    )
  );
