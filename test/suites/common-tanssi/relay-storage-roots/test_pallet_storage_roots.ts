import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "CT0801",
    title: "RelayStorageRoots pallet test suite",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Only 10 latest blocks are stored",
            test: async function () {
                // Relay block list starts empty
                const relayBlocksEmpty = (await polkadotJs.query.relayStorageRoots.relayStorageRootKeys()).toJSON();
                expect(relayBlocksEmpty).to.deep.equal([]);

                // Create 30 blocks
                for (let i = 0; i < 20; i++) {
                    await context.createBlock();
                }

                // Only latest 10 will be stored
                // relay_block_number = tanssi_block_number * 2 + 1000
                const relayBlocks = (await polkadotJs.query.relayStorageRoots.relayStorageRootKeys()).toJSON();
                expect(relayBlocks).to.deep.equal([1020, 1022, 1024, 1026, 1028, 1030, 1032, 1034, 1036, 1038]);

                // The mapping only contains the keys that are in `relayStorageRootKeys`
                const mappingKeys = (await polkadotJs.query.relayStorageRoots.relayStorageRoot.keys()).map((key) => {
                    // Convert "1,020" into 1020
                    return parseInt(key.toHuman().toString().replace(",", ""));
                });
                mappingKeys.sort();
                expect(relayBlocks).to.deep.equal(mappingKeys);
            },
        });
    },
});
