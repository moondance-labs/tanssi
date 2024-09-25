import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, DevModeContext } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { Header, ParaId, HeadData, Digest, DigestItem } from "@polkadot/types/interfaces";
import { KeyringPair } from "@moonwall/util";
import { fetchIssuance, filterRewardFromContainer, jumpToSession } from "util/block";
import { STARLIGHT_BOND } from "util/constants";
import { numberToHex, stringToHex } from "@polkadot/util";
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

    const digestItem: DigestItem = await relayApi.createType("DigestItem", {
        PreRuntime: [aura_engine_id, numberToHex(slotNumber, 64)],
    });
    const digest: Digest = await relayApi.createType("Digest", {
        logs: [digestItem],
    });
    const header: Header = await relayApi.createType("Header", {
        parentHash: "0x0000000000000000000000000000000000000000000000000000000000000000",
        number: blockNumber,
        stateRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        extrinsicsRoot: "0x0000000000000000000000000000000000000000000000000000000000000000",
        digest,
    });

    const headData: HeadData = await relayApi.createType("HeadData", header.toHex());
    const paraHeadKey = await relayApi.query.paras.heads.key(paraId);

    await context.createBlock(
        relayApi.tx.sudo
            .sudo(relayApi.tx.system.setStorage([[paraHeadKey, `0xc101${headData.toHex().slice(2)}`]]))
            .signAsync(sudoAccount),
        { allowFailures: false }
    );
}

describeSuite({
    id: "DTR1101",
    title: "Starlight: InflationRewards test suite",
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
            test: async function () {
                await context.createBlock();
                let expectedAmountParachainBond = 0n;

                const pendingChainRewards = await polkadotJs.query.inflationRewards.chainsToReward();
                if (pendingChainRewards.isSome) {
                    const rewardPerChain = pendingChainRewards.unwrap().rewardsPerChain.toBigInt();
                    const pendingChainsToReward = BigInt(pendingChainRewards.unwrap().paraIds.length);
                    expectedAmountParachainBond += pendingChainsToReward * rewardPerChain;
                }

                const starlightBondBalanceBefore = (
                    await polkadotJs.query.system.account(STARLIGHT_BOND)
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
                const starlightBondBalanceAfter = (
                    await polkadotJs.query.system.account(STARLIGHT_BOND)
                ).data.free.toBigInt();

                expectedAmountParachainBond += (issuance * 3n) / 10n + dust;
                await context.createBlock();

                expect(starlightBondBalanceAfter - starlightBondBalanceBefore).to.equal(
                    expectedAmountParachainBond + 1n
                );
            },
        });

        it({
            id: "E02",
            title: "Collator receives the reward from container-chain block proposal",
            test: async function () {
                // Jump 2 sessions to have collators assigned to containers.
                await jumpToSession(context, 2);
                const assignment = (await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();

                // The first account of container 2000 will be rewarded.
                const accountToReward: string = assignment.containerChains[2000][0];

                const { block } = await context.createBlock();
                const accountBalanceBefore = (
                    await polkadotJs.query.system.account(accountToReward)
                ).data.free.toBigInt();

                await mockAndInsertHeadData(context, 2000, block.duration, 2, alice);
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
