import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { Signer } from "ethers";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR } from "helpers";
import fs from "node:fs/promises";
import {
    chainSpecToContainerChainGenesisData,
    checkLogsNotExist,
    getHeaderFromRelay,
    getTmpZombiePath,
    signAndSendAndInclude,
    waitSessions,
} from "utils";

describeSuite({
    id: "ZOMBIETANSS01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let ethersSigner: Signer;

        let specVersion: number;
        let shouldSkipStarlightCR: boolean;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            container2002Api = context.polkadotJs("Container2002");
            ethersSigner = context.ethers();

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("starlight");

            const container2000Network = container2000Api.consts.system.version.specName.toString();
            const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
            expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
            expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

            const container2001Network = container2001Api.consts.system.version.specName.toString();
            const paraId2001 = (await container2001Api.query.parachainInfo.parachainId()).toString();
            expect(container2001Network, "Container2001 API incorrect").to.contain("frontier-template");
            expect(paraId2001, "Container2001 API incorrect").to.be.equal("2001");

            const container2002Network = container2002Api.consts.system.version.specName.toString();
            const paraId2002 = (await container2002Api.query.parachainInfo.parachainId()).toString();
            expect(container2002Network, "Container2002 API incorrect").to.contain("container-chain-template");
            expect(paraId2002, "Container2002 API incorrect").to.be.equal("2002");

            specVersion = relayApi.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightCR = STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_REGISTRAR.includes(specVersion);
        }, 120000);

        it({
            id: "T01",
            title: "Test block numbers in relay are 0 yet",
            test: async () => {
                const header2000 = await getHeaderFromRelay(relayApi, 2000);
                const header2001 = await getHeaderFromRelay(relayApi, 2001);
                const header2002 = await getHeaderFromRelay(relayApi, 2002);

                expect(header2000.number.toNumber()).to.be.equal(0);
                expect(header2001.number.toNumber()).to.be.equal(0);
                expect(header2002.number.toNumber()).to.be.equal(0);
            },
        });

        it({
            id: "T02",
            title: "Blocks are being produced on tanssi-relay",
            test: async () => {
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("starlight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T03",
            title: "Set config params",
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const tx1 = relayApi.tx.collatorConfiguration.setFullRotationPeriod(5);
                const fillAmount = 990_000_000; // equal to 99% Perbill
                const tx2 = relayApi.tx.collatorConfiguration.setMaxParachainCoresPercentage(fillAmount);
                const txBatch = relayApi.tx.utility.batchAll([tx1, tx2]);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(txBatch), alice);
            },
        });

        it({
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
        });

        it({
            id: "T05",
            title: "Blocks are being produced on container 2000",
            test: async () => {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T06",
            title: "Blocks are being produced on container 2001",
            test: async () => {
                const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
                expect(await ethersSigner.provider.getBlockNumber(), "Safe tag is not present").to.be.greaterThan(0);
            },
        });

        it({
            id: "T07",
            title: "Test container chain 2000 assignation is correct",
            test: async () => {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T08",
            title: "Test container chain 2001 assignation is correct",
            test: async () => {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2001Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2001Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T09",
            title: "Test author noting is correct for both containers",
            timeout: 60000,
            test: async () => {
                const assignment = await relayApi.query.tanssiCollatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();
                const paraId2001 = await container2001Api.query.parachainInfo.parachainId();

                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];
                const containerChainCollators2001 = assignment.containerChains.toJSON()[paraId2001.toString()];

                await context.waitBlock(6, "Tanssi-relay");
                const author2000 = await relayApi.query.authorNoting.latestAuthor(paraId2000);
                const author2001 = await relayApi.query.authorNoting.latestAuthor(paraId2001);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
                expect(containerChainCollators2001.includes(author2001.toJSON().author)).to.be.true;
            },
        });

        it({
            id: "T10",
            title: "Test frontier template isEthereum",
            test: async () => {
                const genesisData2000 = await relayApi.query.containerRegistrar.paraGenesisData(2000);
                expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
                const genesisData2001 = await relayApi.query.containerRegistrar.paraGenesisData(2001);
                expect(genesisData2001.toJSON().properties.isEthereum).to.be.true;
            },
        });

        it({
            id: "T11",
            title: "Transactions can be made with ethers",
            timeout: 30000,
            test: async () => {
                const randomAccount = generateKeyringPair();
                const tx = await context.ethers().sendTransaction({
                    to: randomAccount.address,
                    value: 1_000_000_000_000n,
                });
                await tx.wait();
                expect(await context.ethers().provider.getBalance(randomAccount.address)).to.be.greaterThan(0);
            },
        });

        it({
            id: "T12",
            title: "Test live registration of container chain 2002 - Register",
            timeout: 60000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T12 test for Starlight version ${specVersion}`);
                    return;
                }

                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Read raw chain spec file
                const spec2002 = await fs.readFile("./specs/single-container-template-container-2002.json", "utf8");
                const headData2002 = await fs.readFile("./specs/para-2002-genesis-state", "utf8");

                const header2002 = await getHeaderFromRelay(relayApi, 2002);
                expect(header2002.number.toNumber()).to.be.equal(0);
                const registered1 = await relayApi.query.containerRegistrar.registeredParaIds();
                expect(registered1.toJSON().includes(2002)).to.be.false;

                const chainSpec2002 = JSON.parse(spec2002);
                const genesisCode = chainSpec2002.genesis.raw.top["0x3a636f6465"];
                const containerChainGenesisData = chainSpecToContainerChainGenesisData(relayApi, chainSpec2002);
                const tx0 = relayApi.tx.registrar.reserve();
                const tx1 = relayApi.tx.containerRegistrar.register(2002, containerChainGenesisData, headData2002);
                const purchasedCredits = 100000n;
                const requiredBalance = purchasedCredits * 1_000_000n;
                const tx2 = relayApi.tx.servicesPayment.purchaseCredits(2002, requiredBalance);

                const profileId = await relayApi.query.dataPreservers.nextProfileId();
                const profileTx = relayApi.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx3 = relayApi.tx.dataPreservers.forceStartAssignment(profileId, 2002, "Free");
                const tx4 = relayApi.tx.paras.addTrustedValidationCode(genesisCode);

                const txBatch = relayApi.tx.utility.batchAll([
                    tx0,
                    tx1,
                    tx2,
                    profileTx,
                    relayApi.tx.sudo.sudo(tx3),
                    relayApi.tx.sudo.sudo(tx4),
                ]);
                const { blockHash } = await signAndSendAndInclude(txBatch, alice);

                const apiAt = await relayApi.at(blockHash);
                const events = await apiAt.query.system.events();
                const ev1 = events.filter((a) => a.event.method === "BatchCompleted");
                expect(ev1.length).to.be.equal(1);
            },
        });

        it({
            id: "T13",
            title: "Test live registration of container chain 2002 - Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T14",
            title: "Test live registration of container chain 2002 - MarkValidForCollating",
            timeout: 60000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T14 test for Starlight version ${specVersion}`);
                    return;
                }

                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const tx4 = relayApi.tx.containerRegistrar.markValidForCollating(2002);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx4), alice);

                const registered2 = await relayApi.query.containerRegistrar.pendingParaIds();
                const registered3 = await relayApi.query.containerRegistrar.registeredParaIds();
                expect(registered2.toJSON()[0][1].includes(2002)).to.be.true;
                expect(registered3.toJSON().includes(2002)).to.be.false;
            },
        });

        it({
            id: "T15",
            title: "Test live registration of container chain 2002 - Wait 2 sessions more",
            timeout: 300000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T15 test for Starlight version ${specVersion}`);
                    return;
                }

                await waitSessions(
                    context,
                    relayApi,
                    2,
                    async () => {
                        const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                        return registered.toJSON().includes(2002);
                    },
                    "Tanssi-relay"
                );
            },
        });

        it({
            id: "T16",
            title: "Test live registration of container chain 2002 - Assert",
            timeout: 60000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T16 test for Starlight version ${specVersion}`);
                    return;
                }

                const registered5 = await relayApi.query.containerRegistrar.registeredParaIds();
                expect(registered5.toJSON().includes(2002)).to.be.true;
            },
        });

        it({
            id: "T17",
            title: "Blocks are being produced on container 2002",
            timeout: 180000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T17 test for Starlight version ${specVersion}`);
                    return;
                }

                await context.waitBlock(3, "Container2002");
            },
        });

        it({
            id: "T18",
            title: "Test container chain 2002 assignation is correct",
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T18 test for Starlight version ${specVersion}`);
                    return;
                }

                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2002Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];
                const writtenCollators = (await container2002Api.query.authoritiesNoting.authorities()).toJSON();
                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });

        it({
            id: "T19",
            title: "Deregister container chain 2002",
            timeout: 300000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T19 test for Starlight version ${specVersion}`);
                    return;
                }

                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const registered1 = await relayApi.query.containerRegistrar.registeredParaIds();
                expect(registered1.toJSON().includes(2002)).to.be.true;

                const tx = relayApi.tx.containerRegistrar.deregister(2002);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                await waitSessions(
                    context,
                    relayApi,
                    2,
                    async () => {
                        const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                        return !registered.toJSON().includes(2002);
                    },
                    "Tanssi-relay"
                );

                const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                expect(registered.toJSON().includes(2002)).to.be.false;
            },
        });

        it({
            id: "T20",
            title: "Check collator logs to ensure common errors are fixed",
            timeout: 300000,
            test: async () => {
                if (shouldSkipStarlightCR) {
                    console.log(`Skipping T19 test for Starlight version ${specVersion}`);
                    return;
                }

                const logs = [
                    "/Collator-01.log",
                    "/Collator-02.log",
                    "/Collator-03.log",
                    "/Collator-04.log",
                    "/Collator-05.log",
                    "/Collator-06.log",
                ];
                for (const log of logs) {
                    const logFilePath = getTmpZombiePath() + log;
                    await checkLogsNotExist(logFilePath, [
                        "Shutdown error",
                        "Timeout when waiting for paritydb lock",
                        "Error waiting for chain",
                        "Failed to start container chain",
                        "Shutting down container chain service",
                        "Entering off-chain worker.",
                        "Overweight para inherent data after enacting the candidates",
                    ]);
                }
            },
        });

        it({
            id: "T21",
            title: "Check reward points for validators are distributed",
            test: async () => {
                const keys = await relayApi.query.externalValidatorsRewards.rewardPointsForEra.keys();
                expect(keys.length).to.be.greaterThan(0);
            },
        });
    },
});
