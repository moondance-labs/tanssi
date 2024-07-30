import "@tanssi/api-augment";
import { describeSuite, beforeAll, expect } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { jumpSessions } from "../../../util/block.ts";
import { u64 } from "@polkadot/types-codec";
import { ParaId } from "@polkadot/types/interfaces";
import { ITuple } from "@polkadot/types-codec/types";
import { u8aToHex } from "@polkadot/util";

describeSuite({
    id: "DT0601",
    title: "Pallet XCM core buyer",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let keyring: Keyring;
        let collatorNimbusKey: KeyringPair;
        let collatorAccountKey: KeyringPair;

        beforeAll(async () => {
            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            polkadotJs = context.polkadotJs();
            keyring = new Keyring({ type: "sr25519" });
            collatorNimbusKey = keyring.addFromUri("//" + "COLLATOR_NIMBUS", { name: "COLLATOR" + " NIMBUS" });
            // Collator key of Dave
            collatorAccountKey = keyring.addFromUri("//" + "Bob", { name: "COLLATOR" + " ACCOUNT" });
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
                    min: 1,
                    max: 1,
                });
                const emptyGenesisData = () => {
                    const g = polkadotJs.createType("DpContainerChainGenesisDataContainerChainGenesisData", {
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

                // Let's disable all other parachains and set parathread collator to 4
                // this will make every collator including the one we are registering being assigned to our parathread
                const tx = polkadotJs.tx.registrar.registerParathread(2002, slotFrequency, containerChainGenesisData);

                const profileId = await polkadotJs.query.dataPreservers.nextProfileId();
                const profileTx = polkadotJs.tx.dataPreservers.createProfile({
                    url: "/ip4/127.0.0.1/tcp/33051/ws/p2p/12D3KooWSDsmAa7iFbHdQW4X8B2KbeRYPDLarK6EbevUSYfGkeQw",
                    paraIds: "AnyParaId",
                    mode: "Bootnode",
                    assignmentRequest: "Free",
                });

                const tx2 = polkadotJs.tx.dataPreservers.startAssignment(profileId, 2002, "Free");
                const tx3 = polkadotJs.tx.registrar.markValidForCollating(2002);
                const tx4 = polkadotJs.tx.configuration.setFullRotationPeriod(0);
                const tx5 = polkadotJs.tx.registrar.deregister(2000);
                const tx6 = polkadotJs.tx.registrar.deregister(2001);
                const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
                await context.createBlock([
                    await tx.signAsync(alice, { nonce }),
                    await profileTx.signAsync(charlie),
                    await tx2.signAsync(alice, { nonce: nonce.addn(1) }),
                    await polkadotJs.tx.sudo.sudo(tx3).signAsync(alice, { nonce: nonce.addn(2) }),
                    await polkadotJs.tx.sudo.sudo(tx4).signAsync(alice, { nonce: nonce.addn(3) }),
                    await polkadotJs.tx.sudo.sudo(tx5).signAsync(alice, { nonce: nonce.addn(4) }),
                    await polkadotJs.tx.sudo.sudo(tx6).signAsync(alice, { nonce: nonce.addn(5) }),
                ]);

                const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
                expect(pendingParas.length).to.be.eq(1);
                const sessionScheduling = pendingParas[0][0];
                const parasScheduled = pendingParas[0][1];

                expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

                // These will be the paras in session 2
                // TODO: fix once we have types
                expect(parasScheduled.toJSON()).to.deep.equal([2002]);

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
                expect(parasRegistered.toJSON()).to.deep.equal([2002]);

                // Check that collators have been assigned
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(collators.toJSON().containerChains[2002].length).to.be.greaterThan(0);
            },
        });

        it({
            id: "E03",
            title: "Sudo can forceBuyCore",
            test: async function () {
                const paraId = 2002;

                const encodedMsgBefore = await polkadotJs.query.parachainSystem.upwardMessages();
                expect(encodedMsgBefore.length).to.be.eq(0);

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

        it({
            id: "E04",
            title: "Collator can call buyCore",
            test: async function () {
                const paraId = 2002;

                const nimbusPublicKey = collatorNimbusKey.publicKey;

                const collatorAccountId = context.polkadotJs().createType("AccountId", collatorAccountKey.publicKey);

                await jumpSessions(context, 4);

                await polkadotJs.tx.session.setKeys(u8aToHex(nimbusPublicKey), []).signAndSend(collatorAccountKey);

                await context.createBlock();
                // Check key is reflected in next key
                // But its not yet in queued
                const queuedKeys = await polkadotJs.query.session.queuedKeys();
                const result = queuedKeys.filter((keyItem) => keyItem[1].nimbus == nimbusPublicKey);
                expect(result).is.empty;
                const nextKey = await polkadotJs.query.session.nextKeys(collatorAccountKey.address);
                expect(u8aToHex(nextKey.unwrap().nimbus)).to.be.eq(u8aToHex(nimbusPublicKey));

                // Let's jump one session
                await jumpSessions(context, 2);

                const addInvulnerablesTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.invulnerables.addInvulnerable(collatorAccountId)
                );
                await context.createBlock([await addInvulnerablesTx.signAsync(alice)]);

                await jumpSessions(context, 3);

                // The change should have been applied, and now both nimbus and authorityMapping should reflect
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(collators.toJSON().containerChains["2002"]).to.contain(collatorAccountId.toHuman());

                const dataToEncode: ITuple<[u64, ParaId]> = polkadotJs.createType("(u64, ParaId)", [0, paraId]);
                const signature = u8aToHex(collatorNimbusKey.sign(dataToEncode.toU8a()));
                const proof = polkadotJs.createType("TpXcmCoreBuyerBuyCoreCollatorProof", {
                    nonce: 0,
                    publicKey: u8aToHex(nimbusPublicKey),
                    signature: signature,
                });

                const tx = polkadotJs.tx.xcmCoreBuyer.buyCore(paraId, proof);
                await tx.send();

                await context.createBlock();

                const events = (await polkadotJs.query.system.events()).filter((a) => {
                    return a.event.method == "BuyCoreXcmSent";
                });
                expect(events.length).to.be.equal(1);
            },
        });

        it({
            id: "E05",
            title: "buyCore nonce works properly",
            test: async function () {
                const paraId = 2002;

                const nimbusPublicKey = collatorNimbusKey.publicKey;

                // Older nonce will not work
                let dataToEncode: ITuple<[u64, ParaId]> = polkadotJs.createType("(u64, ParaId)", [0, paraId]);
                let signature = u8aToHex(collatorNimbusKey.sign(dataToEncode.toU8a()));
                let proof = polkadotJs.createType("TpXcmCoreBuyerBuyCoreCollatorProof", {
                    nonce: 0,
                    publicKey: u8aToHex(nimbusPublicKey),
                    signature: signature,
                });
                let tx = polkadotJs.tx.xcmCoreBuyer.buyCore(paraId, proof);
                await expect(tx.send()).rejects.toThrow("1010: Invalid Transaction: Transaction call is not expected");

                // Passing different nonce while signing and creating proof object is rejected
                dataToEncode = polkadotJs.createType("(u64, ParaId)", [1, paraId]);
                signature = u8aToHex(collatorNimbusKey.sign(dataToEncode.toU8a()));
                proof = polkadotJs.createType("TpXcmCoreBuyerBuyCoreCollatorProof", {
                    nonce: 0,
                    publicKey: u8aToHex(nimbusPublicKey),
                    signature: signature,
                });
                tx = polkadotJs.tx.xcmCoreBuyer.buyCore(paraId, proof);
                await expect(tx.send()).rejects.toThrow("1010: Invalid Transaction: Transaction call is not expected");

                dataToEncode = polkadotJs.createType("(u64, ParaId)", [0, paraId]);
                signature = u8aToHex(collatorNimbusKey.sign(dataToEncode.toU8a()));
                proof = polkadotJs.createType("TpXcmCoreBuyerBuyCoreCollatorProof", {
                    nonce: 1,
                    publicKey: u8aToHex(nimbusPublicKey),
                    signature: signature,
                });
                tx = polkadotJs.tx.xcmCoreBuyer.buyCore(paraId, proof);
                await expect(tx.send()).rejects.toThrow("1010: Invalid Transaction: Transaction call is not expected");

                // Correct nonce should be successful
                dataToEncode = polkadotJs.createType("(u64, ParaId)", [1, paraId]);
                signature = u8aToHex(collatorNimbusKey.sign(dataToEncode.toU8a()));
                proof = polkadotJs.createType("TpXcmCoreBuyerBuyCoreCollatorProof", {
                    nonce: 1,
                    publicKey: u8aToHex(nimbusPublicKey),
                    signature: signature,
                });
                tx = polkadotJs.tx.xcmCoreBuyer.buyCore(paraId, proof);
                await tx.send();
            },
        });
    },
});
