import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { u8aToHex } from "@polkadot/util";
import { jumpToSession } from "../../../util/block";
import { generateGrandpaEquivocationProof } from "../../../util/slashes";

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
            title: "Grandpa offences trigger a slashing event",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // Remove alice from invulnerables (just for the slash)
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address))
                    .signAsync(alice);
                await context.createBlock([removeAliceFromInvulnerables]);

                const doubleVotingProof = await generateGrandpaEquivocationProof(polkadotJs, aliceGrandpaPair);

                const keyOwnershipProof = (
                    await polkadotJs.call.grandpaApi.generateKeyOwnershipProof(
                        doubleVotingProof.setId,
                        u8aToHex(aliceGrandpaPair.publicKey)
                    )
                ).unwrap();
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.grandpa.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)
                    ),
                    {
                        refTime: 1n,
                        proofSize: 1n,
                    }
                );

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should be there
                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();

                // scheduled slashes
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(1);
                expect(u8aToHex(expectedSlashes[0].validator)).to.be.eq(u8aToHex(aliceStash.addressRaw));
            },
        });
    },
});
