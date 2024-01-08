import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { generateKeyringPair, KeyringPair } from "@moonwall/util";
import { jumpSessions } from "util/block";

describeSuite({

    id: "CT0701",
    title: "Identity pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let registrar_bob: KeyringPair;
        let general_user_charlie: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            registrar_bob = context.keyring.bob;
            general_user_charlie = context.keyring.charlie;
        });

        it({
            id: "E01",
            title: "Sudo account can add registrars",
            test: async function () {

                const initial_identity_registrars = await polkadotJs.query.identity.registrars();
                
                const tx = polkadotJs.tx.identity.addRegistrar( {
                    Id: registrar_bob.address
                });
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);
                await context.createBlock([signedTx]);

                const identity_registrars = await polkadotJs.query.identity.registrars();
                
                // Added one registrar
                expect(initial_identity_registrars.length).to.equal(identity_registrars.length - 1);

                // Bob is included in the registrars list
                const bob_exists = identity_registrars.toArray().filter(registrar => 
                    registrar.toJSON().account == registrar_bob.address
                );
                expect(bob_exists.length).to.be.equal(1);

                // Registrar addition shows in the events
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method == "RegistrarAdded";
                });
                expect(eventCount.length).to.be.equal(1);
            },
        });

        it({
            id: "E02",
            title: "Non-Sudo account fails when adding registrars",
            test: async function () {

                const initial_identity_registrars = await polkadotJs.query.identity.registrars();
                
                const tx = polkadotJs.tx.identity.addRegistrar( {
                    Id: registrar_bob.address
                });
                const signedTx = await tx.signAsync(general_user_charlie);
                await context.createBlock([signedTx]);

                const identity_registrars = await polkadotJs.query.identity.registrars();
                
                // No registrars added
                expect(initial_identity_registrars.length).to.equal(identity_registrars.length);
                // No addition event
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method == "RegistrarAdded";
                });
                expect(eventCount.length).to.be.equal(0);
            },
        });

    },
});
