import type { ApiDecoration } from "@polkadot/api/types";
import type { Balance } from "@polkadot/types/interfaces";

export const getAccountBalance = async (api: ApiDecoration<"promise">, account: string): Promise<Balance> => {
    const result = await api.query.system.account(account);

    return result.data.free;
};
