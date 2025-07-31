import { expect } from "vitest";
import fs from "node:fs/promises";
import { Keyring, type ApiPromise } from "@polkadot/api";
import { exec } from "node:child_process";
import { getAuthorFromDigestRange, getHeaderFromRelay, signAndSendAndInclude, sleep } from "utils";
import type { PathLike } from "node:fs";
import type { ZombieTestContext } from "@moonwall/types/dist/types/runner";

// Read log file path and check that none of the specified logs are found.
// Only supports single-line logs.
export async function checkLogsNotExist(logFilePath: string, logs: string[]): Promise<void> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    for (let i = 0; i < lines.length; i++) {
        for (const log of logs) {
            if (lines[i].includes(log)) {
                // In case any log is found, show some context around the found log
                const contextSize = 3;
                const contextStart = Math.max(0, i - contextSize);
                const contextEnd = Math.min(lines.length - 1, i + contextSize);
                const contextLines = lines.slice(contextStart, contextEnd + 1);
                const contextStr = contextLines.join("\n");

                expect.fail(
                    `Log entry '${log}' was found in the log file.\nContext around the found log:\n${contextStr}`
                );
            }
        }
    }
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
export async function checkLogs(logFilePath: string, logs: string[]): Promise<void> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    let logIndex = 0;
    let lastFoundLogIndex = 0;

    for (let i = 0; i < lines.length; i++) {
        if (logIndex < logs.length && lines[i].includes(logs[logIndex])) {
            logIndex++;
            lastFoundLogIndex = i;
        }

        if (logIndex === logs.length) {
            break;
        }
    }

    if (logIndex !== logs.length) {
        // In case of missing logs, show some context around the last found log
        const contextSize = 3;
        const contextStart = Math.max(0, lastFoundLogIndex - contextSize);
        const contextEnd = Math.min(lines.length - 1, lastFoundLogIndex + contextSize);
        const contextLines = lines.slice(contextStart, contextEnd + 1);
        const contextStr = contextLines.join("\n");

        expect.fail(
            `Not all logs were found in the correct order. Missing log: '${logs[logIndex]}'\nContext around the last found log:\n${contextStr}`
        );
    }
}

// Same as `checkLogs` but return true if all logs are found in order, and false otherwise
export async function checkLogsNoFail(logFilePath: string, logs: string[]): Promise<boolean> {
    const fileContent = await fs.readFile(logFilePath, "utf8");
    const lines = fileContent.split("\n");

    let logIndex = 0;

    for (let i = 0; i < lines.length; i++) {
        if (logIndex < logs.length && lines[i].includes(logs[logIndex])) {
            logIndex++;
        }

        if (logIndex === logs.length) {
            break;
        }
    }

    return logIndex === logs.length;
}

export async function waitForLogs(logFilePath: string, timeout: number, logs: string[]): Promise<boolean> {
    for (let i = 0; i < timeout; i++) {
        if (await checkLogsNoFail(logFilePath, logs)) {
            return true;
        }

        await sleep(1000);
    }

    return false;
}

// Returns the /tmp/zombie-52234... path
export function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

// Verify that the next `numBlocks` have no more than `numAuthors` different authors
//
// Concepts: blocks and slots.
// A slot is a time-based period where one author can propose a block.
// Block numbers are always consecutive, but some slots may have no block.
// One session consists of a fixed number of blocks, but a variable number of slots.
//
// We want to ensure that all the eligible block authors are trying to propose blocks.
//
// If the authority set changes between `blockStart` and `blockEnd`, this test returns an error.
export async function countUniqueBlockAuthors(
    paraApi: ApiPromise,
    sessionPeriod: number,
    blockStart: number,
    blockEnd: number,
    numAuthors: number
) {
    expect(blockEnd, "Called countUniqueBlockAuthors with empty block range").toBeGreaterThan(blockStart);
    // If the expected numAuthors is greater than the session length, it is possible for some authors to never have a
    // chance to produce a block, in that case this test will fail.
    // This test can also fail if the values are close, because collators sometimes fail to produce a block.
    // For optimal results use a value of `numAuthors` that is much smaller than `sessionPeriod`.
    expect(numAuthors).toBeLessThanOrEqual(sessionPeriod);
    // If the authority set changes at any point, the assumption that numAuthors === authorities.len is not valid:
    // we can always have 1 collator assigned to this chain, but if the authority set changes once in the middle of this
    // test, we will see 2 different block authors. We detect that and return an error, the caller is expected to avoid
    // this case by passing a different block range.
    const authoritiesBySession = await fetchAuthoritySetChanges(paraApi, sessionPeriod, blockStart, blockEnd);
    // If there's more than one set of authorities, it means there was a change
    expect(
        authoritiesBySession.size,
        `Authority set did change in the block range passed to countUniqueBlockAuthors, the results will not be consistent. Authority sets: ${formatAuthoritySets(
            authoritiesBySession
        )}`
    ).toBe(1);
    const actualAuthors = [];
    const blockNumbers = [];

    const authors = await getAuthorFromDigestRange(paraApi, blockStart, blockEnd);
    for (let i = 0; i < authors.length; i++) {
        const [blockNum, author] = authors[i];
        blockNumbers.push(blockNum);
        actualAuthors.push(author);
    }

    const uniq = [...new Set(actualAuthors)];

    if (uniq.length > numAuthors || (uniq.length === 1 && numAuthors > 1)) {
        console.error(
            "Mismatch between authorities and actual block authors: authorities: ",
            formatAuthoritySets(authoritiesBySession),
            "",
            actualAuthors,
            ", block numbers: ",
            blockNumbers,
            `uniq.length=${uniq.length}, numAuthors=${numAuthors}`
        );
        expect(false).to.be.true;
    }
}

// Verify that the next `numBlocks` have exactly `numAuthors` different authors
export async function countUniqueBlockAuthorsExact(
    paraApi: ApiPromise,
    blockStart: number,
    blockEnd: number,
    numAuthors: number,
    authorities: string[]
) {
    const actualAuthors = [];
    const blockNumbers = [];

    const authors = await getAuthorFromDigestRange(paraApi, blockStart, blockEnd);
    for (let i = 0; i < authors.length; i++) {
        const [blockNum, author] = authors[i];
        blockNumbers.push(blockNum);
        actualAuthors.push(author);
    }

    const uniq = [...new Set(actualAuthors)];

    if (uniq.length !== numAuthors) {
        console.error(
            "Mismatch between authorities and actual block authors: authorities: ",
            authorities,
            ", actual authors: ",
            actualAuthors,
            ", block numbers: ",
            blockNumbers
        );
        expect(false).to.be.true;
    }
}

// Returns the initial set of authorities at `blockStart`, and any different sets of authorities if they changed before
// `blockEnd`, in a map indexed by session number.
export async function fetchAuthoritySetChanges(
    paraApi: ApiPromise,
    sessionPeriod: number,
    blockStart: number,
    blockEnd: number
): Promise<Map<number, any>> {
    const authoritiesBySession = new Map<number, any>();
    let lastAuthorities: any = null;

    for (let blockNum = blockStart; blockNum <= blockEnd; blockNum += sessionPeriod) {
        const blockHash = await paraApi.rpc.chain.getBlockHash(blockNum);
        const apiAt = await paraApi.at(blockHash);
        const session = (await apiAt.query.session.currentIndex()).toNumber();
        const authorities = (await apiAt.query.authorityAssignment.collatorContainerChain(session)).toJSON();

        // If this is the first iteration or if the authorities have changed
        if (!lastAuthorities || JSON.stringify(lastAuthorities) !== JSON.stringify(authorities)) {
            authoritiesBySession.set(session, authorities);
        }

        lastAuthorities = authorities;
    }

    return authoritiesBySession;
}

export function formatAuthoritySets(authoritiesBySession: Map<number, any>): string {
    let logString = "";

    authoritiesBySession.forEach((authorities, session) => {
        logString += `Session ${session} authorities:\n${JSON.stringify(authorities, null, 4)}`;
    });

    return logString;
}

export const findCollatorProcessPid = async (collatorName: string) => {
    const pattern = `(tanssi-node.*${collatorName})`;
    const cmd = `ps aux | grep -E "${pattern}"`;
    const { stdout } = await execPromisify(cmd);
    const processes = stdout
        .split("\n")
        .filter((line) => line && !line.includes("grep -E"))
        .map((line) => {
            const parts = line.split(/\s+/);
            const pid = parts[1];
            const command = parts.slice(10).join(" ");
            return {
                name: `PID: ${pid}, Command: ${command}`,
                value: pid,
            };
        });

    if (processes.length === 1) {
        return processes[0].value; // return pid
    }
    const error = {
        message: "Multiple processes found.",
        processes: processes.map((p) => p.name),
    };
    throw error;
};

export const findValidatorProcessPid = async (collatorName: string) => {
    const pattern = `(tanssi-relay.*${collatorName})`;
    const cmd = `ps aux | grep -E "${pattern}"`;
    const { stdout } = await execPromisify(cmd);
    const processes = stdout
        .split("\n")
        .filter((line) => line && !line.includes("grep -E"))
        .map((line) => {
            const parts = line.split(/\s+/);
            const pid = parts[1];
            const command = parts.slice(10).join(" ");
            return {
                name: `PID: ${pid}, Command: ${command}`,
                value: pid,
            };
        });

    if (processes.length === 1) {
        return processes[0].value; // return pid
    }
    const error = {
        message: "Multiple processes found.",
        processes: processes.map((p) => p.name),
    };
    throw error;
};

export function isProcessRunning(pid: number): boolean {
    try {
        // The `kill` function with signal 0 does not terminate the process
        // but will throw an error if the process does not exist.
        process.kill(pid, 0);
        return true;
    } catch (error) {
        if (error.code === "EPERM") {
            // The error code 'EPERM' means the process exists but we don't have permission to send the signal.
            return true;
        }
        return false;
    }
}

const execPromisify = (command: string) => {
    return new Promise<{ stdout: string; stderr: string }>((resolve, reject) => {
        exec(command, (error, stdout, stderr) => {
            if (error) {
                reject(error);
            } else {
                resolve({ stdout, stderr });
            }
        });
    });
};

export async function directoryExists(directoryPath: PathLike) {
    try {
        await fs.access(directoryPath, fs.constants.F_OK);
        return true;
    } catch (err) {
        return false;
    }
}

export const getCommonTests = (
    context: ZombieTestContext,
    relayApi: ApiPromise,
    containerIds: number[],
    containerApis: ApiPromise[]
) => {
    const ethersSigner = context.ethers();
    const checkBlockProductionForContainerChains = (containerApis: ApiPromise[]) => {
        const result = [];
        for (const api of containerApis) {
            result.push({
                title: "Blocks are being produced on container 2001",
                test: async () => {
                    const blockNum = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                    expect(blockNum).to.be.greaterThan(0);
                    expect(await ethersSigner.provider.getBlockNumber(), "Safe tag is not present").to.be.greaterThan(
                        0
                    );
                },
            });
        }

        return result;
    };

    const checkChainAssignation = (containerApis: ApiPromise[]) => {
        const result = [];
        for (const api of containerApis) {
            result.push({
                title: "Test container chain 2000 assignation is correct",
                test: async () => {
                    const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                    const paraId = (await api.query.parachainInfo.parachainId()).toString();
                    const containerChainCollators = (
                        await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                    ).toJSON().containerChains[paraId];

                    const writtenCollators = (await api.query.authoritiesNoting.authorities()).toJSON();

                    expect(containerChainCollators).to.deep.equal(writtenCollators);
                },
            });
        }

        return result;
    };

    return [
        {
            id: "T01",
            title: "Test block numbers in relay are 0 yet",
            test: async () => {
                for (const containerId of containerIds) {
                    const header = await getHeaderFromRelay(relayApi, containerId);
                    expect(header.number.toNumber()).to.be.equal(0);
                }
            },
        },
        {
            id: "T02",
            title: "Blocks are being produced on tanssi-relay",
            test: async () => {
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        },
        {
            id: "T03",
            title: "Set config params",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Disable rotation
                const tx1 = relayApi.tx.collatorConfiguration.setFullRotationPeriod(0);
                const fillAmount = 990_000_000; // equal to 99% Perbill
                const tx2 = relayApi.tx.collatorConfiguration.setMaxParachainCoresPercentage(fillAmount);
                const txBatch = relayApi.tx.utility.batchAll([tx1, tx2]);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(txBatch), alice);
            },
        },
        {
            id: "T04",
            title: "Test assignation did not change",
            test: async () => {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const allCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                expect(allCollators.orchestratorChain.length).to.equal(0);
                expect(allCollators.containerChains["2000"].length).to.equal(2);
                expect(allCollators.containerChains["2001"].length).to.equal(2);
            },
        },
        ...checkBlockProductionForContainerChains(containerApis),
        ...checkChainAssignation(containerApis),
        {
            id: "T09",
            title: "Test author noting is correct for both containers",
            timeout: 60000,
            test: async () => {
                const assignment = await relayApi.query.tanssiCollatorAssignment.collatorContainerChain();
                const paraIds = [];
                const paraCollators = [];
                for (const containerApi of containerApis) {
                    const paraId = await containerApi.query.parachainInfo.parachainId();
                    paraIds.push(paraId);
                    paraCollators.push(assignment.containerChains.toJSON()[paraId.toString()]);
                }

                await context.waitBlock(6, "Tanssi-relay");

                for (const paraId of paraIds) {
                    const author2000 = await relayApi.query.authorNoting.latestAuthor(paraId);
                    expect(paraCollators.includes(author2000.toJSON().author)).to.be.true;
                }
            },
        },
        {
            id: "T10",
            title: "Test frontier template isEthereum",
            test: async () => {
                const genesisData2000 = await relayApi.query.containerRegistrar.paraGenesisData(2000);
                expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
                const genesisData2001 = await relayApi.query.containerRegistrar.paraGenesisData(2001);
                expect(genesisData2001.toJSON().properties.isEthereum).to.be.true;
            },
        },
    ];
};
