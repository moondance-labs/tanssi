import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith, baltathar, charleth, dorothy } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { blake2AsHex, createKeyMulti, encodeMultiAddress } from "@polkadot/util-crypto";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES } from "helpers";
import type { Weight } from "@polkadot/types/interfaces";
import { addressToU8a } from "@polkadot/util-crypto/address/util";

describeSuite({
    id: "C0206",
    title: "Test failed multiblock migration",
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
        let callIsBalanceTransfer: boolean;
        let specVersion: number;
        let isStarlight: boolean;
        let isFrontier: boolean;

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
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            isStarlight = polkadotJs.consts.system.version.specName.toString() === "starlight";
            isFrontier = polkadotJs.consts.system.version.specName.toString() === "frontier-template";
            // Example call and hash to be used in tests
            let example_call: any;
            if (isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_BALANCES.includes(specVersion)) {
                example_call = context.polkadotJs().tx.system.remarkWithEvent("0x1234");
                callIsBalanceTransfer = false;
            } else {
                example_call = context.polkadotJs().tx.balances.transferKeepAlive(charlie_or_charleth.address, 20);
                callIsBalanceTransfer = true;
            }

            call = example_call.method.toHex();
            callHash = blake2AsHex(call);
            const feeInfo = await example_call.paymentInfo(alice_or_alith.address);
            callWeight = feeInfo.weight;
        });

        it({
            id: "E01",
            title: "Write cursor so that migration fails",
            test: async () => {
                const tx = polkadotJs.tx.multiBlockMigrations.forceSetCursor(
                    "Stuck"
                );
                /*
                await context.createBlock(
                    polkadotJs.tx.sudo
                        .sudo(
                            tx
                        )
                        .signAsync(alice_or_alith)
                );
                 */
                const cursor = await polkadotJs.query.multiBlockMigrations.cursor();
                console.log(cursor.toJSON());
                expect(cursor.unwrap().isStuck).to.be.true;
            },
        });

        it({
            id: "E02",
            title: "Migrate runtime to the same runtime to trigger migrations",
            test: async () => {
                const wasmCode = await polkadotJs.rpc.state.getStorage('0x3a636f6465'); // :code
                const wasmCodeU8 = wasmCode.toU8a();
                const wasmCodeHex = wasmCode.toHex();
                console.log(`Current runtime code prefix: ${wasmCodeHex.slice(0, 20)}...`);
                const rtHash = blake2AsHex(wasmCodeHex);

                const tx = polkadotJs.tx.system.authorizeUpgradeWithoutChecks(rtHash);
                let nonce = (await polkadotJs.rpc.system.accountNextIndex(alice_or_alith.address)).toNumber();
                await polkadotJs.tx.sudo
                    .sudo(
                        tx
                    )
                    .signAndSend(alice_or_alith, { nonce: nonce++ });

                await context.createBlock();

                const tx1 = polkadotJs.tx.system.killStorage([
                    polkadotJs.query.system.lastRuntimeUpgrade.key()
                ]);
                const tx2 = polkadotJs.tx.multiBlockMigrations.forceSetCursor(
                    "Stuck"
                );

                await polkadotJs.tx.utility.batchAll([
                    // system or parachainSystem
                    polkadotJs.tx.sudo
                        .sudo(
                            tx1
                        ),
                    polkadotJs.tx.system.applyAuthorizedUpgrade(wasmCodeHex),
                    polkadotJs.tx.sudo
                        .sudo(
                            tx2
                        ),
                ]).signAndSend(alice_or_alith, { nonce: nonce++ });

                /*
                const tx = polkadotJs.tx.system.setCodeWithoutChecks(wasmCodeU8);
                let nonce = (await polkadotJs.rpc.system.accountNextIndex(alice_or_alith.address)).toNumber();
                await polkadotJs.tx.sudo
                    .sudoUncheckedWeight(
                        tx, {
                            proofSize: 1,
                            refTime: 1,
                        }
                    )
                    .signAndSend(alice_or_alith, { nonce: nonce++ });
                 */

                for (let i = 0; i < 10; i++) {
                    await context.createBlock();
                }
            },
        });
    },
});
