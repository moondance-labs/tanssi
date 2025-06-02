import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { PRIMARY_GOVERNANCE_CHANNEL_ID, getCurrentEraStartBlock, getPastEraStartBlock } from "utils";

describeSuite({
    id: "SMOK09",
    title: "Smoke tests for external validators slashes pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Number of bonded eras does not exceed bonding duration",
            test: async () => {
                const bondedEras = await api.query.externalValidatorSlashes.bondedEras();
                const bondedDuration = api.consts.externalValidatorSlashes.bondingDuration;
                const currentEra = (await api.query.externalValidators.activeEra()).unwrap();

                if (currentEra.index.toNumber() < bondedDuration.toNumber()) {
                    expect(bondedEras.length).to.be.lessThanOrEqual(bondedDuration.toNumber() + 1);
                } else {
                    expect(bondedEras.length).to.be.eq(bondedDuration.toNumber() + 1);
                }
            },
        });

        it({
            id: "C02",
            title: "Slash message should increase nonce",
            timeout: 600000,
            test: async () => {
                const bondedDuration = api.consts.externalValidatorSlashes.bondingDuration;
                const currentEra = (await api.query.externalValidators.activeEra()).unwrap();

                let eraToBeAnalyzed = currentEra.index;
                const firstBlockNumber = await getCurrentEraStartBlock(api);
                let blockNumberCheckpointStartEra = firstBlockNumber;

                log(`Analizing backwards ${bondedDuration} eras`);

                // Go backwards bondedDuration eras
                // We will analyze all the bounded eras and not just the last
                while (
                    eraToBeAnalyzed.toNumber() > currentEra.index.toNumber() - bondedDuration.toNumber() &&
                    eraToBeAnalyzed.toNumber() > 0
                ) {
                    const blockNumberCheckpointPreviousEra = blockNumberCheckpointStartEra - 1;

                    try {
                        blockNumberCheckpointStartEra = await getPastEraStartBlock(
                            api,
                            blockNumberCheckpointPreviousEra
                        );
                    } catch (e) {
                        log(`Current era is 0, exiting. Block: ${blockNumberCheckpointPreviousEra}`);
                        break;
                    }

                    const apiAtCheckpointStartEra = await api.at(
                        await api.rpc.chain.getBlockHash(blockNumberCheckpointStartEra)
                    );

                    // We dont have an event, so we will check if we have unreported slashes
                    const apitAtCheckpointPreviousEraEnd = await api.at(
                        await api.rpc.chain.getBlockHash(blockNumberCheckpointStartEra - 1)
                    );
                    const unreportedSlashes =
                        await apitAtCheckpointPreviousEraEnd.query.externalValidatorSlashes.unreportedSlashesQueue();
                    eraToBeAnalyzed = (
                        await apitAtCheckpointPreviousEraEnd.query.externalValidators.activeEra()
                    ).unwrap().index;

                    if (unreportedSlashes.length > 0) {
                        // at the beginning of the next era, we should have sent a message
                        // therefore the nonce has increased
                        const nonceBefore =
                            await apitAtCheckpointPreviousEraEnd.query.ethereumOutboundQueue.nonce(
                                PRIMARY_GOVERNANCE_CHANNEL_ID
                            );
                        const nonceAfter =
                            await apiAtCheckpointStartEra.query.ethereumOutboundQueue.nonce(
                                PRIMARY_GOVERNANCE_CHANNEL_ID
                            );
                        expect(
                            nonceAfter.toNumber(),
                            `Slash message for era ${eraToBeAnalyzed} was not sent even if we have  ${unreportedSlashes.length} to be reported in block ${blockNumberCheckpointStartEra}`
                        ).toBe(nonceBefore.toNumber() + 1);
                    }
                }
            },
        });

        it({
            id: "C03",
            title: "Slashes should expire after bonding period",
            test: async () => {
                const activeEra = (await api.query.externalValidators.activeEra()).unwrap();
                const bondedDuration = api.consts.externalValidatorSlashes.bondingDuration;

                const firstKeptIndex = Math.max(activeEra.index.toNumber() - bondedDuration.toNumber(), 0);

                const allSlashes = await api.query.externalValidatorSlashes.slashes.entries();
                const eraIndexes = allSlashes.map((entry) => {
                    return entry[0].args[0].toNumber();
                });

                // All slashes era indexes should be newest than the first kept index
                expect(
                    eraIndexes.every((eraIndex) => {
                        return eraIndex >= firstKeptIndex;
                    })
                ).toBe(true);
            },
        });

        it({
            id: "C04",
            title: "Bonded eras should be cleaned after bonding period",
            test: async () => {
                const activeEra = (await api.query.externalValidators.activeEra()).unwrap();
                const bondedDuration = api.consts.externalValidatorSlashes.bondingDuration;

                const firstKeptIndex = Math.max(activeEra.index.toNumber() - bondedDuration.toNumber(), 0);
                const allBondedEras = await api.query.externalValidatorSlashes.bondedEras();

                // We shouldn't have more bonded eras than the bondedDuration
                expect(allBondedEras.length).to.be.lessThanOrEqual(bondedDuration.toNumber() + 1);

                // All bondedEras era indexes should be greater or equal than firstKeptIndex
                expect(
                    allBondedEras.every((bondedEra) => {
                        return bondedEra[0].toNumber() >= firstKeptIndex;
                    })
                ).toBe(true);
            },
        });

        it({
            id: "C05",
            title: "A slash message must never exceed the queuedSlashesProcessedPerBlock limit",
            timeout: 600000,
            test: async () => {
                const bondingDuration = api.consts.externalValidatorSlashes.bondingDuration;
                const maxQueuedSlashesPerBlock = api.consts.externalValidatorSlashes.queuedSlashesProcessedPerBlock;

                const currentEra = (await api.query.externalValidators.activeEra()).unwrap();
                const currentEraIndex = currentEra.index.toNumber();

                let eraIndexToAnalyze = currentEraIndex;
                let startBlockNumberOfEra = await getCurrentEraStartBlock(api);

                log(`Analyzing up to ${bondingDuration} eras in the past`);

                /*
                We move backwards through the eras, up to 'bondingDuration' eras.
                For each era, we check if there were any unreported slashes at the start of that era.
                If there were, we then iterate through the blocks within that era looking for
                a 'SlashesMessageSent' event. If found, we verify that the slash message
                does not contain more slashes than 'maxQueuedSlashesPerBlock'.
                */
                while (eraIndexToAnalyze > currentEraIndex - bondingDuration.toNumber() && eraIndexToAnalyze > 0) {
                    // Identify the start/end blocks for the previous era
                    const lastBlockNumberPreviousEra = startBlockNumberOfEra - 1;

                    let firstBlockNumberPreviousEra: number;
                    try {
                        firstBlockNumberPreviousEra = await getPastEraStartBlock(api, lastBlockNumberPreviousEra);
                    } catch (e) {
                        log(`Current era is 0, exiting. Block: ${lastBlockNumberPreviousEra}`);
                        break;
                    }

                    // Get the count of unreported slashes at the very start of the previous era
                    const apiAtStartPreviousEra = await api.at(
                        await api.rpc.chain.getBlockHash(firstBlockNumberPreviousEra)
                    );
                    let unreportedSlashCount = (
                        await apiAtStartPreviousEra.query.externalValidatorSlashes.unreportedSlashesQueue()
                    ).length;

                    // If there were unreported slashes at the start of the era, look for slash messages
                    let blockNumber = firstBlockNumberPreviousEra;
                    while (unreportedSlashCount > 0 && blockNumber <= firstBlockNumberPreviousEra) {
                        const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                        const apiAtBlock = await api.at(blockHash);
                        const events = await apiAtBlock.query.system.events();

                        // Search for a SlashesMessageSent event in this block
                        const slashMessageEvent = events.find((event) => event.event.method === "SlashesMessageSent");

                        if (slashMessageEvent) {
                            // The slashCommand array cannot exceed the queuedSlashesProcessedPerBlock limit
                            const slashCommand = slashMessageEvent.event.data.toJSON()[1].reportSlashes.slashes;
                            expect(slashCommand.length).to.be.lessThanOrEqual(maxQueuedSlashesPerBlock.toNumber());

                            // Reduce our count of unreported slashes accordingly
                            unreportedSlashCount -= slashCommand.length;
                        }

                        blockNumber++;
                    }

                    // Move to the previous era for the next iteration
                    const previousEraInfo = (await apiAtStartPreviousEra.query.externalValidators.activeEra()).unwrap();
                    eraIndexToAnalyze = previousEraInfo.index.toNumber();
                    startBlockNumberOfEra = firstBlockNumberPreviousEra;
                }
            },
        });
    },
});
