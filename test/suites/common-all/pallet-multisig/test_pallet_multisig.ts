import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, baltathar, charleth, dorothy } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { blake2AsHex, createKeyMulti, encodeMultiAddress } from "@polkadot/util-crypto";
import type { Weight } from "@polkadot/types/interfaces";

describeSuite({
    id: "C0201",
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
        let callWeight: Weight;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            // This test will be run against frontier & substrate chains, hence the accounts used
            alice_or_alith = context.isEthereumChain ? alith : context.keyring.alice;
            charlie_or_charleth = context.isEthereumChain ? charleth : context.keyring.charlie;
            // Multisig extrinsics expect accounts to be sorted, that's why we swap bob and dave here
            dave_or_baltathar = context.isEthereumChain ? baltathar : context.keyring.dave;
            bob_or_dorothy = context.isEthereumChain ? dorothy : context.keyring.bob;
            // Need 2 out of 3 signatures to execute multisig call
            threshold = 2;
            // Example call and hash to be used in tests
            const example_call = polkadotJs.tx.balances.transferKeepAlive(charlie_or_charleth.address, 20);
            call = example_call.method.toHex();
            callHash = blake2AsHex(call);
            const feeInfo = await example_call.paymentInfo(alice_or_alith.address);
            callWeight = feeInfo.weight;
        });

        it({
            id: "E01",
            title: "Creates and cancels a multisig operation",
            test: async () => {
                // Multisig creation
                const otherSignatories = [dave_or_baltathar.address, bob_or_dorothy.address];
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(threshold, otherSignatories, null, call, {})
                        .signAsync(alice_or_alith)
                );

                // The multisig is created
                let records = await polkadotJs.query.system.events();
                let eventCount = records.filter((a) => {
                    return a.event.method === "NewMultisig";
                });
                expect(eventCount.length).to.be.equal(1);

                // Multisig Cancelation
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
                    return a.event.method === "MultisigCancelled";
                });
                expect(eventCount.length).to.be.equal(1);

                // Attempting to execute multisig call will now fail
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(
                            threshold,
                            [dave_or_baltathar.address, alice_or_alith.address],
                            multisigInfo.unwrap().when,
                            call,
                            callWeight
                        )
                        .signAsync(bob_or_dorothy)
                );
                records = await polkadotJs.query.system.events();
                eventCount = records.filter((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });
                expect(eventCount.length).to.be.equal(1);
            },
        });

        it({
            id: "E02",
            title: "Approves a multisig operation",
            test: async () => {
                // Multisig creation
                const otherSignatories = [dave_or_baltathar.address, bob_or_dorothy.address];
                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(threshold, otherSignatories, null, call, {})
                        .signAsync(alice_or_alith)
                );

                // Fund multisig address with some balance, needed for the balance transfer call to succeed
                const multisigAddress = encodeMultiAddress(
                    [alice_or_alith.address, dave_or_baltathar.address, bob_or_dorothy.address],
                    threshold
                );
                await context.createBlock(
                    polkadotJs.tx.balances
                        .transferKeepAlive(multisigAddress, 100_000_000_000_000_000n)
                        .signAsync(alice_or_alith)
                );
                const multisigBalanceBefore = (await polkadotJs.query.system.account(multisigAddress)).data.free;
                expect(multisigBalanceBefore.toBigInt() > 0n).toBeTruthy();

                // Multisig call is a balance transfer to this address, so check that balance will increase
                const balanceBefore = (await polkadotJs.query.system.account(charlie_or_charleth.address)).data.free;

                // This is only needed to get the time point parameter
                const encodedMultisigId = createKeyMulti(
                    [alice_or_alith.address, dave_or_baltathar.address, bob_or_dorothy.address],
                    threshold
                );
                const multisigId = u8aToHex(encodedMultisigId);
                const multisigInfo = await polkadotJs.query.multisig.multisigs(multisigId, callHash);

                await context.createBlock(
                    polkadotJs.tx.multisig
                        .asMulti(
                            threshold,
                            [dave_or_baltathar.address, alice_or_alith.address],
                            multisigInfo.unwrap().when,
                            call,
                            callWeight
                        )
                        .signAsync(bob_or_dorothy)
                );

                // Multisig call is approved and executed
                const records = await polkadotJs.query.system.events();
                const eventCount = records.filter((a) => {
                    return a.event.method === "MultisigExecuted";
                });
                expect(eventCount.length).to.be.equal(1);
                // Balance transfer is executed
                const balanceAfter = (await polkadotJs.query.system.account(charlie_or_charleth.address)).data.free;
                expect(balanceAfter.toBigInt() > balanceBefore.toBigInt()).toBeTruthy();
            },
        });
    },
});
