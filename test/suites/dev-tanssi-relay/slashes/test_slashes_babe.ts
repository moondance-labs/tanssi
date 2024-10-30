import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { u8aToHex } from "@polkadot/util";
import { jumpToSession } from "../../../util/block";
import { generateBabeEquivocationProof } from "../../../util/slashes";

describeSuite({
    id: "DTR1301",
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
            title: "Babe offences trigger a slash",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // Remove alice from invulnerables (just for the slash)
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address))
                    .signAsync(alice);
                await context.createBlock([removeAliceFromInvulnerables]);

                // let's inject the equivocation proof
                const doubleVotingProof = await generateBabeEquivocationProof(polkadotJs, aliceBabePair);

                // generate key ownership proof
                const keyOwnershipProof = (
                    await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                        doubleVotingProof.slotNumber,
                        u8aToHex(aliceBabePair.publicKey)
                    )
                ).unwrap();

                console.log(keyOwnershipProof)

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProof)
                    ),
                    {
                        refTime: 1n,
                        proofSize: 1n,
                    }
                );

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should be there
                const DeferPeriod = await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration;

                console.log(DeferPeriod)
                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(1);
                expect(u8aToHex(expectedSlashes[0].validator)).to.be.eq(u8aToHex(aliceStash.addressRaw));
            },
        });
    },
});
