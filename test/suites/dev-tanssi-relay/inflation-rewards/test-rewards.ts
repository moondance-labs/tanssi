// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import {
    mockAndInsertHeadData,
    DANCELIGHT_BOND,
    fetchIssuance,
    filterRewardFromContainer,
    jumpToSession,
    jumpSessions,
    perbillMul,
} from "utils";
//5EYCAe5cHUC3LZehbwavqEb95LcNnpBzfQTsAxeUibSo1Gtb

describeSuite({
    id: "DEVT0701",
    title: "InflationRewards test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;
        let isStarlight: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            // Add keys to pallet session. In dancebox they are already there in genesis.
            // We need 4 collators because we have 2 chains with 2 collators per chain.
            const newKey1 = await polkadotJs.rpc.author.rotateKeys();
            const newKey2 = await polkadotJs.rpc.author.rotateKeys();
            const newKey3 = await polkadotJs.rpc.author.rotateKeys();
            const newKey4 = await polkadotJs.rpc.author.rotateKeys();

            await context.createBlock([
                await polkadotJs.tx.session.setKeys(newKey1, []).signAsync(alice),
                await polkadotJs.tx.session.setKeys(newKey2, []).signAsync(bob),
                await polkadotJs.tx.session.setKeys(newKey3, []).signAsync(charlie),
                await polkadotJs.tx.session.setKeys(newKey4, []).signAsync(dave),
            ]);

            // At least 2 sessions for the change to have effect
            await jumpSessions(context, 2);
            // +2 because in tanssi-relay sessions start 1 block later
            await context.createBlock();
            await context.createBlock();
        });

        it({
            id: "E01",
            title: "Parachain bond receives pending rewards plus non-reward part of new inflation",
            test: async () => {
                await context.createBlock();

                const assignment = (await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();
                // Assert 2 collators in each chain
                expect(Object.values(assignment.containerChains).map((x) => x.length)).to.deep.equal([2, 2]);

                let expectedAmountParachainBond = 0n;

                const pendingChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.size);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const dancelightBondBalanceBefore = (
                    await polkadotJs.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

                await context.createBlock();

                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                // collator reward is different in dancelight and in starlight
                // dancelight: 70%
                // starlight: 4/7 of 100%
                let chainRewards: bigint;
                if (isStarlight) {
                    const BILLION = 1_000_000_000n;
                    const perBill = (4n * BILLION) / 7n;
                    chainRewards = perbillMul(issuance, perBill);
                } else {
                    // dancelight
                    const BILLION = 1_000_000_000n;
                    const perBill = (7n * BILLION) / 10n;
                    chainRewards = perbillMul(issuance, perBill);
                }
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % 2n);
                const currentChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                // Sanity check: calculated chainRewards matches on chain
                const currentRewardPerChain = currentChainRewards.unwrap().rewardsPerChain.toBigInt();
                const realRewardsMulChains = currentRewardPerChain * 2n;
                expect(realRewardsMulChains).to.equal(chainRewards);

                expectedAmountParachainBond += issuance - chainRewards;

                const dancelightBondBalanceAfter = (
                    await polkadotJs.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

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
