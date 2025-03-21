import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { generateEmptyGenesisData, jumpSessions } from "utils";
import type { TpTraitsSlotFrequency } from "@polkadot/types/lookup";

describeSuite({
    id: "DEV0701",
    title: "Registrar test suite",
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
            test: async () => {
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

                // These are registered in genesis
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);
            },
        });

        it({
            id: "E02",
            title: "Checking that registering paraIds is possible",
            test: async () => {
                await context.createBlock();

                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const slotFrequency = polkadotJs.createType<TpTraitsSlotFrequency>("TpTraitsSlotFrequency", {
                    min: 1,
                    max: 1,
                });
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);

                const tx = polkadotJs.tx.registrar.registerParathread(
                    2002,
                    slotFrequency,
                    containerChainGenesisData,
                    null
                );

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const tx2 = polkadotJs.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx3 = polkadotJs.tx.dataPreservers.startAssignment(profileId, 2002, "Free");
                const tx4 = polkadotJs.tx.registrar.markValidForCollating(2002);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await tx.signAsync(alice, { nonce }),
                    await tx2.signAsync(alice, { nonce: nonce.addn(1) }),
                    await tx3.signAsync(alice, { nonce: nonce.addn(2) }),
                    await polkadotJs.tx.sudo.sudo(tx4).signAsync(alice, { nonce: nonce.addn(3) }),
                ]);

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001, 2002]);

                // Check that the on chain genesis data is set correctly
                const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2002);
                // TODO: fix once we have types
                expect(containerChainGenesisData.toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                // Check the para id has been given some free credits
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(2002)).toJSON();
                expect(credits, "Container chain 2002 should have been given credits").toBeGreaterThan(0);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);

                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2002]);
            },
        });

        it({
            id: "E03",
            title: "Registered paraId has been given free credits, and flag can be cleared",
            test: async () => {
                const paraId = 2002;
                const givenFreeCredits = await polkadotJs.query.servicesPayment.givenFreeCredits(paraId);
                expect(givenFreeCredits.isNone).to.be.false;
                // Test that the storage can be cleared as root
                const tx = polkadotJs.tx.servicesPayment.setGivenFreeCredits(paraId, false);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                // Flag has been cleared
                const givenFreeCredits2 = await polkadotJs.query.servicesPayment.givenFreeCredits(paraId);
                expect(givenFreeCredits2.isNone).to.be.true;
            },
        });

        it({
            id: "E04",
            title: "Parathread params can be changed",
            test: async () => {
                const paraId = 2002;
                const slotFrequency = polkadotJs.createType<TpTraitsSlotFrequency>("TpTraitsSlotFrequency", {
                    min: 2,
                    max: 2,
                });
                const tx = polkadotJs.tx.registrar.setParathreadParams(paraId, slotFrequency);
                await context.createBlock([await polkadotJs.tx.sudo.sudo(tx).signAsync(alice)]);

                // Checking that in session 2 params have changed
                await jumpSessions(context, 2);

                const params = await polkadotJs.query.registrar.parathreadParams(paraId);
                expect(params.unwrap().slotFrequency.toJSON()).to.deep.equal({
                    min: 2,
                    max: 2,
                });
            },
        });
    },
});
