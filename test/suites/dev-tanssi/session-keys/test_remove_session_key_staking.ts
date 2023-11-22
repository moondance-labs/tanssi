import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { DANCE, STAKING_ACCOUNT } from "util/constants";

describeSuite({
    id: "DT0304",
    title: "Removing session keys assignment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();

            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();
            let bobNonce = (await polkadotJs.rpc.system.accountNextIndex(bob.address)).toNumber();

            // We need to remove from invulnerables and add to staking
            // for that we need to remove Alice and Bob from invulnerables first

            // Additionally, we need to pass to the staking account the minimum balance
            // We delegate with manual rewards to make sure the candidate does not update position
            const existentialDeposit = polkadotJs.consts.balances.existentialDeposit;

            await context.createBlock([
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.invulnerables.removeInvulnerable(alice.address))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.invulnerables.removeInvulnerable(bob.address))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "ManualRewards", 10000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "ManualRewards", 10000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
                await polkadotJs.tx.balances
                    .transfer(STAKING_ACCOUNT, existentialDeposit)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
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
