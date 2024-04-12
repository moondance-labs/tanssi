import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { MIN_GAS_PRICE, customWeb3Request, generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { Signer } from "ethers";
import fs from "fs/promises";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitToSession } from "../../util/block";
import { createTransfer, waitUntilEthTxIncluded } from "../../util/ethereum";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";

describeSuite({
    id: "R01",
    title: "Zombie Tanssi Rotation Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let paraApi: ApiPromise;
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let ethersSigner: Signer;
        let assignment3;
        let assignment5;
        let allCollators: string[];
        let collatorName: Record<string, string>;
        let containerDbPaths: string[];

        beforeAll(async () => {
            paraApi = context.polkadotJs("Tanssi");
            relayApi = context.polkadotJs("Relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            ethersSigner = context.ethers();

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

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
            expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);
            const header2001 = await getHeaderFromRelay(relayApi, 2001);

            expect(header2000.number.toNumber()).to.be.equal(0);
            expect(header2001.number.toNumber()).to.be.equal(0);

            // Initialize list of all collators, this should match the names from build-spec.sh script
            allCollators = [
                "Collator1000-01",
                "Collator1000-02",
                "Collator2000-01",
                "Collator2000-02",
                "Collator2001-01",
                "Collator2001-02",
                "Collator2002-01",
                "Collator2002-02",
            ];
            // Initialize reverse map of collator key to collator name
            collatorName = createCollatorKeyToNameMap(paraApi, allCollators);
            console.log(collatorName);

            containerDbPaths = [
                "/data/containers/chains/simple_container_2000/paritydb/full-container-2000",
                "/data/containers/chains/frontier_container_2001/paritydb/full-container-2001",
            ];
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
            title: "Set 1 collator per parachain, and full_rotation every 5 sessions",
            timeout: 120000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const tx1 = await paraApi.tx.configuration.setCollatorsPerContainer(1);
                const tx2 = await paraApi.tx.configuration.setMinOrchestratorCollators(1);
                const tx3 = await paraApi.tx.configuration.setMaxOrchestratorCollators(1);
                const tx4 = await paraApi.tx.configuration.setFullRotationPeriod(5);
                const tx1234 = paraApi.tx.utility.batchAll([tx1, tx2, tx3, tx4]);
                await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx1234), alice);
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
                        getKeyringNimbusIdHex("Collator2002-01"),
                        getKeyringNimbusIdHex("Collator2002-02"),
                    ],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator2000-02")],
                        "2001": [getKeyringNimbusIdHex("Collator2001-01"), getKeyringNimbusIdHex("Collator2001-02")],
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
            id: "T05",
            title: "Blocks are being produced on container 2001",
            test: async function () {
                const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();

                expect(blockNum).to.be.greaterThan(0);
                expect(await ethersSigner.provider.getBlockNumber(), "Safe tag is not present").to.be.greaterThan(0);
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
            id: "T07",
            title: "Test container chain 2001 assignation is correct",
            test: async function () {
                const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T08",
            title: "Test author noting is correct for both containers",
            timeout: 120000,
            test: async function () {
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();
                const paraId2001 = await container2001Api.query.parachainInfo.parachainId();

                // TODO: fix once we have types
                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];
                const containerChainCollators2001 = assignment.containerChains.toJSON()[paraId2001.toString()];

                await context.waitBlock(3, "Tanssi");
                const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);
                const author2001 = await paraApi.query.authorNoting.latestAuthor(paraId2001);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
                expect(containerChainCollators2001.includes(author2001.toJSON().author)).to.be.true;
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
                const genesisData2001 = await paraApi.query.registrar.paraGenesisData(2001);
                expect(genesisData2001.toJSON().properties.isEthereum).to.be.true;
            },
        });
        it({
            id: "T11",
            title: "Transactions can be made with ethers",
            timeout: 120000,
            test: async function () {
                const randomAccount = generateKeyringPair();
                const tx = await createTransfer(context, randomAccount.address, 1_000_000_000_000, {
                    gasPrice: MIN_GAS_PRICE,
                });
                const txHash = await customWeb3Request(context.web3(), "eth_sendRawTransaction", [tx]);
                await waitUntilEthTxIncluded(
                    () => context.waitBlock(1, "Container2001"),
                    context.web3(),
                    txHash.result
                );
                expect(Number(await context.web3().eth.getBalance(randomAccount.address))).to.be.greaterThan(0);
            },
        });
        it({
            id: "T12",
            title: "On session 3 we have 1 collator per chain",
            timeout: 240000,
            test: async function () {
                await waitToSession(context, paraApi, 3);

                // The node detects assignment when the block is finalized, but "waitSessions" ignores finality.
                // So wait a few blocks more hoping that the current block will be finalized by then.
                await context.waitBlock(6, "Tanssi");
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                assignment3 = assignment.toJSON();
                console.log("assignment session 3:");
                logAssignment(collatorName, assignment3);

                expect(assignment.orchestratorChain.length).toBe(1);
                expect(assignment.containerChains.toJSON()[2000].length).toBe(1);
                expect(assignment.containerChains.toJSON()[2001].length).toBe(1);
            },
        });
        it({
            id: "T13",
            title: "On session 4 collators start syncing the new chains",
            timeout: 240000,
            test: async function () {
                await waitToSession(context, paraApi, 4);

                // The node detects assignment when the block is finalized, but "waitSessions" ignores finality.
                // So wait a few blocks more hoping that the current block will be finalized by then.
                await context.waitBlock(6, "Tanssi");
                const futureAssignment = await paraApi.query.collatorAssignment.pendingCollatorContainerChain();
                // The assignment is random, so there is a small chance that it will be the same,
                // and in that case this test shouldn't fail
                if (futureAssignment.isNone) {
                    assignment5 = assignment3;
                } else {
                    assignment5 = futureAssignment.toJSON();
                }
                console.log("assignment session 5:");
                logAssignment(collatorName, assignment5);

                // First, check that nodes are still running in their previously assigned chain
                const oldC2000 = collatorName[assignment3.containerChains[2000][0]];
                const oldC2001 = collatorName[assignment3.containerChains[2001][0]];
                const oldContainer2000DbPath =
                    getTmpZombiePath() +
                    `/${oldC2000}/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const oldContainer2001DbPath =
                    getTmpZombiePath() +
                    `/${oldC2001}/data/containers/chains/frontier_container_2001/paritydb/full-container-2001`;
                expect(await directoryExists(oldContainer2000DbPath)).to.be.true;
                expect(await directoryExists(oldContainer2001DbPath)).to.be.true;

                // Check that new assigned collators have started syncing
                const c2000 = collatorName[assignment5.containerChains[2000][0]];
                const c2001 = collatorName[assignment5.containerChains[2001][0]];
                let unassignedCollators = getUnassignedCollators(allCollators, [c2000, c2001]);
                // Remove old collators because they will still have some chains running
                unassignedCollators = unassignedCollators.filter((x) => x !== oldC2000);
                unassignedCollators = unassignedCollators.filter((x) => x !== oldC2001);

                // Verify that collators have container chain running by looking at db path,
                // and unassignedCollators should not have any db path
                const container2000DbPath =
                    getTmpZombiePath() +
                    `/${c2000}/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const container2001DbPath =
                    getTmpZombiePath() +
                    `/${c2001}/data/containers/chains/frontier_container_2001/paritydb/full-container-2001`;
                expect(await directoryExists(container2000DbPath)).to.be.true;
                expect(await directoryExists(container2001DbPath)).to.be.true;

                await ensureContainerDbPathsDontExist(unassignedCollators, containerDbPaths);
            },
        });
        it({
            id: "T14",
            title: "On session 5 collators stop the previously assigned chains",
            timeout: 240000,
            test: async function () {
                await waitToSession(context, paraApi, 5);
                const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
                expect(assignment.toJSON()).to.deep.equal(assignment5);

                // The node detects assignment when the block is finalized, but "waitSessions" ignores finality.
                // So wait a few blocks more hoping that the current block will be finalized by then.
                // This also serves to check that Tanssi is producing blocks after the rotation
                await context.waitBlock(6, "Tanssi");

                // First, check that nodes have stopped in their previously assigned chain
                const oldC2000 = collatorName[assignment3.containerChains[2000][0]];
                const oldC2001 = collatorName[assignment3.containerChains[2001][0]];
                const c2000 = collatorName[assignment5.containerChains[2000][0]];
                const c2001 = collatorName[assignment5.containerChains[2001][0]];
                const oldContainer2000DbPath =
                    getTmpZombiePath() +
                    `/${oldC2000}/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const oldContainer2001DbPath =
                    getTmpZombiePath() +
                    `/${oldC2001}/data/containers/chains/frontier_container_2001/paritydb/full-container-2001`;
                // Edge case: collators may be assigned to the same chain, in that case the directory will still exist
                if (oldC2000 != c2000) {
                    expect(await directoryExists(oldContainer2000DbPath)).to.be.false;
                }
                if (oldC2001 != c2001) {
                    expect(await directoryExists(oldContainer2001DbPath)).to.be.false;
                }

                // Check that new assigned collators are running
                const unassignedCollators = getUnassignedCollators(allCollators, [c2000, c2001]);

                // Verify that collators have container chain running by looking at db path,
                // and unassignedCollators should not have any db path
                const container2000DbPath =
                    getTmpZombiePath() +
                    `/${c2000}/data/containers/chains/simple_container_2000/paritydb/full-container-2000`;
                const container2001DbPath =
                    getTmpZombiePath() +
                    `/${c2001}/data/containers/chains/frontier_container_2001/paritydb/full-container-2001`;
                expect(await directoryExists(container2000DbPath)).to.be.true;
                expect(await directoryExists(container2001DbPath)).to.be.true;
                await ensureContainerDbPathsDontExist(unassignedCollators, containerDbPaths);
            },
        });

        it({
            id: "T15",
            title: "Blocks are being produced on container 2000",
            test: async function () {
                await context.waitBlock(1, "Container2000");
            },
        });

        it({
            id: "T16",
            title: "Blocks are being produced on container 2001",
            test: async function () {
                await context.waitBlock(1, "Container2001");
            },
        });
    },
});

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

/// Given a list of collators and a list of dbPaths, checks that the path does not exist for all the collators.
/// This can be used to ensure that all the unassigned collators do not have any container chains running.
async function ensureContainerDbPathsDontExist(collators: string[], pathsToVerify: string[]) {
    for (const collator of collators) {
        for (const path of pathsToVerify) {
            const fullPath = getTmpZombiePath() + `/${collator}${path}`;
            expect(await directoryExists(fullPath), `Container DB path exists for ${collator}: ${fullPath}`).to.be
                .false;
        }
    }
}

/// Create a map of collator key "5C5p..." to collator name "Collator1000-01".
function createCollatorKeyToNameMap(paraApi, collatorNames: string[]): Record<string, string> {
    const collatorName: Record<string, string> = {};

    collatorNames.forEach((name) => {
        const hexAddress = getKeyringNimbusIdHex(name);
        const k = paraApi.createType("AccountId", hexAddress);
        collatorName[k] = name;
    });

    return collatorName;
}

/// Given a list of all collators and collators assigned to containers, returns the collators that are not assigned to
/// containers.
function getUnassignedCollators(allCollators: string[], assignedToContainers: string[]): string[] {
    return allCollators.filter((collator) => !assignedToContainers.includes(collator));
}

function logAssignment(collatorName, assignment) {
    const nameAssignment = {
        orchestratorChain: assignment.orchestratorChain.map((x) => collatorName[x]),
        containerChains: Object.keys(assignment.containerChains).reduce((result, key) => {
            result[key] = assignment.containerChains[key].map((x) => collatorName[x]);
            return result;
        }, {}),
    };

    console.log(nameAssignment);
}
