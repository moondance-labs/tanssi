import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { mockAndInsertHeadData, DANCELIGHT_BOND, fetchIssuance, filterRewardFromContainer, jumpToSession } from "utils";
//5EYCAe5cHUC3LZehbwavqEb95LcNnpBzfQTsAxeUibSo1Gtb

describeSuite({
    id: "DEVT0701",
    title: "Dancelight: InflationRewards test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "E01",
            title: "Parachain bond receives 30% of the inflation and pending rewards plus division dust",
            test: async () => {
                await context.createBlock();
                let expectedAmountParachainBond = 0n;

                const pendingChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.length);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const dancelightBondBalanceBefore = (
                    await polkadotJs.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

                await context.createBlock();

                const currentChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();

                let dust = 0n;
                if (currentChainRewards.isSome) {
                    const currentRewardPerChain = currentChainRewards.unwrap().rewardsPerChain.toBigInt();
                    dust = (issuance * 7n) / 10n - 2n * currentRewardPerChain;
                }
                const dancelightBondBalanceAfter = (
                    await polkadotJs.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

                expectedAmountParachainBond += (issuance * 3n) / 10n + dust + 1n;
                await context.createBlock();

                expect(dancelightBondBalanceAfter - dancelightBondBalanceBefore).to.equal(expectedAmountParachainBond);
            },
        });

        it({
            id: "E02",
            title: "Collator receives the reward from container-chain block proposal",
            test: async () => {
                // Jump 2 sessions to have collators assigned to containers.
                await jumpToSession(context, 2);
                const assignment = (await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();

                // The first account of container 2000 will be rewarded.
                const accountToReward: string = assignment.containerChains[2000][0];

                const accountBalanceBefore = (
                    await polkadotJs.query.system.account(accountToReward)
                ).data.free.toBigInt();

                await mockAndInsertHeadData(context, 2000, 2, 2, alice);
                await context.createBlock();

                const currentChainRewards = (await polkadotJs.query.inflationRewards.chainsToReward()).unwrap();
                const events = await polkadotJs.query.system.events();
                const receivedRewards = filterRewardFromContainer(events, accountToReward, 2000);

                const accountBalanceAfter = (
                    await polkadotJs.query.system.account(accountToReward)
                ).data.free.toBigInt();

                expect(accountBalanceAfter - accountBalanceBefore).to.equal(
                    currentChainRewards.rewardsPerChain.toBigInt()
                );
                expect(accountBalanceAfter - accountBalanceBefore).to.equal(receivedRewards);
            },
        });
    },
});
