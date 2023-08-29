import { ApiPromise } from "@polkadot/api";
import type { Header, ParaId } from "@polkadot/types/interfaces";

export async function getHeaderFromRelay(relayApi: ApiPromise, paraId: ParaId): Promise<Header | null> {
    // Get the latest header from relay storage
    const encoded = await relayApi.query.paras.heads(paraId);
    const header = await relayApi.createType("Header", encoded);
    return header;
}
