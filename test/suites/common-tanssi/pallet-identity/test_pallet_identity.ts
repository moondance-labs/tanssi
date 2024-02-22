import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { hexToString } from "viem";

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

                const tx = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
                });
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);
                await context.createBlock([signedTx]);

                const identity_registrars = await polkadotJs.query.identity.registrars();

                // Added one registrar
                expect(initial_identity_registrars.length + 1).to.equal(identity_registrars.length);

                // Bob is included in the registrars list
                const bob_exists = identity_registrars
                    .toArray()
                    .filter((registrar) => registrar.toJSON().account == registrar_bob.address);
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

                const tx = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
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

        it({
            id: "E03",
            title: "User sets its identity",
            test: async function () {
                const tx = polkadotJs.tx.identity.setIdentity({
                    display: { raw: "0x49742773206D652C20436861726C6965" },
                    web: { raw: "0x68747470733A2F2F636861726C69652E696F" },
                });
                const signedTx = await tx.signAsync(general_user_charlie);
                await context.createBlock([signedTx]);

                const charlie_identity = await polkadotJs.query.identity.identityOf(general_user_charlie.address);
                // Display has been set
                const charlie_display = hexToString(charlie_identity.toJSON()[0].info.display["raw"]);
                expect(charlie_display).to.equal("It's me, Charlie");

                // Web has been set
                const charlie_web = hexToString(charlie_identity.toJSON()[0].info.web["raw"]);
                expect(charlie_web).to.equal("https://charlie.io");

                // Event triggered
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method == "IdentitySet";
                });
                expect(eventCount.length).to.be.equal(1);

                // Currency reserved as deposit from Charlie's account
                const charlie_balance = await polkadotJs.query.system.account(general_user_charlie.address);
                const charlie_balance_reserved = charlie_balance.toJSON().data.reserved;
                const expected_reserve = 13010000000000; // Basic deposit (1 item, 301 bytes)
                expect(charlie_balance_reserved).to.be.equal(expected_reserve);
            },
        });

        it({
            id: "E04",
            title: "Registrar sets fee and fields",
            test: async function () {
                await context.createBlock();

                const tx1 = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
                });
                const signedTx1 = await polkadotJs.tx.sudo.sudo(tx1).signAsync(sudo_alice);
                await context.createBlock([signedTx1]);

                const tx2 = polkadotJs.tx.identity.setFee(0, 100);
                const signedTx2 = await tx2.signAsync(registrar_bob);
                await context.createBlock([signedTx2]);

                const tx3 = polkadotJs.tx.identity.setFields(0, 2); // 2 as fields equals Display + Web
                const signedTx3 = await tx3.signAsync(registrar_bob);
                await context.createBlock([signedTx3]);

                const identity_registrars = await polkadotJs.query.identity.registrars();
                const bob_registrar_on_chain = identity_registrars.toArray()[0].toJSON();

                expect(bob_registrar_on_chain.fee).to.be.equal(100);
                expect(bob_registrar_on_chain.fields).to.be.equal(2);
            },
        });
    },
});
