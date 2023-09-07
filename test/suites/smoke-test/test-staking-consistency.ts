import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { BN } from "@polkadot/util";

describeSuite({
    id: "S05",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "All eligible candidates have enough self delegation",
            test: async function () {
                let eligibleCandidates = await api.query.pooledStaking.sortedEligibleCandidates();

                let minimum = 10_000_000_000_000_000n;

                for (let c of eligibleCandidates) {
                    let candidate = c.candidate.toHex();

                    // TODO: Currently it is not possible to directly get the
                    // stake of a delegator in a given pool without having to
                    // compute it based on the amont of shares, total shares and
                    // total staked of that pool.
                    // However as no rewards are distributed yet, the stake
                    // held in that pool should always correspond to the staked
                    // amount.
                    // We should add a runtime API to easily get the staked
                    // amount and update this test, before rewards or slashing
                    // is introduced.    
                    
                    let joining = (await api.query.pooledStaking.pools(candidate, {
                        JoiningSharesHeldStake: {
                            delegator: candidate,
                        }
                    })).toBigInt();

                    let auto = (await api.query.pooledStaking.pools(candidate, {
                        AutoCompoundingSharesHeldStake: {
                            delegator: candidate,
                        }
                    })).toBigInt();

                    let manual = (await api.query.pooledStaking.pools(candidate, {
                        ManualRewardsSharesHeldStake: {
                            delegator: candidate,
                        }
                    })).toBigInt();

                    let selfDelegation = joining + auto + manual;

                    expect(
                        selfDelegation,
                        `Candidate ${candidate} have self-delegation\n\
                        ${selfDelegation} which is below the minimum of\n\
                        ${minimum}`
                    ).toBeGreaterThanOrEqual(minimum);
                }
               
            },
        });
    },
});
