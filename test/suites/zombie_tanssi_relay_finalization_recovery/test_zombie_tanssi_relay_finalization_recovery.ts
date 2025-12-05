// @ts-nocheck

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
        let finalizedBlockStalled: number;
        let alice: KeyringPair;
        let keyring: Keyring;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
            relayCharlieApi = context.polkadotJs("Tanssi-charlie");
            relayDaveApi = context.polkadotJs("Tanssi-dave");

            keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });
        }, 120000);

        it({
            id: "T01",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T02",
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
                    await relayApi.rpc.author.rotateKeys(),
                    await relayApi.rpc.author.rotateKeys(),
                    await relayApi.rpc.author.rotateKeys(),
                    await relayApi.rpc.author.rotateKeys(),
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
            id: "T03",
            title: "Wait 3 sessions so that finalized block gets stuck (2 sessions to apply new authority + 1 session of stuck finalization)",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 3, null, "Tanssi-relay");
            },
        });

        it({
            id: "T04",
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
            id: "T05",
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
            id: "T06",
            title: "Wait 2 sessions so that faulty validators are removed from grandpa.authorities",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T07",
            title: "Check that faulty validators have been removed from grandpa.authorities",
            timeout: 300000,
            test: async () => {
                const authorities = await relayApi.query.grandpa.authorities();
                expect(authorities.toJSON().length).to.equal(4);
            },
        });

        it({
            id: "T08",
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
            id: "T09",
            title: "Wait 2 sessions",
            timeout: 300000,
            test: async () => {
                await waitSessions(context, relayApi, 2, null, "Tanssi-relay");
            },
        });

        it({
            id: "T10",
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
