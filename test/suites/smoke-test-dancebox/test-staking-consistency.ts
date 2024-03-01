import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S05",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "All eligible candidates have enough self delegation",
            test: async function () {
                if (runtimeVersion < 200) {
                    return;
                }

                const eligibleCandidates = await api.query.pooledStaking.sortedEligibleCandidates();

                const minimum = 10_000_000_000_000_000n;

                for (const c of eligibleCandidates) {
                    const candidate = c.candidate.toHex();

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

                    const joining = (
                        await api.query.pooledStaking.pools(candidate, {
                            JoiningSharesHeldStake: {
                                delegator: candidate,
                            },
                        })
                    ).toBigInt();

                    const auto = (
                        await api.query.pooledStaking.pools(candidate, {
                            AutoCompoundingSharesHeldStake: {
                                delegator: candidate,
                            },
                        })
                    ).toBigInt();

                    const manual = (
                        await api.query.pooledStaking.pools(candidate, {
                            ManualRewardsSharesHeldStake: {
                                delegator: candidate,
                            },
                        })
                    ).toBigInt();

                    const selfDelegation = joining + auto + manual;

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
