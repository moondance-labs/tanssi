import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getCurrentEraStartBlock, getPastEraStartBlock } from "utils/block";

describeSuite({
    id: "SMOK16",
    title: "Era changes suit that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let currentEraIndex: number;
        let currentEraStartBlock: number;
        let pastEraStartBlock: number;

        beforeAll(async () => {
            api = context.polkadotJs();
            currentEraStartBlock = await getCurrentEraStartBlock(api);
            currentEraIndex = (await api.query.externalValidators.activeEra()).unwrap().index.toNumber();
            pastEraStartBlock = await getPastEraStartBlock(api, currentEraStartBlock - 1);
        });

        it({
            id: "C01",
            title: "Era changes are happening as expected",
            test: async () => {
                const sessionsPerEra = api.consts.externalValidators.sessionsPerEra.toNumber();
                const currentBlockNumber = await api.query.system.number();
                const currentEraIndex = (await api.query.externalValidators.activeEra()).unwrap().index.toNumber();
                const currentSessionIndex = (await api.query.session.currentIndex()).toNumber();

                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));
                const currentEraStartSessionIndex = (
                    await apiAtCurrentEraStart.query.session.currentIndex()
                ).toNumber();
                const newEraEvents = (await apiAtCurrentEraStart.query.system.events()).filter(
                    (eventRecord) => eventRecord.event.method === "NewEra"
                );
                const apiAtPreviousEraStart = await api.at(await api.rpc.chain.getBlockHash(pastEraStartBlock));
                const previousEraStartSessionIndex = (
                    await apiAtPreviousEraStart.query.session.currentIndex()
                ).toNumber();

                expect(newEraEvents.length).to.be.greaterThan(
                    0,
                    `No NewEra event found at the start of era ${currentEraIndex}`
                );
                expect(newEraEvents[0].event.data[0].toHuman()).to.be.equal(
                    currentEraIndex.toString(),
                    `NewEra event data does not match current era index ${currentEraIndex} at the start of the era.`
                );
                expect(previousEraStartSessionIndex + sessionsPerEra).to.be.equal(
                    currentEraStartSessionIndex,
                    `Error at block number ${currentBlockNumber}: Era change between era ${currentEraIndex - 1} and ${currentEraIndex} happened in ${currentEraStartSessionIndex - previousEraStartSessionIndex} sessions instead of ${sessionsPerEra} sessions.`
                );
                expect(currentSessionIndex).to.be.within(
                    currentEraIndex * sessionsPerEra,
                    (currentEraIndex + 1) * sessionsPerEra,
                    `Error at block number ${currentBlockNumber}: Current session index ${currentSessionIndex} is not within the expected range for era ${currentEraIndex}.`
                );
            },
        });

        it({
            id: "C02",
            title: "Era slashes records are pruned as expected",
            test: async () => {
                const bondingDuration = api.consts.externalValidatorSlashes.bondingDuration.toNumber();
                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));

                // Verify that BoundedEras records are pruned correctly
                const boundedEras = await apiAtCurrentEraStart.query.externalValidatorSlashes.bondedEras();
                const boundedErasErrorRecords = [];
                for (let i = 0; i < boundedEras.length; i++) {
                    const eraIndex: number = boundedEras[i][0].toNumber();
                    if (eraIndex < currentEraIndex - bondingDuration) {
                        boundedErasErrorRecords.push(boundedEras[i]);
                    }
                }
                expect(boundedErasErrorRecords.length).to.be.equal(
                    0,
                    `Found BoundedEras records outside of bonding duration: ${boundedErasErrorRecords.join("\n")}`
                );

                // Verify that ValidatorSlashInEra are pruned correctly
                const validatorSlashInEra =
                    await apiAtCurrentEraStart.query.externalValidatorSlashes.validatorSlashInEra.keys();
                const validatorSlashInEraErrorRecords = [];
                for (let i = 0; i < validatorSlashInEra.length; i++) {
                    const validatorSlashEra = validatorSlashInEra[i].args[0].toNumber();
                    if (validatorSlashEra < currentEraIndex - bondingDuration) {
                        validatorSlashInEraErrorRecords.push(validatorSlashInEra[i]);
                    }
                }
                expect(validatorSlashInEraErrorRecords.length).to.be.equal(
                    0,
                    `Found ValidatorSlashInEra records outside of bonding duration: ${validatorSlashInEraErrorRecords.join("\n")}`
                );

                // Verify that Slashes are pruned correctly
                const slashes = await apiAtCurrentEraStart.query.externalValidatorSlashes.slashes.keys();
                const slashesErrorRecords: number[] = [];
                for (let i = 0; i < slashes.length; i++) {
                    const slashesEra = slashes[i].args[0].toNumber();
                    if (slashesEra < currentEraIndex - bondingDuration) {
                        slashesErrorRecords.push(slashesEra);
                    }
                }
                expect(slashesErrorRecords.length).to.be.equal(
                    0,
                    `Found Slashes records outside of bonding duration: ${slashesErrorRecords.join("\n")}`
                );
            },
        });

        it({
            id: "C03",
            title: "Era rewards records are pruned as expected",
            test: async () => {
                const rewardsHistoryDepth = api.consts.externalValidatorsRewards.historyDepth.toNumber();
                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));

                const currentEraRewards =
                    await apiAtCurrentEraStart.query.externalValidatorsRewards.rewardPointsForEra.keys();
                const currentEraRewardsErrorRecords = [];

                for (let i = 0; i < currentEraRewards.length; i++) {
                    const eraIndex = currentEraRewards[i].args[0].toNumber();
                    if (eraIndex <= currentEraIndex - rewardsHistoryDepth) {
                        currentEraRewardsErrorRecords.push(currentEraRewards[i]);
                    }
                }

                expect(currentEraRewardsErrorRecords.length).to.be.equal(
                    0,
                    `Found RewardPointsForEra records outside of history depth: ${currentEraRewardsErrorRecords.join("\n")}`
                );
            },
        });
    },
});
