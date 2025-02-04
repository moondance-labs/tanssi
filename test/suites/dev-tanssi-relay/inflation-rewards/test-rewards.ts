import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, type DevModeContext } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { Header, ParaId, HeadData, Digest, DigestItem, Slot } from "@polkadot/types/interfaces";
import type { KeyringPair } from "@moonwall/util";
import { fetchIssuance, filterRewardFromContainer, jumpToSession } from "util/block";
import { DANCELIGHT_BOND } from "util/constants";
import { stringToHex } from "@polkadot/util";
//5EYCAe5cHUC3LZehbwavqEb95LcNnpBzfQTsAxeUibSo1Gtb

// Helper function to make rewards work for a specific block and slot.
// We need to mock a proper HeadData object for AuthorNoting inherent to work, and thus
// rewards take place.
//
// Basically, if we don't call this function before testing the rewards given
// to collators in a block, the HeadData object mocked in genesis will not be decoded properly
// and the AuthorNoting inherent will fail.
async function mockAndInsertHeadData(
    context: DevModeContext,
    paraId: ParaId,
    blockNumber: number,
    slotNumber: number,
    sudoAccount: KeyringPair
) {
    const relayApi = context.polkadotJs();
    const aura_engine_id = stringToHex("aura");

    const slotNumberT: Slot = relayApi.createType("Slot", slotNumber);
    const digestItem: DigestItem = relayApi.createType("DigestItem", {
        PreRuntime: [aura_engine_id, slotNumberT.toHex(true)],
    });
    const digest: Digest = relayApi.createType("Digest", {
        logs: [digestItem],
    });
    const header: Header = relayApi.createType("Header", {
        parentHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
        number: blockNumber,
        stateRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        extrinsicsRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        digest,
    });

    const headData: HeadData = relayApi.createType("HeadData", header.toHex());
    const paraHeadKey = relayApi.query.paras.heads.key(paraId);

    await context.createBlock(
        relayApi.tx.sudo
            .sudo(relayApi.tx.system.setStorage([[paraHeadKey, `0xc101${headData.toHex().slice(2)}`]]))
            .signAsync(sudoAccount),
        { allowFailures: false }
    );
}

describeSuite({
    id: "DTR1101",
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

                expectedAmountParachainBond += (issuance * 3n) / 10n + dust;
                await context.createBlock();

                expect(dancelightBondBalanceAfter - dancelightBondBalanceBefore).to.equal(
                    expectedAmountParachainBond + 1n
                );
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
