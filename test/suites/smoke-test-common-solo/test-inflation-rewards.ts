import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import {
    DANCELIGHT_BLOCK_INFLATION_PERBILL,
    DANCELIGHT_BOND,
    DANCELIGHT_REWARDS_PORTION_PER_BILL_RATIO,
    fetchIssuance,
    fetchRewardAuthorContainers,
    PER_BILL_RATIO,
    perbillMul,
    STARLIGHT_BLOCK_INFLATION_PERBILL,
    STARLIGHT_REWARDS_PORTION_PER_BILL_RATIO,
} from "utils";
import { isStarlightRuntime } from "../../utils/runtime.ts";

describeSuite({
    id: "SMOK06",
    title: "Inflation and Reward Distribution Mechanisms",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let apiAt: ApiDecoration<"promise">;
        let api: ApiPromise;
        let isStarlight: boolean;
        let numberOfChains: number;
        beforeAll(async () => {
            api = context.polkadotJs();
            const latestBlock = await api.rpc.chain.getBlock();
            const latestBlockHash = latestBlock.block.hash;
            isStarlight = isStarlightRuntime(api);

            // ApiAt to evaluate rewards
            apiAt = await api.at(latestBlockHash);

            // If the number of chains with collators is 0, there is no point on running this
            numberOfChains = Object.entries(
                (await apiAt.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON().containerChains
            ).length;
        });

        it({
            id: "C01",
            title: "Inflation for containers should match with expected number of containers",
            test: async ({ skip }) => {
                if (numberOfChains === 0) {
                    skip();
                }
                const tolerancePerBill = 1n; // = 0.0000001%
                const events = await apiAt.query.system.events();
                const issuance = fetchIssuance(events).amount.toBigInt();
                let chainRewards: bigint;
                if (isStarlight) {
                    const BILLION = 1_000_000_000n;
                    const perBill =
                        (STARLIGHT_REWARDS_PORTION_PER_BILL_RATIO[0] * BILLION) /
                        STARLIGHT_REWARDS_PORTION_PER_BILL_RATIO[1];
                    chainRewards = perbillMul(issuance, perBill);
                } else {
                    // dancelight
                    const BILLION = 1_000_000_000n;
                    const perBill =
                        (DANCELIGHT_REWARDS_PORTION_PER_BILL_RATIO[0] * BILLION) /
                        DANCELIGHT_REWARDS_PORTION_PER_BILL_RATIO[1];
                    chainRewards = perbillMul(issuance, perBill);
                }
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % BigInt(numberOfChains));
                const expectedChainReward = chainRewards / BigInt(numberOfChains);
                const rewardEvents = fetchRewardAuthorContainers(events);
                const toleranceDiff = (expectedChainReward * tolerancePerBill) / PER_BILL_RATIO;
                const failures = rewardEvents.filter(
                    ({ balance }) =>
                        !(
                            balance.toBigInt() >= expectedChainReward - toleranceDiff &&
                            balance.toBigInt() <= expectedChainReward + toleranceDiff
                        )
                );

                for (const { accountId, balance } of failures) {
                    log(
                        `${accountId.toHuman()} reward ${balance.toBigInt()} , not in the range of ${expectedChainReward}. Diff: ${expectedChainReward - balance.toBigInt()}. Tolerance value: ${toleranceDiff}`
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

                const blockInflation = isStarlightRuntime(api)
                    ? STARLIGHT_BLOCK_INFLATION_PERBILL
                    : DANCELIGHT_BLOCK_INFLATION_PERBILL;

                // expected issuance block increment in prod
                const expectedIssuanceIncrement = (supplyBefore * blockInflation) / 1_000_000_000n;

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
            test: async ({ skip }) => {
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                let expectedAmountParachainBond = 0n;

                // Pending chains to reward should be read with previous api
                const pendingChainRewards = await apiAtIssuanceBefore.query.inflationRewards.chainsToReward();
                const numberOfChains = BigInt(
                    Object.entries(
                        (await apiAtIssuanceBefore.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON()
                            .containerChains
                    ).length
                );
                // Issuance is 0 when number of chain is 0
                if (numberOfChains === 0n) {
                    skip();
                }

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
                    await apiAtIssuanceBefore.query.system.account(DANCELIGHT_BOND)
                ).data.free.toBigInt();

                const currentChainRewards = await apiAtIssuanceAfter.query.inflationRewards.chainsToReward();
                const events = await apiAtIssuanceAfter.query.system.events();
                const issuance = fetchIssuance(events).amount.toBigInt();
                // Dust from computations also goes to parachainBond
                let chainRewards: bigint;
                if (isStarlight) {
                    const BILLION = 1_000_000_000n;
                    const perBill =
                        (STARLIGHT_REWARDS_PORTION_PER_BILL_RATIO[0] * BILLION) /
                        STARLIGHT_REWARDS_PORTION_PER_BILL_RATIO[1];
                    chainRewards = perbillMul(issuance, perBill);
                } else {
                    // dancelight
                    const BILLION = 1_000_000_000n;
                    const perBill =
                        (DANCELIGHT_REWARDS_PORTION_PER_BILL_RATIO[0] * BILLION) /
                        DANCELIGHT_REWARDS_PORTION_PER_BILL_RATIO[1];
                    chainRewards = perbillMul(issuance, perBill);
                }
                // Chain rewards must be a multiple of number of chains.
                chainRewards = chainRewards - (chainRewards % BigInt(numberOfChains));
                const parachainBondBalanceAfter = (
                    await apiAtIssuanceAfter.query.system.account(DANCELIGHT_BOND)
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
