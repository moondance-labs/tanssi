// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import {
    fetchIssuance,
    fetchRewardAuthorContainers,
    filterRewardFromOrchestratorWithFailure,
    getAuthorFromDigest,
    getBlockNumberForDebug,
    PARACHAIN_BOND,
    PER_BILL_RATIO,
    perbillMul,
} from "utils";

const BLOCK_NUMBER_TO_DEBUG = getBlockNumberForDebug();

describeSuite({
    id: "SM04",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let apiAt: ApiDecoration<"promise">;
        let api: ApiPromise;

        let runtimeVersion: number;

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
            test: async () => {
                if (runtimeVersion < 300) {
                    return;
                }

                const author = await getAuthorFromDigest(apiAt);
                const events = await apiAt.query.system.events();

                // Fetch current session
                const currentSession = await apiAt.query.session.currentIndex();
                const keys = await apiAt.query.authorityMapping.authorityIdMapping(currentSession);
                const account = keys.toJSON()[author];
                // +1 for orchestrator chain
                const numberOfChains =
                    Object.entries(
                        (await apiAt.query.collatorAssignment.collatorContainerChain()).toJSON().containerChains
                    ).length + 1;
                // 70% is distributed across all rewards
                const issuance = await fetchIssuance(events).amount.toBigInt();
                let chainRewards: bigint;
                const BILLION = 1_000_000_000n;
                const perBill = (7n * BILLION) / 10n;
                chainRewards = perbillMul(issuance, perBill);
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % BigInt(numberOfChains));
                const expectedOrchestratorReward = chainRewards / BigInt(numberOfChains);
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
            test: async () => {
                if (runtimeVersion < 300) {
                    return;
                }
                // 70% is distributed across all rewards
                const events = await apiAt.query.system.events();
                // +1 for orchestrator chain
                const numberOfChains =
                    Object.entries(
                        (await apiAt.query.collatorAssignment.collatorContainerChain()).toJSON().containerChains
                    ).length + 1;
                const issuance = await fetchIssuance(events).amount.toBigInt();
                let chainRewards: bigint;
                const BILLION = 1_000_000_000n;
                const perBill = (7n * BILLION) / 10n;
                chainRewards = perbillMul(issuance, perBill);
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % BigInt(numberOfChains));
                const expectedChainReward = chainRewards / BigInt(numberOfChains);
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
            test: async () => {
                if (runtimeVersion < 300) {
                    return;
                }
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = BLOCK_NUMBER_TO_DEBUG
                    ? await api.rpc.chain.getBlockHash(BLOCK_NUMBER_TO_DEBUG)
                    : latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const issuance = await fetchIssuance(events).amount.toBigInt();

                // expected issuance block increment in prod
                const expectedIssuanceIncrement =
                    runtimeVersion > 500 ? (supplyBefore * 9n) / 1_000_000_000n : (supplyBefore * 19n) / 1_000_000_000n;

                const tolerancePerBill = 1n; // = 0.0000001%
                const toleranceDiff = (expectedIssuanceIncrement * tolerancePerBill) / PER_BILL_RATIO;

                expect(
                    issuance >= expectedIssuanceIncrement - toleranceDiff &&
                        issuance <= expectedIssuanceIncrement + toleranceDiff,
                    `Block: ${latestBlock.block.header.number.toString()} Issuance not in the range, Actual: ${issuance}, Expected:  ${expectedIssuanceIncrement}. toleranceDiff: ${toleranceDiff.toString()}`
                ).to.be.true;
            },
        });

        it({
            id: "C04",
            title: "Parachain bond receives dust plus 30% plus non-distributed rewards",
            test: async () => {
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
                    const paraIds = pendingChainRewards.unwrap().paraIds;
                    // paraIds was Vec in old runtimes, now is Set. Need to use size/length accordingly.
                    const pendingChainsToReward = BigInt(
                        paraIds.size ??
                            // @ts-expect-error transition period: old metadata exposes `.length`
                            paraIds.length ??
                            (() => {
                                throw new Error("paraIds has neither .size nor .length");
                            })()
                    );
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const parachainBondBalanceBefore = (
                    await apiAtIssuanceBefore.query.system.account(PARACHAIN_BOND)
                ).data.free.toBigInt();

                const currentChainRewards = await apiAtIssuanceAfter.query.inflationRewards.chainsToReward();
                const events = await apiAtIssuanceAfter.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();

                // Dust from computations also goes to parachainBond
                let chainRewards: bigint;
                const BILLION = 1_000_000_000n;
                const perBill = (7n * BILLION) / 10n;
                chainRewards = perbillMul(issuance, perBill);
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % BigInt(numberOfChains));
                const parachainBondBalanceAfter = (
                    await apiAtIssuanceAfter.query.system.account(PARACHAIN_BOND)
                ).data.free.toBigInt();
                expectedAmountParachainBond += issuance - chainRewards;

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
