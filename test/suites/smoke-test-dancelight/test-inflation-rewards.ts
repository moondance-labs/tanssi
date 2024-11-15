import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

import { ApiDecoration } from "@polkadot/api/types";
import { fetchIssuance, fetchRewardAuthorContainers } from "util/block";
import { DANCELIGHT_BOND } from "util/constants";

describeSuite({
    id: "S18",
    title: "Sample suite that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let apiAt: ApiDecoration<"promise">;
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
            const latestBlock = await api.rpc.chain.getBlock();
            const latestBlockHash = latestBlock.block.hash;

            // ApiAt to evaluate rewards
            apiAt = await api.at(latestBlockHash);
        });

        it({
            id: "C01",
            title: "Inflation for containers should match with expected number of containers",
            test: async function () {
                // 70% is distributed across all rewards
                const events = await apiAt.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const numberOfChains = await apiAt.query.containerRegistrar.registeredParaIds();
                const expectedChainReward = chainRewards / BigInt(numberOfChains.length);
                const rewardEvents = await fetchRewardAuthorContainers(events);
                for (const index in rewardEvents) {
                    expect(
                        rewardEvents[index].balance.toBigInt() >= expectedChainReward - 1n &&
                            rewardEvents[index].balance.toBigInt() <= expectedChainReward + 1n,
                        `rewardEvents not in the range, Index: ${index} Actual: ${rewardEvents[
                            index
                        ].balance.toBigInt()}, Expected:  ${expectedChainReward}`
                    ).to.be.true;
                }
            },
        });

        it({
            id: "C02",
            title: "Issuance is correct",
            test: async function () {
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const issuance = await fetchIssuance(events).amount.toBigInt();

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
            test: async function () {
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
                const issuance = await fetchIssuance(events).amount.toBigInt();

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
