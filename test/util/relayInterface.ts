import { ApiPromise } from "@polkadot/api";
import type { Header, ParaId, HeadData } from "@polkadot/types/interfaces";
import { u8aToHex, u8aToU8a, isU8a, hexToU8a, compactToU8a, compactFromU8a, compactFromU8aLim } from "@polkadot/util"
import { bool, u32, u8, Bytes } from "@polkadot/types-codec";
import { TypeRegistry } from "@polkadot/types";

export async function getHeaderFromRelay(relayApi: ApiPromise, paraId: ParaId): Promise<Header | null> {
    // Get the latest header from relay storage
    const encoded = await relayApi.query.paras.heads(paraId);
    const registry = new TypeRegistry();
    const headerEncoded: HeadData = await relayApi.createType("HeadData", encoded.toHex());
    const nonEncodedHeader = new Bytes(registry, headerEncoded.toU8a(true)).toHex()

    const header = await relayApi.createType("SpRuntimeHeader", nonEncodedHeader);
    return header;
}