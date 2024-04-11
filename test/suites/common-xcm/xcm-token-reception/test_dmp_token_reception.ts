import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "../../../util/xcm.ts";
import { RELAY_SOURCE_LOCATION, RELAY_SOURCE_LOCATION_2 } from "../../../util/constants.ts";

describeSuite({
    id: "TX0101",
    title: "Mock XCM - Succeeds receiving tokens DMP",
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

            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;

            // We register the token
            const txSigned = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.utility.batch([
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION,
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
                                interior: { Here: null },
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
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                // Make sure the state has ALITH's to DOT tokens
                const alice_dot_balance = (await context.polkadotJs().query.foreignAssets.account(1, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alice_dot_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alice_dot_balance < transferredBalance).to.be.true;
            },
        });

        it({
            id: "T02",
            title: "Should not succeed receiving tokens if asset rate is not defined",
            test: async function () {
                // We register the token
                const txSigned = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION_2,
                        // id 2
                        2,
                        alice.address,
                        true,
                        1
                    )
                );

                await context.createBlock(await txSigned.signAsync(alice), {
                    allowFailures: false,
                });

                // Send an XCM and create block to execute it
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            // we change parents to 2
                            multilocation: {
                                parents: 2,
                                interior: { Here: null },
                            },
                            fungible: 10000000000000n,
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
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                // Make sure the state has not ALITH's DOT_2 tokens
                const alice_dot_2_balance = await context.polkadotJs().query.foreignAssets.account(2, alice.address);
                expect(alice_dot_2_balance.isNone).to.be.true;
            },
        });
    },
});
