import type { Address } from "viem";

import { expectAddress, expectString } from "./parse";

const getRequiredEnv = (name: string): string => expectString(process.env[name], name);
const getRequiredEnvAddress = (name: string): Address => expectAddress(getRequiredEnv(name), name);

export const getUntronV3Address = (): Address => getRequiredEnvAddress("UNTRON_V3_ADDRESS");
export const getTronLightClientAddress = (): Address =>
  getRequiredEnvAddress("TRON_LIGHT_CLIENT_ADDRESS");
export const getTronTxReaderAddress = (): Address =>
  getRequiredEnvAddress("TRON_TX_READER_ADDRESS");
