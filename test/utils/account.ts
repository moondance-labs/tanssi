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
