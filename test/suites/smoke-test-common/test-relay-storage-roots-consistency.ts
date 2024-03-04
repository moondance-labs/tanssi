import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S11",
    title: "Test relay storage roots max number",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(() => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Only MaxRelayStorageRoots should be stored",
            test: async function () {
                if (runtimeVersion < 500) {
                    return;
                }
                const maxStorageRoots = (await api.consts.relayStorageRoots.maxStorageRoots).toNumber();
                const relayStorageRoots = (await api.query.relayStorageRoots.relayStorageRoot.keys()).length;
                const relayStorageRootKeys = (await api.query.relayStorageRoots.relayStorageRootKeys()).length;
                expect(
                    maxStorageRoots,
                    `We should store ${maxStorageRoots} roots at most and we have ${relayStorageRoots}`
                ).toBe(relayStorageRoots);

                expect(
                    maxStorageRoots,
                    `We should store ${maxStorageRoots} keys at most and we have ${relayStorageRootKeys}`
                ).toBe(relayStorageRootKeys);
            },
        });
        it({
            id: "C02",
            title: "All numbers should have its corresponding root",
            test: async function () {
                if (runtimeVersion < 500) {
                    return;
                }
                const latestBlock = await api.rpc.chain.getBlock();
                const latestBlockHash = latestBlock.block.hash;
                const apiAtBlock = await api.at(latestBlockHash);
                const relayStorageRootKeys = await apiAtBlock.query.relayStorageRoots.relayStorageRootKeys();
                for (const number of relayStorageRootKeys) {
                    expect(
                        (await apiAtBlock.query.relayStorageRoots.relayStorageRoot(number)).isSome,
                        `Block Number ${number} should have a corresponding root`
                    ).to.be.true;
                }
            },
        });
    },
});
