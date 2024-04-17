import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import {
    fetchIssuance,
    fetchRewardAuthorOrchestrator,
    filterRewardStakingCollator,
    filterRewardStakingDelegators,
    jumpSessions,
} from "util/block";
import { DANCE } from "util/constants";
import { createBlockAndRemoveInvulnerables } from "util/invulnerables";

describeSuite({
    id: "DT0303",
    title: "Staking candidate reward test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;

            // We need to remove all the invulnerables and add to staking
            // Remove all invulnerables, otherwise they have priority
            await createBlockAndRemoveInvulnerables(context, alice);

            const invulnerables = await polkadotJs.query.invulnerables.invulnerables();
            expect(invulnerables.length).to.be.equal(0);

            // We will make each of them self-delegate the min amount, while
            // we will make each of them delegate the other with 50%
            // Alice autocompounding, Bob will be manual
            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();
            let bobNonce = (await polkadotJs.rpc.system.accountNextIndex(bob.address)).toNumber();

            await context.createBlock([
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
            ]);
            // At least 2 sessions for the change to have effect
            await jumpSessions(context, 2);
        });
        it({
            id: "E01",
            title: "Alice should receive rewards through staking now",
            test: async function () {
                // 70% is distributed across all rewards
                // But we have 2 container chains, so it should get 1/3 of this
                // Since it is an invulnerable, it receives all payment
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const rounding = chainRewards % 3n > 0 ? 1n : 0n;
                const expectedOrchestratorReward = chainRewards - (chainRewards * 2n) / 3n - rounding;
                const reward = await fetchRewardAuthorOrchestrator(events);
                const stakingRewardedCollator = await filterRewardStakingCollator(events, reward.accountId.toString());
                const stakingRewardedDelegators = await filterRewardStakingDelegators(
                    events,
                    reward.accountId.toString()
                );

                // How much should the author have gotten?
                // For now everything as we did not execute the pending operations
                // How much should the author have gotten?
                // For now everything as we did not execute the pending operations
                expect(reward.balance.toBigInt()).toBeGreaterThanOrEqual(expectedOrchestratorReward - 1n);
                expect(reward.balance.toBigInt()).toBeLessThanOrEqual(expectedOrchestratorReward + 1n);
                expect(stakingRewardedCollator.manualRewards).to.equal(reward.balance.toBigInt());
                expect(stakingRewardedCollator.autoCompoundingRewards).to.equal(0n);
                expect(stakingRewardedDelegators.manualRewards).to.equal(0n);
                expect(stakingRewardedDelegators.autoCompoundingRewards).to.equal(0n);
            },
        });

        it({
            id: "E02",
            title: "Alice should receive shared rewards with delegators through staking now",
            test: async function () {
                // All pending operations where in session 0
                await context.createBlock([
                    await polkadotJs.tx.pooledStaking
                        .executePendingOperations([
                            {
                                delegator: alice.address,
                                operation: {
                                    JoiningAutoCompounding: {
                                        candidate: alice.address,
                                        at: 0,
                                    },
                                },
                            },
                            {
                                delegator: bob.address,
                                operation: {
                                    JoiningManualRewards: {
                                        candidate: alice.address,
                                        at: 0,
                                    },
                                },
                            },
                        ])
                        .signAsync(context.keyring.alice),
                ]);

                const totalBacked = (
                    await polkadotJs.query.pooledStaking.pools(alice.address, "CandidateTotalStake")
                ).toBigInt();
                const totalManual = (
                    await polkadotJs.query.pooledStaking.pools(alice.address, "ManualRewardsSharesTotalStaked")
                ).toBigInt();
                const totalManualShareSupply = (
                    await polkadotJs.query.pooledStaking.pools(alice.address, "ManualRewardsSharesSupply")
                ).toBigInt();

                // We create one more block
                await context.createBlock();
                const events = await polkadotJs.query.system.events();
                const reward = await fetchRewardAuthorOrchestrator(events);

                // 20% collator percentage
                const collatorPercentage = reward.balance.toBigInt() - (80n * reward.balance.toBigInt()) / 100n;

                // Rounding
                const delegatorRewards = reward.balance.toBigInt() - collatorPercentage;

                // First, manual rewards
                const delegatorManualRewards = (totalManual * delegatorRewards) / totalBacked;
                // Check its
                const delegatorManualRewardsPerShare = delegatorManualRewards / totalManualShareSupply;
                const realDistributedManualDelegatorRewards = delegatorManualRewardsPerShare * totalManualShareSupply;

                // Second, autocompounding
                const delegatorsAutoCompoundRewards = delegatorRewards - realDistributedManualDelegatorRewards;

                const stakingRewardedCollator = await filterRewardStakingCollator(events, reward.accountId.toString());
                const stakingRewardedDelegators = await filterRewardStakingDelegators(
                    events,
                    reward.accountId.toString()
                );

                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(stakingRewardedDelegators.manualRewards).toBeGreaterThanOrEqual(
                    realDistributedManualDelegatorRewards - 1n
                );
                expect(stakingRewardedDelegators.manualRewards).toBeLessThanOrEqual(
                    realDistributedManualDelegatorRewards + 1n
                );
                expect(stakingRewardedDelegators.autoCompoundingRewards).toBeGreaterThanOrEqual(
                    delegatorsAutoCompoundRewards - 1n
                );
                expect(stakingRewardedDelegators.autoCompoundingRewards).toBeLessThanOrEqual(
                    delegatorsAutoCompoundRewards + 1n
                );

                // TODO: test better what goes into auto and what goes into manual for collator
                const delegatorDust =
                    delegatorRewards - realDistributedManualDelegatorRewards - delegatorsAutoCompoundRewards;
                expect(
                    stakingRewardedCollator.manualRewards + stakingRewardedCollator.autoCompoundingRewards
                ).toBeGreaterThanOrEqual(collatorPercentage + delegatorDust - 1n);
                expect(
                    stakingRewardedCollator.manualRewards + stakingRewardedCollator.autoCompoundingRewards
                ).toBeLessThanOrEqual(collatorPercentage + delegatorDust + 1n);
            },
        });
    },
});
