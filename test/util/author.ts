import { ApiPromise } from "@moonwall/cli";
import { stringToHex } from "@polkadot/util";

export async function getAuthorFromDigest(paraApi: ApiPromise): Promise<string | null> {
    // Get the latest author from Digest
    const digests = (await paraApi.query.system.digest()).logs;
    const filtered = digests.filter(
        (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex("nmbs")
    );
    return filtered[0].asPreRuntime[1].toHex();
}

/// Range inclusive
export async function getAuthorFromDigestRange(
    paraApi: ApiPromise,
    blockStart: number,
    blockEnd: number
): Promise<any> {
    const authors = [];

    for (let blockNumber = blockStart; blockNumber <= blockEnd; blockNumber += 1) {
        // Get the latest author from Digest
        const blockHash = await paraApi.rpc.chain.getBlockHash(blockNumber);
        const apiAt = await paraApi.at(blockHash);
        const digests = (await apiAt.query.system.digest()).logs;
        const filtered = digests.filter(
            (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex("nmbs")
        );
        const author = filtered[0].asPreRuntime[1].toHex();
        authors.push([blockNumber, author]);
    }

    return authors;
}
