import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { fetchStorageProofFromValidationData, generateEmptyGenesisData, jumpSessions } from "utils";
import type { DpContainerChainGenesisDataContainerChainGenesisData } from "@polkadot/types/lookup";

describeSuite({
    id: "DEV0302",
    title: "Registrar test suite: register with relay proof",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let relayManager: KeyringPair;
        let containerChainGenesisData: DpContainerChainGenesisDataContainerChainGenesisData;

        beforeAll(() => {
            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();

            // We must generate the same account as in service.rs, using get_ed25519_pairs
            // Instead of having to implement that function in javascript, we can just hardcode the private key here
            // Create a new keyring because we need to use ed25519
            const relayManagerPrivateKey = "0x3132333435363738393031323334353637383930313233343536373839303132";
            const relayKeyring = new Keyring({
                type: "ed25519",
            });
            relayManager = relayKeyring.addFromUri(relayManagerPrivateKey);
            containerChainGenesisData = generateEmptyGenesisData(context.pjsApi);
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
                const { relayProofBlockNumber, relayStorageProof } =
                    await fetchStorageProofFromValidationData(polkadotJs);

                const parathreadParams = null;
                const relayStorageRoots = await polkadotJs.query.relayStorageRoots.relayStorageRootKeys();
                const lastStoredRelayBlockNumber = relayStorageRoots.toJSON()[relayStorageRoots.toJSON().length - 1];
                const lastRelayStorageRoot =
                    await polkadotJs.query.relayStorageRoots.relayStorageRoot(lastStoredRelayBlockNumber);

                // message: paraId || account (alice) || lastRelayStorageRoot
                const message = new Uint8Array([
                    ...new Uint8Array([0xd2, 0x07, 0x00, 0x00]), // 2002 as a little endian u32
                    ...alice.addressRaw,
                    ...lastRelayStorageRoot.toU8a({ isBare: true }),
                ]);
                // Construct the signature
                const managerSignature = { Ed25519: relayManager.sign(message) };

                const tx = polkadotJs.tx.registrar.registerWithRelayProof(
                    2002,
                    parathreadParams,
                    relayProofBlockNumber,
                    relayStorageProof,
                    managerSignature,
                    containerChainGenesisData,
                    null
                );

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const profileTx = polkadotJs.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx2 = polkadotJs.tx.dataPreservers.startAssignment(profileId, 2002, "Free");
                const tx3 = polkadotJs.tx.registrar.markValidForCollating(2002);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await tx.signAsync(alice, { nonce }),
                    await profileTx.signAsync(charlie),
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
    },
});
