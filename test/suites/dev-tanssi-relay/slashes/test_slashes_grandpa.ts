import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { fetchCollatorAssignmentTip, jumpSessions } from "util/block";
import { Keyring } from "@polkadot/keyring";
import { Header, GrandpaEquivocationProof, GrandpaEquivocation, GrandpaEquivocationValue } from "@polkadot/types/interfaces";
import { SpRuntimeHeader, FinalityGrandpaPrevote } from '@polkadot/types/lookup';
import { extrinsics } from "@polkadot/types/interfaces/definitions";
import { u8aToHex, hexToU8a, stringToHex, numberToHex, stringToU8a, identity } from "@polkadot/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import { jumpToSession } from "../../../util/block";

describeSuite({
    id: "DTR1306",
    title: "Grandpa offences should trigger a slash",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceGrandpaPair: KeyringPair;
        let aliceStash: KeyringPair;
        beforeAll(async () => {
            const keyringGrandpa = new Keyring({ type: "ed25519" });
            const keyringSr25519 = new Keyring({ type: "sr25519" });
            aliceGrandpaPair = keyringGrandpa.addFromUri("//Alice");
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            aliceStash = keyringSr25519.addFromUri("//Alice//stash");
        });
        it({
            id: "E01",
            title: "Grandpa offences do not trigger a slash against invulnerables",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // Remove alice from invulnerables (just for the slash)
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address)
                ).signAsync(alice)
                await context.createBlock([removeAliceFromInvulnerables]);

                let prevote1: FinalityGrandpaPrevote = polkadotJs.createType("FinalityGrandpaPrevote", {
                    targetHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
                    targetNumber: 1
                });

                let prevote2: FinalityGrandpaPrevote = polkadotJs.createType("FinalityGrandpaPrevote", {
                    targetHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
                    targetNumber: 2
                });

                let roundNumber = polkadotJs.createType("u64", 1);
                let setId = await polkadotJs.query.grandpa.currentSetId();

                // I could not find the proper struct that holds all this into a singl message
                // ergo I need to construct the signing payload myself
                // the first 0 is because of this enum variant
                // https://github.com/paritytech/finality-grandpa/blob/8c45a664c05657f0c71057158d3ba555ba7d20de/src/lib.rs#L228
                // then we have the prevote message
                // then the round number
                // then the set id
                const toSign1 = new Uint8Array([
                    ...hexToU8a("0x00"),
                    ...prevote1.toU8a(),
                    ...roundNumber.toU8a(),
                    ...setId.toU8a()
                ]);

                const toSign2 = new Uint8Array([
                    ...hexToU8a("0x00"),
                    ...prevote2.toU8a(),
                    ...roundNumber.toU8a(),
                    ...setId.toU8a()
                ]);
                const sig1 = aliceGrandpaPair.sign(toSign1);
                const sig2 = aliceGrandpaPair.sign(toSign2);

                const equivocationValue: GrandpaEquivocationValue = polkadotJs.createType("GrandpaEquivocationValue", {
                    roundNumber,
                    identity: aliceGrandpaPair.address,
                    first: [prevote1, sig1],
                    second: [prevote2, sig2]
                });

                const equivocation: GrandpaEquivocation = polkadotJs.createType("GrandpaEquivocation",
                {   
                    'Prevote': equivocationValue
                });

                const doubleVotingProof: GrandpaEquivocationProof = polkadotJs.createType(
                    "GrandpaEquivocationProof",
                    {
                        setId,
                        equivocation
                    }
                );

                const keyOwnershipProof = (await polkadotJs.call.grandpaApi.generateKeyOwnershipProof(
                    setId,
                    u8aToHex(aliceGrandpaPair.publicKey)
                )).unwrap();

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.grandpa.reportEquivocation(doubleVotingProof, keyOwnershipProof)), {
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

            },
        });
    },
});
