import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR } from "helpers";

describeSuite({
    id: "DEVT2001",
    title: "Foreign assets creation",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let specVersion: number;
        let shouldSkipStarlightForeignAssetsCreation: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;

            const chain = polkadotJs.consts.system.version.specName.toString();
            const isStarlight = chain === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightForeignAssetsCreation =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_FOREIGN_ASSETS_CREATOR.includes(specVersion);
        });

        it({
            id: "T01",
            title: "Should succeed calling foreign assets creation for ethereum token and sudo",
            test: async () => {
                if (shouldSkipStarlightForeignAssetsCreation) {
                    console.log(`Skipping DEVT2001T01 test for Starlight version ${specVersion}`);
                    return;
                }

                const tokenLocation = {
                    parents: 1,
                    interior: {
                        x2: [
                            {
                                globalConsensus: {
                                    ethereum: { chainId: 1 },
                                },
                            },
                            {
                                accountKey20: {
                                    network: {
                                        ethereum: { chainId: 1 },
                                    },
                                    key: "0x0000000000000000000000000000000000000000",
                                },
                            },
                        ],
                    },
                };

                const assetId = 42;

                const tx = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                            tokenLocation,
                            assetId,
                            bob.address,
                            true,
                            1
                        )
                    )
                    .signAsync(alice);

                await context.createBlock([tx], { allowFailures: false });

                // Check events
                const events = await polkadotJs.query.system.events();
                const foreignAssetsCreatorEvent = events
                    .find(
                        ({ event: { section, method } }) =>
                            section === "foreignAssetsCreator" && method === "ForeignAssetCreated"
                    )
                    .event.data.toJSON();

                expect(foreignAssetsCreatorEvent).toBeDefined();
            },
        });

        it({
            id: "T02",
            title: "Bad origin when trying to create foreign asset without sudo",
            test: async () => {
                if (shouldSkipStarlightForeignAssetsCreation) {
                    console.log(`Skipping DEVT2001T02 test for Starlight version ${specVersion}`);
                    return;
                }

                const tokenLocation = {
                    parents: 1,
                    interior: {
                        x2: [
                            {
                                globalConsensus: {
                                    ethereum: { chainId: 1 },
                                },
                            },
                            {
                                accountKey20: {
                                    network: {
                                        ethereum: { chainId: 1 },
                                    },
                                    key: "0x0000000000000000000000000000000000000000",
                                },
                            },
                        ],
                    },
                };

                const assetId = 42;

                const tx = await polkadotJs.tx.foreignAssetsCreator
                    .createForeignAsset(tokenLocation, assetId, bob.address, true, 1)
                    .signAsync(alice);

                const {
                    result: [foreignAssetsCreatorAttempt],
                } = await context.createBlock([tx]);
                expect(foreignAssetsCreatorAttempt.successful).toEqual(false);
                expect(foreignAssetsCreatorAttempt.error.name).toEqual("BadOrigin");
            },
        });
    },
});
