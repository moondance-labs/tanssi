import "@tanssi/api-augment";
import {describeSuite, expect, beforeAll, customDevRpcRequest} from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { jumpBlocks, jumpSessions, jumpToSession } from "util/block";
import { filterAndApply, generateKeyringPair } from "@moonwall/util";
import { EventRecord } from "@polkadot/types/interfaces";
import { bool, u32, u8, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "DTR0303",
    title: "Collator assignment tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice;
        let bob;
        let charlie;
        let dave;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;

            // Enable randomness for this test
            await customDevRpcRequest("mock_activateRandomness", []);
        });

        it({
            id: "E01",
            title: "Collator should rotate",
            test: async function () {

                const orchestrator = null;
                const parachain = { KeepCollators: { keep: 1 } };
                const parathread = { KeepPerbill: { percentage: 500_000_000n } }; // 50%
                const tx = context.polkadotJs().tx.configuration.setFullRotationMode(orchestrator, parachain, parathread);
                await context.createBlock(polkadotJs.tx.sudo.sudo(tx).signAsync(alice));
                const tx2 = context.polkadotJs().tx.configuration.setFullRotationPeriod(3); // Full rotation every 3 sessions
                await context.createBlock(polkadotJs.tx.sudo.sudo(tx2).signAsync(alice));

                // TODO: need to register parathread to be able to test different modes

                // Add 2 collators more
                // Use random accounts
                let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();
                const randomAccounts = [];

                for (let i = 0; i < 2; i++) {
                    const randomAccount = generateKeyringPair("sr25519");
                    randomAccounts.push(randomAccount);
                }

                // First block, send some balance to each account. This needs to go first because `.signAndSend(randomAccount)`
                // given an error if the account has no balance, even though we send some balance and it's pending.
                for (let randomAccount of randomAccounts) {
                    const value = 100_000_000_000n;
                    await polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, value).signAndSend(alice, { nonce: aliceNonce++ });
                }

                await context.createBlock();

                // Second block, add keys and register them as invulnerables
                for (let randomAccount of randomAccounts) {
                    const newKey1 = await polkadotJs.rpc.author.rotateKeys();
                    await polkadotJs.tx.session.setKeys(newKey1, []).signAndSend(randomAccount);

                    await polkadotJs.tx.sudo.sudo(polkadotJs.tx.invulnerables.addInvulnerable(randomAccount.address)).signAndSend(alice, { nonce: aliceNonce++ });
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

                const initialAssignment = (
                    await polkadotJs.query.collatorAssignment.collatorContainerChain()
                ).toJSON();

                console.log("initial assignment: ", initialAssignment);

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

                console.log("rotationEndAssignment: ", rotationEndAssignment);

                // As randomness isn't deterministic in dancelight we can't be
                // 100% certain that the assignation will indeed change. So the
                // best we can do is verify that the pending rotation event for
                // next session is emitted and is a full rotation as expected
                const events = await polkadotJs.query.system.events();
                const filteredEvents = filterAndApply(
                    events,
                    "collatorAssignment",
                    ["NewPendingAssignment"],
                    ({ event }: EventRecord) =>
                        event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
                );
                expect(filteredEvents[0].fullRotation.toJSON()).toBe(true);

                // Check that the randomness is set in CollatorAssignment the
                // block previous to the full rotation
                const sessionDuration = 10;
                await jumpBlocks(context, sessionDuration - 1);
                const assignmentRandomness = await polkadotJs.query.collatorAssignment.randomness();
                expect(assignmentRandomness.isEmpty).toBe(false);

                await jumpSessions(context, 1);
                const rotationRotateAssignment = (
                    await polkadotJs.query.collatorAssignment.collatorContainerChain()
                ).toJSON();
                console.log("rotationRotateAssignment: ", rotationRotateAssignment);
            },
        });
    },
});
