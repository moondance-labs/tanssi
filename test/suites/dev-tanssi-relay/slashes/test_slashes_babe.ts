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

describeSuite({
    id: "DTR1301",
    title: "Babe offences should trigger a slash",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceBabePair: KeyringPair;
        beforeAll(async () => {
            const keyringBabe = new Keyring({ type: "sr25519" });
            aliceBabePair = keyringBabe.addFromUri("//Alice");
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Babe offences trigger a slash+",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await context.createBlock();
                await context.createBlock();

                let baseHeader = await polkadotJs.rpc.chain.getHeader();

                console.log("baseHeader ", blake2AsHex(baseHeader.toU8a()))
                const header1: SpRuntimeHeader = polkadotJs.createType("SpRuntimeHeader", {
                    digest: baseHeader.digest,
                    extrinsicsRoot: baseHeader.extrinsicsRoot,
                    stateRoot: baseHeader.stateRoot,
                    parentHash: baseHeader.parentHash,
                    number: 1,
                });

                // we just change the block number
                const header2: SpRuntimeHeader = polkadotJs.createType("SpRuntimeHeader", {
                    digest: baseHeader.digest,
                    extrinsicsRoot: baseHeader.extrinsicsRoot,
                    stateRoot: baseHeader.stateRoot,
                    parentHash: baseHeader.parentHash,
                    number: 2,
                });

                console.log("haeder 1");
                console.log("header 1 bytes ", header1.toU8a());
                console.log("header 2 bytes ", header2.toU8a());

                console.log(header1.hash.toHuman())
                console.log(blake2AsHex(header1.toU8a()))
                console.log("haeder 2");
                console.log(blake2AsHex(header2.toU8a()))

                const sig1 = aliceBabePair.sign(header1.toU8a());
                const sig2 = aliceBabePair.sign(header2.toU8a());

                const slot = await polkadotJs.query.babe.currentSlot();

                console.log(aliceBabePair.addressRaw);
                // let's inject the equivocation proof

                const validatorSetId = 1;
                const keyOwnershipProof = await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                    validatorSetId,
                    u8aToHex(aliceBabePair.publicKey)
                );

                // We don't care about the first 8 characters of the proof, as they
                // correspond to SCALE encoded wrapping stuff we don't need.
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                console.log("first")
                const digestItemSeal1: SpRuntimeDigestDigestItem = polkadotJs.createType(
                    "SpRuntimeDigestDigestItem",
                    { Seal:  [
                        stringToHex('BABE'),
                        u8aToHex(sig1)
                        ]
                    }
                );

                console.log("second")
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
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)), {
                            refTime: 1n,
                            proofSize: 1n
                    })

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

            },
        });
    },
});
