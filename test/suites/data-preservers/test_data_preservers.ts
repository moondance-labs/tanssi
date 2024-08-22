import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex, stringToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";
import fs from "fs/promises";

describeSuite({
    id: "DP01",
    title: "Data Preservers Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
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
            test: async function () {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Data preservers watcher properly starts",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver.log";
                await waitForLogs(logFilePath, 300, ["Assignement for block"]);
            },
        });

        it({
            id: "T03",
            title: "Change assignment",
            test: async function () {
                const logFilePath = getTmpZombiePath() + "/DataPreserver.log";
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const profile = {
                    url: "exemple",
                    paraIds: "AnyParaId",
                    mode: { rpc: { supportsEthereumRpc: false }},
                };
                
                {
                    const tx = paraApi.tx.dataPreservers.forceCreateProfile(profile, alice.address);
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                {
                    const tx = paraApi.tx.dataPreservers.forceStartAssignment(0, 2000, "Free");
                    await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                    await context.waitBlock(1, "Tanssi");
                }

                await waitForLogs(logFilePath, 300, ["Active(Id(2000))"]);
            },
        });
    },
});

// Checks every second the log file to find the watcher best block notification until it is found or
// timeout is reached.
async function waitForLogs(logFilePath: string, timeout: number, logs: string[]): Promise<void> {
    for (let i = 0; i < timeout; i++) {
        if (checkLogsNoFail(logFilePath, logs)) {
            return;
        }

        await delay(1000);
    }

    expect.fail(
        `RPC Assignment Watch log was not found after ${timeout} seconds.`
    );
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
async function checkLogsNoFail(logFilePath: string, logs: string[]): Promise<boolean> {
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

    return (logIndex === logs.length);
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
async function checkLogs(logFilePath: string, logs: string[]): Promise<void> {
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

// Read log file path and check that none of the specified logs are found.
// Only supports single-line logs.
async function checkLogsNotExist(logFilePath: string, logs: string[]): Promise<void> {
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

async function directoryExists(directoryPath) {
    try {
        await fs.access(directoryPath, fs.constants.F_OK);
        return true;
    } catch (err) {
        return false;
    }
}

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

const delay = ms => new Promise(res => setTimeout(res, ms));
