import "@polkadot/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { u8aToHex } from "@polkadot/util";

describeSuite({
    id: "DT0203",
    title: "Session keys assignment test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;

            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that authority assignment is correct on genesis",
            test: async function () {
                // for session 0
                // TODO: fix once we have types
                const assignment0 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(0))
                    .unwrap()
                    .toJSON();
                const assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1))
                    .unwrap()
                    .toJSON();

                expect(assignment0.orchestratorChain).to.deep.equal([u8aToHex(alice.publicKey)]);
                expect(assignment0.containerChains).to.deep.equal({
                    2000: [u8aToHex(bob.publicKey), u8aToHex(charlie.publicKey)],
                    2001: [],
                });

                // Session 1 is the same as session 0
                expect(assignment0).to.deep.equal(assignment1);
                // Session 2 is empty
                expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(2)).isNone).to.be.true;

                // Check authorities are correct
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                const authorities = await polkadotJs.query.authorityAssignment.collatorContainerChain(sessionIndex);
                expect(authorities.toJSON().orchestratorChain).to.deep.equal([u8aToHex(alice.publicKey)]);
            },
        });

        it({
            id: "E02",
            title: "Checking that session keys can be changed and are reflected",
            test: async function () {
                const newKey = await polkadotJs.rpc.author.rotateKeys();
                await polkadotJs.tx.session.setKeys(newKey, []).signAndSend(alice);

                await context.createBlock();
                // Check key is reflected in next key
                // But its not yet in queued
                const queuedKeys = await polkadotJs.query.session.queuedKeys();
                const result = queuedKeys.filter((keyItem) => keyItem[1].nimbus == newKey);
                expect(result).is.empty;
                const nextKey = await polkadotJs.query.session.nextKeys(alice.address);
                expect(u8aToHex(nextKey.unwrap().nimbus)).to.be.eq(u8aToHex(newKey));

                // TODO: fix once we have types
                const initial_assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1))
                    .unwrap()
                    .toJSON();

                // Let's jump one session
                await jumpSessions(context, 1);

                // The key should be queued at this point, to be applied on the next session
                const queuedKeysSession1 = await polkadotJs.query.session.queuedKeys();

                const result1 = queuedKeysSession1.filter((keyItem) => u8aToHex(keyItem[1].nimbus) == u8aToHex(newKey));
                expect(result1.length).to.be.eq(1);

                expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(0)).isNone).to.be.true;
                // TODO: fix once we have types
                const assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1))
                    .unwrap()
                    .toJSON();
                const assignment2 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(2))
                    .unwrap()
                    .toJSON();
                expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(3)).isNone).to.be.true;

                // Assignment for session 1 did not change
                expect(assignment1).to.deep.equal(initial_assignment1);

                // Assignment for session 2 uses the new keys
                expect(assignment2.orchestratorChain).to.deep.equal([
                    // This is alice's new key
                    u8aToHex(newKey),
                ]);
                expect(assignment2.containerChains).to.deep.equal({
                    2000: [u8aToHex(bob.publicKey), u8aToHex(charlie.publicKey)],
                    2001: [],
                });

                // Let's jump one more session
                await jumpSessions(context, 1);

                // The change should have been applied, and now both aura and authorityMapping should reflect
                const keys = await polkadotJs.query.authorityMapping.authorityIdMapping(2);
                // TODO: fix once we have types
                expect(keys.toJSON()[u8aToHex(newKey)]).to.be.eq(alice.address);

                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                const authorities = await polkadotJs.query.authorityAssignment.collatorContainerChain(sessionIndex);
                // TODO: fix once we have types
                expect(authorities.toJSON().orchestratorChain).to.deep.equal([u8aToHex(newKey)]);
                // AuthorityMapping should no-longer contain the session 1
                expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(1)).isNone).to.be.true;
            },
        });
    },
});
