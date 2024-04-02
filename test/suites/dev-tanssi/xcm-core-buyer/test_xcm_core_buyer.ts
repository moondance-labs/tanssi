import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block.ts";

describeSuite({
    id: "DT0601",
    title: "Pallet XCM core buyer",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Sudo can set XCM weights storage",
            test: async function () {
                // 1st block
                const tx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.xcmCoreBuyer.setRelayXcmWeightConfig({
                        buyExecutionCost: 50_000_000,
                        weightAtMost: {
                            refTime: 1_000_000_000,
                            proofSize: 100_000,
                        },
                    })
                );
                await context.createBlock([await tx.signAsync(alice)]);

                const storageWeights = await polkadotJs.query.xcmCoreBuyer.relayXcmWeightConfig();
                expect(storageWeights.isSome).to.be.eq(true);
            },
        });

        it({
            id: "E02",
            title: "Register para id 2002 as a parathread and assign collators to it",
            test: async function () {
                const currentSesssion = await polkadotJs.query.session.currentIndex();
                const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
                const expectedScheduledOnboarding =
                    BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

                const slotFrequency = polkadotJs.createType("TpTraitsSlotFrequency", {
                    min: 5,
                    max: 5,
                });
                const emptyGenesisData = () => {
                    const g = polkadotJs.createType("TpContainerChainGenesisDataContainerChainGenesisData", {
                        storage: [
                            {
                                key: "0x636f6465",
                                value: "0x010203040506",
                            },
                        ],
                        name: "0x436f6e7461696e657220436861696e2032303030",
                        id: "0x636f6e7461696e65722d636861696e2d32303030",
                        forkId: null,
                        extensions: "0x",
                        properties: {
                            tokenMetadata: {
                                tokenSymbol: "0x61626364",
                                ss58Format: 42,
                                tokenDecimals: 12,
                            },
                            isEthereum: false,
                        },
                    });
                    return g;
                };
                const containerChainGenesisData = emptyGenesisData();
                const bootNodes = [
                    "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                ];

                const tx = polkadotJs.tx.registrar.registerParathread(2002, slotFrequency, containerChainGenesisData);
                const tx2 = polkadotJs.tx.dataPreservers.setBootNodes(2002, bootNodes);
                const tx3 = polkadotJs.tx.registrar.markValidForCollating(2002);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await tx.signAsync(alice, { nonce }),
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
                expect(emptyGenesisData().toJSON()).to.deep.equal(onChainGenesisData.toJSON());

                // Check the para id has been given some free credits
                const credits = (await polkadotJs.query.servicesPayment.blockProductionCredits(2002)).toJSON();
                expect(credits, "Container chain 2002 should have been given credits").toBeGreaterThan(0);

                // Checking that in session 2 paras are registered
                await jumpSessions(context, 2);

                // Expect now paraIds to be registered
                const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
                // TODO: fix once we have types
                expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2002]);

                // Check that collators have been assigned
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(collators.toJSON().containerChains[2002].length).to.be.greaterThan(0);
            },
        });

        it({
            id: "E03",
            title: "Sudo can forceBuyCore",
            test: async function () {
                const encodedMsgBefore = await polkadotJs.query.parachainSystem.upwardMessages();
                expect(encodedMsgBefore.length).to.be.eq(0);

                const paraId = 2002;
                const tx = polkadotJs.tx.sudo.sudo(polkadotJs.tx.xcmCoreBuyer.forceBuyCore(paraId));
                await context.createBlock([await tx.signAsync(alice)]);

                const events = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "BuyCoreXcmSent";
                });
                expect(events.length).to.be.equal(1);

                // Check that the XCM message has been sent. This returns an encoded message
                const encodedMsg = await polkadotJs.query.parachainSystem.upwardMessages();
                expect(encodedMsg.length).to.be.eq(1);
            },
        });
    },
});
