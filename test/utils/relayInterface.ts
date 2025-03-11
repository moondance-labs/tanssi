import type { ApiPromise } from "@polkadot/api";
import type { ParaId, HeadData } from "@polkadot/types/interfaces";
import { Bytes } from "@polkadot/types-codec";
import { TypeRegistry } from "@polkadot/types";
import type { AnyNumber } from "@polkadot/types-codec/types";
import type { SpRuntimeHeader } from "@polkadot/types/lookup";

export async function getHeaderFromRelay(relayApi: ApiPromise, paraId: ParaId | AnyNumber) {
    // Get the latest header from relay storage
    const encoded = await relayApi.query.paras.heads(paraId);
    const registry = new TypeRegistry();
    const headerEncoded: HeadData = relayApi.createType("HeadData", encoded.toHex());
    const nonEncodedHeader = new Bytes(registry, headerEncoded.toU8a(true)).toHex();

    return relayApi.createType("SpRuntimeHeader", nonEncodedHeader) as unknown as SpRuntimeHeader;
}
