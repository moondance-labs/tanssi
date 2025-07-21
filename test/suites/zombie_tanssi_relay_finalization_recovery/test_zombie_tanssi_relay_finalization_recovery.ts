import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { Signer } from "ethers";
import {
    findValidatorProcessPid,
    getHeaderFromRelay,
    isProcessRunning,
    signAndSendAndInclude,
    waitSessions,
} from "utils";
// (Optional) explicitly import the process global and its types
import process from "node:process";

/**
 * Kill a process by PID (string form) on Linux.
 * @param pidStr - Process ID as a string.
 * @param signal  - Signal to send (e.g. 'SIGTERM', 'SIGKILL') or number. Defaults to 'SIGTERM'.
 */
export async function killProcessByPid(pidStr: string, signal: NodeJS.Signals | number = "SIGTERM"): Promise<void> {
    // Parse and validate PID
    const pid = Number.parseInt(pidStr, 10);
    if (Number.isNaN(pid) || pid <= 0) {
        throw new Error(`Invalid PID: "${pidStr}"`);
    }

    try {
        process.kill(pid, signal);
        console.log(`Successfully sent ${signal} to process ${pid}`);
    } catch (err: any) {
        // No such process
        if (err.code === "ESRCH") {
            console.warn(`No process found with PID ${pid}`);
        }
        // Permission denied
        else if (err.code === "EPERM") {
            console.error(`Permission denied when trying to kill PID ${pid}`);
        }
        // Other errors
        else {
            console.error(`Failed to kill process ${pid}:`, err);
        }
        throw err;
    }
}

describeSuite({
    id: "ZOMBIETANSS01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let relayCharlieApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let ethersSigner: Signer;
        let finalizedBlockStalled: number;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            relayCharlieApi = context.polkadotJs("Tanssi-charlie");
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
            title: "Test add 4 external validators that will not produce blocks",
            timeout: 240000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                // Register keys in pallet_session
                // Use charlie rpc, not sure if charlie will be able to author, I hope not.
                // If charlie starts producing all the blocks, just kill the process.
                async function createKeys(count) {
                    const txs = [];
                    const newValidators = [];

                    for (let i = 0; i < count; i++) {
                        const randomAccount = generateKeyringPair("sr25519");
                        const newKey = await relayCharlieApi.rpc.author.rotateKeys();
                        const setKeysTx = relayApi.tx.session.setKeys(newKey, []);
                        const dispatchTx = relayApi.tx.utility.dispatchAs(
                            {
                                system: { Signed: randomAccount.address },
                            } as any,
                            setKeysTx
                        );

                        newValidators.push(randomAccount.address);
                        txs.push(dispatchTx);
                    }

                    return [txs, newValidators];
                }
                const [txs, newValidators] = await createKeys(4);

                const pidCharlie = await findValidatorProcessPid("charlie");
                expect(isProcessRunning(pidCharlie)).to.be.true;
                killProcessByPid(pidCharlie);

                const txs3 = [];
                for (const newAccount of newValidators) {
                    const tx3 = relayApi.tx.balances.forceSetBalance(newAccount, 1_000_000_000_000_000n);
                    txs3.push(tx3);
                }
                const tx3 = relayApi.tx.sudo.sudo(relayApi.tx.utility.batchAll(txs3));
                await signAndSendAndInclude(tx3, alice);

                // Maybe this match is too big because the session keys are big, if so just send the txs one by one
                const tx = relayApi.tx.sudo.sudo(relayApi.tx.utility.batchAll(txs));

                await signAndSendAndInclude(tx, alice);

                const tx2 = relayApi.tx.sudo.sudo(
                    relayApi.tx.externalValidators.setExternalValidators(newValidators, 0)
                );

                await signAndSendAndInclude(tx2, alice);
            },
        });

        it({
            id: "T13",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T14",
            title: "New validators have been selected, blocks are not being finalized now",
            timeout: 300000,
            test: async () => {
                const validators = await relayApi.query.session.validators();

                console.log("validators", validators.toJSON());
            },
        });

        it({
            id: "T15",
            title: "noteStalled",
            timeout: 300000,
            test: async () => {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

                const lastFinalizedBlock = await relayApi.rpc.chain.getBlock(
                    await relayApi.rpc.chain.getFinalizedHead()
                );
                const lastFinalizedBlockNumber = lastFinalizedBlock.block.header.number.toNumber();
                finalizedBlockStalled = lastFinalizedBlockNumber;
                const tx = relayApi.tx.grandpa.noteStalled(2, lastFinalizedBlockNumber);
                // Do not use signAndSendAndInclude because that waits for the tx to be finalized, and this chain cannot
                // finalize anything until we fix it
                await relayApi.tx.sudo.sudo(tx).signAndSend(alice);
            },
        });

        it({
            id: "T16",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T17",
            title: "Finalization has been fixed",
            timeout: 300000,
            test: async () => {
                const lastFinalizedBlock = await relayApi.rpc.chain.getBlock(
                    await relayApi.rpc.chain.getFinalizedHead()
                );
                const lastFinalizedBlockNumber = lastFinalizedBlock.block.header.number.toNumber();

                // We expect to have more finalized blocks than before the noteStalled. If not, it means that the chain
                // is still stuck and not finalizing blocks.
                expect(lastFinalizedBlockNumber).toBeGreaterThan(finalizedBlockStalled);
            },
        });
    },
});
