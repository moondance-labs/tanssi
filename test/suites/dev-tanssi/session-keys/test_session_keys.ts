import { describeSuite, expect, beforeAll} from "@moonwall/cli";
import { setupLogger } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";
import { u8aToHex } from '@polkadot/util';

import "@polkadot/api-augment";

describeSuite({
  id: "D04",
  title: "Session keys test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    const anotherLogger = setupLogger("anotherLogger");
    let alice, bob;
    beforeAll(() => {
      const keyring = new Keyring({ type: 'sr25519' });
      alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
      bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
      polkadotJs = context.polkadotJs();
    });

    it({
        id: "E01",
        title: "Checking that session keys are correct on genesis",
        test: async function () {
            // for session 0
            const keys = await polkadotJs.query.authorityMapping.authorityIdMapping(0);
            // TODO: fix once we have types
            expect(keys.toJSON()[u8aToHex(alice.publicKey)]).to.be.eq(alice.address);
            expect(keys.toJSON()[u8aToHex(bob.publicKey)]).to.be.eq(bob.address);

            // Check authorities are correct
            const authorities = (await polkadotJs.query.aura.authorities());
            // TODO: fix once we have types
            expect(authorities.toJSON()).to.deep.equal([u8aToHex(alice.publicKey)]);
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
            const result = queuedKeys.filter(keyItem => keyItem[1].nimbus == newKey);
            expect(result).is.empty;
            const nextKey = await polkadotJs.query.session.nextKeys(alice.address);
            expect(u8aToHex(nextKey.unwrap().nimbus)).to.be.eq(u8aToHex(newKey));

            // Let's jump one session
            await jumpSessions(context, 1);

            // The key should be queued at this point, to be applied on the next session
            const queuedKeysSession1 = await polkadotJs.query.session.queuedKeys();

            const result1 = queuedKeysSession1.filter(keyItem => 
                u8aToHex(keyItem[1].nimbus) == u8aToHex(newKey)
            );
            expect(result1.length).to.be.eq(1);

            // Let's jump one more session
            await jumpSessions(context, 1);

            // The change should have been applied, and now both aura and authorityMapping should reflect
            const keys = await polkadotJs.query.authorityMapping.authorityIdMapping(2);
            // TODO: fix once we have types
            expect(keys.toJSON()[u8aToHex(newKey)]).to.be.eq(alice.address);

            const authorities = (await polkadotJs.query.aura.authorities());
            // TODO: fix once we have types
            expect(authorities.toJSON()).to.deep.equal([
                u8aToHex(newKey),
            ]);

            // AuthorityMapping should no-longer contain the session 0 keys
            expect((await polkadotJs.query.authorityMapping.authorityIdMapping(0)).isNone).to.be.true;
        },
    });
    },
});
