// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    checkLogsNotExist,
    getAuthorFromDigest,
    getHeaderFromRelay,
    getKeyringNimbusIdHex,
    getTmpZombiePath,
    waitSessions,
    signAndSendAndInclude,
    sleep,
    escapeRegex,
    printBlockAuthorsWindow,
} from "utils";

type PeerSeries = { peers: number[]; times: string[] };

/**
 * Parse peer counts over time for a given node from a multiline log string.
 *
 * The function scans the given text line by line and, for each log line that looks like this:
 *
 * 2025-11-14 15:21:53 [Container-2000] 💤 Idle (1 peers), best: #0 (0xb844…07ae), finalized #0 (0xb844…07ae), ⬇ 295.0kiB/s ⬆ 2.8kiB/s
 *
 * It extracts:
 *   - the peer count `N` as a number, and
 *   - the timestamp at the start of the line (everything before the first `[`).
 *
 * So in this example it would return:
 *
 * { peers: [1], times: ["2025-11-14 15:21:53"] }
 *
 * All other lines are ignored. The result is two parallel arrays where
 * `peers[i]` corresponds to `times[i]`.
 *
 * @param txt - Multiline log text to parse.
 * @param nodeLabel - The node label inside square brackets, e.g. "Container-2000".
 * @returns An object with `peers` (number[]) and `times` (string[]) arrays.
 */
export function parsePeerSeries(txt: string, nodeLabel: string): PeerSeries {
    const peers: number[] = [];
    const times: string[] = [];

    const escapedLabel = escapeRegex(nodeLabel);
    // 1 regex, 1 match attempt per line:
    //   group 1: timestamp (everything before first "[")
    //   group 2: peer count number
    const lineRegex = new RegExp(`^([^\\[]+)\\s+\\[${escapedLabel}\\].*?Idle\\s*\\(\\s*(\\d+)\\s*peers?\\s*\\)`, "i");

    for (const line of txt.split(/\r?\n/)) {
        if (!line) continue;

        const match = lineRegex.exec(line);
        if (!match) continue;

        const rawTime = match[1];
        const rawPeers = match[2];

        if (!rawTime || !rawPeers) {
            throw new Error(
                `Failed to parse log line for [${nodeLabel}]: missing timestamp or peer count in line: ${JSON.stringify(
                    line
                )}`
            );
        }

        const ts = rawTime.trim();
        if (!ts) {
            throw new Error(`Failed to parse timestamp for [${nodeLabel}] from line: ${JSON.stringify(line)}`);
        }

        const n = Number.parseInt(rawPeers, 10);
        if (!Number.isFinite(n)) {
            throw new Error(
                `Failed to parse peer count "${rawPeers}" for [${nodeLabel}] from line: ${JSON.stringify(line)}`
            );
        }

        peers.push(n);
        times.push(ts);
    }

    return { peers, times };
}

export const collectSeries = async (
    logPath: string,
    nodeLabel: string,
    minSamples = 8,
    maxTries = 5,
    delayMs = 5000
): Promise<PeerSeries> => {
    const { readFile } = await import("node:fs/promises");

    let lastSeries: PeerSeries = { peers: [], times: [] };

    for (let attempt = 1; attempt <= maxTries; attempt++) {
        let txt: string;

        try {
            txt = await readFile(logPath, "utf8");
        } catch (err) {
            throw new Error(`Failed to read log file "${logPath}" on attempt ${attempt}: ${(err as Error).message}`);
        }

        try {
            lastSeries = parsePeerSeries(txt, nodeLabel);
        } catch (err) {
            // Add context but keep the original error message
            throw new Error(
                `Failed to parse peer series for [${nodeLabel}] from "${logPath}" on attempt ${attempt}: ${
                    (err as Error).message
                }`
            );
        }

        if (lastSeries.peers.length >= minSamples) {
            return lastSeries;
        }

        // Not enough data yet, wait for more logs to accumulate
        await sleep(delayMs);
    }

    // We never reached the required number of samples
    throw new Error(
        `Expected at least ${minSamples} samples for [${nodeLabel}], ` +
            `but only got ${lastSeries.peers.length} after ${maxTries} attempts reading "${logPath}".`
    );
};

/**
 * Render a tiny unicode sparkline for a sequence of numbers.
 *
 * - Downsamples long series to at most `maxWidth` points (by averaging chunks).
 * - Maps values linearly between the min/max to unicode “height” blocks.
 * - Returns an empty string for an empty input.
 */
function sparkline(values: number[], maxWidth = 120): string {
    if (values.length === 0) return "";

    const blocks = "▁▂▃▄▅▆▇█";

    const data = values.length <= maxWidth ? values : downsample(values, maxWidth);
    const min = Math.min(...data);
    const max = Math.max(...data);

    // Flat line → pick a “middle” block and repeat it.
    if (max === min) {
        const mid = Math.floor(blocks.length / 2);
        const idx = Math.min(blocks.length - 1, Math.max(0, mid));
        return blocks[idx].repeat(data.length);
    }

    return data
        .map((v) => {
            const t = (v - min) / (max - min); // 0..1
            const idx = Math.max(0, Math.min(blocks.length - 1, Math.round(t * (blocks.length - 1))));
            return blocks[idx];
        })
        .join("");
}

/**
 * Downsample `values` to at most `width` points by averaging contiguous chunks.
 */
function downsample(values: number[], width: number): number[] {
    if (values.length <= width) return values;

    const chunkSize = Math.ceil(values.length / width);
    const out: number[] = [];

    for (let i = 0; i < values.length; i += chunkSize) {
        const chunk = values.slice(i, i + chunkSize);
        const sum = chunk.reduce((a, b) => a + b, 0);
        out.push(sum / chunk.length);
    }

    return out;
}

/**
 * Basic statistics for a numeric array:
 * - min, max, mean
 * - p50 (median-ish) and p90 (90th percentile)
 *
 * Returns zeros when the input array is empty.
 */
function stats(arr: number[]) {
    if (arr.length === 0) {
        return { min: 0, max: 0, mean: 0, p50: 0, p90: 0 };
    }

    const min = Math.min(...arr);
    const max = Math.max(...arr);
    const mean = arr.reduce((a, b) => a + b, 0) / arr.length;
    const sorted = [...arr].sort((a, b) => a - b);

    const quantile = (p: number) => {
        const idx = Math.min(sorted.length - 1, Math.floor(p * (sorted.length - 1)));
        return sorted[idx];
    };

    return { min, max, mean, p50: quantile(0.5), p90: quantile(0.9) };
}

/**
 * Read peer-count series from collator log files, print a short report
 * for each, and assert that “1 peer” is not the majority of samples.
 *
 * @param baseDir       Directory where the collator log files live.
 * @param collatorNames Names of collators (without ".log"), e.g. ["Collator2000-01", "Collator2000-02"].
 * @param nodeLabel     Node label passed to `collectSeries` and used in log output (e.g. "Container-2000").
 */
export async function analyzeCollatorPeers(baseDir: string, collatorNames: string[], nodeLabel: string): Promise<void> {
    for (const name of collatorNames) {
        const path = `${base}/${name}.log`;
        const { peers, times } = await collectSeries(path, nodeLabel);

        expect(peers.length, `${name}: no 'Idle (N peers)' lines found in ${path}`).to.be.greaterThan(0);

        const st = stats(peers);
        const ones = peers.filter((n) => n === 1).length;
        const zeros = peers.filter((n) => n === 0).length;
        const frac1 = ones / peers.length;
        const frac0 = zeros / peers.length;

        const firstTs = times[0] || "(unknown start)";
        const lastTs = times[times.length - 1] || "(unknown end)";

        console.log(`\n[${name}] peers over time (${peers.length} samples)`);
        console.log(`[${name}] window: ${firstTs}  →  ${lastTs}`);
        console.log(
            `[${name}] min=${st.min}, max=${st.max}, mean=${st.mean.toFixed(
                2
            )}, p50=${st.p50}, p90=${st.p90} | 1-peer=${(frac1 * 100).toFixed(
                1
            )}% (${ones}/${peers.length}) | 0-peer=${(frac0 * 100).toFixed(1)}% (${zeros}/${peers.length})`
        );
        console.log(`[${name}] sparkline:\n${sparkline(peers)}\n`);

        expect(
            frac1,
            `${name}: majority of samples report exactly 1 peer (${ones}/${peers.length} = ${(frac1 * 100).toFixed(
                1
            )}%). Expected ≤ 50%.`
        ).to.be.at.most(0.5);
    }
}

// (log file, node kind) -> list of discovered addresses (unique, in order seen)
type DiscoveryMap = Map<string, Map<string, string[]>>;

// file -> kind -> Set<port>
type PortMap = Map<string, Map<string, Set<number>>>;

/**
 * Scan all *.log files in `baseDir` for
 * "Discovered new external address for our node: <multiaddr>" lines.
 *
 * Returns:
 *   filePath -> nodeKind -> [multiaddr, ...]   (unique per kind, in order seen)
 */
async function buildDiscoveryMap(baseDir: string): Promise<DiscoveryMap> {
    const { readFile, readdir } = await import("node:fs/promises");

    const map: DiscoveryMap = new Map();
    const names = await readdir(baseDir);
    const files = names.filter((n) => n.endsWith(".log")).map((n) => `${baseDir}/${n}`);

    // Example line:
    // 2025-10-16 12:38:12 [Container-2000] 🔍 Discovered new external address for our node: /ip4/127.0.0.1/tcp/46873/ws/p2p/...
    const re =
        /^\s*\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}(?:\.\d+)?[^\[]*\[([^\]]+)\].*?Discovered new external address for our node:\s*(\S+)/gim;

    for (const file of files) {
        let txt: string;
        try {
            txt = await readFile(file, "utf8");
        } catch {
            continue;
        }

        for (const m of txt.matchAll(re)) {
            const nodeKind = m[1]; // e.g. "Container-2000", "Parachain", "Relaychain"
            const addr = m[2]; // multiaddr

            let inner = map.get(file);
            if (!inner) {
                inner = new Map<string, string[]>();
                map.set(file, inner);
            }

            let list = inner.get(nodeKind);
            if (!list) {
                list = [];
                inner.set(nodeKind, list);
            }

            if (!list.includes(addr)) {
                list.push(addr);
            }
        }
    }

    return map;
}

function printDiscoverySummary(discovered: DiscoveryMap): void {
    for (const [file, byKind] of discovered) {
        for (const [kind, addrs] of byKind) {
            console.log(`[Discovery] ${file} [${kind}] (${addrs.length} addrs):\n  ${addrs.join("\n  ")}`);
        }
    }
}

/**
 * Extract all tcp/udp port numbers from a multiaddr string.
 * e.g. ".../tcp/46873/ws..." or ".../udp/30333/quic-v1/..." => [46873] / [30333]
 */
function extractPorts(addr: string): number[] {
    const out: number[] = [];
    for (const m of addr.matchAll(/\/(?:tcp|udp)\/(\d+)\b/gi)) {
        const n = Number.parseInt(m[1] ?? "", 10);
        if (Number.isFinite(n)) out.push(n);
    }
    return out;
}

/**
 * From the discovery map, build:
 *   - portMap: file -> kind -> Set<port>
 *   - ownersByPort: port -> Set<"file [kind]">
 */
function buildPortMaps(discovered: DiscoveryMap): {
    portMap: PortMap;
    ownersByPort: Map<number, Set<string>>;
} {
    const portMap: PortMap = new Map();
    const ownersByPort: Map<number, Set<string>> = new Map();

    for (const [file, byKind] of discovered) {
        let kindsMap = portMap.get(file);
        if (!kindsMap) {
            kindsMap = new Map<string, Set<number>>();
            portMap.set(file, kindsMap);
        }

        for (const [kind, addrs] of byKind) {
            let portSet = kindsMap.get(kind);
            if (!portSet) {
                portSet = new Set<number>();
                kindsMap.set(kind, portSet);
            }

            for (const addr of addrs) {
                const ports = extractPorts(addr);
                for (const p of ports) {
                    portSet.add(p);

                    const owner = `${file} [${kind}]`;
                    let owners = ownersByPort.get(p);
                    if (!owners) {
                        owners = new Set<string>();
                        ownersByPort.set(p, owners);
                    }
                    owners.add(owner);
                }
            }
        }
    }

    return { portMap, ownersByPort };
}

function printPortSummary(portMap: PortMap): void {
    for (const [file, kinds] of portMap) {
        for (const [kind, ports] of kinds) {
            const sorted = [...ports].sort((a, b) => a - b);
            console.log(`[DiscoveryPorts] ${file} [${kind}] (${sorted.length} ports): ${sorted.join(", ")}`);
        }
    }
}

/**
 * Assert that no TCP/UDP port is reused by more than one (file, kind) owner.
 * Fails the test with a clear message listing all owners of the colliding port.
 */
function assertUniquePorts(ownersByPort: Map<number, Set<string>>): void {
    for (const [port, owners] of ownersByPort) {
        if (owners.size > 1) {
            const list = [...owners].join(" , ");
            expect.fail(`Port ${port} reused by multiple nodes: ${list}`);
        }
    }
}

/**
 * End-to-end check:
 *   1. Load discovery lines from all *.log files in baseDir.
 *   2. Print a human-readable summary (addresses + ports).
 *   3. Assert that no TCP/UDP port is reused by multiple nodes.
 */
export async function assertNoPortCollisionsInDiscoveryLogs(baseDir: string): Promise<void> {
    const discovered = await buildDiscoveryMap(baseDir);

    // Optional summaries; keep or remove as you prefer.
    printDiscoverySummary(discovered);

    const { portMap, ownersByPort } = buildPortMaps(discovered);
    printPortSummary(portMap);

    assertUniquePorts(ownersByPort);
}

describeSuite({
    id: "ZOMBIETANSSICP01",
    title: "Zombie Tanssi Collator Peers Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

            const paraNetwork = paraApi.consts.system.version.specName.toString();
            const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
            expect(paraNetwork, "Para API incorrect").to.contain("dancebox");
            expect(paraId1000, "Para API incorrect").to.be.equal("1000");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);

            expect(header2000.number.toNumber()).to.be.equal(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async () => {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Set config params",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Disable rotation
                const tx1 = paraApi.tx.configuration.setFullRotationPeriod(0);
                const fillAmount = 990_000_000; // equal to 99% Perbill
                const tx2 = paraApi.tx.configuration.setMaxParachainCoresPercentage(fillAmount);
                const txBatch = paraApi.tx.utility.batchAll([tx1, tx2]);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(txBatch), alice);
            },
        });

        it({
            id: "T03",
            title: "Test assignation did not change",
            test: async () => {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                // TODO: fix once we have types
                const allCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                const expectedAllCollators = {
                    orchestratorChain: [
                        getKeyringNimbusIdHex("Collator1000-01"),
                        getKeyringNimbusIdHex("Collator1000-02"),
                        getKeyringNimbusIdHex("Collator1000-03"),
                    ],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator2000-02")],
                    },
                };

                expect(allCollators).to.deep.equal(expectedAllCollators);
            },
        });

        it({
            id: "T04",
            title: "Blocks are being produced on container 2000",
            test: async () => {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T06",
            title: "Test container chain 2000 assignation is correct",
            test: async () => {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                // TODO: fix once we have types
                const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T08",
            title: "Test author noting is correct for both containers",
            timeout: 60000,
            test: async () => {
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();

                // TODO: fix once we have types
                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];

                await context.waitBlock(3, "Tanssi");
                const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
            },
        });

        it({
            id: "T09",
            title: "Test author is correct in Orchestrator",
            test: async () => {
                const sessionIndex = (await paraApi.query.session.currentIndex()).toNumber();
                const authorities = await paraApi.query.authorityAssignment.collatorContainerChain(sessionIndex);
                const author = await getAuthorFromDigest(paraApi);
                // TODO: fix once we have types
                expect(authorities.toJSON().orchestratorChain.includes(author.toString())).to.be.true;
            },
        });

        it({
            id: "T10",
            title: "Test frontier template isEthereum",
            test: async () => {
                // TODO: fix once we have types
                const genesisData2000 = await paraApi.query.registrar.paraGenesisData(2000);
                expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
            },
        });

        it({
            id: "T11",
            title: "Gather logs: Discovered new external address",
            test: async () => {
                const baseDir = getTmpZombiePath();
                await assertNoPortCollisionsInDiscoveryLogs(baseDir);
            },
        });

        it({
            id: "T12",
            title: "Wait 2 sessions",
            timeout: 600000,
            test: async () => {
                await waitSessions(context, relayApi, 3, null, "Tanssi");
            },
        });

        it({
            id: "T13",
            title: "Peers over time for container collators (fail if majority == 1) + block authors list",
            timeout: 240000,
            test: async () => {
                // Print block authors list for Container-2000
                // This is to detect issues that affect a single collator: we expect to see both collators producing
                // blocks. If only one collator is producing blocks, then it means that the other collator crashed or
                // failed to start, so it is expected to see that the other one only has 1 peer.
                await printBlockAuthorsWindow(container2000Api, "Container-2000");

                // Parse collator logs and get the number of peers over time from there
                // This will fail the test if both collators only have 1 peer
                const baseDir = getTmpZombiePath();
                const collators = ["Collator2000-01", "Collator2000-02"];
                const nodeLabel = "Container-2000";
                await analyzeCollatorPeers(baseDir, collators, nodeLabel);
            },
        });

        it({
            id: "T15",
            title: "Check Collator2000-02.log to ensure shutdown error bug is fixed",
            timeout: 300000,
            test: async () => {
                const logFilePath = `${getTmpZombiePath()}/Collator2000-02.log`;
                await checkLogsNotExist(logFilePath, [
                    "Entering off-chain worker.",
                    "Shutdown error",
                    "Timeout when waiting for paritydb lock",
                    "Error waiting for chain",
                    "Failed to start container chain",
                    "Shutting down container chain service",
                ]);
            },
        });
    },
});
