import type { ApiDecoration } from "@polkadot/api/types";

export const isLightRuntime = (api: ApiDecoration<"promise">): boolean => {
    const runtimeName = api.runtimeVersion.specName.toString();

    return runtimeName.includes("light");
};

export const isDancebox = (api: ApiDecoration<"promise">): boolean => {
    const runtimeName = api.runtimeVersion.specName.toString();

    return runtimeName === "dancebox";
};
