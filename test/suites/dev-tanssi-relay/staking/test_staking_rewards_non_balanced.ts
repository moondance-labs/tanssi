import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll, DevModeContext } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Header, ParaId, HeadData, Digest, DigestItem, Slot } from "@polkadot/types/interfaces";
import {
    fetchIssuance,
    fetchRewardAuthorContainers,
    filterRewardStakingCollator,
    filterRewardStakingDelegators,
    jumpSessions,
} from "util/block";
import { DANCE } from "util/constants";
import { stringToHex } from "@polkadot/util";

export async function createBlockAndRemoveInvulnerables(context: DevModeContext, sudoKey: KeyringPair) {
    let nonce = (await context.polkadotJs().rpc.system.accountNextIndex(sudoKey.address)).toNumber();
    const invulnerables = await context.polkadotJs().query.tanssiInvulnerables.invulnerables();

    const txs = invulnerables.map((invulnerable) =>
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.tanssiInvulnerables.removeInvulnerable(invulnerable))
            .signAsync(sudoKey, { nonce: nonce++ })
    );

    await context.createBlock(txs);
}

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
    id: "DT0303",
    title: "Staking candidate reward test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let charlie: KeyringPair;
        let dave: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            bob = context.keyring.bob;
            charlie = context.keyring.charlie;
            dave = context.keyring.dave;

            await createBlockAndRemoveInvulnerables(context, alice);

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

            // We will make each of them self-delegate the min amount, while
            // we will make each of them delegate the other with 50%
            // Alice autocompounding, Bob will be manual
            let aliceNonce = (await polkadotJs.rpc.system.accountNextIndex(alice.address)).toNumber();
            let bobNonce = (await polkadotJs.rpc.system.accountNextIndex(bob.address)).toNumber();
            let charlieNonce = (await polkadotJs.rpc.system.accountNextIndex(charlie.address)).toNumber();
            let daveNonce = (await polkadotJs.rpc.system.accountNextIndex(dave.address)).toNumber();

            await context.createBlock([
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(alice.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.alice, { nonce: aliceNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(bob.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.bob, { nonce: bobNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(charlie.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.charlie, { nonce: charlieNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(charlie.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.dave, { nonce: daveNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(dave.address, "AutoCompounding", 18000n * DANCE)
                    .signAsync(context.keyring.charlie, { nonce: charlieNonce++ }),
                await polkadotJs.tx.pooledStaking
                    .requestDelegate(dave.address, "ManualRewards", 2000n * DANCE)
                    .signAsync(context.keyring.dave, { nonce: daveNonce++ }),
            ]);
            // At least 2 sessions for the change to have effect
            await jumpSessions(context, 2);
            // +2 because in tanssi-relay sessions start 1 block later
            await context.createBlock();
            await context.createBlock();
        });
        it({
            id: "E01",
            title: "Alice should receive rewards through staking now",
            test: async function () {
                const assignment = (await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();
                console.log(
                    "Assignment at block ",
                    (await polkadotJs.query.system.number()).toJSON(),
                    ": ",
                    assignment
                );

                // Find alice in list of collators
                let paraId = null;
                let slotOffset = 0;
                const containerIds = [2000, 2001];
                for (const id of containerIds) {
                    const index = assignment.containerChains[id].indexOf(alice.address);
                    if (index !== -1) {
                        paraId = id;
                        slotOffset = index;
                        break;
                    }
                }

                expect(paraId, `Alice not found in list of collators: ${assignment}`).to.not.be.null;

                //const accountToReward: string = assignment.containerChains[2000][0];
                const accountToReward = alice.address;
                console.log("accountToReward: ", accountToReward);
                // 70% is distributed across all rewards
                // But we have 2 container chains, so it should get 1/2 of this
                const { block } = await context.createBlock();
                const accountBalanceBefore = (
                    await polkadotJs.query.system.account(accountToReward)
                ).data.free.toBigInt();

                //await customDevRpcRequest("mock_enableParaInherentCandidate", []);
                console.log("Mocking head data at block ", 1 + (await polkadotJs.query.system.number()).toJSON());
                await mockAndInsertHeadData(context, paraId, block.duration, 2 + slotOffset, alice);
                await context.createBlock();
                const events = await polkadotJs.query.system.events();
                const issuance = await fetchIssuance(events).amount.toBigInt();
                const chainRewards = (issuance * 7n) / 10n;
                const rounding = chainRewards % 3n > 0 ? 1n : 0n;
                const expectedContainerReward = chainRewards / 2n - rounding;
                const rewards = fetchRewardAuthorContainers(events);
                expect(rewards.length).toBe(1);
                const reward = rewards[0];
                const stakingRewardedCollator = filterRewardStakingCollator(events, reward.accountId.toString());
                const stakingRewardedDelegators = filterRewardStakingDelegators(events, reward.accountId.toString());

                // How much should the author have gotten?
                // For now everything as we did not execute the pending operations
                expect(reward.balance.toBigInt()).toBeGreaterThanOrEqual(expectedContainerReward - 1n);
                expect(reward.balance.toBigInt()).toBeLessThanOrEqual(expectedContainerReward + 1n);
                expect(stakingRewardedCollator.manualRewards).to.equal(reward.balance.toBigInt());
                expect(stakingRewardedCollator.autoCompoundingRewards).to.equal(0n);
                expect(stakingRewardedDelegators.manualRewards).to.equal(0n);
                expect(stakingRewardedDelegators.autoCompoundingRewards).to.equal(0n);

                const accountBalanceAfter = (
                    await polkadotJs.query.system.account(accountToReward)
                ).data.free.toBigInt();

                expect(accountBalanceAfter - accountBalanceBefore).to.equal(reward.balance.toBigInt());
            },
        });

        it({
            id: "E02",
            title: "Alice should receive shared rewards with delegators through staking now",
            test: async function () {
                await jumpSessions(context, 1);
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

                const assignment = (await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()).toJSON();
                console.log(
                    "Assignment at block ",
                    (await polkadotJs.query.system.number()).toJSON(),
                    ": ",
                    assignment
                );
                // Find alice in list of collators
                let paraId = null;
                let slotOffset = 0;
                const containerIds = [2000, 2001];
                for (const id of containerIds) {
                    const index = assignment.containerChains[id].indexOf(alice.address);
                    if (index !== -1) {
                        paraId = id;
                        slotOffset = index;
                        break;
                    }
                }

                expect(paraId, `Alice not found in list of collators: ${assignment}`).to.not.be.null;

                // We create one more block
                const { block } = await context.createBlock();
                await mockAndInsertHeadData(context, paraId, block.duration + 1, 4 + slotOffset, alice);
                await context.createBlock();
                const events = await polkadotJs.query.system.events();
                const rewards = fetchRewardAuthorContainers(events);
                expect(rewards.length).toBe(1);
                const reward = rewards[0];

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

                const stakingRewardedCollator = filterRewardStakingCollator(events, reward.accountId.toString());
                const stakingRewardedDelegators = filterRewardStakingDelegators(events, reward.accountId.toString());

                // Test ranges, as we can have rounding errors for Perbill manipulation
                expect(stakingRewardedDelegators.manualRewards).toBeGreaterThanOrEqual(
                    realDistributedManualDelegatorRewards - 1n
                );
                expect(stakingRewardedDelegators.manualRewards).toBeLessThanOrEqual(
                    realDistributedManualDelegatorRewards + 1n
                );
                expect(stakingRewardedDelegators.autoCompoundingRewards).toBeGreaterThanOrEqual(
                    delegatorsAutoCompoundRewards - 1n
                );
                expect(stakingRewardedDelegators.autoCompoundingRewards).toBeLessThanOrEqual(
                    delegatorsAutoCompoundRewards + 1n
                );

                // TODO: test better what goes into auto and what goes into manual for collator
                const delegatorDust =
                    delegatorRewards - realDistributedManualDelegatorRewards - delegatorsAutoCompoundRewards;
                expect(
                    stakingRewardedCollator.manualRewards + stakingRewardedCollator.autoCompoundingRewards
                ).toBeGreaterThanOrEqual(collatorPercentage + delegatorDust - 1n);
                expect(
                    stakingRewardedCollator.manualRewards + stakingRewardedCollator.autoCompoundingRewards
                ).toBeLessThanOrEqual(collatorPercentage + delegatorDust + 1n);
            },
        });
    },
});
