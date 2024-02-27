import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "DT0201",
    title: "Removing session keys assignment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;

            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that removing a session key makes the key dissappear from eligibility",
            test: async function () {
                // Bob is an invulnerable, but the keys will be removed and we will see what happens
                const bobKey = (await polkadotJs.query.session.nextKeys(bob.address)).toJSON().nimbus;
                const aliceKey = (await polkadotJs.query.session.nextKeys(alice.address)).toJSON().nimbus;
                // Bob's key should be an authority
                const authoritiesGenesis = await polkadotJs.query.authorityAssignment.collatorContainerChain(0);
                expect(authoritiesGenesis.toJSON()["containerChains"]["2000"]).toContainEqual(bobKey);

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
