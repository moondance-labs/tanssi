import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectHrmpMessageAndSeal } from "../../../util/xcm.ts";
import { STATEMINT_LOCATION_EXAMPLE } from "../../../util/constants.ts";

describeSuite({
    id: "TX0102",
    title: "Mock XCM - Succeeds receiving tokens through HRMP",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let transferredBalance;
        let alice: KeyringPair;
        let chain;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            // since in the future is likely that we are going to add this to containers, I leave it here
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });

            transferredBalance = 10_000_000_000_000n;

            // We register the token
            const txSigned = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.utility.batch([
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        STATEMINT_LOCATION_EXAMPLE,
                        1,
                        alice.address,
                        true,
                        1
                    ),
                    polkadotJs.tx.assetRate.create(
                        1,
                        // this defines how much the asset costs with respect to the
                        // new asset
                        // in this case, asset*2=native
                        // that means that we will charge 0.5 of the native balance
                        2000000000000000000n
                    ),
                ])
            );

            await context.createBlock(await txSigned.signAsync(alice), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should succeed receiving tokens",
            test: async function () {
                // Send an XCM and create block to execute it
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 1,
                                interior: {
                                    X3: [{ Parachain: 1000 }, { PalletInstance: 50 }, { GeneralIndex: 0n }],
                                },
                            },
                            fungible: transferredBalance,
                        },
                    ],
                    beneficiary: u8aToHex(alice.addressRaw),
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectHrmpMessageAndSeal(context, 1000, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                // Make sure the state has Alice's tatemint tokens
                const alice_statemint_balance = (
                    await context.polkadotJs().query.foreignAssets.account(1, alice.address)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(alice_statemint_balance > 0n).to.be.true;
            },
        });
    },
});
