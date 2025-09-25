import type { ApiPromise } from "@polkadot/api";
import { type BlockCreationResponse, expect } from "@moonwall/cli";
import { BN } from "@polkadot/util";
import type { ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";

export type ExtrinsicFailedEventDataType = {
    dispatchError: {
        Module: {
            index: string;
            error: string;
        };
    };
};

export type SubmittedEventDataType = {
    index: number;
};

export async function checkIfErrorIsEmitted(
    api: ApiPromise,
    moduleName: string,
    failedBlock: BlockCreationResponse<ApiTypes, SubmittableExtrinsic<any>>,
    errorName: string
): Promise<boolean> {
    const metadata = await api.rpc.state.getMetadata();
    const palletIndex = metadata.asLatest.pallets.find(({ name }) => name.toString() === moduleName).index.toString();

    const errorData = failedBlock.result.events.find((e) => e.event.method === "ExtrinsicFailed").event.toHuman()
        .data as unknown as ExtrinsicFailedEventDataType;
    expect(errorData.dispatchError.Module.index).toEqual(palletIndex);

    const errorBytes = Uint8Array.from(Buffer.from(errorData.dispatchError.Module.error.slice(2), "hex"));
    const errorIndex = errorBytes[0];

    const errorMeta = api.registry.findMetaError({
        index: new BN(errorData.dispatchError.Module.index),
        error: new BN(errorIndex),
    });

    if (errorMeta.method === errorName) {
        return true;
    }
    return false;
}
