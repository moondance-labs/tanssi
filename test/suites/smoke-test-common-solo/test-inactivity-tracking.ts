import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { getLastSessionEndBlock } from "utils/block";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK17",
    title: "Inactivity tracking suit",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let lastSessionIndex: number;
        let lastSessionEndBlock: number;
        let isStarlightRuntime: boolean;
        let runtimeName: string;
        let specVersion: number;
        let shouldSkipStarlightSmokeTest: boolean;

        beforeAll(async () => {
            api = context.polkadotJs();
            // Check if the runtime is Starlight and set the spec version
            runtimeName = api.runtimeVersion.specName.toString();
            specVersion = api.consts.system.version.specVersion.toNumber();
            isStarlightRuntime = runtimeName === "starlight";
            shouldSkipStarlightSmokeTest = isStarlightRuntime && specVersion < 1500;
            if (shouldSkipStarlightSmokeTest) {
                return;
            }

            lastSessionIndex = (await api.query.session.currentIndex()).toNumber() - 1;
            lastSessionEndBlock = await getLastSessionEndBlock(api, lastSessionIndex);
        });

        it({
            id: "C01",
            title: "Collators marked as inactive have not produced any blocks in the last session",
            test: async () => {
                if (shouldSkipStarlightSmokeTest) {
                    console.log(`Skipping C01 test for Starlight runtime version ${specVersion}`);
                    return;
                }

                const inactiveCollators = await api.query.inactivityTracking.inactiveCollators(lastSessionIndex);

                if (inactiveCollators.size === 0) {
                    log(`No inactive collators found for session ${lastSessionIndex}. Skipping check...`);
                    return;
                }

                let currentBlockNumber = lastSessionEndBlock;
                let currentBlockHash = await api.rpc.chain.getBlockHash(currentBlockNumber);
                let currentBlockApi = await api.at(currentBlockHash);
                let currentSessionIndex = (await currentBlockApi.query.session.currentIndex()).toNumber();

                const registeredParaIds = await currentBlockApi.query.containerRegistrar.registeredParaIds();

                const failureMessages: string[] = [];

                log("Expecting no inactive collators to be block authors for any paraId in the last session!");
                while (currentSessionIndex === lastSessionIndex) {
                    // For every registered paraId, check if the latest author is in the inactive collators list
                    for (const paraId of registeredParaIds) {
                        const latestAuthorInfo = await currentBlockApi.query.authorNoting.latestAuthor(paraId);
                        if (latestAuthorInfo.isSome) {
                            const authorInfo = latestAuthorInfo.unwrap();
                            const authorId = authorInfo.author;
                            if (inactiveCollators.has(authorId)) {
                                failureMessages.push(
                                    `Collator ${authorId.toString()} is marked as inactive but authored block ${authorInfo.blockNumber} for container chain ${paraId} in session ${lastSessionIndex}.`
                                );
                            }
                        }
                    }
                    // Move to the previous block
                    currentBlockNumber -= 1;
                    currentBlockHash = await api.rpc.chain.getBlockHash(currentBlockNumber);
                    currentBlockApi = await api.at(currentBlockHash);
                    currentSessionIndex = (await currentBlockApi.query.session.currentIndex()).toNumber();
                }
                // Log all records of inactive collators being block authors for the last session
                for (const message of failureMessages) {
                    log(message);
                }
                expect(failureMessages.length).toBe(0);
            },
        });
    },
});
