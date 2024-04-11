import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "../../../util/xcm.ts";
import { RELAY_SOURCE_LOCATION, RELAY_SOURCE_LOCATION_2 } from "../../../util/constants.ts";

// This assumes that the XcmExecutorUtils ReserveDefaultTrustPolicy set in the runtime is AllNative
describeSuite({
    id: "DC0201",
    title: "XcmExecutorUtils - Default policies",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: string;
        let transferredBalance;

        beforeAll(async function () {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;
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
                                    alice.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(1, 2_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alice),
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
                    beneficiary: u8aToHex(alice.addressRaw),
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

                const alice_asset_balance = (await polkadotJs.query.foreignAssets.account(1, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alice_asset_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alice_asset_balance < transferredBalance).to.be.true;
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
                                    alice.address,
                                    true,
                                    1
                                ),
                                polkadotJs.tx.assetRate.create(2, 2_000_000_000_000_000_000n),
                            ])
                        )
                        .signAsync(alice),
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
                    beneficiary: u8aToHex(alice.addressRaw),
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

                // Grandparent tokens should have been rejected, so asset balance for Alice shouldn't exist
                const alice_asset_balance = await polkadotJs.query.foreignAssets.account(2, alice.address);
                console.log(alice_asset_balance.toHuman());
                expect(alice_asset_balance.isNone).to.be.true;
            },
        });
    },
});
