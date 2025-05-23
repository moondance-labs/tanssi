import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { stringToHex, u8aToHex } from "@polkadot/util";
import { decodeAddress } from "@polkadot/util-crypto";
import {
    checkLogs,
    checkLogsNotExist,
    directoryExists,
    getAuthorFromDigest,
    getHeaderFromRelay,
    getKeyringNimbusIdHex,
    getTmpZombiePath,
    signAndSendAndInclude,
    waitSessions,
} from "utils";

describeSuite({
    id: "ZOMBIETANSSIW01",
    title: "Zombie Tanssi Warp Sync Test",
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
            id: "T12",
            title: "Test warp sync: collator rotation from tanssi to container with blocks",
            timeout: 300000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Collator2000-02 should have a container 2000 db, and Collator1000-03 should not
                const collator100003DbPath = `${getTmpZombiePath()}/Collator1000-03/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const container200002DbPath = `${getTmpZombiePath()}/Collator2000-02/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                expect(await directoryExists(container200002DbPath)).to.be.true;
                expect(await directoryExists(collator100003DbPath)).to.be.false;

                // Deregister Collator2000-02, it should delete the db
                const invuln = (await paraApi.query.invulnerables.invulnerables()).toJSON();

                const invulnerable_to_remove = invuln.filter((addr) => {
                    return u8aToHex(decodeAddress(addr)) === getKeyringNimbusIdHex("Collator2000-02");
                })[0];

                const tx = paraApi.tx.invulnerables.removeInvulnerable(invulnerable_to_remove);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);

                // New collators will be set after 2 sessions, but because `signAndSendAndInclude` waits
                // until the block that includes the extrinsic is finalized, it is possible that we only need to wait
                // 1 session. So use a callback to wait 1 or 2 sessions.
                await waitSessions(context, paraApi, 2, async () => {
                    const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                    const allCollators = (
                        await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                    ).toJSON();
                    // Stop waiting if orchestrator chain has 2 collators instead of 3
                    return allCollators.orchestratorChain.length === 2;
                });

                // Collator1000-03 should rotate to container chain 2000

                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                // TODO: fix once we have types
                const allCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                const expectedAllCollators = {
                    orchestratorChain: [
                        getKeyringNimbusIdHex("Collator1000-01"),
                        getKeyringNimbusIdHex("Collator1000-02"),
                    ],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator1000-03")],
                    },
                };

                expect(allCollators).to.deep.equal(expectedAllCollators);

                // The node detects assignment when the block is finalized, but "waitSessions" ignores finality.
                // So wait a few blocks more hoping that the current block will be finalized by then.
                await context.waitBlock(6, "Tanssi");

                // Collator2000-02 container chain db should have been deleted
                expect(await directoryExists(container200002DbPath)).to.be.false;

                // Collator1000-03 container chain db should be created
                expect(await directoryExists(collator100003DbPath)).to.be.true;
            },
        });

        it({
            id: "T13",
            title: "Collator1000-03 is producing blocks on Container 2000",
            timeout: 300000,
            test: async () => {
                const blockStart = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber() - 3;
                // Wait up to 8 blocks, giving the new collator 4 chances to build a block
                const blockEnd = blockStart + 8;
                const authors = [];

                for (let blockNumber = blockStart; blockNumber <= blockEnd; blockNumber += 1) {
                    // Get the latest author from Digest
                    const blockHash = await container2000Api.rpc.chain.getBlockHash(blockNumber);
                    const apiAt = await container2000Api.at(blockHash);
                    const digests = (await apiAt.query.system.digest()).logs;
                    const filtered = digests.filter(
                        (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() === stringToHex("nmbs")
                    );
                    const author = filtered[0].asPreRuntime[1].toHex();
                    authors.push(author);
                    if (author === getKeyringNimbusIdHex("Collator1000-03")) {
                        break;
                    }
                    const currentBlock = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                    if (currentBlock === blockNumber) {
                        await context.waitBlock(1, "Container2000");
                    }
                }

                expect(authors).to.contain(getKeyringNimbusIdHex("Collator1000-03"));
            },
        });

        it({
            id: "T14",
            title: "Check Collator1000-03.log to ensure it used warp sync",
            timeout: 300000,
            test: async () => {
                // Use collator logs to ensure that it used warp sync to first the first time.
                // Not ideal because logs can change, but better than nothing.
                const logFilePath = `${getTmpZombiePath()}/Collator1000-03.log`;
                await checkLogs(logFilePath, [
                    "[Orchestrator] Detected assignment for container chain 2000",
                    "[Orchestrator] Loaded chain spec for container chain 2000",
                    "[Orchestrator] This is a syncing container chain, using random ports",
                    "[Orchestrator] Container chain sync mode: Warp",
                    "[Container-2000] Warp sync is complete",
                    "[Orchestrator] Detected assignment for container chain 2000",
                    "[Orchestrator] Loaded chain spec for container chain 2000",
                    "[Orchestrator] Container chain sync mode: Warp",
                    "[Container-2000] Can't use warp sync mode with a partially synced database. Reverting to full sync mode.",
                ]);
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

        it({
            id: "T16",
            title: "Check Collator1000-03.log to ensure it did not download the block history",
            timeout: 300000,
            test: async () => {
                // Not ideal because logs can change, but better than nothing.
                const logFilePath = `${getTmpZombiePath()}/Collator1000-03.log`;
                /*
TRACE tokio-runtime-worker sync: [Container-2000] State sync: imported 1 of 1.
 INFO tokio-runtime-worker sync: [Container-2000] State sync is complete, continuing with block sync.
DEBUG tokio-runtime-worker sync: [Container-2000] Starting gap sync #1 - #7
TRACE tokio-runtime-worker sync: [Container-2000] Restarted sync at #8 (0x19a3cf7d3224bd4f56328df3102487b0fb3ce0abefede96cee7c0afae3c07bb3)

TRACE tokio-runtime-worker sync: [Container-2000] BlockResponse 0 from 12D3KooWR3cXp1h9Qguvt5yjX3Y48Z3bcmYNy6dxQRHmWNoKiTVk with 7 blocks  (7..1)
TRACE tokio-runtime-worker sync: [Container-2000] Reversing incoming block list
TRACE tokio-runtime-worker sync: [Container-2000] 7 blocks ready for import
DEBUG tokio-runtime-worker sync: [Container-2000] Drained 7 gap blocks from 0
TRACE tokio-runtime-worker sync: [Container-2000] Accepted 7 blocks (0xf634a6692aef17af72ccc9ae716d991a3bd8c8fa2c52218eae441d3097c8a503) with origin NetworkInitialSync
TRACE tokio-runtime-worker sync::import-queue: [Container-2000] Scheduling 7 blocks for import
                 */
                // Can't check the other logs because we also see them during normal sync:
                // When the collator warp syncs to block 8, in the meantime the latest block is 11,
                // so it syncs blocks 9-11 printing almost the same logs as the gap sync.
                await checkLogsNotExist(logFilePath, ["Starting gap sync"]);
            },
        });

        it({
            id: "T17",
            title: "Check Collator1000-03 RPC to ensure it does not have block 1",
            timeout: 30000,
            test: async () => {
                const wsProvider = new WsProvider("ws://127.0.0.1:9952");
                const api = await ApiPromise.create({ provider: wsProvider });

                // Collator1000-03 should return block hash 0x000 for blocks it doesn't have
                // From testing, it warp syncs at block 9, so blocks [1, 8] should be missing
                // But to make this test more reliable, let's only check block 1
                const blockHash1 = await api.rpc.chain.getBlockHash(1);
                expect(blockHash1.toString()).toBe(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );
            },
        });
    },
});
