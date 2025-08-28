import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { generateKeyringPair, type KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { signAndSendAndInclude, waitSessions } from "utils";

describeSuite({
    id: "ZOMBIETANSS01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;
        let relayCharlieApi: ApiPromise;
        let relayDaveApi: ApiPromise;
        let container2000Api: ApiPromise;
        let container2001Api: ApiPromise;
        let container2002Api: ApiPromise;
        let finalizedBlockStalled: number;
        let alice: KeyringPair;
        let keyring: Keyring;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            relayCharlieApi = context.polkadotJs("Tanssi-charlie");
            relayDaveApi = context.polkadotJs("Tanssi-dave");
            container2000Api = context.polkadotJs("Container2000");
            container2001Api = context.polkadotJs("Container2001");
            container2002Api = context.polkadotJs("Container2002");

            keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

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
            title: "Add Charlie and Dave to the validators list",
            timeout: 240000,
            test: async () => {
                async function createCharlieDaveKeys() {
                    const txs = [];
                    const newValidators = [];

                    for (const relayValidatorApi of [relayCharlieApi, relayDaveApi]) {
                        const randomAccount = generateKeyringPair("sr25519");

                        const newKey = await relayValidatorApi.rpc.author.rotateKeys();

                        newValidators.push(randomAccount.address);
                        txs.push(
                            relayApi.tx.utility.dispatchAs(
                                {
                                    system: { Signed: randomAccount.address },
                                } as any,
                                relayApi.tx.session.setKeys(newKey, [])
                            ),
                            relayApi.tx.sudo.sudo(relayApi.tx.externalValidators.addWhitelisted(randomAccount.address))
                        );
                    }

                    return [txs, newValidators];
                }
                const [txs, newValidators] = await createCharlieDaveKeys();

                const txs3 = [];
                for (const newAccount of newValidators) {
                    txs3.push(relayApi.tx.balances.forceSetBalance(newAccount, 1_000_000_000_000_000n));
                }
                const tx3 = relayApi.tx.sudo.sudo(relayApi.tx.utility.batchAll(txs3));
                await signAndSendAndInclude(tx3, alice);

                // Maybe this match is too big because the session keys are big, if so just send the txs one by one
                const tx = relayApi.tx.sudo.sudo(relayApi.tx.utility.batchAll(txs));

                await signAndSendAndInclude(tx, alice);
            },
        });

        it({
            id: "T02",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T03",
            title: "Add 4 external validators that will not produce blocks",
            timeout: 240000,
            test: async () => {
                const moreValidators = [
                    generateKeyringPair("sr25519"),
                    generateKeyringPair("sr25519"),
                    generateKeyringPair("sr25519"),
                    generateKeyringPair("sr25519"),
                ];

                const moreSessionKeys = [
                    await relayCharlieApi.rpc.author.rotateKeys(),
                    await relayCharlieApi.rpc.author.rotateKeys(),
                    await relayCharlieApi.rpc.author.rotateKeys(),
                    await relayCharlieApi.rpc.author.rotateKeys(),
                ];

                await signAndSendAndInclude(
                    relayApi.tx.sudo.sudo(
                        relayApi.tx.utility.batch(
                            moreValidators.map((validator) =>
                                relayApi.tx.balances.forceSetBalance(validator.address, 1_000_000_000_000_000n)
                            )
                        )
                    ),
                    alice
                );

                await signAndSendAndInclude(
                    relayApi.tx.sudo.sudo(
                        relayApi.tx.utility.batch([
                            ...moreValidators.map((validator, i) =>
                                relayApi.tx.utility.dispatchAs(
                                    {
                                        system: { Signed: validator.address },
                                    } as any,
                                    relayApi.tx.session.setKeys(moreSessionKeys[i], [])
                                )
                            ),
                            relayApi.tx.externalValidators.setExternalValidators(
                                moreValidators.map((pair) => pair.address),
                                0
                            ),
                            relayApi.tx.externalValidators.forceEra("ForceNew"),
                        ])
                    ),
                    alice
                );
            },
        });

        it({
            id: "T04",
            title: "Wait 3 sessions so that finalized block gets stuck (2 sessions to apply new authority + 1 session of stuck finalization)",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 3, null, "Tanssi-relay");
            },
        });

        it({
            id: "T05",
            title: "Finalized block is now stuck",
            timeout: 300000,
            test: async () => {
                // We should have 8 authorities, and 4 of them are not producing blocks
                const authorities = await relayApi.query.grandpa.authorities();
                expect(authorities.toJSON().length).to.equal(8);

                const lastFinalizedBlock = await relayApi.rpc.chain.getBlock(
                    await relayApi.rpc.chain.getFinalizedHead()
                );
                const lastFinalizedBlockNumber = lastFinalizedBlock.block.header.number.toNumber();
                finalizedBlockStalled = lastFinalizedBlockNumber;

                const currentBlock = await relayApi.rpc.chain.getBlock();
                const currentBlockNumber = currentBlock.block.header.number.toNumber();

                console.log(
                    `Finalized block number: ${lastFinalizedBlockNumber}, Latest block number: ${currentBlockNumber}`
                );

                expect(currentBlockNumber - lastFinalizedBlockNumber).greaterThan(
                    5,
                    "Expect diff between latest block and finalized block to be greater than 5"
                );
            },
        });

        it({
            id: "T06",
            title: "Remove faulty validators and force new era",
            timeout: 300000,
            test: async () => {
                // Do not use signAndSendAndInclude because that waits for the tx to be finalized, and this chain cannot
                // finalize anything until we fix it
                await relayApi.tx.sudo
                    .sudo(
                        relayApi.tx.utility.batchAll([
                            relayApi.tx.externalValidators.setExternalValidators([], 0),
                            relayApi.tx.externalValidators.forceEra("ForceNew"),
                        ])
                    )
                    .signAndSend(alice);
            },
        });

        it({
            id: "T07",
            title: "Wait 2 sessions so that faulty validators are removed from grandpa.authorities",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T08",
            title: "Check that faulty validators have been removed from grandpa.authorities",
            timeout: 300000,
            test: async () => {
                const authorities = await relayApi.query.grandpa.authorities();
                expect(authorities.toJSON().length).to.equal(4);
            },
        });

        it({
            id: "T09",
            title: "noteStalled",
            timeout: 300000,
            test: async () => {
                const lastFinalizedBlock = await relayApi.rpc.chain.getBlock(
                    await relayApi.rpc.chain.getFinalizedHead()
                );
                // Make sure finalization still stuck
                const lastFinalizedBlockNumber = lastFinalizedBlock.block.header.number.toNumber();
                expect(finalizedBlockStalled).toEqual(lastFinalizedBlockNumber);

                const tx = relayApi.tx.grandpa.noteStalled(2, finalizedBlockStalled);
                // Do not use signAndSendAndInclude because that waits for the tx to be finalized, and this chain cannot
                // finalize anything until we fix it
                await relayApi.tx.sudo.sudo(tx).signAndSend(alice);
            },
        });

        it({
            id: "T10",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T11",
            title: "Finalization has been fixed",
            timeout: 300000,
            test: async () => {
                const lastFinalizedBlock = await relayApi.rpc.chain.getBlock(
                    await relayApi.rpc.chain.getFinalizedHead()
                );
                const lastFinalizedBlockNumber = lastFinalizedBlock.block.header.number.toNumber();
                const currentBlock = await relayApi.rpc.chain.getBlock();
                const currentBlockNumber = currentBlock.block.header.number.toNumber();

                console.log(
                    `Finalized block number: ${lastFinalizedBlockNumber}, Latest block number: ${currentBlockNumber}`
                );

                // We expect to have more finalized blocks than before the noteStalled. If not, it means that the chain
                // is still stuck and not finalizing blocks.
                expect(currentBlockNumber - lastFinalizedBlockNumber).toBeLessThan(5);
            },
        });
    },
});
