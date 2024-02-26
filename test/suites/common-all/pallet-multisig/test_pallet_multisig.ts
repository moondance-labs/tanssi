import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { blake2AsHex, createKeyMulti } from "@polkadot/util-crypto";
import { u8aToHex } from "@polkadot/util";
import { alith, charleth, baltathar, dorothy } from "@moonwall/util";

describeSuite({
    id: "C0401",
    title: "Multisig pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice_or_alith: KeyringPair;
        let charlie_or_charleth: KeyringPair;
        let dave_or_baltathar: KeyringPair;
        let bob_or_dorothy: KeyringPair;
        let call: string;
        let callHash: string;
        let threshold: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            // This test will be run against frontier & substrate chains, hence the accounts used
            alice_or_alith = context.isEthereumChain ? alith : context.keyring.alice;
            charlie_or_charleth = context.isEthereumChain ? charleth : context.keyring.charlie;
            dave_or_baltathar = context.isEthereumChain ? baltathar : context.keyring.dave;
            bob_or_dorothy = context.isEthereumChain ? dorothy : context.keyring.bob;
            threshold = 2;
            // exmple call and hash to be used in tests
            const example_call = context.polkadotJs().tx.balances.transferKeepAlive(charlie_or_charleth.address, 20);
            call = example_call.method.toHex();
            callHash = blake2AsHex(call);
        });

        it({
            id: "E01",
            title: "Creates and cancel a multisig operation",
            test: async () => {
                //Multisig creation
                const otherSignatories = [dave_or_baltathar.address, bob_or_dorothy.address];
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(threshold, otherSignatories, null, call, {})
                        .signAsync(alice_or_alith)
                );

                // The multisig is created
                let records = await polkadotJs.query.system.events();
                let eventCount = records.filter((a) => {
                    return a.event.method == "NewMultisig";
                });
                expect(eventCount.length).to.be.equal(1);

                //Multisig Cancelation
                const encodedMultisigId = createKeyMulti(
                    [alice_or_alith.address, dave_or_baltathar.address, bob_or_dorothy.address],
                    threshold
                );
                const multisigId = u8aToHex(encodedMultisigId);
                const multisigInfo = await polkadotJs.query.multisig.multisigs(multisigId, callHash);
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .cancelAsMulti(threshold, otherSignatories, multisigInfo.unwrap().when, callHash)
                        .signAsync(alice_or_alith)
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
                const otherSignatories = [dave_or_baltathar.address, bob_or_dorothy.address];
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(threshold, otherSignatories, null, call, {})
                        .signAsync(alice_or_alith)
                );

                //Multisig Approval

                // This is only needed to get get time point parameter
                const encodedMultisigId = createKeyMulti(
                    [alice_or_alith.address, dave_or_baltathar.address, bob_or_dorothy.address],
                    threshold
                );
                const multisigId = u8aToHex(encodedMultisigId);
                const multisigInfo = await polkadotJs.query.multisig.multisigs(multisigId, callHash);

                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.multisig.approveAsMulti(
                            threshold,
                            [dave_or_baltathar.address, alice_or_alith.address],
                            multisigInfo.unwrap().when,
                            callHash,
                            {}
                        )
                        .signAsync(bob_or_dorothy)
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
