import type { Context as PonderContext, EventNames } from "ponder:registry";

import { relayJob, relayJobKindEnum, relayJobStatusEnum } from "ponder:schema";

export type BlockEventName = Extract<EventNames, `${string}:block`>;
export type ContractName = keyof PonderContext["contracts"];
export type PonderRegistry = (typeof import("ponder:registry"))["ponder"];

export type RelayJobKind = (typeof relayJobKindEnum.enumValues)[number];
export type RelayJobStatus = (typeof relayJobStatusEnum.enumValues)[number];
export type RelayJobRow = typeof relayJob.$inferSelect;
