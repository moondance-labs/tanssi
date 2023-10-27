import { afterAll, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";
import { exec } from "child_process";
import { ExecaChildProcess, execa } from "execa";

describeSuite({
    id: "ZK01",
    title: "Zombie Tanssi KeepDb Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        const restartedHandles: Array<ExecaChildProcess<string>> = [];

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

        afterAll(async () => {
            for (const h of restartedHandles) {
                h.kill();
            }
        });

        const runZombienetRestart = async (pid: number): Promise<void> => {
            // Wait 10 seconds to have enough time to check if db exists
            // Need to use `pnpm tsx` instead of `pnpm run` to ensure that the process gets killed properly
            const handle = execa(
                "pnpm",
                ["tsx", "scripts/zombienetRestart.ts", "restart", "--wait-ms", "10000", "--pid", pid.toString()],
                {
                    stdio: "inherit",
                }
            );

            restartedHandles.push(handle);
        };

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async function () {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T03",
            title: "Test assignation did not change",
            test: async function () {
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
            test: async function () {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T06",
            title: "Test container chain 2000 assignation is correct",
            test: async function () {
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
            test: async function () {
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
            test: async function () {
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
            test: async function () {
                // TODO: fix once we have types
                const genesisData2000 = await paraApi.query.registrar.paraGenesisData(2000);
                expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
            },
        });

        it({
            id: "T11",
            title: "Test restarting both container chain collators",
            test: async function () {
                const pidCollator200001 = await findCollatorProcessPid("Collator2000-01");
                const pidCollator200002 = await findCollatorProcessPid("Collator2000-02");
                await runZombienetRestart(pidCollator200001);
                await runZombienetRestart(pidCollator200002);

                await sleep(5000);

                // TODO: Check db has not been deleted

                // TODO: Check both collators are still producing blocks
            },
        });

        it({
            id: "T12",
            title: "Test container chain deregister: only nodes without keep-db should delete db",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const registered1 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2000)).to.be.true;

                const tx = paraApi.tx.registrar.deregister(2000);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                await waitSessions(context, paraApi, 2);

                // Check that pending para ids removes 2000
                const registered = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered.toJSON().includes(2000)).to.be.false;

                // TODO: check Collator2000-01 db path exists, and Collator2000-02 has deleted it
            },
        });
    },
});

const sleep = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};

const findCollatorProcessPid = async (collatorName: string) => {
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
    } else {
        const error = {
            message: "Multiple processes found.",
            processes: processes.map((p) => p.name),
        };
        throw error;
    }
};

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
