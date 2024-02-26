import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { alith } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "../../../util/xcm.ts";
import { RELAY_SOURCE_LOCATION, RELAY_SOURCE_LOCATION_2 } from "../../../util/constants.ts";

describeSuite({
    id: "DF0901",
    title: "XcmExecutorUtils - Default policies",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        const transferredBalance = 10_000_000_000_000n;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "T01",
            title: "Should allow native asset from parent",
            test: async function () {
                // Register parent asset
                await context.createBlock(
                    await polkadotJs.tx.sudo
                        .sudo(
                            polkadotJs.tx.utility.batch([
                                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    RELAY_SOURCE_LOCATION,
                                    1,
                                    alith.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(1, 2_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alith),
                    {
                        allowFailures: false,
                    }
                );

                // Send parent native asset
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

                const alith_asset_balance = (await polkadotJs.query.foreignAssets.account(1, alith.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alith_asset_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alith_asset_balance < transferredBalance).to.be.true;
            },
        });

        it({
            id: "T02",
            title: "Should reject grandparent asset from parent",
            test: async function () {
                // Register grandparent asset
                await context.createBlock(
                    await polkadotJs.tx.sudo
                        .sudo(
                            polkadotJs.tx.utility.batch([
                                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                    RELAY_SOURCE_LOCATION_2,
                                    2,
                                    alith.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(2, 2_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alith),
                    {
                        allowFailures: false,
                    }
                );

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

                await context.createBlock();

                // Tokens should have been rejected
                const alith_asset_balance = await polkadotJs.query.foreignAssets.account(2, alith.address);
                expect(alith_asset_balance.isNone).to.be.true;
            },
        });
    },
});
