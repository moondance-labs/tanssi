import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "utils";

describeSuite({
    id: "COMMO1101",
    title: "Registrar test suite: de-register",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        beforeAll(() => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E03",
            title: "Checking that fetching registered paraIds is possible",
            test: async () => {
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                // These are registered in genesis
                expect(parasRegistered).to.contain(2000);
                expect(parasRegistered).to.contain(2001);

                // Set storage of pallet_author_noting and pallet_services_payment to test that it gets deleted later
                const tx1 = polkadotJs.tx.authorNoting.setAuthor(2000, 1, alice.address, 1);
                const tx2 = polkadotJs.tx.authorNoting.setAuthor(2001, 1, alice.address, 1);
                await polkadotJs.tx.sudo.sudo(polkadotJs.tx.utility.batchAll([tx1, tx2])).signAndSend(alice);

                // Credits already exist
                const credits2000 = (await polkadotJs.query.servicesPayment.blockProductionCredits(2000)).toJSON();
                expect(credits2000).toBeGreaterThan(0);
                const credits2001 = (await polkadotJs.query.servicesPayment.blockProductionCredits(2001)).toJSON();
                expect(credits2001).toBeGreaterThan(0);
            },
        });

        it({
            id: "E04",
            title: "Checking that de-registering paraIds is possible",
            test: async () => {
                await context.createBlock();

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const tx = polkadotJs.tx.registrar.deregister(2001);
                await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);

                await context.createBlock();

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2000]);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);

                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000]);
            },
        });

        it({
            id: "E05",
            title: "Checking that de-registering all paraIds does not leave extra keys in storage",
            test: async () => {
                await context.createBlock();

                // Check the number of keys in storage
                const palletKeysWithOnePara = await polkadotJs.rpc.state.getKeys("0x3fba98689ebed1138735e0e7a5a790ab");
                // 5 fixed keys + genesis data
                expect(palletKeysWithOnePara.length).to.be.eq(6);

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const tx = polkadotJs.tx.registrar.deregister(2000);
                await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);

                await context.createBlock();

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([]);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);

                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([]);

                // Check the number of keys in storage
                const palletKeysOnSessionChange = await polkadotJs.rpc.state.getKeys(
                    "0x3fba98689ebed1138735e0e7a5a790ab"
                );

                // 5 keys: Version, RegisteredParas, PendingParas, PendingToRemove, PendingParathreadParams, BufferedParasToRegister
                expect(palletKeysOnSessionChange.length).to.be.eq(6);
                // After one block BufferedParasToRegister should be cleaned
                await context.createBlock();
                // Check the number of keys in storage
                const palletKeysAfterSessionChange = await polkadotJs.rpc.state.getKeys(
                    "0x3fba98689ebed1138735e0e7a5a790ab"
                );
                // 5 keys: Version, RegisteredParas, PendingParas, PendingToRemove, PendingParathreadParams
                expect(palletKeysAfterSessionChange.length).to.be.eq(5);

                // Check that deregistered hook cleared storage of pallet_author_noting and pallet_services_payment
                const authorData2000 = (await polkadotJs.query.authorNoting.latestAuthor(2000)).toJSON();
                expect(authorData2000).to.be.null;
                const authorData2001 = (await polkadotJs.query.authorNoting.latestAuthor(2001)).toJSON();
                expect(authorData2001).to.be.null;

                const credits2000 = (await polkadotJs.query.servicesPayment.blockProductionCredits(2000)).toJSON();
                expect(credits2000).to.be.null;
                const credits2001 = (await polkadotJs.query.servicesPayment.blockProductionCredits(2001)).toJSON();
                expect(credits2001).to.be.null;
            },
        });
    },
});
