import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import {
    fetchIssuance,
    fetchRewardAuthorOrchestrator,
    filterRewardStakingDistributed,
    jumpSessions,
    perbillMul,
} from "utils";
import { DANCE } from "utils";
import { createBlockAndRemoveInvulnerables } from "utils";

describeSuite({
    id: "DEV0803",
    title: "Staking candidate reward test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let randomAccount: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            randomAccount = generateKeyringPair("sr25519");

            const value = 400_000n * DANCE;
            await context.createBlock([
                await polkadotJs.tx.balances.transferAllowDeath(randomAccount.address, value).signAsync(alice),
            ]);
            // Add an additional collator because we need 5 in total
            const newKey1 = await polkadotJs.rpc.author.rotateKeys();
            await context.createBlock([await polkadotJs.tx.session.setKeys(newKey1, "0x").signAsync(randomAccount)]);

            // We need to remove all the invulnerables and add to staking
            // Remove all invulnerables, otherwise they have priority
            await createBlockAndRemoveInvulnerables(context, alice);

            const invulnerables = await polkadotJs.query.invulnerables.invulnerables();
            expect(invulnerables.length).to.be.equal(0);

            // We will make each of them self-delegate the min amount, while
            // we will make each of them delegate the other with 50%
            // Alice autocompounding, Bob will be manual
            const collators = [alice, bob, charlie, dave, randomAccount];
            // Pre-fetch nonces for all collators/delegators we’ll use
            const nonces: Record<string, number> = {};
            for (const acc of collators) {
                nonces[acc.address] = (await polkadotJs.rpc.system.accountNextIndex(acc.address)).toNumber();
            }

            // Delegation plan: all collators self-delegate using auto compounding
            // And alice is a manual delegator of bob
            const delegationPlan = [
                { candidate: alice, autoDelegator: alice },
                { candidate: bob, autoDelegator: alice },
                { candidate: alice, manualDelegator: bob },
                { candidate: bob, manualDelegator: bob },
                { candidate: charlie, autoDelegator: charlie, manualDelegator: charlie },
                { candidate: dave, autoDelegator: dave, manualDelegator: dave },
                { candidate: randomAccount, autoDelegator: randomAccount, manualDelegator: randomAccount },
            ];

            const stakingTxs = [];
            for (const { candidate, autoDelegator, manualDelegator } of delegationPlan) {
                // Auto-compounding delegation
                if (autoDelegator) {
                    stakingTxs.push(
                        await polkadotJs.tx.pooledStaking
                            .requestDelegate(candidate.address, "AutoCompounding", 180000n * DANCE)
                            .signAsync(autoDelegator, { nonce: nonces[autoDelegator.address]++ })
                    );
                }

                // Manual rewards delegation
                if (manualDelegator) {
                    stakingTxs.push(
                        await polkadotJs.tx.pooledStaking
                            .requestDelegate(candidate.address, "ManualRewards", 20000n * DANCE)
                            .signAsync(manualDelegator, { nonce: nonces[manualDelegator.address]++ })
                    );
                }
            }

            await context.createBlock(stakingTxs, { allowFailures: false });
            // At least 2 sessions for the change to have effect
            await jumpSessions(context, 2);
        });
        it({
            id: "E01",
            title: "Alice should receive rewards through staking now",
            test: async () => {
                const assignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();
                // Assert 2 collators in each chain
                expect(Object.values(assignment.containerChains).map((x) => x.length)).to.deep.equal([2, 2]);
                // 70% is distributed across all rewards
                // But we have 2 container chains, so it should get 1/3 of this
                // Since it is an invulnerable, it receives all payment
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const BILLION = 1_000_000_000n;
                const perBill = (7n * BILLION) / 10n;
                let chainRewards = perbillMul(issuance, perBill);
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % 3n);
                const expectedOrchestratorReward = chainRewards / 3n;
                const reward = await fetchRewardAuthorOrchestrator(events);
                const stakingRewarded = await filterRewardStakingDistributed(events, reward.accountId.toString());

                // How much should the author have gotten?
                // For now everything as we did not execute the pending operations
                expect(reward.balance.toBigInt()).toBeGreaterThanOrEqual(expectedOrchestratorReward - 1n);
                expect(reward.balance.toBigInt()).toBeLessThanOrEqual(expectedOrchestratorReward + 1n);
                expect(stakingRewarded.collatorMcRewards).to.equal(reward.balance.toBigInt());
                expect(stakingRewarded.collatorAcRewards).to.equal(0n);
                expect(stakingRewarded.delegatorsMcRewards).to.equal(0n);
                expect(stakingRewarded.delegatorsAcRewards).to.equal(0n);
            },
        });

        it({
            id: "E02",
            title: "Alice should receive shared rewards with delegators through staking now",
            test: async () => {
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

                await jumpSessions(context, 1);

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

                const stakingRewarded = await filterRewardStakingDistributed(events, reward.accountId.toString());

                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(stakingRewarded.delegatorsMcRewards).toBeGreaterThanOrEqual(
                    realDistributedManualDelegatorRewards - 1n
                );
                expect(stakingRewarded.delegatorsMcRewards).toBeLessThanOrEqual(
                    realDistributedManualDelegatorRewards + 1n
                );
                expect(stakingRewarded.delegatorsAcRewards).toBeGreaterThanOrEqual(delegatorsAutoCompoundRewards - 1n);
                expect(stakingRewarded.delegatorsAcRewards).toBeLessThanOrEqual(delegatorsAutoCompoundRewards + 1n);

                // TODO: test better what goes into auto and what goes into manual for collator
                const delegatorDust =
                    delegatorRewards - realDistributedManualDelegatorRewards - delegatorsAutoCompoundRewards;
                expect(stakingRewarded.collatorMcRewards + stakingRewarded.collatorAcRewards).toBeGreaterThanOrEqual(
                    collatorPercentage + delegatorDust - 1n
                );
                expect(stakingRewarded.collatorMcRewards + stakingRewarded.collatorAcRewards).toBeLessThanOrEqual(
                    collatorPercentage + delegatorDust + 1n
                );
            },
        });
    },
});
