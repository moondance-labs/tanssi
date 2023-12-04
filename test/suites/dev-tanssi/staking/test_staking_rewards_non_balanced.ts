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
import { DANCE, STAKING_ACCOUNT } from "util/constants";

describeSuite({
    id: "SR0401",
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
            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();
            let bobNonce = (await polkadotJs.rpc.system.accountNextIndex(bob.address)).toNumber();
            // We need to remove from invulnerables and add to staking
            // for that we need to remove Alice and Bob from invulnerables first
            // We will make each of them self-delegate the min amount, while
            // we will make each of them delegate the other with 50%
            // Alice autocompounding, Bob will be manual

            // Additionally, we need to pass to the staking account the minimum balance
            const existentialDeposit = polkadotJs.consts.balances.existentialDeposit;

            await context.createBlock([
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.invulnerables.removeInvulnerable(alice.address))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.invulnerables.removeInvulnerable(bob.address))
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
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
                await polkadotJs.tx.balances
                    .transferAllowDeath(STAKING_ACCOUNT, existentialDeposit)
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
                const expectedOrchestratorReward = chainRewards / 3n;
                const reward = await fetchRewardAuthorOrchestrator(events);
                const stakingRewardedCollator = await filterRewardStakingCollator(events, reward.accountId.toString());
                const stakingRewardedDelegators = await filterRewardStakingDelegators(
                    events,
                    reward.accountId.toString()
                );

                // How much should the author have gotten?
                // For now everything as we did not execute the pending operations
                expect(reward.balance.toBigInt()).to.equal(expectedOrchestratorReward);
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
                const collatorPercentage = (20n * reward.balance.toBigInt()) / 100n;

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
                expect(stakingRewardedDelegators.manualRewards).to.equal(realDistributedManualDelegatorRewards);
                expect(stakingRewardedDelegators.autoCompoundingRewards).to.equal(delegatorsAutoCompoundRewards);

                // TODO: test better what goes into auto and what goes into manual for collator
                const delegatorDust =
                    delegatorRewards - realDistributedManualDelegatorRewards - delegatorsAutoCompoundRewards;
                expect(stakingRewardedCollator.manualRewards + stakingRewardedCollator.autoCompoundingRewards).to.equal(
                    collatorPercentage + delegatorDust
                );
            },
        });
    },
});
