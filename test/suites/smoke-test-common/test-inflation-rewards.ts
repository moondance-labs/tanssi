import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

import { ApiDecoration } from "@polkadot/api/types";
import { getAuthorFromDigest } from "util/author";
import { fetchIssuance, filterRewardFromOrchestratorWithFailure, fetchRewardAuthorContainers } from "util/block";
import { PARACHAIN_BOND } from "util/constants";

describeSuite({
    id: "S08",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let apiAt: ApiDecoration<"promise">;
        let api: ApiPromise;

        let runtimeVersion;

        beforeAll(async () => {
            api = context.polkadotJs();
            const latestBlock = await api.rpc.chain.getBlock();
            const latestBlockHash = latestBlock.block.hash;

            // ApiAt to evaluate rewards, otherwise orchestrator reward might not be correct
            apiAt = await api.at(latestBlockHash);

            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Inflation for orchestrator should match with expected number of containers",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }

                const author = await getAuthorFromDigest(apiAt);
                const events = await apiAt.query.system.events();

                // Fetch current session
                const currentSession = await apiAt.query.session.currentIndex();
                const keys = await apiAt.query.authorityMapping.authorityIdMapping(currentSession);
                const account = keys.toJSON()[author];
                // 70% is distributed across all rewards
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const numberOfChains = await apiAt.query.registrar.registeredParaIds();
                const expectedOrchestratorReward = chainRewards / BigInt(numberOfChains.length + 1);
                const reward = await filterRewardFromOrchestratorWithFailure(events, account);
                // we know there might be rounding errors, so we always check it is in the range +-1
                expect(
                    reward >= expectedOrchestratorReward - 1n && reward <= expectedOrchestratorReward + 1n,
                    `orchestrator rewards not in the range, Actual: ${reward}, Expected:  ${expectedOrchestratorReward}`
                ).to.be.true;
            },
        });

        it({
            id: "C02",
            title: "Inflation for containers should match with expected number of containers",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }
                // 70% is distributed across all rewards
                const events = await apiAt.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const numberOfChains = await apiAt.query.registrar.registeredParaIds();
                const expectedChainReward = chainRewards / BigInt(numberOfChains.length + 1);
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
            id: "C03",
            title: "Issuance is correct",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const issuance = await fetchIssuance(events).amount.toBigInt();

                // expected issuance block increment in prod
                const expectedIssuanceIncrement =
                    runtimeVersion > 500 ? (supplyBefore * 9n) / 1_000_000_000n : (supplyBefore * 19n) / 1_000_000_000n;

                // we know there might be rounding errors, so we always check it is in the range +-1
                expect(
                    issuance >= expectedIssuanceIncrement - 1n && issuance <= expectedIssuanceIncrement + 1n,
                    `Issuance not in the range, Actual: ${issuance}, Expected:  ${expectedIssuanceIncrement}`
                ).to.be.true;
            },
        });

        it({
            id: "C04",
            title: "Parachain bond receives dust plus 30% plus non-distributed rewards",
            test: async function () {
                if (runtimeVersion < 300) {
                    return;
                }
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                let expectedAmountParachainBond = 0n;

                // Pending chains to reward should be read with previous api
                const pendingChainRewards = await apiAtIssuanceBefore.query.inflationRewards.chainsToReward();
                const numberOfChains = BigInt(
                    (await apiAtIssuanceBefore.query.registrar.registeredParaIds()).length + 1
                );

                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.length);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const parachainBondBalanceBefore = (
                    await apiAtIssuanceBefore.query.system.account(PARACHAIN_BOND)
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
                    await apiAtIssuanceAfter.query.system.account(PARACHAIN_BOND)
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
