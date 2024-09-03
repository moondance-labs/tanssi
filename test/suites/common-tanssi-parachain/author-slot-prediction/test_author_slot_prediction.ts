import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { u8aToHex, stringToHex } from "@polkadot/util";

describeSuite({
    id: "CT0101",
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
                const assignment0 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(0))
                    .unwrap()
                    .toHuman();
                const assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1))
                    .unwrap()
                    .toHuman();
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
                const authorities = await context
                    .polkadotJs()
                    .query.authorityAssignment.collatorContainerChain(sessionIndex);
                expect(authorities.unwrap().orchestratorChain[0].toString()).to.be.eq(u8aToHex(alice.publicKey));
            },
        });

        it({
            id: "E02",
            title: "Checking session key changes are reflected at the session length boundary block",
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

                // Let's jump one session
                await jumpSessions(context, 2);

                // The very first block produced by the second session should contain the new key

                // The change should have been applied, and now both nimbus and authorityMapping should reflect
                const digests = (await polkadotJs.query.system.digest()).logs;
                const filtered = digests.filter(
                    (log) => log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex("nmbs")
                );

                expect(filtered[0].asPreRuntime[1].toHex()).to.be.eq(u8aToHex(nextKey.unwrap().nimbus));
            },
        });
    },
});
