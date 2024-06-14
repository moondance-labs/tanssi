export default {
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
