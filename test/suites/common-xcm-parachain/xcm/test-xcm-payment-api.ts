import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { STATEMINT_LOCATION_EXAMPLE } from "utils";

const runtimeApi = {
    runtime: {
        XcmPaymentApi: [
            {
                methods: {
                    query_acceptable_payment_assets: {
                        description: "The API to query acceptable payment assets",
                        params: [
                            {
                                name: "version",
                                type: "u32",
                            },
                        ],
                        type: "Result<Vec<XcmVersionedAssetId>, XcmPaymentApiError>",
                    },
                    query_weight_to_asset_fee: {
                        description: "",
                        params: [
                            {
                                name: "weight",
                                type: "WeightV2",
                            },
                            {
                                name: "asset",
                                type: "XcmVersionedAssetId",
                            },
                        ],
                        type: "Result<u128, XcmPaymentApiError>",
                    },
                    query_xcm_weight: {
                        description: "",
                        params: [
                            {
                                name: "message",
                                type: "XcmVersionedXcm",
                            },
                        ],
                        type: "Result<WeightV2, XcmPaymentApiError>",
                    },
                },
                version: 1,
            },
        ],
    },
    types: {
        XcmPaymentApiError: {
            _enum: {
                Unimplemented: "Null",
                VersionedConversionFailed: "Null",
                WeightNotComputable: "Null",
                UnhandledXcmVersion: "Null",
                AssetNotFound: "Null",
            },
        },
    },
};

describeSuite({
    id: "COMMON0307",
    title: "XCM - XcmPaymentApi",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: any;

        beforeAll(async () => {
            // Not using context.polkadotJs() because we need to add the runtime api
            // This won't be needed after @polkadot/api adds the XcmPaymentApi
            polkadotJs = await ApiPromise.create({
                provider: new WsProvider(`ws://localhost:${process.env.MOONWALL_RPC_PORT}/`),
                ...runtimeApi,
            });
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });

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
            title: "Should succeed calling runtime api",
            test: async () => {
                const chainInfo = polkadotJs.registry.getChainProperties();
                const metadata = await polkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();

                console.log(chainInfo.toHuman());

                const assets = await polkadotJs.call.xcmPaymentApi.queryAcceptablePaymentAssets(3);
                const weightToNativeAssets = await polkadotJs.call.xcmPaymentApi.queryWeightToAssetFee(
                    {
                        refTime: 10_000_000_000n,
                        proofSize: 0n,
                    },
                    {
                        V3: {
                            Concrete: {
                                parents: 0,
                                interior: {
                                    X1: { PalletInstance: Number(balancesPalletIndex) },
                                },
                            },
                        },
                    }
                );

                const weightToForeingAssets = await polkadotJs.call.xcmPaymentApi.queryWeightToAssetFee(
                    {
                        refTime: 10_000_000_000n,
                        proofSize: 0n,
                    },
                    {
                        V3: {
                            Concrete: STATEMINT_LOCATION_EXAMPLE,
                        },
                    }
                );

                const transactWeightAtMost = {
                    refTime: 200000000n,
                    proofSize: 3000n,
                };
                const xcmToWeight = await polkadotJs.call.xcmPaymentApi.queryXcmWeight({
                    V3: [
                        {
                            Transact: {
                                originKind: "Superuser",
                                requireWeightAtMost: transactWeightAtMost,
                                call: {
                                    encoded:
                                        "0x0408001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0284d717",
                                },
                            },
                        },
                    ],
                });
                // Uncomment to debug if test fails
                /*
                console.log(
                    "assets:",
                    JSON.stringify(assets.toJSON()),
                    "\nweightToNativeAsset: ",
                    weightToNativeAssets.toHuman(),
                    "\nweightToForeingAsset: ",
                    weightToForeingAssets.toHuman(),
                    "\nxcmToWeight: ",
                    xcmToWeight.toHuman()
                );
                */

                expect(assets.isOk).to.be.true;
                // Includes the native asset and the asset registered in foreignAssetsCreator
                expect(assets.asOk.toJSON().length).to.be.equal(2);
                expect(xcmToWeight.isOk).to.be.true;
                // Weight estimated by queryXcmWeight will always be greater than the weight passed to the transact call as requireWeightAtMost
                expect(xcmToWeight.asOk.refTime.toBigInt() > transactWeightAtMost.refTime).to.be.true;
                expect(xcmToWeight.asOk.proofSize.toBigInt() > transactWeightAtMost.proofSize).to.be.true;

                // foreign*2=native
                const diff = weightToNativeAssets.asOk.toBigInt() - 2n * weightToForeingAssets.asOk.toBigInt();
                // Allow rounding error of +/- 1
                expect(diff >= -1n && diff <= 1n).to.be.true;
            },
        });
    },
});
