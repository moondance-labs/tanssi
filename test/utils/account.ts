import type { ApiDecoration } from "@polkadot/api/types";
import type { Balance } from "@polkadot/types/interfaces";
import { TREASURY_ADDRESS_BOX_CHAINS, TREASURY_ADDRESS_LIGHT_CHAINS } from "./constants.ts";

export const getAccountBalance = async (api: ApiDecoration<"promise">, account: string): Promise<Balance> => {
    const result = await api.query.system.account(account);

    return result.data.free;
};

export const getTreasuryAddress = (api: ApiDecoration<"promise">): string => {
    const runtimeName = api.runtimeVersion.specName.toString();

    return runtimeName.includes("light") ? TREASURY_ADDRESS_LIGHT_CHAINS : TREASURY_ADDRESS_BOX_CHAINS;
};

// https://github.com/moondance-labs/tanssi/blob/5c37826cfb655f86701cf1b0d39d359adaa9622b/chains/orchestrator-relays/runtime/dancelight/constants/src/lib.rs#L44
export const deposit = (items: number, bytes: number, itemFee = 100_000_000_000n, byteFee = 100_000_000n): bigint =>
    itemFee * BigInt(items) + byteFee * BigInt(bytes);
