import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import type { Signer } from "ethers";
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
        let collator01RelayApi: ApiPromise;
        let collator02RelayApi: ApiPromise;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            container2002Api = context.polkadotJs("Container2002");
            ethersSigner = context.ethers();

            const relayNetwork = relayApi.consts.system.version.specName.toString();
            expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");

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

            const wsProvider1 = new WsProvider("ws://127.0.0.1:9961");
            collator01RelayApi = await ApiPromise.create({ provider: wsProvider1 });
            const wsProvider2 = new WsProvider("ws://127.0.0.1:9962");
            collator02RelayApi = await ApiPromise.create({ provider: wsProvider2 });
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
                expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");
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

                // Disable rotation
                const tx1 = relayApi.tx.collatorConfiguration.setFullRotationPeriod(0);
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
            title: "Test rotateKeys for Collator-01 and Collator02",
            timeout: 60000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const collator01 = keyring.addFromUri("//Collator-01", { name: "Collator-01 default" });
                const collator02 = keyring.addFromUri("//Collator-02", { name: "Collator-02 default" });

                // Add keys to pallet session. In dancebox they are already there in genesis.
                // We need 4 collators because we have 2 chains with 2 collators per chain.
                const newKeys1 = await collator01RelayApi.rpc.author.rotateKeys();
                const newKeys2 = await collator02RelayApi.rpc.author.rotateKeys();

                await signAndSendAndInclude(relayApi.tx.session.setKeys(newKeys1, []), collator01);
                await signAndSendAndInclude(relayApi.tx.session.setKeys(newKeys2, []), collator02);
            },
        });

        it({
            id: "T13",
            title: "Test rotateKeys - Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T14",
            title: "Check that keys have changed and collators keep producing blocks",
            timeout: 60000,
            test: async () => {

            },
        });

        it({
            id: "T20",
            title: "Check collator logs to ensure common errors are fixed",
            timeout: 300000,
            test: async () => {
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
