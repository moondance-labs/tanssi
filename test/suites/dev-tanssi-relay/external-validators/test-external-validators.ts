import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "util/block";

describeSuite({
    id: "DTR1501",
    title: "External validators tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Collator should rotate",
            test: async function () {
                const keyring = new Keyring({ type: "sr25519" });
                const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                const aliceStash = keyring.addFromUri("//Alice//stash");
                const bob = keyring.addFromUri("//Bob", { name: "Bob default" });
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                const validators = (await polkadotJs.query.session.validators()).toJSON();

                console.log(validators);
                // TODO: initial validator is not alice?
                // - Expected
                // + Received
                //
                //   Array [
                // -   "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                // +   "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
                //   ]

                expect(validators).to.deep.equal([aliceStash.address]);

                const tx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.addWhitelisted(bob.address))
                    .signAsync(alice);
                await context.createBlock([tx]);

                await jumpToSession(context, 2);

                const validators2 = (await polkadotJs.query.session.validators()).toJSON();
                expect(validators2).to.deep.equal([aliceStash.address]);

                await jumpToSession(context, 3);

                const validators3 = (await polkadotJs.query.session.validators()).toJSON();
                expect(validators3).to.deep.equal([aliceStash.address, bob.address]);
            },
        });
    },
});
