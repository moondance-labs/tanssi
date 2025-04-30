import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { hexToString } from "viem";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_IDENTITY, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT1001",
    title: "Identity pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let sudo_alice: KeyringPair;
        let registrar_bob: KeyringPair;
        let general_user_charlie: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightIdentity: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            sudo_alice = context.keyring.alice;
            registrar_bob = context.keyring.bob;
            general_user_charlie = context.keyring.charlie;

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightIdentity =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_IDENTITY.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Sudo account can add registrars",
            test: async () => {
                const initial_identity_registrars = await polkadotJs.query.identity.registrars();

                const tx = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
                });

                if (shouldSkipStarlightIdentity) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(sudo_alice));
                    return;
                }

                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(sudo_alice);
                await context.createBlock([signedTx]);

                const identity_registrars = await polkadotJs.query.identity.registrars();

                // Added one registrar
                expect(initial_identity_registrars.length + 1).to.equal(identity_registrars.length);

                // Bob is included in the registrars list
                const bob_exists = identity_registrars
                    .toArray()
                    .filter((registrar) => registrar.toJSON().account === registrar_bob.address);
                expect(bob_exists.length).to.be.equal(1);

                // Registrar addition shows in the events
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method === "RegistrarAdded";
                });
                expect(eventCount.length).to.be.equal(1);
            },
        });

        it({
            id: "E02",
            title: "Non-Sudo account fails when adding registrars",
            test: async () => {
                const initial_identity_registrars = await polkadotJs.query.identity.registrars();

                const tx = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
                });
                const signedTx = await tx.signAsync(general_user_charlie);

                if (shouldSkipStarlightIdentity) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

                const identity_registrars = await polkadotJs.query.identity.registrars();

                // No registrars added
                expect(initial_identity_registrars.length).to.equal(identity_registrars.length);

                // No addition event
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method === "RegistrarAdded";
                });
                expect(eventCount.length).to.be.equal(0);
            },
        });

        it({
            id: "E03",
            title: "User sets its identity",
            test: async () => {
                const tx = polkadotJs.tx.identity.setIdentity({
                    display: { raw: "0x49742773206D652C20436861726C6965" },
                    web: { raw: "0x68747470733A2F2F636861726C69652E696F" },
                });
                const signedTx = await tx.signAsync(general_user_charlie);

                if (shouldSkipStarlightIdentity) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

                const charlie_identity = await polkadotJs.query.identity.identityOf(general_user_charlie.address);
                // Display has been set
                const charlie_display = hexToString(charlie_identity.toJSON().info.display.raw);
                expect(charlie_display).to.equal("It's me, Charlie");

                // Web has been set
                const charlie_web = hexToString(charlie_identity.toJSON().info.web.raw);
                expect(charlie_web).to.equal("https://charlie.io");

                // Event triggered
                const events = await polkadotJs.query.system.events();
                const eventCount = events.filter((a) => {
                    return a.event.method === "IdentitySet";
                });
                expect(eventCount.length).to.be.equal(1);

                // Currency reserved as deposit from Charlie's account
                const charlie_balance = await polkadotJs.query.system.account(general_user_charlie.address);
                const charlie_balance_reserved = charlie_balance.toJSON().data.reserved;
                const expected_reserve = 463333333000; // Basic deposit (1 item, 301 bytes)
                expect(charlie_balance_reserved).to.be.equal(expected_reserve);
            },
        });

        it({
            id: "E04",
            title: "Registrar sets fee and fields",
            test: async () => {
                await context.createBlock();
                const tx1 = polkadotJs.tx.identity.addRegistrar({
                    Id: registrar_bob.address,
                });

                if (shouldSkipStarlightIdentity) {
                    console.log(`Skipping E03 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx1.signAsync(sudo_alice));

                    // Both setFee and setFields should be filtered as well
                    const tx2 = polkadotJs.tx.identity.setFee(0, 100);
                    await checkCallIsFiltered(context, polkadotJs, await tx2.signAsync(registrar_bob));
                    const tx3 = polkadotJs.tx.identity.setFields(0, 2);
                    await checkCallIsFiltered(context, polkadotJs, await tx3.signAsync(registrar_bob));

                    return;
                }

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
