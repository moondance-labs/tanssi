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
            timeout: 120000,
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

                    const autoCompoundingSharesTotalStaked = (
                        await api.query.pooledStaking.pools(candidate, {
                            AutoCompoundingSharesTotalStaked: {},
                        })
                    ).toBigInt();

                    const autoCompoundingSharesSupply = (
                        await api.query.pooledStaking.pools(candidate, {
                            AutoCompoundingSharesSupply: {},
                        })
                    ).toBigInt();

                    const autoCompoundingSharesOfCandidate = (
                        await api.query.pooledStaking.pools(candidate, {
                            AutoCompoundingShares: {
                                delegator: candidate,
                            },
                        })
                    ).toBigInt();

                    // auto stake is calculated using this method as the AutoCompoundingSharesHeldStake is not updated with rewards received
                    // by the candidate, rather the value of each share of candidate increases.
                    const auto =
                        autoCompoundingSharesSupply == 0n
                            ? 0n
                            : (autoCompoundingSharesOfCandidate * autoCompoundingSharesTotalStaked) /
                              autoCompoundingSharesSupply;

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
