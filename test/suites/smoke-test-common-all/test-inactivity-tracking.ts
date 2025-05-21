import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK14",
    title: "Inactivity tracking suit",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let lastSessionIndex: number;
        let lastSessionEndBlock: number;

        beforeAll(async () => {
            api = context.polkadotJs();
            lastSessionIndex = (await api.query.session.currentIndex()).toNumber() - 1;

            const getLastSessionEndBlock = async (lastSessionIndex: number) => {
                let blockNumber = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
                let currentSessionIndex = (await api.query.session.currentIndex()).toNumber();
                while (currentSessionIndex > lastSessionIndex) {
                    blockNumber -= 1;
                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const apiAtBlock = await api.at(blockHash);
                    currentSessionIndex = (await apiAtBlock.query.session.currentIndex()).toNumber();
                }
                return blockNumber;
            };

            lastSessionEndBlock = await getLastSessionEndBlock(lastSessionIndex);
        });

        it({
            id: "C01",
            title: "Collator marked as inactive has not produced any blocks in the last session",
            test: async () => {
                const inactiveCollators = await api.query.inactivityTracking.inactiveCollators(lastSessionIndex);
                if (inactiveCollators.size === 0) {
                    log("No inactive collators found");
                    return;
                }
                let currentBlockNumber = lastSessionEndBlock;
                let currentBlockHash = await api.rpc.chain.getBlockHash(currentBlockNumber);
                let currentBlockApi = await api.at(currentBlockHash);
                let currentSessionIndex = (await currentBlockApi.query.session.currentIndex()).toNumber();

                while (currentSessionIndex == lastSessionIndex) {
                    // TODO: Verify that all inactive collators for the last session
                    // haven't produced blocks in the past session

                    currentBlockNumber -= 1;
                    currentBlockHash = await api.rpc.chain.getBlockHash(currentBlockNumber);
                    currentBlockApi = await api.at(currentBlockHash);
                    currentSessionIndex = (await currentBlockApi.query.session.currentIndex()).toNumber();
                }
            },
        });
    },
});
