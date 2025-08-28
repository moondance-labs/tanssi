import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK13",
    title: "Pooled staking holds consistency smoke tests",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Sum of holds across all delegations matches computed stakes",
            timeout: 300000,
            test: async () => {
                // Get all delegator-candidate pairs
                const delegatorCandidateSummaries = await api.query.pooledStaking.delegatorCandidateSummaries.entries();
                
                log(`Checking consistency for ${delegatorCandidateSummaries.length} delegator-candidate pairs...`);

                const inconsistencies: Array<{
                    delegator: string;
                    candidate: string;
                    totalHeld: bigint;
                    totalComputed: bigint;
                    pools: {
                        joining: { held: bigint; computed: bigint };
                        autoCompounding: { held: bigint; computed: bigint };
                        manualRewards: { held: bigint; computed: bigint };
                        leaving: { held: bigint; computed: bigint };
                    };
                }> = [];

                for (const [key, summary] of delegatorCandidateSummaries) {
                    const [delegator, candidate] = key.args;
                    const delegatorHex = delegator.toHex();
                    const candidateHex = candidate.toHex();

                    // Get held amounts for each pool
                    const joiningHeld = (await api.query.pooledStaking.pools(candidateHex, {
                        JoiningSharesHeldStake: { delegator: delegatorHex }
                    })).toBigInt();

                    const autoCompoundingHeld = (await api.query.pooledStaking.pools(candidateHex, {
                        AutoCompoundingSharesHeldStake: { delegator: delegatorHex }
                    })).toBigInt();

                    const manualRewardsHeld = (await api.query.pooledStaking.pools(candidateHex, {
                        ManualRewardsSharesHeldStake: { delegator: delegatorHex }
                    })).toBigInt();

                    const leavingHeld = (await api.query.pooledStaking.pools(candidateHex, {
                        LeavingSharesHeldStake: { delegator: delegatorHex }
                    })).toBigInt();

                    // Calculate computed stakes from shares
                    const joiningComputed = await calculateComputedStake(api, candidateHex, delegatorHex, "Joining");
                    const autoCompoundingComputed = await calculateComputedStake(api, candidateHex, delegatorHex, "AutoCompounding");
                    const manualRewardsComputed = await calculateComputedStake(api, candidateHex, delegatorHex, "ManualRewards");
                    const leavingComputed = await calculateComputedStake(api, candidateHex, delegatorHex, "Leaving");

                    const totalHeld = joiningHeld + autoCompoundingHeld + manualRewardsHeld + leavingHeld;
                    const totalComputed = joiningComputed + autoCompoundingComputed + manualRewardsComputed + leavingComputed;

                    // Allow for small rounding differences (up to 10 units)
                    const tolerance = 10n;
                    const difference = totalHeld > totalComputed 
                        ? totalHeld - totalComputed 
                        : totalComputed - totalHeld;

                    if (difference > tolerance) {
                        inconsistencies.push({
                            delegator: delegatorHex,
                            candidate: candidateHex,
                            totalHeld,
                            totalComputed,
                            pools: {
                                joining: { held: joiningHeld, computed: joiningComputed },
                                autoCompounding: { held: autoCompoundingHeld, computed: autoCompoundingComputed },
                                manualRewards: { held: manualRewardsHeld, computed: manualRewardsComputed },
                                leaving: { held: leavingHeld, computed: leavingComputed },
                            }
                        });
                    }
                }

                // Report any inconsistencies found
                if (inconsistencies.length > 0) {
                    log(`Found ${inconsistencies.length} inconsistencies:`);
                    for (const inconsistency of inconsistencies) {
                        log(`Delegator: ${inconsistency.delegator}`);
                        log(`Candidate: ${inconsistency.candidate}`);
                        log(`Total held: ${inconsistency.totalHeld}`);
                        log(`Total computed: ${inconsistency.totalComputed}`);
                        log(`Difference: ${inconsistency.totalHeld > inconsistency.totalComputed 
                            ? inconsistency.totalHeld - inconsistency.totalComputed 
                            : inconsistency.totalComputed - inconsistency.totalHeld}`);
                        log(`Pool breakdown:`);
                        log(`  Joining - Held: ${inconsistency.pools.joining.held}, Computed: ${inconsistency.pools.joining.computed}`);
                        log(`  AutoCompounding - Held: ${inconsistency.pools.autoCompounding.held}, Computed: ${inconsistency.pools.autoCompounding.computed}`);
                        log(`  ManualRewards - Held: ${inconsistency.pools.manualRewards.held}, Computed: ${inconsistency.pools.manualRewards.computed}`);
                        log(`  Leaving - Held: ${inconsistency.pools.leaving.held}, Computed: ${inconsistency.pools.leaving.computed}`);
                        log(`---`);
                    }
                }

                expect(inconsistencies.length, 
                    `Found ${inconsistencies.length} inconsistencies between held amounts and computed stakes`
                ).to.equal(0);
            },
        });

        it({
            id: "C02", 
            title: "Total candidate stake matches sum of all pool stakes",
            timeout: 300000,
            test: async () => {
                // Get all candidates that have stakes
                const candidateSummaries = await api.query.pooledStaking.candidateSummaries.entries();
                
                log(`Checking stake consistency for ${candidateSummaries.length} candidates...`);

                const inconsistencies: Array<{
                    candidate: string;
                    totalStake: bigint;
                    poolStakes: {
                        joining: bigint;
                        autoCompounding: bigint;
                        manualRewards: bigint;
                        leaving: bigint;
                        sum: bigint;
                    };
                }> = [];

                for (const [key, summary] of candidateSummaries) {
                    const candidate = key.args[0];
                    const candidateHex = candidate.toHex();

                    // Get total stake for candidate
                    const totalStake = (await api.query.pooledStaking.pools(candidateHex, {
                        CandidateTotalStake: {}
                    })).toBigInt();

                    // Get individual pool stakes
                    const joiningStake = (await api.query.pooledStaking.pools(candidateHex, {
                        JoiningSharesTotalStaked: {}
                    })).toBigInt();

                    const autoCompoundingStake = (await api.query.pooledStaking.pools(candidateHex, {
                        AutoCompoundingSharesTotalStaked: {}
                    })).toBigInt();

                    const manualRewardsStake = (await api.query.pooledStaking.pools(candidateHex, {
                        ManualRewardsSharesTotalStaked: {}
                    })).toBigInt();

                    const leavingStake = (await api.query.pooledStaking.pools(candidateHex, {
                        LeavingSharesTotalStaked: {}
                    })).toBigInt();

                    const poolStakeSum = joiningStake + autoCompoundingStake + manualRewardsStake + leavingStake;

                    // Check if total matches sum of pools (leaving pool not included in candidate total)
                    const expectedTotal = joiningStake + autoCompoundingStake + manualRewardsStake;

                    if (totalStake !== expectedTotal) {
                        inconsistencies.push({
                            candidate: candidateHex,
                            totalStake,
                            poolStakes: {
                                joining: joiningStake,
                                autoCompounding: autoCompoundingStake,
                                manualRewards: manualRewardsStake,
                                leaving: leavingStake,
                                sum: poolStakeSum,
                            }
                        });
                    }
                }

                // Report any inconsistencies found
                if (inconsistencies.length > 0) {
                    log(`Found ${inconsistencies.length} candidate stake inconsistencies:`);
                    for (const inconsistency of inconsistencies) {
                        log(`Candidate: ${inconsistency.candidate}`);
                        log(`Total stake: ${inconsistency.totalStake}`);
                        log(`Expected (J+A+M): ${inconsistency.poolStakes.joining + inconsistency.poolStakes.autoCompounding + inconsistency.poolStakes.manualRewards}`);
                        log(`Pool breakdown:`);
                        log(`  Joining: ${inconsistency.poolStakes.joining}`);
                        log(`  AutoCompounding: ${inconsistency.poolStakes.autoCompounding}`);
                        log(`  ManualRewards: ${inconsistency.poolStakes.manualRewards}`);
                        log(`  Leaving: ${inconsistency.poolStakes.leaving}`);
                        log(`---`);
                    }
                }

                expect(inconsistencies.length,
                    `Found ${inconsistencies.length} inconsistencies in candidate total stakes`
                ).to.equal(0);
            },
        });
    },
});

/**
 * Calculate the computed stake for a delegator in a specific pool
 */
async function calculateComputedStake(
    api: ApiPromise, 
    candidate: string, 
    delegator: string, 
    poolType: "Joining" | "AutoCompounding" | "ManualRewards" | "Leaving"
): Promise<bigint> {
    // Get delegator's shares in this pool
    const shares = (await api.query.pooledStaking.pools(candidate, {
        [`${poolType}Shares`]: { delegator }
    })).toBigInt();

    if (shares === 0n) {
        return 0n;
    }

    // Get total shares and total staked for this pool
    const totalShares = (await api.query.pooledStaking.pools(candidate, {
        [`${poolType}SharesSupply`]: {}
    })).toBigInt();

    const totalStaked = (await api.query.pooledStaking.pools(candidate, {
        [`${poolType}SharesTotalStaked`]: {}
    })).toBigInt();

    if (totalShares === 0n) {
        return 0n;
    }

    // Calculate computed stake: (shares * totalStaked) / totalShares
    return (shares * totalStaked) / totalShares;
}