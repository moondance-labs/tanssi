import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getCurrentEraStartBlock, getPastEraStartBlock } from "utils/block";

describeSuite({
    id: "SMOKD02",
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
            title: "Era changes are happening as expected and information is correct",
            test: async () => {
                const sessionsPerEra = api.consts.externalValidators.sessionsPerEra.toNumber();
                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));
                const currentEraStartSessionIndex = (
                    await apiAtCurrentEraStart.query.session.currentIndex()
                ).toNumber();
                const apiAtPreviousEraStart = await api.at(await api.rpc.chain.getBlockHash(pastEraStartBlock));
                const previousEraStartSessionIndex = (
                    await apiAtPreviousEraStart.query.session.currentIndex()
                ).toNumber();

                expect(previousEraStartSessionIndex + sessionsPerEra).toEqual(currentEraStartSessionIndex);
            },
        });

        it({
            id: "C02",
            title: "Era slashes are pruned as expected",
            test: async () => {
                const bondingDuration = api.consts.externalValidatorSlashes.bondingDuration.toNumber();
                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));

                // Verify that BoundedEras records are pruned correctly
                const boundedEras = await apiAtCurrentEraStart.query.externalValidatorSlashes.bondedEras();
                const boundedErasErrorRecords = [];
                for (let i = 0; i < boundedEras.length; i++) {
                    const eraIndex: number = boundedEras[i][0].toNumber();
                    if (eraIndex <= currentEraIndex - bondingDuration) {
                        boundedErasErrorRecords.push(boundedEras[i]);
                    }
                }
                expect(boundedErasErrorRecords.length).to.be.equal(
                    0,
                    `Found BoundedEras records outside of bonding duration: ${boundedErasErrorRecords.join("; ")}`
                );

                // Verify that ValidatorSlashInEra are pruned correctly
                const validatorSlashInEra =
                    await apiAtCurrentEraStart.query.externalValidatorSlashes.validatorSlashInEra.keys();
                const validatorSlashInEraErrorRecords = [];
                for (let i = 0; i < validatorSlashInEra.length; i++) {
                    const validatorSlashEra = validatorSlashInEra[i].args[0].toNumber();
                    if (validatorSlashEra <= currentEraIndex - bondingDuration) {
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
                    if (slashesEra <= currentEraIndex - bondingDuration) {
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
            title: "Era rewards are updated as expected",
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
