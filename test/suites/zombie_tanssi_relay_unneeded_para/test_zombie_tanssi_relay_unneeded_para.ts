import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getHeaderFromRelay } from "../../util/relayInterface.ts";
import { customWeb3Request, generateKeyringPair, MIN_GAS_PRICE } from "@moonwall/util";
import { createTransfer, waitUntilEthTxIncluded } from "../../util/ethereum.ts";
import { type ApiPromise, Keyring } from "@polkadot/api";
import fs from "node:fs/promises";
import { signAndSendAndInclude, waitSessions } from "../../util/block.ts";
import type { Signer } from "ethers";

describeSuite({
    id: "ZOMBIETANSSIR01",
    title: "Zombie Tanssi Relay Unneeded Para Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
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
            test: async () => {
                const relayNetwork = relayApi.consts.system.version.specName.toString();
                expect(relayNetwork, "Relay API incorrect").to.contain("dancelight");
                const blockNum = (await relayApi.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T02",
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
            id: "T03",
            timeout: 600000,
            title: "Test assignation did not change",
            test: async () => {
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
            test: async () => {
                const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
                expect(blockNum).to.be.greaterThan(0);
            },
        });

        it({
            id: "T05",
            title: "Blocks are being produced on container 2001",
            test: async () => {
                const blockNum = (await container2001Api.rpc.chain.getBlock()).block.header.number.toNumber();

                expect(blockNum).to.be.greaterThan(0);
                expect(await ethersSigner.provider.getBlockNumber(), "Safe tag is not present").to.be.greaterThan(0);
            },
        });

        it({
            id: "T06",
            title: "Test container chain 2000 assignation is correct",
            test: async () => {
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
            id: "T08",
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
            test: async () => {
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
            title: "Set 0 credits for all chains",
            timeout: 60000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const tx = relayApi.tx.utility.batchAll([
                    relayApi.tx.sudo.sudo(relayApi.tx.servicesPayment.setBlockProductionCredits(2000, 0)),
                    relayApi.tx.sudo.sudo(relayApi.tx.servicesPayment.setBlockProductionCredits(2001, 0)),
                ]);

                const { blockHash } = await signAndSendAndInclude(tx, alice);

                // Assert that the batch succeeded with no error
                const apiAt = await relayApi.at(blockHash);
                const events = await apiAt.query.system.events();
                const ev1 = events.filter((a) => {
                    return a.event.method === "BatchCompleted";
                });
                expect(ev1.length).to.be.equal(1);
            },
        });

        it({
            id: "T12b",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T12c",
            title: "Wait 2 sessions more",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T13",
            title: "Collators have been unassigned",
            timeout: 180000,
            test: async () => {
                const currentSession = (await relayApi.query.session.currentIndex()).toNumber();
                const containerChainCollators = (
                    await relayApi.query.tanssiAuthorityAssignment.collatorContainerChain(currentSession)
                ).toJSON().containerChains;

                const emptyCollators = {};

                expect(containerChainCollators).to.deep.equal(emptyCollators);
            },
        });

        it({
            id: "T14",
            title: "Manually check validator logs",
            test: async () => {
                // Need to do this manually: check relay chain logs.
                // Some validators should have this message:
                // 2024-11-27 15:04:30.423 DEBUG tokio-runtime-worker parachain::collator-protocol: Declared as collator for unneeded para. Current assignments: {} peer_id=PeerId("12D3KooWPJT4QoqgwDWJzHZHDL8iCgkjKzswTwWGcYHHfEjBEerv") collator_id=Public(8e6e0feedba7494a19662e3178bc66b6801716ee4c12e304c78fde02cc96941c (14DkVhzA...)) para_id=Id(2001)
                // If the bug is fixed this should happen for around 3 seconds at most.
                // If it happens every second and doesn't stop, it means the bug is still present.
            },
        });

        it({
            id: "T18",
            title: "Check validator logs",
            test: async () => {
                // This test will always fail as per comment above, even if the issue is fixed it can still happen right when collators get de-assigned

                const logs = ["/alice.log", "/bob.log", "/charlie.log", "/dave.log"];
                for (const log of logs) {
                    const logFilePath = getTmpZombiePath() + log;
                    await checkLogsNotExist(logFilePath, ["Declared as collator for unneeded para."]);
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
