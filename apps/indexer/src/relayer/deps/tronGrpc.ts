import { createTronClients } from "@untron/tron-protocol";

import type { TronGrpcClients } from "./types";

export function createTronGrpcClientsGetter() {
  let tronGrpcClients: TronGrpcClients | null = null;

  const getTronGrpcClients = (): TronGrpcClients => {
    if (tronGrpcClients) return tronGrpcClients;

    const host = process.env.TRON_GRPC_HOST;
    if (!host) throw new Error("Missing env var TRON_GRPC_HOST");

    const apiKey = process.env.TRON_API_KEY;
    const insecure = process.env.TRON_GRPC_INSECURE === "true";

    tronGrpcClients = createTronClients(host, apiKey, { insecure });
    return tronGrpcClients;
  };

  return { getTronGrpcClients };
}
