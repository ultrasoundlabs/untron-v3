import { z } from "zod";
import { parseEnv } from "../lib/env.js";
import { log } from "../lib/logger.js";
import { createTronClients } from "@untron/tron-protocol";
import type { BytesMessage } from "@untron/tron-protocol/api";
import {
  Transaction,
  Transaction_Contract_ContractType,
  transaction_Contract_ContractTypeToJSON,
  type Transaction_Contract,
} from "@untron/tron-protocol/tron";
import { TriggerSmartContract } from "@untron/tron-protocol/core/contract/smart_contract";

function toHex(buf: Uint8Array | Buffer): string {
  return Buffer.from(buf).toString("hex");
}

function toHex0x(buf: Uint8Array | Buffer): string {
  return `0x${Buffer.from(buf).toString("hex")}`;
}

function tronAddressToEvmAddress(bytes: Uint8Array | Buffer): string | null {
  // Tron address is 21 bytes: 0x41 prefix + 20-byte EVM address
  if (bytes.length !== 21 || bytes[0] !== 0x41) return null;
  return `0x${Buffer.from(bytes.subarray(1)).toString("hex")}`;
}

function readVarint(buf: Uint8Array, startOffset: number): { value: bigint; offset: number } {
  let result = 0n;
  let shift = 0n;
  let offset = startOffset;

  while (offset < buf.length) {
    const byteValue = buf[offset];
    if (byteValue === undefined) {
      throw new Error("Unexpected end of buffer while reading varint");
    }
    const byte = BigInt(byteValue);
    result |= (byte & 0x7fn) << shift;
    offset += 1;
    if ((byte & 0x80n) === 0n) break;
    shift += 7n;
  }

  return { value: result, offset };
}

type ParsedTriggerSmartContract = {
  ownerAddress?: string;
  contractAddress?: string;
  callValue?: bigint;
  data?: string;
  callTokenValue?: bigint;
  tokenId?: bigint;
};

function parseTriggerSmartContractBytes(bytes: Uint8Array): ParsedTriggerSmartContract {
  let offset = 0;
  const out: ParsedTriggerSmartContract = {};

  while (offset < bytes.length) {
    const keyRes = readVarint(bytes, offset);
    const key = keyRes.value;
    offset = keyRes.offset;

    const fieldNumber = Number(key >> 3n);
    const wireType = Number(key & 0x7n);

    if (wireType === 0) {
      // VARINT
      const valRes = readVarint(bytes, offset);
      const value = valRes.value;
      offset = valRes.offset;

      switch (fieldNumber) {
        case 3: {
          // call_value
          out.callValue = value;
          break;
        }
        case 5: {
          // call_token_value
          out.callTokenValue = value;
          break;
        }
        case 6: {
          // token_id
          out.tokenId = value;
          break;
        }
        default: {
          // unknown varint field – parsed but not stored
          break;
        }
      }
    } else if (wireType === 2) {
      // LEN-DELIMITED (bytes)
      const lenRes = readVarint(bytes, offset);
      const len = Number(lenRes.value);
      offset = lenRes.offset;

      const end = offset + len;
      const fieldBytes = bytes.slice(offset, end);
      offset = end;

      switch (fieldNumber) {
        case 1: {
          // owner_address
          out.ownerAddress = toHex(fieldBytes);
          break;
        }
        case 2: {
          // contract_address
          out.contractAddress = toHex(fieldBytes);
          break;
        }
        case 4: {
          // data
          out.data = toHex(fieldBytes);
          break;
        }
        default: {
          // unknown bytes field – ignore
          break;
        }
      }
    } else {
      throw new Error(
        `Unsupported protobuf wire type ${wireType} in TriggerSmartContract at offset ${offset}`
      );
    }
  }

  return out;
}

function parseTxIdArg(argv: string[]): string {
  const args = argv.slice(2);
  let candidate: string | undefined;

  if (args.length === 1) {
    // Direct invocation: `tsx parseTronTransaction.ts <txId>`
    candidate = args[0];
  } else if (args.length >= 2) {
    // Runner style: `tsx src/run.ts parseTronTransaction <txId>`
    candidate = args[1];
  }

  const value = candidate ?? "";

  if (!/^(0x)?[0-9a-fA-F]{64}$/.test(value)) {
    throw new Error("Expected txId as 32-byte hex string (with or without 0x prefix)");
  }

  return value.replace(/^0x/i, "").toLowerCase();
}

function splitSelectorAndArgs(data: Buffer): { selector?: string; args?: string } {
  if (data.length === 0) return {};
  if (data.length <= 4) {
    return { selector: toHex0x(data) };
  }

  const selector = data.subarray(0, 4);
  const args = data.subarray(4);

  return {
    selector: toHex0x(selector),
    args: toHex0x(args),
  };
}

function decodeTriggerContracts(contracts: Transaction_Contract[]) {
  return contracts
    .filter((c) => c.type === Transaction_Contract_ContractType.TriggerSmartContract)
    .map((contract, index) => {
      const param = contract.parameter;
      if (!param) {
        return {
          index,
          type: transaction_Contract_ContractTypeToJSON(contract.type),
          error: "Missing parameter",
        };
      }

      const rawBytes = param.value as Buffer;

      const manual = parseTriggerSmartContractBytes(rawBytes);
      const decoded = TriggerSmartContract.decode(rawBytes);

      const ownerTron = decoded.ownerAddress;
      const contractTron = decoded.contractAddress;
      const ownerEvm = tronAddressToEvmAddress(ownerTron);
      const contractEvm = tronAddressToEvmAddress(contractTron);
      const dataBuf = decoded.data;
      const { selector, args } = splitSelectorAndArgs(dataBuf);

      return {
        index,
        type: transaction_Contract_ContractTypeToJSON(contract.type),
        manual: {
          ownerAddress: manual.ownerAddress,
          contractAddress: manual.contractAddress,
          callValue: manual.callValue !== undefined ? manual.callValue.toString() : undefined,
          data: manual.data ? `0x${manual.data}` : undefined,
          callTokenValue:
            manual.callTokenValue !== undefined ? manual.callTokenValue.toString() : undefined,
          tokenId: manual.tokenId !== undefined ? manual.tokenId.toString() : undefined,
        },
        tsProto: {
          ownerAddress: toHex0x(ownerTron),
          ownerAddressEvm: ownerEvm ?? undefined,
          contractAddress: toHex0x(contractTron),
          contractAddressEvm: contractEvm ?? undefined,
          callValue: decoded.callValue.toString(),
          data: toHex0x(dataBuf),
          selector,
          args,
          callTokenValue: decoded.callTokenValue.toString(),
          tokenId: decoded.tokenId.toString(),
        },
      };
    });
}

async function main() {
  const env = parseEnv(
    z.object({
      TRON_GRPC_HOST: z.string().min(1),
      TRON_API_KEY: z.string().optional(),
    })
  );

  const txIdHex = parseTxIdArg(process.argv);

  const { wallet, callOpts } = createTronClients(env.TRON_GRPC_HOST, env.TRON_API_KEY, {
    insecure: true,
  });

  const request: BytesMessage = { value: Buffer.from(txIdHex, "hex") };

  const tx = await new Promise<Transaction>((resolve, reject) => {
    wallet.getTransactionById(request, callOpts.metadata, (err, res) => {
      if (err || !res) return reject(err ?? new Error("Empty response from getTransactionById"));
      resolve(res);
    });
  });

  const raw = tx.rawData;
  if (!raw) {
    throw new Error("Transaction rawData is missing");
  }

  const encodedTx = Transaction.encode(tx).finish();

  log.info("Transaction encoded bytes (hex)", {
    txId: `0x${txIdHex}`,
    hex: toHex(encodedTx),
  });

  const triggerContracts = decodeTriggerContracts(raw.contract ?? []);

  log.info("TriggerSmartContract transaction decoding", {
    txId: `0x${txIdHex}`,
    rawData: {
      refBlockBytes: toHex0x(raw.refBlockBytes),
      refBlockNum: raw.refBlockNum.toString(),
      refBlockHash: toHex0x(raw.refBlockHash),
      expiration: raw.expiration.toString(),
      timestamp: raw.timestamp.toString(),
      feeLimit: raw.feeLimit.toString(),
      contracts: (raw.contract ?? []).map((c, index) => ({
        index,
        type: transaction_Contract_ContractTypeToJSON(c.type),
        hasParameter: !!c.parameter,
      })),
    },
    triggerContracts,
    triggerContractsPretty: JSON.stringify(triggerContracts, null, 2),
  });
}

main().catch((err) => {
  log.error(err);
  process.exit(1);
});
