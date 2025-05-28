import type { ApiDecoration } from "@polkadot/api/types";

export const isDancebox = (api: ApiDecoration<"promise">): boolean => {
    const runtimeName = api.runtimeVersion.specName.toString();

    return runtimeName === "dancebox";
};
