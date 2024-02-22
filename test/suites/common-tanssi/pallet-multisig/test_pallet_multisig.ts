import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { blake2AsHex, createKeyMulti } from "@polkadot/util-crypto";
import { u8aToHex } from "@polkadot/util";

describeSuite({
    id: "CT1001",
    title: "Multisig pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let bob: KeyringPair;
        let call: string;
        let callHash: string;
        let threshold: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            bob = context.keyring.bob;
            threshold = 2;
            // exmple call and hash to be used in tests
            const example_call = context.polkadotJs().tx.balances.transferKeepAlive(charlie.address, 20);
            call = example_call.method.toHex();
            callHash = blake2AsHex(call);
        });

        it({
            id: "E01",
            title: "Creates and cancel a multisig operation",
            test: async () => {
                //Multisig creation
                const otherSignatories = [dave.address, bob.address];
                await context.createBlock(
                    polkadotJs.tx.multisig.asMulti(threshold, otherSignatories, null, call, {}).signAsync(alice)
                );

                // The multisig is created
                let records = await polkadotJs.query.system.events();
                let eventCount = records.filter((a) => {
                    return a.event.method == "NewMultisig";
                });
                expect(eventCount.length).to.be.equal(1);

                //Multisig Cancelation
                const encodedMultisigId = createKeyMulti([alice.address, dave.address, bob.address], threshold);
                const multisigId = u8aToHex(encodedMultisigId);
                const multisigInfo = await polkadotJs.query.multisig.multisigs(multisigId, callHash);
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .cancelAsMulti(threshold, otherSignatories, multisigInfo.unwrap().when, callHash)
                        .signAsync(alice)
                );

                // Multisig is cancelled
                records = await polkadotJs.query.system.events();
                eventCount = records.filter((a) => {
                    return a.event.method == "MultisigCancelled";
                });
                expect(eventCount.length).to.be.equal(1);
            },
        });

        it({
            id: "E02",
            title: "Approves a multisig operation",
            test: async function () {
                //Multisig creation
                const otherSignatories = [dave.address, bob.address];
                await context.createBlock(
                    polkadotJs.tx.multisig.asMulti(threshold, otherSignatories, null, call, {}).signAsync(alice)
                );

                //Multisig Approval

                // This is only needed to get get time point parameter
                const encodedMultisigId = createKeyMulti([alice.address, dave.address, bob.address], threshold);
                const multisigId = u8aToHex(encodedMultisigId);
                const multisigInfo = await polkadotJs.query.multisig.multisigs(multisigId, callHash);

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.multisig.approveAsMulti(
                            threshold,
                            [dave.address, alice.address],
                            multisigInfo.unwrap().when,
                            callHash,
                            {}
                        )
                        .signAsync(bob)
                );

                // Multisig call is approved
                const records = await polkadotJs.query.system.events();
                const eventCount = records.filter((a) => {
                    return a.event.method == "MultisigApproval";
                });
                expect(eventCount.length).to.be.equal(1);
            },
        });
    },
});
