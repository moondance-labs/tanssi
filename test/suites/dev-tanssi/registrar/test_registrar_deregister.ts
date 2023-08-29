import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "DT0601",
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
            test: async function () {
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                // These are registered in genesis
                expect(parasRegistered).to.contain(2000);
                expect(parasRegistered).to.contain(2001);
            },
        });

        it({
            id: "E04",
            title: "Checking that de-registering paraIds is possible",
            test: async function () {
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
            test: async function () {
                await context.createBlock();

                // Check the number of keys in storage
                const palletKeysWithOnePara = await polkadotJs.rpc.state.getKeys("0x3fba98689ebed1138735e0e7a5a790ab");
                // 3 fixed keys + genesis data + bootnode
                expect(palletKeysWithOnePara.length).to.be.eq(5);

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
                const palletKeys = await polkadotJs.rpc.state.getKeys("0x3fba98689ebed1138735e0e7a5a790ab");
                // 3 keys: version, registeredParas, pendingParas
                expect(palletKeys.length).to.be.eq(3);
            },
        });
    },
});
