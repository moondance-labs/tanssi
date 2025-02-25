import { expect } from "vitest";
import fs from "node:fs/promises";
import type { ApiPromise } from "@polkadot/api";
import { exec } from "node:child_process";
import { getAuthorFromDigestRange } from "utils";

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

/// Returns the /tmp/zombie-52234... path
export function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

/// Verify that the next `numBlocks` have no more than `numAuthors` different authors
///
/// Concepts: blocks and slots.
/// A slot is a time-based period where one author can propose a block.
/// Block numbers are always consecutive, but some slots may have no block.
/// One session consists of a fixed number of blocks, but a variable number of slots.
///
/// We want to ensure that all the eligible block authors are trying to propose blocks.
///
/// If the authority set changes between `blockStart` and `blockEnd`, this test returns an error.
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

/// Verify that the next `numBlocks` have exactly `numAuthors` different authors
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

export async function directoryExists(directoryPath) {
    try {
        await fs.access(directoryPath, fs.constants.F_OK);
        return true;
    } catch (err) {
        return false;
    }
}
