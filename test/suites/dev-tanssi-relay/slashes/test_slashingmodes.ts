import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { u8aToHex } from "@polkadot/util";
import { generateBabeEquivocationProof } from "../../../util/slashes";

describeSuite({
    id: "DTR1309",
    title: "Slashing modes behave as expected",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceBabePair: KeyringPair;
        let aliceStash: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyringSr25519 = new Keyring({ type: "sr25519" });
            const keyringBabe = new Keyring({ type: "sr25519" });

            alice = context.keyring.alice;
            aliceStash = keyringSr25519.addFromUri("//Alice//stash");
            aliceBabePair = keyringBabe.addFromUri("//Alice");
        });

        it({
            id: "E01",
            title: "Slashing mode LogOnly should generate an event but not trigger a slash",
            test: async function () {
                // Set slashing mode to LogOnly
                const setSlashingMode = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.setSlashingMode("LogOnly"))
                    .signAsync(alice);
                await context.createBlock([setSlashingMode]);

                // Remove alice from invulnerables to make it slashable
                const removeAliceFromInvulnerables = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidators.removeWhitelisted(aliceStash.address))
                    .signAsync(alice);
                await context.createBlock([removeAliceFromInvulnerables]);

                // Slashable action
                const doubleVotingProof = await generateBabeEquivocationProof(polkadotJs, aliceBabePair);
                const keyOwnershipProof = (
                    await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                        doubleVotingProof.slotNumber,
                        u8aToHex(aliceBabePair.publicKey)
                    )
                ).unwrap();
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)
                    ),
                    {
                        refTime: 1n,
                        proofSize: 1n,
                    }
                );

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should not be there
                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(0);

                // Event should be there
                const events = await polkadotJs.query.system.events();
                const event = events.find(
                    ({ event }) => event.section === "externalValidatorSlashes" && event.method === "SlashReported"
                );
                expect(event).not.be.undefined;
            },
        });

        it({
            id: "E02",
            title: "Slashing mode Disabled should not generate neither an event nor a slash",
            test: async function () {
                // Set slashing mode to Disabled
                const setSlashingMode = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.setSlashingMode("Disabled"))
                    .signAsync(alice);
                await context.createBlock([setSlashingMode]);

                // Slashable action
                const doubleVotingProof = await generateBabeEquivocationProof(polkadotJs, aliceBabePair);
                const keyOwnershipProof = (
                    await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                        doubleVotingProof.slotNumber,
                        u8aToHex(aliceBabePair.publicKey)
                    )
                ).unwrap();
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)
                    ),
                    {
                        refTime: 1n,
                        proofSize: 1n,
                    }
                );

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should not be there
                const DeferPeriod = (await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration).toNumber();
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(0);

                // Event should not be there
                const events = await polkadotJs.query.system.events();
                const event = events.find(
                    ({ event }) => event.section === "externalValidatorSlashes" && event.method === "SlashReported"
                );
                expect(event).to.be.undefined;
            },
        });

        it({
            id: "E03",
            title: "Slashing mode Enabled should generate an event and a slash",
            test: async function () {
                // Set slashing mode to Enabled
                const setSlashingMode = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.externalValidatorSlashes.setSlashingMode("Enabled"))
                    .signAsync(alice);
                await context.createBlock([setSlashingMode]);

                // Slashable action
                const doubleVotingProof = await generateBabeEquivocationProof(polkadotJs, aliceBabePair);
                const keyOwnershipProof = (
                    await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                        doubleVotingProof.slotNumber,
                        u8aToHex(aliceBabePair.publicKey)
                    )
                ).unwrap();
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)
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
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod + 1);
                expect(expectedSlashes.length).to.be.eq(1);

                // Event should be there
                const events = await polkadotJs.query.system.events();
                const event = events.find(
                    ({ event }) => event.section === "externalValidatorSlashes" && event.method === "SlashReported"
                );
                expect(event).not.be.undefined;
            },
        });
    },
});
