import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT2001",
    title: "Foreign assets creation",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
        });

        it({
            id: "T01",
            title: "Should succeed calling foreign assets creation for ethereum token and sudo",
            test: async () => {
                const tokenLocation = {
                    parents: 0,
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
            title: "Should succeed calling foreign assets creation for ethereum token and sudo",
            test: async () => {
                const tokenLocation = {
                    parents: 0,
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
