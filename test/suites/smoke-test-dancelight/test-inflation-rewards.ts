import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import { DANCELIGHT_BOND, fetchIssuance, fetchRewardAuthorContainers } from "utils";

describeSuite({
    id: "SMOK06",
    title: "Inflation and Reward Distribution Mechanisms",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let apiAt: ApiDecoration<"promise">;
        let api: ApiPromise;
        let numberOfChains: number;
        beforeAll(async () => {
            api = context.polkadotJs();
            const latestBlock = await api.rpc.chain.getBlock();
            const latestBlockHash = latestBlock.block.hash;

            // ApiAt to evaluate rewards
            apiAt = await api.at(latestBlockHash);

            // If the number of registered chains is 0, there is no point on running this
            numberOfChains = (await apiAt.query.containerRegistrar.registeredParaIds()).length;
        });

        it({
            id: "C01",
            title: "Inflation for containers should match with expected number of containers",
            test: async ({ skip }) => {
                if (numberOfChains === 0) {
                    skip();
                }
                // 70% is distributed across all rewards
                const events = await apiAt.query.system.events();
                const issuance = fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const expectedChainReward = chainRewards / BigInt(numberOfChains);
                const rewardEvents = fetchRewardAuthorContainers(events);
                const failures = rewardEvents.filter(
                    ({ balance }) =>
                        !(
                            balance.toBigInt() >= expectedChainReward - 1n &&
                            balance.toBigInt() <= expectedChainReward + 1n
                        )
                );

                for (const { accountId, balance } of failures) {
                    log(
                        `${accountId.toHuman()} reward ${balance.toBigInt()} , not in the range of ${expectedChainReward}`
                    );
                }

                expect(failures.length).to.eq(0);
            },
        });

        it({
            id: "C02",
            title: "Issuance is correct",
            test: async ({ skip }) => {
                if (numberOfChains === 0) {
                    skip();
                }

                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const issuance = fetchIssuance(events).amount.toBigInt();

                // expected issuance block increment in prod
                const expectedIssuanceIncrement = (supplyBefore * 9n) / 1_000_000_000n;

                // we know there might be rounding errors, so we always check it is in the range +-1
                expect(
                    issuance >= expectedIssuanceIncrement - 1n && issuance <= expectedIssuanceIncrement + 1n,
                    `Issuance not in the range, Actual: ${issuance}, Expected:  ${expectedIssuanceIncrement}`
                ).to.be.true;
            },
        });

        it({
            id: "C03",
            title: "Dancelight bond receives dust plus 30% plus non-distributed rewards",
            test: async () => {
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                let expectedAmountParachainBond = 0n;

                // Pending chains to reward should be read with previous api
                const pendingChainRewards = await apiAtIssuanceBefore.query.inflationRewards.chainsToReward();
                const numberOfChains = BigInt(
                    (await apiAtIssuanceBefore.query.containerRegistrar.registeredParaIds()).length
                );

                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.length);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const parachainBondBalanceBefore = (
                    await apiAtIssuanceBefore.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

                const currentChainRewards = await apiAtIssuanceAfter.query.inflationRewards.chainsToReward();
                const events = await apiAtIssuanceAfter.query.system.events();
                const issuance = fetchIssuance(events).amount.toBigInt();

                // Dust from computations also goes to parachainBond
                let dust = 0n;
                if (currentChainRewards.isSome) {
                    const currentRewardPerChain = currentChainRewards.unwrap().rewardsPerChain.toBigInt();
                    dust = (issuance * 7n) / 10n - numberOfChains * currentRewardPerChain;
                }
                const parachainBondBalanceAfter = (
                    await apiAtIssuanceAfter.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();
                expectedAmountParachainBond += (issuance * 3n) / 10n + dust;

                // we know there might be rounding errors, so we always check it is in the range +-1
                expect(
                    parachainBondBalanceAfter - parachainBondBalanceBefore >= expectedAmountParachainBond - 1n &&
                        parachainBondBalanceAfter - parachainBondBalanceBefore <= expectedAmountParachainBond + 1n,
                    `Parachain Bond rewards not in the range, Actual: ${
                        parachainBondBalanceAfter - parachainBondBalanceBefore
                    }, Expected:  ${expectedAmountParachainBond}`
                ).to.be.true;
            },
        });
    },
});
