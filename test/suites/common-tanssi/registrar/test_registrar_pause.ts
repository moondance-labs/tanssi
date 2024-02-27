import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
    id: "CT0504",
    title: "Registrar test suite: pause",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(() => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Checking that fetching registered paraIds is possible",
            test: async function () {
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                // These are registered in genesis
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);
            },
        });

        it({
            id: "E02",
            title: "Checking that pausing paraIds is possible",
            test: async function () {
                await context.createBlock();

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const tx = polkadotJs.tx.registrar.pauseContainerChain(2001);
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
            id: "E03",
            title: "Checking that unpausing paraIds is possible",
            test: async function () {
                await context.createBlock();

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const tx = polkadotJs.tx.registrar.unpauseContainerChain(2001);
                await polkadotJs.tx.sudo.sudo(tx).signAndSend(alice);

                await context.createBlock();

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001]);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);

                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);
            },
        });
    },
});
