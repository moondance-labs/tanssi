import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { extractFeeAuthor, fetchStorageProofFromValidationData, generateEmptyGenesisData, jumpSessions } from "utils";

describeSuite({
    id: "DEV0301",
    title: "Registrar test suite: de-register with relay proof",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        beforeAll(() => {
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            polkadotJs = context.polkadotJs();
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
                const containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);
                const tx = polkadotJs.tx.registrar.register(2003, containerChainGenesisData, null);

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const profileTx = polkadotJs.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx2 = polkadotJs.tx.dataPreservers.startAssignment(profileId, 2003, "Free");

                const tx3 = polkadotJs.tx.registrar.markValidForCollating(2003);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await tx.signAsync(alice, { nonce }),
                    await profileTx.signAsync(bob),
                    await tx2.signAsync(alice, { nonce: nonce.addn(1) }),
                    await polkadotJs.tx.sudo.sudo(tx3).signAsync(alice, { nonce: nonce.addn(2) }),
                ]);

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001, 2003]);

                // Check that the on chain genesis data is set correctly
                const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2003);
                // TODO: fix once we have types
                expect(containerChainGenesisData.toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                // Check the para id has been given some free credits
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(2003)).toJSON();
                expect(credits, "Container chain 2002 should have been given credits").toBeGreaterThan(0);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);
            },
        });

        it({
            id: "E03",
            title: "Checking that fetching registered paraIds is possible",
            test: async () => {
                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2003]);

                // Set storage of pallet_author_noting and pallet_services_payment to test that it gets deleted later
                const tx1 = polkadotJs.tx.authorNoting.setAuthor(2003, 1, alice.address, 1);
                await polkadotJs.tx.sudo.sudo(tx1).signAndSend(alice);
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

                const balanceBeforeAlice = (await polkadotJs.query.system.account(alice.address)).data;
                const balanceBeforeBob = (await polkadotJs.query.system.account(bob.address)).data;

                const { relayProofBlockNumber, relayStorageProof } =
                    await fetchStorageProofFromValidationData(polkadotJs);
                const tx = polkadotJs.tx.registrar.deregisterWithRelayProof(
                    2003,
                    relayProofBlockNumber,
                    relayStorageProof
                );
                await tx.signAndSend(bob);

                await context.createBlock();

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001]);

                const balanceAfterAlice = (await polkadotJs.query.system.account(alice.address)).data;
                const balanceAfterBob = (await polkadotJs.query.system.account(bob.address)).data;

                const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2003);
                const pricePerByte = 1000000000000n / 30000n;
                const expectedDepositValue = BigInt(onChainGenesisData.unwrap().toU8a().length) * pricePerByte;

                expect(balanceBeforeAlice.reserved.toBigInt()).to.be.eq(expectedDepositValue);
                expect(balanceAfterAlice.reserved.toBigInt()).to.be.eq(0n);

                const events = await polkadotJs.query.system.events();
                const fee = extractFeeAuthor(events, bob.address).amount.toBigInt();
                expect(balanceAfterBob.free.toBigInt()).toEqual(
                    balanceBeforeBob.free.toBigInt() + expectedDepositValue - fee
                );

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
