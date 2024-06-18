import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { KeyringPair, alith } from "@moonwall/util";
import {
    MultiLocation,
    extractPaidDeliveryFees,
    getLastSentHrmpMessageFee,
    XcmFragment,
    mockHrmpChannelExistanceTx,
} from "../../../util/xcm";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";

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
    id: "CX0207",
    title: "XCM - XcmPaymentApi",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let baseDelivery: bigint;
        let chain;
        const destinationPara = 3000;
        const txByteFee = 1n;

        beforeAll(async function () {
            // Not using context.polkadotJs() because we need to add the runtime api
            // This won't be needed after @polkadot/api adds the XcmPaymentApi
            polkadotJs = await ApiPromise.create({
                provider: new WsProvider(`ws://localhost:${process.env.MOONWALL_RPC_PORT}/`),
                ...runtimeApi,
            });
            chain = polkadotJs.consts.system.version.specName.toString();
            alice =
                chain == "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });
            baseDelivery = chain == "frontier-template" ? 100_000_000_000_000n : 100_000_000n;
        });

        it({
            id: "T01",
            title: "Should succeed calling runtime api",
            test: async function () {
                const api = polkadotJs;
                const chainInfo = api.registry.getChainProperties();
                const metadata = await api.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() == "Balances")!
                    .index.toNumber();

                console.log(chainInfo.toHuman());

                const assets = await api.call.xcmPaymentApi.queryAcceptablePaymentAssets(3);
                const weightToNativeAssets = await api.call.xcmPaymentApi.queryWeightToAssetFee(
                    {
                        refTime: 10_000_000_000n,
                        profSize: 0n,
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

                const weightToForeingAssets = await api.call.xcmPaymentApi.queryWeightToAssetFee(
                    {
                        refTime: 10_000_000_000n,
                        profSize: 0n,
                    },
                    {
                        V3: {
                            Concrete: {
                                parents: 1,
                                interior: {
                                    x1: {
                                        parachain: 2040,
                                    },
                                },
                            },
                        },
                    }
                );

                const transactWeightAtMost = {
                    refTime: 200000000n,
                    proofSize: 3000n,
                };
                const xcmToWeight = await api.call.xcmPaymentApi.queryXcmWeight({
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
                console.log(
                    "assets:",
                    assets.toJSON(),
                    "\nweightToNativeAsset: ",
                    weightToNativeAssets.toHuman(),
                    "\nweightToForeingAsset: ",
                    weightToForeingAssets.toHuman(),
                    "\nxcmToWeight: ",
                    xcmToWeight.toHuman()
                );

                expect(assets.isOk).to.be.true;
                expect(assets.asOk.toJSON().length).to.be.equal(1);
                expect(xcmToWeight.isOk).to.be.true;
                // Weight estimated by queryXcmWeight will always be greater than the weight passed to the transact call as requireWeightAtMost
                expect(xcmToWeight.asOk.refTime.toBigInt() > transactWeightAtMost.refTime).to.be.true;
                expect(xcmToWeight.asOk.proofSize.toBigInt() > transactWeightAtMost.proofSize).to.be.true;

                // Output of console.log:
                // TODO: add expects?
                /*
                assets: { ok: [ { v3: [Object] } ] } 
                weightToNativeAsset:  { Ok: '93,393,354,128,920' } 
                weightToForeingAsset:  { Err: 'AssetNotFound' } 
                xcmToWeight:  { Ok: { refTime: '265,490,000', proofSize: '6,997' } }
                */
            },
        });
    },
});
