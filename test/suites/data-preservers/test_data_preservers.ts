import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { signAndSendAndInclude } from "../../util/block";
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
                    mode: { rpc: { supportsEthereumRpc: false } },
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

        it({
            id: "T04",
            title: "RPC endpoint is properly started",
            test: async function () {
                const preserverApi = context.polkadotJs("DataPreserver");
                const container2000Network = preserverApi.consts.system.version.specName.toString();
                const paraId2000 = (await preserverApi.query.parachainInfo.parachainId()).toString();
                expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
                expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");
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

    expect.fail(`RPC Assignment Watch log was not found after ${timeout} seconds.`);
}

// Read log file path and check that all the logs are found in order.
// Only supports single-line logs.
async function checkLogsNoFail(logFilePath: string, logs: string[]): Promise<boolean> {
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

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}

const delay = (ms) => new Promise((res) => setTimeout(res, ms));
