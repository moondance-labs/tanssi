// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    getAuthorFromDigestRange,
    checkLogsNotExist,
    getAuthorFromDigest,
    getHeaderFromRelay,
    getKeyringNimbusIdHex,
    getTmpZombiePath,
    waitSessions,
    signAndSendAndInclude,
} from "utils";

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
                // Parse "Discovered new external address" lines across all logs
                const { readFile, readdir } = await import("node:fs/promises");

                // (log file, node kind) -> list of discovered addresses (unique, in order seen)
                type DiscoveryMap = Map<string, Map<string, string[]>>;

                const buildDiscoveryMap = async (baseDir: string): Promise<DiscoveryMap> => {
                    const map: DiscoveryMap = new Map();
                    const names = await readdir(baseDir);
                    const files = names.filter((n) => n.endsWith(".log")).map((n) => `${baseDir}/${n}`);

                    // Example line:
                    // 2025-10-16 12:38:12 [Container-2000] 🔍 Discovered new external address for our node: /ip4/127.0.0.1/tcp/46873/ws/p2p/...
                    const re =
                        /^\s*\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}(?:\.\d+)?[^\[]*\[([^\]]+)\].*?Discovered new external address for our node:\s*(\S+)/gim;

                    for (const file of files) {
                        let txt = "";
                        try {
                            txt = await readFile(file, "utf8");
                        } catch {
                            continue;
                        }

                        // If `re` is global (/g or /y), reset lastIndex before reusing it.
                        if ("lastIndex" in re) re.lastIndex = 0;

                        let m: RegExpExecArray | null;
                        while (true) {
                            m = re.exec(txt);
                            if (m === null) break;

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

                            if (!list.includes(addr)) list.push(addr);
                        }
                    }
                    return map;
                };

                const base = getTmpZombiePath();
                const discoveredAddressMap = await buildDiscoveryMap(base);

                // Optional: print a readable summary
                for (const [file, byKind] of discoveredAddressMap) {
                    for (const [kind, addrs] of byKind) {
                        console.log(`[Discovery] ${file} [${kind}] (${addrs.length} addrs):\n  ${addrs.join("\n  ")}`);
                    }
                }

                // ---- Extract TCP/UDP ports from multiaddrs, build port maps, assert uniqueness ----
                const extractPorts = (addr: string): number[] => {
                    const out: number[] = [];
                    // capture all tcp/udp segments, e.g. .../tcp/46873/ws..., .../udp/30333/quic-v1/...
                    for (const m of addr.matchAll(/\/(?:tcp|udp)\/(\d+)\b/gi)) {
                        const n = Number.parseInt(m[1] ?? "", 10);
                        if (Number.isFinite(n)) out.push(n);
                    }
                    return out;
                };

                // file -> kind -> Set<port>
                const portMap: Map<string, Map<string, Set<number>>> = new Map();
                // port -> Set<ownerPair> (ownerPair = `${file} [${kind}]`)
                const ownersByPort: Map<number, Set<string>> = new Map();

                for (const [file, byKind] of discoveredAddressMap) {
                    let kindsMap = portMap.get(file);
                    if (!kindsMap) {
                        kindsMap = new Map(); // Map<Kind, Set<number>>
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
                                let set = ownersByPort.get(p);
                                if (!set) {
                                    set = new Set<typeof owner>();
                                    ownersByPort.set(p, set);
                                }
                                set.add(owner);
                            }
                        }
                    }
                }

                // Optional: print a summary of discovered ports per (file, kind)
                for (const [file, kinds] of portMap) {
                    for (const [kind, ports] of kinds) {
                        const sorted = [...ports].sort((a, b) => a - b);
                        console.log(
                            `[DiscoveryPorts] ${file} [${kind}] (${sorted.length} ports): ${sorted.join(", ")}`
                        );
                    }
                }

                // Assertion A: within the same file, no port reused by different node kinds
                for (const [file, kinds] of portMap) {
                    const ownerByPort = new Map<number, string>();
                    for (const [kind, ports] of kinds) {
                        for (const p of ports) {
                            const prev = ownerByPort.get(p);
                            if (prev && prev !== kind) {
                                expect.fail(`Port collision in ${file}: port ${p} used by [${prev}] and [${kind}]`);
                            }
                            ownerByPort.set(p, kind);
                        }
                    }
                }

                // Assertion B: across files, no ports in common
                const files = [...portMap.keys()];
                for (let i = 0; i < files.length; i++) {
                    const fi = files[i];
                    const portsI = new Set<number>();
                    const setsI = portMap.get(fi);
                    for (const set of setsI.values()) {
                        for (const p of set) {
                            portsI.add(p);
                        }
                    }

                    for (let j = i + 1; j < files.length; j++) {
                        const fj = files[j];
                        const portsJ = new Set<number>();
                        const setsJ = portMap.get(fj);
                        if (setsJ) {
                            for (const set of setsJ.values()) {
                                for (const p of set) {
                                    portsJ.add(p);
                                }
                            }
                        }

                        for (const p of portsI) {
                            if (portsJ.has(p)) {
                                expect.fail(`Port collision across files: port ${p} appears in both ${fi} and ${fj}`);
                            }
                        }
                    }
                }

                // Bonus single-pass global check (covers both A & B). Comment out if you prefer the tailored messages above.
                // for (const [p, owners] of ownersByPort) {
                //     if (owners.size > 1) {
                //         expect.fail(`Port ${p} reused by multiple nodes: ${[...owners].join(" , ")}`);
                //     }
                // }
            },
        });

        it({
            id: "T13",
            title: "Wait 2 sessions",
            timeout: 600000,
            test: async () => {
                await waitSessions(context, relayApi, 3, null, "Tanssi");
            },
        });
        it({
            id: "T12",
            title: "Peers over time for container collators (fail if majority == 1) + block authors list",
            timeout: 240000,
            test: async () => {
                const { readFile } = await import("node:fs/promises");

                const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

                // -------- Peer series helpers --------
                const parsePeerSeries = (txt: string) => {
                    const peers: number[] = [];
                    const times: string[] = [];
                    const lines = txt.split(/\r?\n/);
                    const rePeers = /Idle\s*\(\s*(\d+)\s*peers?\s*\)/i;

                    for (const line of lines) {
                        if (!line.includes("[Container-2000]") || !line.includes("Idle")) continue;
                        const m = rePeers.exec(line);
                        if (!m) continue;
                        const n = Number.parseInt(m[1], 10);
                        if (!Number.isNaN(n)) {
                            peers.push(n);
                            const bracket = line.indexOf("[");
                            const ts = bracket > 0 ? line.slice(0, bracket).trim() : "";
                            times.push(ts);
                        }
                    }
                    return { peers, times };
                };

                const sparkline = (nums: number[]) => {
                    if (nums.length === 0) return "";
                    const blocks = "▁▂▃▄▅▆▇█";
                    const target = 120;
                    const downsample = (arr: number[], width: number) => {
                        if (arr.length <= width) return arr;
                        const size = Math.ceil(arr.length / width);
                        const out: number[] = [];
                        for (let i = 0; i < arr.length; i += size) {
                            const chunk = arr.slice(i, i + size);
                            const avg = chunk.reduce((a, b) => a + b, 0) / chunk.length;
                            out.push(avg);
                        }
                        return out;
                    };
                    const data = downsample(nums, target);
                    const min = Math.min(...data);
                    const max = Math.max(...data);
                    if (max === min)
                        return blocks[Math.min(blocks.length - 1, Math.max(0, Math.floor(blocks.length / 2)))].repeat(
                            data.length
                        );
                    return data
                        .map((v) => {
                            const t = (v - min) / (max - min);
                            const idx = Math.max(0, Math.min(blocks.length - 1, Math.round(t * (blocks.length - 1))));
                            return blocks[idx];
                        })
                        .join("");
                };

                const stats = (arr: number[]) => {
                    if (arr.length === 0) return { min: 0, max: 0, mean: 0, p50: 0, p90: 0 };
                    const min = Math.min(...arr);
                    const max = Math.max(...arr);
                    const mean = arr.reduce((a, b) => a + b, 0) / arr.length;
                    const sorted = [...arr].sort((a, b) => a - b);
                    const q = (p: number) => sorted[Math.min(sorted.length - 1, Math.floor(p * (sorted.length - 1)))];
                    return { min, max, mean, p50: q(0.5), p90: q(0.9) };
                };

                const collectSeries = async (logPath: string) => {
                    let tries = 0;
                    let series = { peers: [] as number[], times: [] as string[] };
                    while (tries < 5) {
                        try {
                            const txt = await readFile(logPath, "utf8");
                            series = parsePeerSeries(txt);
                            if (series.peers.length >= 8) break;
                        } catch {
                            // file may not exist yet
                        }
                        await sleep(5000);
                        tries++;
                    }
                    return series;
                };

                const base = getTmpZombiePath();
                const collatorLogs = [
                    { name: "Collator2000-01", path: `${base}/Collator2000-01.log` },
                    { name: "Collator2000-02", path: `${base}/Collator2000-02.log` },
                ];

                // -------- Block authors list (Container-2000) --------
                // Prefer project utility.
                const getAuthorsRange = async (start: number, end: number): Promise<Array<[number, string]>> => {
                    return await getAuthorFromDigestRange(container2000Api, start, end);
                };

                // Fetch block timestamp (ms) at a given height. Try storage first; fallback to scanning extrinsics.
                const fetchTimestampMs = async (num: number): Promise<number | undefined> => {
                    try {
                        const hash = await container2000Api.rpc.chain.getBlockHash(num);
                        try {
                            const ts = await (container2000Api as any).query?.timestamp?.now?.at?.(hash);
                            if (ts && typeof ts.toNumber === "function") return ts.toNumber();
                        } catch {
                            // fall through to extrinsic scan
                        }
                        const blk = await container2000Api.rpc.chain.getBlock(hash);
                        for (const ex of blk.block.extrinsics as any[]) {
                            const m = ex.method;
                            if (m?.section === "timestamp" && m?.method === "set" && m?.args?.length) {
                                const v = m.args[0];
                                const n =
                                    typeof v?.toNumber === "function"
                                        ? v.toNumber()
                                        : Number(v?.toString?.() ?? Number.NaN);
                                if (Number.isFinite(n)) return n;
                            }
                        }
                    } catch {
                        // ignore
                    }
                    return undefined;
                };

                const head = await container2000Api.rpc.chain.getBlock();
                const current = head.block.header.number.toNumber();
                const window = Math.min(60, Math.max(10, current >= 10 ? 40 : current + 1)); // 40 by default, clamp 10..60
                const start = Math.max(1, current - window + 1);
                const end = current;

                const authorPairs = await getAuthorsRange(start, end);

                // Pretty print list + time deltas + a tiny summary
                const byAuthor = new Map<string, number>();
                console.log(`\n[Container-2000] Block authors for #${start}..#${end} (${authorPairs.length} blocks):`);

                // Seed previous timestamp with block (start-1) if available; otherwise first delta will be 0s.
                let prevTs = null;

                const rows: Array<{ n: number; author: string; delta: number }> = [];
                for (const [n, author] of authorPairs) {
                    const ts = await fetchTimestampMs(n);
                    const deltaSec = prevTs != null && ts != null ? Math.max(0, Math.round((ts - prevTs) / 1000)) : 0;

                    rows.push({ n, author, delta: deltaSec });

                    if (ts != null) prevTs = ts;
                    byAuthor.set(author, (byAuthor.get(author) ?? 0) + 1);
                }

                const numWidth = Math.max(...rows.map((r) => r.n)).toString().length; // width for block numbers
                const deltaWidth = Math.max(...rows.map((r) => r.delta), 0).toString().length; // width for delta seconds

                const lines = rows.map(
                    ({ n, author, delta }) =>
                        `#${n.toString().padStart(numWidth)} (+${delta.toString().padStart(deltaWidth)}s): ${author}`
                );

                console.log(lines.join("\n"));
                const entries = [...byAuthor.entries()].sort((a, b) => b[1] - a[1]);
                const summaryLines = entries.map(([a, c]) => `  - ${a}: ${c}`).join("\n") || "  (none)";
                console.log(`[Container-2000] Authors summary:\n${summaryLines}`);

                // -------- Run peers analysis + assertions --------
                for (const { name, path } of collatorLogs) {
                    const { peers, times } = await collectSeries(path);

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
                        `[${name}] min=${st.min}, max=${st.max}, mean=${st.mean.toFixed(2)}, p50=${st.p50}, p90=${st.p90} | 1-peer=${(
                            frac1 * 100
                        ).toFixed(
                            1
                        )}% (${ones}/${peers.length}) | 0-peer=${(frac0 * 100).toFixed(1)}% (${zeros}/${peers.length})`
                    );
                    console.log(`[${name}] sparkline:\n${sparkline(peers)}\n`);

                    expect(
                        frac1,
                        `${name}: majority of samples report exactly 1 peer (${ones}/${peers.length} = ${(frac1 * 100).toFixed(1)}%). Expected ≤ 50%.`
                    ).to.be.at.most(0.5);
                }

                // Optional cross-check: current RPC health peers for Container2000 (non-fatal)
                try {
                    const health: any = await container2000Api.rpc.system.health();
                    const peersNow =
                        typeof health?.peers?.toNumber === "function"
                            ? health.peers.toNumber()
                            : (health?.toJSON?.().peers ?? 0);
                    console.log(`[Container2000 RPC] current peers=${peersNow}`);
                } catch {
                    // ignore if not available
                }
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
