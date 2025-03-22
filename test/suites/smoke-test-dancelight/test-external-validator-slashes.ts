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

                if (currentEra.index < bondedDuration) {
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

                    blockNumberCheckpointStartEra = await getPastEraStartBlock(api, blockNumberCheckpointPreviousEra);
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

                const firstKeptIndex = activeEra.index.toNumber() - bondedDuration.toNumber();

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

                const firstKeptIndex = activeEra.index.toNumber() - bondedDuration.toNumber();
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
            title: "Slash message should never contain more than QueuedSlashesProcessedPerBlock",
            timeout: 600000,
            test: async () => {
                let currentEra = (await api.query.externalValidators.activeEra()).unwrap();
                const bondedDuration = api.consts.externalValidatorSlashes.bondingDuration;

                const lastBondedEra = currentEra.index.toNumber() - bondedDuration.toNumber();
                const firstBlockNumber = await getCurrentEraStartBlock(api);

                let blockNumber = firstBlockNumber;

                // Go backwards checking for blocks with slashes events
                log(`Analizing backwards ${bondedDuration} eras blocks`);

                while (currentEra.index.toNumber() > lastBondedEra && blockNumber > 0) {
                    const getApiCheckpointAtBlock = await api.at(await api.rpc.chain.getBlockHash(blockNumber));

                    // Check for slashes message events
                    const events = await getApiCheckpointAtBlock.query.system.events();
                    const slashEvent = events.find((event) => event.event.method === "SlashesMessageSent");

                    if (slashEvent) {
                        // Check event command slashes is lower than queuedSlashesProcessedPerBlock
                        const command = slashEvent.event.data.toJSON()[1].reportSlashes.slashes;
                        const queuedSlashesPerBlock =
                            await api.consts.externalValidatorSlashes.queuedSlashesProcessedPerBlock;
                        expect(command.length).to.be.lessThanOrEqual(queuedSlashesPerBlock.toNumber());
                    }

                    currentEra = (await api.query.externalValidators.activeEra()).unwrap();
                    blockNumber--;
                }
            },
        });
    },
});
