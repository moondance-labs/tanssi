import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, customDevRpcRequest } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { jumpBlocks, jumpSessions, jumpToSession } from "util/block";
import { filterAndApply, generateKeyringPair } from "@moonwall/util";
import type { EventRecord } from "@polkadot/types/interfaces";
import type { bool, u32, u8, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "DEV0204",
    title: "Collator assignment tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            // Enable randomness for this test
            await customDevRpcRequest("mock_activateRandomness", []);
        });

        it({
            id: "E01",
            title: "Collator should rotate",
            test: async () => {
                const orchestrator = "KeepAll";
                const parachain = { KeepCollators: { keep: 1 } };
                const parathread = "RotateAll";
                const tx = context
                    .polkadotJs()
                    .tx.configuration.setFullRotationMode(orchestrator, parachain, parathread);
                await context.createBlock(polkadotJs.tx.sudo.sudo(tx).signAsync(alice));

                // Add 4 collators more
                // Use random accounts
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();
                const randomAccounts = [];

                for (let i = 0; i < 4; i++) {
                    const randomAccount = generateKeyringPair("sr25519");
                    randomAccounts.push(randomAccount);
                }

                // First block, send some balance to each account. This needs to go first because `.signAndSend(randomAccount)`
                // given an error if the account has no balance, even though we send some balance and it's pending.
                for (const randomAccount of randomAccounts) {
                    const value = 100_000_000_000n;
                    await polkadotJs.tx.balances
                        .transferAllowDeath(randomAccount.address, value)
                        .signAndSend(alice, { nonce: aliceNonce++ });
                }

                await context.createBlock();

                // Second block, add keys and register them as invulnerables
                for (const randomAccount of randomAccounts) {
                    const newKey1 = await polkadotJs.rpc.author.rotateKeys();
                    await polkadotJs.tx.session.setKeys(newKey1, []).signAndSend(randomAccount);

                    await polkadotJs.tx.sudo
                        .sudo(polkadotJs.tx.invulnerables.addInvulnerable(randomAccount.address))
                        .signAndSend(alice, { nonce: aliceNonce++ });
                }
                await context.createBlock();

                // Collators are registered, wait 2 sessions for them to be assigned
                await jumpSessions(context, 2);

                const fullRotationPeriod = (await polkadotJs.query.configuration.activeConfig())[
                    "fullRotationPeriod"
                ].toString();
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                // Calculate the remaining sessions for next full rotation
                // This is a workaround for running moonwall in run mode
                // as it runs all tests in the same chain instance
                const remainingSessionsForRotation =
                    sessionIndex > fullRotationPeriod ? sessionIndex % fullRotationPeriod : fullRotationPeriod;

                await jumpToSession(context, remainingSessionsForRotation - 2);

                const initialAssignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();

                expect(initialAssignment.containerChains[2000].length).to.eq(2);
                expect((await polkadotJs.query.collatorAssignment.pendingCollatorContainerChain()).isNone);

                // remainingSessionsForRotation - 1
                await jumpSessions(context, 1);
                const rotationEndAssignment = (
                    await polkadotJs.query.collatorAssignment.collatorContainerChain()
                ).toJSON();

                expect((await polkadotJs.query.collatorAssignment.pendingCollatorContainerChain()).isSome);
                // Assignment shouldn't have changed yet
                expect(initialAssignment.containerChains[2000].toSorted()).to.deep.eq(
                    rotationEndAssignment.containerChains[2000].toSorted()
                );

                // In dev-tanssi, randomness depends only on the block number so it is actually deterministic.
                // First, check that the event has randomness
                const events = await polkadotJs.query.system.events();
                const filteredEvents = filterAndApply(
                    events,
                    "collatorAssignment",
                    ["NewPendingAssignment"],
                    ({ event }: EventRecord) =>
                        event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
                );
                expect(filteredEvents[0].fullRotation.toJSON()).toBe(true);
                // Randomness is deterministic so the seed should not change, but we only want to check that it's not 0x0000...,
                // so it doesn't matter if it changes.
                expect(filteredEvents[0].randomSeed.toHex()).to.deep.eq(
                    "0xf497424c947d1b548a64ce4697f8fa8f84c6d73b7ceedf4d4f9cb65665fb9cc1"
                );

                // Check that the randomness is set in CollatorAssignment the
                // block previous to the full rotation
                const sessionDuration = 10;
                await jumpBlocks(context, sessionDuration - 1);

                const assignmentRandomness = await polkadotJs.query.collatorAssignment.randomness();
                expect(assignmentRandomness.isEmpty).toBe(false);

                // Start session 5, with the new random assignment
                await jumpSessions(context, 1);

                const newAssignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();

                // Assignment should have changed
                expect(newAssignment).to.not.deep.eq(initialAssignment);

                // Orchestrator collators should not change
                expect(newAssignment.orchestratorChain).to.deep.eq(initialAssignment.orchestratorChain);

                const arrayIntersection = (arr1, arr2) => {
                    const set2 = new Set(arr2);
                    return arr1.filter((item) => set2.has(item));
                };

                // Parachain collators should keep 1 and rotate the other one
                expect(newAssignment.containerChains["2000"].length).toBe(2);
                const sameCollators2000 = arrayIntersection(
                    newAssignment.containerChains["2000"],
                    initialAssignment.containerChains["2000"]
                );
                expect(sameCollators2000.length).toBe(1);
                expect(newAssignment.containerChains["2001"].length).toBe(2);
                const sameCollators2001 = arrayIntersection(
                    newAssignment.containerChains["2001"],
                    initialAssignment.containerChains["2001"]
                );
                expect(sameCollators2001.length).toBe(1);
            },
        });
    },
});
