import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { blake2AsHex } from "@polkadot/util-crypto";
import { extractFeeAuthor } from "util/block";

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

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            bob = context.keyring.bob;
            // exmple call and hash to be used in tests
            const example_call = context.polkadotJs().tx.balances.transferKeepAlive(charlie.address, 20);
            call = example_call.method.toHex();
            callHash = blake2AsHex(call);
        });

        it({
            id: "E01",
            title: "Creates and cancel a multisig operation",
            test: async () => {

                const otherSignatories = [dave.address, bob.address];
                await context.createBlock(
                    polkadotJs
                        .tx.multisig.asMulti(1, otherSignatories, null, call, {})
                        .signAsync(alice)
                    );
        
                // The multisig is created
                let records = await context.polkadotJs().query.system.events();
                let events = records.filter(
                ({ event }) => event.section == "multisig" && event.method == "NewMultisig"
                );
                expect(events).to.have.lengthOf(1);
              
                await context.createBlock(
                    context
                        .polkadotJs()
                        .tx.multisig.cancelAsMulti(1, otherSignatories, null, callHash)
                        .signAsync(alice)
                    );
                
                // The multisig is Canceled
                records = await context.polkadotJs().query.system.events();
                events = records.filter(
                ({ event }) => event.section == "multisig" && event.method == "NewMultisig"
                );
                expect(events).to.have.lengthOf(1);
            },
          });
    },
});
