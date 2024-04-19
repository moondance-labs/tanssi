import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import { RawXcmMessage, XcmFragment, injectDmpMessageAndSeal } from "../../../util/xcm.ts";
import { RELAY_SOURCE_LOCATION, RELAY_SOURCE_LOCATION_2 } from "../../../util/constants.ts";

describeSuite({
    id: "DC0101",
    title: "XcmExecutorUtils - Custom policies",
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

            const createForeignAsset = await polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.utility.batch([
                    // Register parent asset as 1
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION,
                        1,
                        alice.address,
                        true,
                        1
                    ),
                    // Register grandparent asset as 2
                    polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        RELAY_SOURCE_LOCATION_2,
                        2,
                        alice.address,
                        true,
                        1
                    ),
                    polkadotJs.tx.assetRate.create(1, 2_000_000_000_000_000_000n),
                    polkadotJs.tx.assetRate.create(2, 2_000_000_000_000_000_000n),
                    // Set custom policy for parent origin to only allowing grandparent asset
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
                                },
                            ],
                        }
                    ),
                ])
            );

            await context.createBlock(await createForeignAsset.signAsync(alice), {
                allowFailures: false,
            });
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

                const alice_asset_balance = (await polkadotJs.query.foreignAssets.account(2, alice.address))
                    .unwrap()
                    .balance.toBigInt();
                expect(alice_asset_balance > 0n).to.be.true;
                // we should expect to have received less than the amount transferred
                expect(alice_asset_balance < transferredBalance).to.be.true;
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

                // Parent tokens should have been rejected, so asset balance for Alice shouldn't exist
                const alice_asset_balance = await polkadotJs.query.foreignAssets.account(1, alice.address);
                expect(alice_asset_balance.isNone).to.be.true;
            },
        });
    },
});
