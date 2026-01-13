import type { ApiPromise } from "@polkadot/api";
import { stringToHex } from "@polkadot/util";

export async function getAuthorFromDigest(paraApi: ApiPromise): Promise<string | null> {
    // Get the latest author from Digest
    const digests = (await paraApi.query.system.digest()).logs;
    const filtered = digests.filter(
        (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() === stringToHex("nmbs")
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
            (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() === stringToHex("nmbs")
        );
        const author = filtered[0].asPreRuntime[1].toHex();
        authors.push([blockNumber, author]);
    }

    return authors;
}

// Fetch block timestamp (ms) at a given height, using pallet timestamp storage
export const fetchBlockTimestampMs = async (api: ApiPromise, blockNumber: number): Promise<number | undefined> => {
    const hash = await api.rpc.chain.getBlockHash(blockNumber);
    const apiAt = await api.at(hash);
    const ts = await apiAt.query.timestamp.now();
    return ts.toNumber();
};

/**
 * Print a short “who authored which block” report for a chain.
 * This doesn't assert or check anything, it just prints.
 *
 * Example output:
 *
 * # 1 (+ 0s): 0x884a1b28ae04bef60698f4fab5651c02bd9df4f784f6ac59c989857da8e1d15f
 * # 2 (+ 3s): 0x884a1b28ae04bef60698f4fab5651c02bd9df4f784f6ac59c989857da8e1d15f
 * # 3 (+ 3s): 0x4273e5483ebed8ef633700152986e5a43d8b89f3fd4eeb4c54d68fa93c227f28
 * # 4 (+12s): 0x4273e5483ebed8ef633700152986e5a43d8b89f3fd4eeb4c54d68fa93c227f28
 * # 5 (+ 6s): 0x884a1b28ae04bef60698f4fab5651c02bd9df4f784f6ac59c989857da8e1d15f
 * # 6 (+18s): 0x4273e5483ebed8ef633700152986e5a43d8b89f3fd4eeb4c54d68fa93c227f28
 * [Container-2000] Authors summary:
 *   - 0x4273e5483ebed8ef633700152986e5a43d8b89f3fd4eeb4c54d68fa93c227f28: 3
 *   - 0x884a1b28ae04bef60698f4fab5651c02bd9df4f784f6ac59c989857da8e1d15f: 3
 *
 * The output is meant for human inspection in test logs, to quickly see:
 *   - whether authorship is rotating as expected
 *   - whether there are large gaps in block production
 *
 * The block timestamp is read from the timestamp pallet, from on chain storage.
 *
 * @param api       Polkadot / Substrate API instance.
 * @param nodeLabel Label shown in log output (e.g. `"Container-2000"`). Cosmetic only.
 */
export async function printBlockAuthorsWindow(api: ApiPromise, nodeLabel: string): Promise<void> {
    const head = await api.rpc.chain.getBlock();
    const current = head.block.header.number.toNumber();

    // Show the latest 40 blocks, or up to 40 if current block is less than 40
    const start = Math.max(1, current - 40 + 1);
    const end = current;

    const authorPairs = await getAuthorFromDigestRange(api, start, end);

    const byAuthor = new Map<string, number>();
    console.log(`\n[${nodeLabel}] Block authors for #${start}..#${end} (${authorPairs.length} blocks):`);

    let prevTs: number | null = null;
    const rows: { n: number; author: string; delta: number }[] = [];

    for (const [n, author] of authorPairs) {
        const ts = await fetchBlockTimestampMs(api, n);
        const deltaSec = prevTs != null && ts != null ? Math.max(0, Math.round((ts - prevTs) / 1000)) : 0;

        rows.push({ n, author, delta: deltaSec });

        if (ts != null) prevTs = ts;
        byAuthor.set(author, (byAuthor.get(author) ?? 0) + 1);
    }

    if (rows.length === 0) {
        console.log(`[${nodeLabel}] No blocks in range, nothing to print.`);
        return;
    }

    const numWidth = Math.max(...rows.map((r) => r.n)).toString().length;
    const deltaWidth = Math.max(...rows.map((r) => r.delta), 0).toString().length;

    const lines = rows.map(
        ({ n, author, delta }) =>
            `#${n.toString().padStart(numWidth)} (+${delta.toString().padStart(deltaWidth)}s): ${author}`
    );

    console.log(lines.join("\n"));

    const entries = [...byAuthor.entries()].sort((a, b) => b[1] - a[1]);
    const summaryLines = entries.map(([a, c]) => `  - ${a}: ${c}`).join("\n") || "  (none)";

    console.log(`[${nodeLabel}] Authors summary:\n${summaryLines}`);
}
