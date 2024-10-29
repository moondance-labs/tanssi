import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { fetchCollatorAssignmentTip, jumpSessions } from "util/block";
import { Keyring } from "@polkadot/keyring";
import { Header, BabeEquivocationProof } from "@polkadot/types/interfaces";
import { SpRuntimeHeader } from '@polkadot/types/lookup';
import { extrinsics } from "@polkadot/types/interfaces/definitions";
import { u8aToHex, hexToU8a, stringToHex, numberToHex, stringToU8a } from "@polkadot/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import { jumpToSession } from "../../../util/block";

describeSuite({
    id: "DTR1304",
    title: "Babe offences should trigger a slash",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceBabePair: KeyringPair;
        let aliceStash: KeyringPair;
        beforeAll(async () => {
            const keyringBabe = new Keyring({ type: "sr25519" });
            aliceBabePair = keyringBabe.addFromUri("//Alice");
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            aliceStash = keyringBabe.addFromUri("//Alice//stash");
        });
        it({
            id: "E01",
            title: "Babe offences trigger a slash+",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // Remove alice from invulnerables (just for the slash)
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address)
                ).signAsync(alice)
                await context.createBlock([removeAliceFromInvulnerables]);

                let baseHeader = await polkadotJs.rpc.chain.getHeader();
                let baseHeader2 = await polkadotJs.rpc.chain.getHeader();

                const header1: SpRuntimeHeader = polkadotJs.createType("SpRuntimeHeader", {
                    digest: baseHeader.digest,
                    extrinsicsRoot: baseHeader.extrinsicsRoot,
                    stateRoot: baseHeader.stateRoot,
                    parentHash: baseHeader.parentHash,
                    number: 1,
                });

                // we just change the block number
                const header2: SpRuntimeHeader = polkadotJs.createType("SpRuntimeHeader", {
                    digest: baseHeader2.digest,
                    extrinsicsRoot: baseHeader2.extrinsicsRoot,
                    stateRoot: baseHeader2.stateRoot,
                    parentHash: baseHeader2.parentHash,
                    number: 2,
                });

                const sig1 = aliceBabePair.sign(blake2AsHex(header1.toU8a()));
                const sig2 = aliceBabePair.sign(blake2AsHex(header2.toU8a()));

                const slot = await polkadotJs.query.babe.currentSlot();

                // let's inject the equivocation proof

                const keyOwnershipProof = (await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                    slot,
                    u8aToHex(aliceBabePair.publicKey)
                )).unwrap();

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                //const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const digestItemSeal1: SpRuntimeDigestDigestItem = polkadotJs.createType(
                    "SpRuntimeDigestDigestItem",
                    { Seal:  [
                        stringToHex('BABE'),
                        u8aToHex(sig1)
                        ]
                    }
                );

                const digestItemSeal2: SpRuntimeDigestDigestItem = polkadotJs.createType(
                    "SpRuntimeDigestDigestItem",
                    { Seal:  [
                        stringToHex('BABE'),
                        u8aToHex(sig2)
                        ]
                    }
                );

                header1.digest.logs.push(digestItemSeal1);
                header2.digest.logs.push(digestItemSeal2);

                const doubleVotingProof: BabeEquivocationProof = polkadotJs.createType(
                    "BabeEquivocationProof",
                    {
                        offender: aliceBabePair.publicKey,
                        slotNumber: slot,
                        firstHeader: header1,
                        secondHeader: header2
                    }
                );
                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProof)), {
                            refTime: 1n,
                            proofSize: 1n
                    })

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should be there
                const DeferPeriod = 2;

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod +1);
                expect(expectedSlashes.length).to.be.eq(1);
                expect(u8aToHex(expectedSlashes[0].validator)).to.be.eq(u8aToHex(aliceStash.addressRaw));
                
                // Put alice back to invulnerables 
                const addAliceFromInvulnerables = await polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.externalValidators.addWhitelisted(aliceStash.address)
                ).signAsync(alice)
                await context.createBlock([addAliceFromInvulnerables]);

                let sessionsPerEra = await polkadotJs.consts.externalValidators.sessionsPerEra;

                let currentIndex = await polkadotJs.query.session.currentIndex();

                let targetSession = currentIndex*sessionsPerEra*(DeferPeriod +1);
                while((await polkadotJs.query.session.currentIndex()).toNumber() < targetSession) {
                    let currentIndex = await polkadotJs.query.session.currentIndex();
                    await jumpToSession(context, currentIndex.toNumber()+1);
                }
                
                // scheduled slashes
                const expectedSlashesAfterDefer = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod +1);
                expect(expectedSlashesAfterDefer.length).to.be.eq(1);
                expect(expectedSlashesAfterDefer[0].confirmed.toHuman()6).to.be.true;

            },
        });
    },
});
