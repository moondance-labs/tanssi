import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "COM0401",
    title: "Ensure assets are registered in genesis",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let chain: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            chain = polkadotJs.consts.system.version.specName.toString();
        });

        it({
            id: "T01",
            title: "Genesis should register ETH tokens",
            test: async () => {
                const assetId = "1";
                const assetLocation = {
                    interior: {
                        x1: [
                            {
                                globalConsensus: {
                                    ethereum: {
                                        chainId: 11155111,
                                    },
                                },
                            },
                        ],
                    },
                    parents: 2,
                };

                const mappedLocationFromId = await polkadotJs.query.foreignAssetsCreator.assetIdToForeignAsset(assetId);
                expect(mappedLocationFromId.toJSON()).to.deep.eq(assetLocation);

                const mappedIdFromLocation =
                    await polkadotJs.query.foreignAssetsCreator.foreignAssetToAssetId(assetLocation);
                expect(mappedIdFromLocation.toHuman()).to.eq(assetId);

                const assetDetails = (await polkadotJs.query.foreignAssets.asset(assetId)).unwrap();
                expect(assetDetails.admin.toHuman()).to.eq(alice.address);
                expect(assetDetails.minBalance.toHuman()).to.eq("1");
                expect(assetDetails.isSufficient.toHuman()).to.eq(true);
            },
        });

        it({
            id: "T02",
            title: "Genesis should register TANSSI tokens",
            test: async () => {
                const assetId = "2";
                const assetLocation = {
                    interior: { here: null },

                    parents: 1,
                };

                const mappedLocationFromId = await polkadotJs.query.foreignAssetsCreator.assetIdToForeignAsset(assetId);
                expect(mappedLocationFromId.toJSON()).to.deep.eq(assetLocation);

                const mappedIdFromLocation =
                    await polkadotJs.query.foreignAssetsCreator.foreignAssetToAssetId(assetLocation);
                expect(mappedIdFromLocation.toHuman()).to.eq(assetId);

                const assetDetails = (await polkadotJs.query.foreignAssets.asset(assetId)).unwrap();
                expect(assetDetails.admin.toHuman()).to.eq(alice.address);
                expect(assetDetails.minBalance.toHuman()).to.eq("1");
                expect(assetDetails.isSufficient.toHuman()).to.eq(true);
            },
        });
    },
});
