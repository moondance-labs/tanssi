import { beforeAll, describeSuite, expect, } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "../../../util/xcm.ts";
import { RELAY_SOURCE_LOCATION, RELAY_SOURCE_LOCATION_2 } from "../../../util/constants.ts";

describeSuite({
    id: "DF0902",
    title: "XcmExecutorUtils - Custom policies",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        const transferredBalance = 10_000_000_000_000n;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();

            // Create parent asset
            const createForeignAsset = await polkadotJs.tx.sudo
            .sudo(
                polkadotJs.tx.utility.batch([
                    // Register parent asset as 1
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION,
                        1,
                        alith.address,
                        true,
                        1
                    ),
                    // Register grandparent asset as 2
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION_2,
                        2,
                        alith.address,
                        true,
                        1
                    ),
                    polkadotJs.tx.assetRate.create(1, 2_000_000_000_000_000_000n),
                    polkadotJs.tx.assetRate.create(2, 2_000_000_000_000_000_000n),
                    // Create custom policy only allowing grandparent asset from parent origin
                    polkadotJs.tx.xcmExecutorUtils.setReservePolicy(
                        // Origin
                        {
                            parents: 1,
                            interior: { Here: null },
                        },
                        // Allow only grandparent asset
                        {
                            allowedAssets: [
                                {
                                    concrete: {
                                            parents: 2,
                                            interior: { Here: null },
                                    },
                                    fun: {
                                        Fungible: 1_000,
                                    },
                                }
                            ]
                        
                        },
                    )
                ])
            );

            await context.createBlock(
                [await createForeignAsset.signAsync(alith)],
                // [await createForeignAsset.signAsync(alith), await setReservePolicy.signAsync(alith)],
                {
                    allowFailures: false,
                }
            );

        });

        it({
            id: "T01",
            title: "Should accept grandparent asset from parent",
            test: async function () {
                // Send grandparent native asset
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: {
                                parents: 2,
                                interior: { Here: null },
                            },
                            fungible: transferredBalance,
                        },
                    ],
                    beneficiary: u8aToHex(alith.addressRaw),
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const alith_asset_balance = (await polkadotJs.query.foreignAssets.account(2, alith.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alith_asset_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alith_asset_balance < transferredBalance).to.be.true;
            },
        });

        it({
            id: "T02",
            title: "Should reject parent native asset from parent",
            test: async function () {
                // Send grandparent native asset
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
                    beneficiary: u8aToHex(alith.addressRaw),
                })
                    .reserve_asset_deposited()
                    .clear_origin()
                    .buy_execution()
                    .deposit_asset()
                    .as_v3();

                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const alith_asset_balance = await polkadotJs.query.foreignAssets.account(1, alith.address);
                expect(alith_asset_balance.isNone).to.be.true;
            },
        });
    },
});
