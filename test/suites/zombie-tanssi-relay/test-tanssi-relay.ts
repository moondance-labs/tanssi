import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
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
        let ethersSigner: Signer;

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
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
            title: "Set config params",
            test: async function () {
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
            id: "T03",
            timeout: 600000,
            title: "Test assignation did not change",
            test: async function () {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                // TODO: fix once we have types
                const allCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON();
                // We can only check the number because zombienet dancelight has randomness and rotation,
                // so the assigned collators will change every time this test is run.
                // 0 collators in orchestrator and 2 collators in each para
                expect(allCollators.orchestratorChain.length).to.equal(0);
                expect(allCollators.containerChains["2000"].length).to.equal(2);
                expect(allCollators.containerChains["2001"].length).to.equal(2);
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
            id: "T12a",
            title: "Test live registration of container chain 2002 - Register",
            timeout: 60000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                // Read raw chain spec file
                const spec2002 = await fs.readFile("./specs/single-container-template-container-2002.json", "utf8");
                const headData2002 = await fs.readFile("./specs/para-2002-genesis-state", "utf8");

                // Before registering container chain 2002, ensure that it has 0 blocks
                // Since the RPC doesn't exist at this point, we need to get that from the relay
                const header2002 = await getHeaderFromRelay(relayApi, 2002);
                expect(header2002.number.toNumber()).to.be.equal(0);
                const registered1 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
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
                // In Dancelight we must wait 2 session before calling markValidForCollating, because the para needs to be
                // onboarded in the relay registrar first.
                // And before being allowed to do that, we must mark the validationCode as trusted
                const tx4 = relayApi.tx.paras.addTrustedValidationCode(genesisCode);

                // Send the batch transaction: [register, purchaseCredits, createProfile, sudo(forceStartAssignment), sudo(addTrustedValidationCode)]
                const txBatch = relayApi.tx.utility.batchAll([
                    tx0,
                    tx1,
                    tx2,
                    profileTx,
                    relayApi.tx.sudo.sudo(tx3),
                    relayApi.tx.sudo.sudo(tx4),
                ]);
                const { blockHash } = await signAndSendAndInclude(txBatch, alice);

                // Assert that the batch succeeded with no error
                const apiAt = await relayApi.at(blockHash);
                const events = await apiAt.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method == "BatchCompleted";
                });
                expect(ev1.length).to.be.equal(1);
            },
        });

        it({
            id: "T12b",
            title: "Test live registration of container chain 2002 - Wait 2 sessions",
            timeout: 300000,
            test: async function () {
                // This needs to wait until registrar.paraLifecycle is "parathread"
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T12c",
            title: "Test live registration of container chain 2002 - MarkValidForCollating",
            timeout: 60000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const tx4 = relayApi.tx.containerRegistrar.markValidForCollating(2002);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx4), alice);

                // Check that pending para ids contains 2002
                const registered2 = await relayApi.query.containerRegistrar.pendingParaIds();
                const registered3 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered2.toJSON()[0][1].includes(2002)).to.be.true;
                // But registered does not contain 2002 yet
                // TODO: fix once we have types
                expect(registered3.toJSON().includes(2002)).to.be.false;
            },
        });

        it({
            id: "T12d",
            title: "Test live registration of container chain 2002 - Wait 2 sessions more",
            timeout: 300000,
            test: async function () {
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
            },
        });

        it({
            id: "T12e",
            title: "Test live registration of container chain 2002 - Assert",
            timeout: 60000,
            test: async function () {
                // Check that registered para ids contains 2002
                const registered5 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered5.toJSON().includes(2002)).to.be.true;
            },
        });

        it({
            id: "T13",
            title: "Blocks are being produced on container 2002",
            timeout: 180000,
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

        it({
            id: "T15",
            title: "Deregister container chain 2002",
            timeout: 300000,
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const registered1 = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered1.toJSON().includes(2002)).to.be.true;

                const tx = relayApi.tx.containerRegistrar.deregister(2002);
                await signAndSendAndInclude(relayApi.tx.sudo.sudo(tx), alice);
                // Container chain will be deregistered after 2 sessions, but because `signAndSendAndInclude` waits
                // until the block that includes the extrinsic is finalized, it is possible that we only need to wait
                // 1 session. So use a callback to wait 1 or 2 sessions.
                await waitSessions(
                    context,
                    relayApi,
                    2,
                    async () => {
                        const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                        // Stop waiting if 2002 is no longer registered
                        return !registered.toJSON().includes(2002);
                    },
                    "Tanssi-relay"
                );

                // Check that pending para ids removes 2002
                const registered = await relayApi.query.containerRegistrar.registeredParaIds();
                // TODO: fix once we have types
                expect(registered.toJSON().includes(2002)).to.be.false;
            },
        });

        it({
            id: "T18",
            title: "Check collator logs to ensure common errors are fixed",
            timeout: 300000,
            test: async function () {
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
                    ]);
                }
            },
        });
    },
});

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

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
    return process.env.MOON_ZOMBIE_DIR;
}
