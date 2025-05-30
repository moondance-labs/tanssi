import { afterAll, beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { spawn } from "node:child_process";
import { createWriteStream } from "node:fs";
import {
    countUniqueBlockAuthorsExact,
    directoryExists,
    findCollatorProcessPid,
    getAuthorFromDigest,
    getHeaderFromRelay,
    getKeyringNimbusIdHex,
    getTmpZombiePath,
    isProcessRunning,
    signAndSendAndInclude,
    sleep,
    waitSessions,
} from "utils";

describeSuite({
    id: "ZOMBIET01",
    title: "Zombie Tanssi KeepDb Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let blockNumberOfRestart: number;
        let authoritiesAtRestart: any;
        const restartedHandles = [];

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
            // Kill restared processes
            for (const h of restartedHandles) {
                h.kill();
            }
        });

        const runZombienetRestart = async (pid: number, collatorLogFile: string): Promise<void> => {
            // Wait 10 seconds to have enough time to check if db exists
            // Need to use `pnpm tsx` instead of `pnpm run` to ensure that the process gets killed properly
            const command = "pnpm";
            const args = [
                "tsx",
                "scripts/zombienetRestart.ts",
                "restart",
                "--wait-ms",
                "10000",
                "--pid",
                pid.toString(),
            ];

            const child = spawn(command, args, {
                stdio: ["inherit", "pipe", "pipe"],
            });

            // Pipe both stdout and stderr to the log file
            const log = createWriteStream(collatorLogFile, { flags: "a" });
            child.stdout.pipe(log);
            child.stderr.pipe(log);

            // Handle errors and exit events if needed
            child.on("error", (error) => {
                console.error(`spawn error: ${error}`);
            });

            child.on("exit", (code, signal) => {
                if (code) {
                    console.error(`Child process exited with code ${code}`);
                }
                if (signal) {
                    console.error(`Child process was killed with signal ${signal}`);
                }
            });

            restartedHandles.push(child);
        };

        it({
            id: "T01",
            title: "Blocks are being produced on parachain",
            test: async () => {
                const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
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
            title: "Test restarting both container chain collators",
            test: async () => {
                // Fetch block number before restarting because the RPC may no longer work after the restart
                blockNumberOfRestart = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                // Fetch authorities for a later test
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                authoritiesAtRestart = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();

                const pidCollator200001 = await findCollatorProcessPid("Collator2000-01");
                const pidCollator200002 = await findCollatorProcessPid("Collator2000-02");
                expect(isProcessRunning(pidCollator200001)).to.be.true;
                expect(isProcessRunning(pidCollator200002)).to.be.true;
                await runZombienetRestart(pidCollator200001, `${getTmpZombiePath()}/Collator2000-01.log`);
                await runZombienetRestart(pidCollator200002, `${getTmpZombiePath()}/Collator2000-02.log`);

                await sleep(5000);
                // Check that both collators have been stopped
                expect(isProcessRunning(pidCollator200001)).to.be.false;
                expect(isProcessRunning(pidCollator200002)).to.be.false;

                // Check db has not been deleted
                const dbPath01 = `${getTmpZombiePath()}/Collator2000-01/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const dbPath02 = `${getTmpZombiePath()}/Collator2000-02/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;

                expect(await directoryExists(dbPath01)).to.be.true;
                expect(await directoryExists(dbPath02)).to.be.true;
            },
        });

        it({
            id: "T12",
            title: "Test container chain deregister: only nodes without keep-db should delete db",
            timeout: 300000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const registered1 = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2000)).to.be.true;

                const tx = paraApi.tx.registrar.deregister(2000);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);
                await waitSessions(context, paraApi, 2, async () => {
                    const registered = await paraApi.query.registrar.registeredParaIds();
                    // Stop waiting if 2000 is no longer registered
                    return !registered.toJSON().includes(2000);
                });

                // The node detects assignment when the block is finalized, but "waitSessions" ignores finality.
                // So wait a few blocks more hoping that the current block will be finalized by then.
                await context.waitBlock(6, "Tanssi");

                // Check that pending para ids removes 2000
                const registered = await paraApi.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered.toJSON().includes(2000)).to.be.false;

                // Collator2000-01 db path exists because it was started with `--keep-db`, Collator2000-02 has deleted it
                const dbPath01 = `${getTmpZombiePath()}/Collator2000-01/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const dbPath02 = `${getTmpZombiePath()}/Collator2000-02/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;

                expect(await directoryExists(dbPath01)).to.be.true;
                expect(await directoryExists(dbPath02)).to.be.false;
            },
        });

        it({
            id: "T13",
            title: "Both container chain collators keep producing blocks after restart",
            test: async () => {
                const currentBlock = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                console.log(
                    `Checking block authors for container chain 2000 in range ${blockNumberOfRestart} - ${currentBlock}`
                );
                expect(
                    currentBlock,
                    "container chain 2000 should have produced more than 5 blocks already"
                ).toBeGreaterThan(blockNumberOfRestart + 5);
                await countUniqueBlockAuthorsExact(
                    container2000Api,
                    blockNumberOfRestart,
                    currentBlock,
                    2,
                    authoritiesAtRestart
                );
            },
        });
    },
});
