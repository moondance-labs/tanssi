import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
import { getKeyringNimbusIdHex } from "../../util/keys.ts";
import { getAuthorFromDigest } from "../../util/author.ts";
import { customWeb3Request, generateKeyringPair, MIN_GAS_PRICE } from "@moonwall/util";
import { createTransfer, waitUntilEthTxIncluded } from "../../util/ethereum.ts";
import { ApiPromise, Keyring } from "@polkadot/api";
import fs from "fs/promises";
import { chainSpecToContainerChainGenesisData } from "../../util/genesis_data.ts";
import { signAndSendAndInclude, waitSessions } from "../../util/block.ts";
import { Signer } from "ethers";

describeSuite({
    id: "ZR-01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: function ({ it, context }) {
        let relayApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let blockNumber2002Start;
        let blockNumber2002End;
        let ethersSigner: Signer;
        const sessionPeriod = 10;

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

            // Test block numbers in relay are 0 yet
            const header2000 = await getHeaderFromRelay(relayApi, 2000);
            const header2001 = await getHeaderFromRelay(relayApi, 2001);
            const header2002 = await getHeaderFromRelay(relayApi, 2002);

            expect(header2000.number.toNumber()).to.be.equal(0);
            expect(header2001.number.toNumber()).to.be.equal(0);
            expect(header2002.number.toNumber()).to.be.equal(0);
        }, 120000);

        it({
            id: "T01",
            title: "Blocks are being produced on tanssi-relay",
            test: async function () {
                console.log("executing");
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("starlight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);

                const para2000LifeCycle = await relayApi.query.paras.paraLifecycles(2002);
                console.log("PARA LIFE CYCLE: ", para2000LifeCycle.toHuman());
            },
        });

        it({
            id: "T03",
            timeout: 300000,
            title: "Test assignation did not change",
            test: async function () {
                // TODO: starlight collator assignment is not set properly on genesis, so we need to wait 2 sessions
                // for the collators to be assigned, and also for container chains to start producing blocks
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                // TODO: fix once we have types
                const allCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                const expectedAllCollators = {
                    orchestratorChain: [],
                    containerChains: {
                        "2000": [getKeyringNimbusIdHex("Collator1000-01"), getKeyringNimbusIdHex("Collator1000-02")],
                        "2001": [getKeyringNimbusIdHex("Collator1000-03"), getKeyringNimbusIdHex("Collator1000-04")],
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
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
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
            id: "T08",
            title: "Test author noting is correct for both containers",
            timeout: 60000,
            test: async function () {
                const assignment = await relayApi.query.tanssiCollatorAssignment.collatorContainerChain();
                const paraId2000 = await container2000Api.query.parachainInfo.parachainId();
                const paraId2001 = await container2001Api.query.parachainInfo.parachainId();

                // TODO: fix once we have types
                const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];
                const containerChainCollators2001 = assignment.containerChains.toJSON()[paraId2001.toString()];

                await context.waitBlock(3, "Tanssi-relay");
                const author2000 = await relayApi.query.authorNoting.latestAuthor(paraId2000);
                const author2001 = await relayApi.query.authorNoting.latestAuthor(paraId2001);

                expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
                expect(containerChainCollators2001.includes(author2001.toJSON().author)).to.be.true;
            },
        });

        it({
            id: "T10",
            title: "Test frontier template isEthereum",
            test: async function () {
                // TODO: fix once we have types
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
            title: "Test live registration of container chain 2002",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Read raw chain spec file
                const spec2002 = await fs.readFile("./specs/single-container-template-container-2002.json", "utf8");
                const headData2002 = await fs.readFile("./specs/para-2002-genesis-state", "utf8");
                console.log("HEAD DATA 2002: ", headData2002)

                // Before registering container chain 2002, ensure that it has 0 blocks
                // Since the RPC doesn't exist at this point, we need to get that from the relay
                const header2002 = await getHeaderFromRelay(relayApi, 2002);
                expect(header2002.number.toNumber()).to.be.equal(0);
                const registered1 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2002)).to.be.false;

                const chainSpec2002 = JSON.parse(spec2002);
                const containerChainGenesisData = chainSpecToContainerChainGenesisData(relayApi, chainSpec2002);

                const tx1 = relayApi.tx.containerRegistrar.register(2002, containerChainGenesisData, headData2002);
                const purchasedCredits = 100000n;
                const requiredBalance = purchasedCredits * 1_000_000n;
                //const tx2 = relayApi.tx.servicesPayment.purchaseCredits(2002, requiredBalance);

                // TODO: uncomment once we have data preservers
/*              const profileId = await relayApi.query.dataPreservers.nextProfileId();
                const profileTx = relayApi.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });
 */
                // const tx3 = relayApi.tx.dataPreservers.forceStartAssignment(profileId, 2002, "Free");
                const tx4 = relayApi.tx.containerRegistrar.markValidForCollating(2002);
                // Send the batch transaction: [register, purchaseCredits, sudo(setBootNodes), sudo(markValidForCollating)]
                const txBatch = relayApi.tx.utility.batchAll([
                    tx1,
/*                     tx2,
                    profileTx,
                    relayApi.tx.sudo.sudo(tx3), */
                    relayApi.tx.sudo.sudo(tx4),
                ]);
                await signAndSendAndInclude(txBatch, alice);
                
                // Check that pending para ids contains 2002
                const registered2 = await relayApi.query.containerRegistrar.pendingParaIds();
                const registered3 = await relayApi.query.containerRegistrar.registeredParaIds();

                // TODO: fix once we have types
                expect(registered2.toJSON()[0][1].includes(2002)).to.be.true;
                // But registered does not contain 2002 yet
                // TODO: fix once we have types
                expect(registered3.toJSON().includes(2002)).to.be.false;
                // Container chain will be registered after 2 sessions, but because `signAndSendAndInclude` waits
                // until the block that includes the extrinsic is finalized, it is possible that we only need to wait
                // 1 session. So use a callback to wait 1 or 2 sessions.
                await waitSessions(
                    context,
                    relayApi,
                    2,
                    async () => {
                        const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                        // Stop waiting when 2002 is registered
                        return registered.toJSON().includes(2002);
                    },
                    "Tanssi-relay"
                );
                // Check that registered para ids contains 2002
                const registered5 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered5.toJSON().includes(2002)).to.be.true;

                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                // Round block number to start of session, sometimes the rpc returns the block number of the next block
                blockNumber2002Start = blockNum - (blockNum % sessionPeriod);
            },
        });

        it({
            id: "T13",
            title: "Blocks are being produced on container 2002",
            timeout: 120000,
            test: async function () {
                // Wait 3 blocks because the next test needs to get a non empty value from
                // container2002Api.query.authoritiesNoting()
                await context.waitBlock(3, "Container2002");
            },
        });

        it({
            id: "T14",
            title: "Test container chain 2002 assignation is correct",
            test: async function () {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const paraId = (await container2002Api.query.parachainInfo.parachainId()).toString();
                // TODO: fix once we have types
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains[paraId];

                const writtenCollators = (await container2002Api.query.authoritiesNoting.authorities()).toJSON();

                expect(containerChainCollators).to.deep.equal(writtenCollators);
            },
        });
    },
});
