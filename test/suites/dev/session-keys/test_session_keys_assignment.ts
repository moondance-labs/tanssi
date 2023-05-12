import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { u8aToHex } from '@polkadot/util';

import "@polkadot/api-augment";

describeSuite({
  id: "D05",
  title: "Session keys assignment test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob, charlie, dave;
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      charlie = keyring.addFromUri('//Charlie', { name: 'Charlie default' });
      dave = keyring.addFromUri('//Dave', { name: 'Dave default' });
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Checking that authority assignment is correct on genesis",
        test: async function () {
            // for session 0
            const assignment0 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(0)).unwrap().toHuman();
            const assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1)).unwrap().toHuman();

            expect(assignment0.orchestratorChain).to.deep.equal([
                u8aToHex(alice.publicKey).toString(),
                u8aToHex(bob.publicKey).toString(),
            ]);
            expect(assignment0.containerChains).to.deep.equal({
                2000: [
                    u8aToHex(charlie.publicKey).toString(),
                    u8aToHex(dave.publicKey).toString(),
                ],
                2001: [],
            });

            // Session 1 is the same as session 0
            expect(assignment0).to.deep.equal(assignment1);
            // Session 2 is empty
            expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(2)).isNone).to.be.true;

            // Check authorities are correct
            const authorities = (await polkadotJs.query.aura.authorities());
            expect(u8aToHex(authorities[0])).to.be.eq(u8aToHex(alice.publicKey));
            expect(u8aToHex(authorities[1])).to.be.eq(u8aToHex(bob.publicKey))
        },
    });

    it({
        id: "E02",
        title: "Checking that session keys can be changed and are reflected",
        test: async function () {
            const newKey = await polkadotJs.rpc.author.rotateKeys();
            await polkadotJs.tx.session
                .setKeys(newKey, [] as any)
                .signAndSend(alice);

            await context.createBlock();
            // Check key is reflected in next key
            // But its not yet in queued
            const queuedKeys = await polkadotJs.query.session.queuedKeys();
            const result = queuedKeys.filter(keyItem => keyItem[1].aura == newKey);
            expect(result).is.empty;
            const nextKey = await polkadotJs.query.session.nextKeys(alice.address);
            expect(u8aToHex(nextKey.unwrap().aura)).to.be.eq(u8aToHex(newKey));

            const initial_assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1)).unwrap().toHuman();

            // Let's jump one session
            await jumpSessions(context, 1);

            // The key should be queued at this point, to be applied on the next session
            const queuedKeysSession1 = await polkadotJs.query.session.queuedKeys();

            const result1 = queuedKeysSession1.filter(keyItem => 
                u8aToHex(keyItem[1].aura) == u8aToHex(newKey)
            );
            expect(result1.length).to.be.eq(1);

            expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(0)).isNone).to.be.true;
            const assignment1 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(1)).unwrap().toHuman();
            const assignment2 = (await polkadotJs.query.authorityAssignment.collatorContainerChain(2)).unwrap().toHuman();
            expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(3)).isNone).to.be.true;

            // Assignment for session 1 did not change
            expect(assignment1).to.deep.equal(initial_assignment1);

            // Assignemnt for session 2 uses the new keys
            expect(assignment2.orchestratorChain).to.deep.equal([
                // This is alice's new key
                u8aToHex(newKey).toString(),
                u8aToHex(bob.publicKey).toString(),
            ]);
            expect(assignment2.containerChains).to.deep.equal({
                2000: [
                    u8aToHex(charlie.publicKey).toString(),
                    u8aToHex(dave.publicKey).toString(),
                ],
                2001: [],
            });


            // Let's jump one more session
            await jumpSessions(context, 1);

            // The change should have been applied, and now both aura and authorityMapping should reflect
            const keys = await polkadotJs.query.authorityMapping.authorityIdMapping(2);
            expect(keys.toHuman()[u8aToHex(newKey).toString()]).to.be.eq(alice.address.toString());

            const authorities = (await polkadotJs.query.aura.authorities());
            expect(u8aToHex(authorities[0])).to.be.eq(u8aToHex(newKey));

            // AuthorityMapping should no-longer contain the session 1
            expect((await polkadotJs.query.authorityAssignment.collatorContainerChain(1)).isNone).to.be.true;
        },
    });
    },
});
