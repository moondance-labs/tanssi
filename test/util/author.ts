import { ApiPromise } from "@moonwall/cli";
import { stringToHex } from '@polkadot/util';

export async function getAuthorFromDigest(paraApi: ApiPromise): Promise<string | null> {
    // Get the latest author from Digest
    const digests = (await paraApi.query.system.digest()).logs;
    const filtered = digests.filter(log => 
        log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex('nmbs')
    );
    return filtered[0].asPreRuntime[1].toHex()
}