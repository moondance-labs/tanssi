import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { DANCE } from "util/constants";
import { createBlockAndRemoveInvulnerables } from "util/invulnerables";

describeSuite({
    id: "DT0202",
    title: "Removing session keys assignment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();

            // We need to remove all the invulnerables and add to staking
            // Remove all invulnerables, otherwise they have priority
            await createBlockAndRemoveInvulnerables(context, alice);

            const invulnerables = await polkadotJs.query.invulnerables.invulnerables();
            expect(invulnerables.length).to.be.equal(0);

            // We delegate with manual rewards to make sure the candidate does not update position
            // We also need charlie to join staking because the settings for the dev environment are 1 collator for
            // tanssi and 2 collators for containers, so we need 3 collators for bob to be assigned.
            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();
            let bobNonce = (await polkadotJs.rpc.system.accountNextIndex(bob.address)).toNumber();

            await context.createBlock([
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "ManualRewards", 10000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "ManualRewards", 10000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(charlie.address, "ManualRewards", 10000n * DANCE)
                    .signAsync(context.keyring.charlie, { nonce: 0 }),
            ]);
            // At least 2 sessions for the change to have effect
            await jumpSessions(context, 2);
        });

        it({
            id: "E01",
            title: "Checking that removing a session key makes the key dissappear from eligibility",
            test: async function () {
                // Bob is a staking candidate, but the keys will be removed and we will see what happens
                const bobKey = (await polkadotJs.query.session.nextKeys(bob.address)).toJSON().nimbus;
                const aliceKey = (await polkadotJs.query.session.nextKeys(alice.address)).toJSON().nimbus;
                const currentSessionBeforePurge = await polkadotJs.query.session.currentIndex();

                // Bob's key should be an authority
                const authoritiesBeforePurge = await polkadotJs.query.authorityAssignment.collatorContainerChain(
                    currentSessionBeforePurge
                );
                expect(authoritiesBeforePurge.toJSON()["containerChains"]["2000"]).toContainEqual(bobKey);

                // now purge keys
                await polkadotJs.tx.session.purgeKeys().signAndSend(bob);

                // Let's jump two sessions
                await jumpSessions(context, 2);

                // Bob's key should no longer be an authority
                const currentSession = await polkadotJs.query.session.currentIndex();
                const authorities = await polkadotJs.query.authorityAssignment.collatorContainerChain(currentSession);
                // Bob is no longer an authority, but alice is
                expect(authorities.toJSON().orchestratorChain).not.toContainEqual(bobKey);
                expect(authorities.toJSON().orchestratorChain).toContainEqual(aliceKey);
                expect(authorities.toJSON()["containerChains"]["2000"]).not.toContainEqual(bobKey);
                expect(authorities.toJSON()["containerChains"]["2001"]).not.toContainEqual(bobKey);

                // But not only authority assignment, collator assignment should also not have bob
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                // Bob is no longer an assigned collator, but alice is
                expect(collators.toJSON().orchestratorChain).not.toContainEqual(bob.address);
                expect(collators.toJSON().orchestratorChain).toContainEqual(alice.address);
                expect(collators.toJSON()["containerChains"]["2000"]).not.toContainEqual(bob.address);
                expect(collators.toJSON()["containerChains"]["2001"]).not.toContainEqual(bob.address);
            },
        });
    },
});
