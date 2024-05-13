import { DevModeContext, expect } from "@moonwall/cli";
import { filterAndApply } from "@moonwall/util";

import { ApiPromise } from "@polkadot/api";
import { AccountId32, EventRecord } from "@polkadot/types/interfaces";
import { Vec, u8, u32, bool } from "@polkadot/types-codec";
export async function jumpSessions(context: DevModeContext, count: number): Promise<string | null> {
    const session = (await context.polkadotJs().query.session.currentIndex()).addn(count.valueOf()).toNumber();

    return jumpToSession(context, session);
}

export async function jumpToSession(context: DevModeContext, session: number): Promise<string | null> {
    let lastBlockHash = null;
    for (;;) {
        const currentSession = (await context.polkadotJs().query.session.currentIndex()).toNumber();
        if (currentSession === session) {
            return lastBlockHash;
        } else if (currentSession > session) {
            return null;
        }

        lastBlockHash = (await context.createBlock()).block.hash.toString();
    }
}

export async function jumpBlocks(context: DevModeContext, blockCount: number) {
    while (blockCount > 0) {
        await context.createBlock();
        blockCount--;
    }
}

export async function jumpToBlock(context: DevModeContext, targetBlockNumber: number) {
    let blockNumber = (await context.polkadotJs().rpc.chain.getBlock()).block.header.number.toNumber();

    while (blockNumber + 1 < targetBlockNumber) {
        await context.createBlock();
        blockNumber = (await context.polkadotJs().rpc.chain.getBlock()).block.header.number.toNumber();
    }
}

export async function waitSessions(
    context,
    paraApi: ApiPromise,
    count: number,
    earlyExit?: () => Promise<boolean> | boolean
): Promise<string | null> {
    const session = (await paraApi.query.session.currentIndex()).addn(count.valueOf()).toNumber();

    return waitToSession(context, paraApi, session, earlyExit);
}

export async function waitToSession(
    context,
    paraApi: ApiPromise,
    session: number,
    earlyExit?: () => Promise<boolean> | boolean
): Promise<string | null> {
    for (;;) {
        if (earlyExit && (await earlyExit())) {
            // Exit early if the callback returns true
            return null;
        }

        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        if (currentSession === session) {
            const signedBlock = await paraApi.rpc.chain.getBlock();
            return signedBlock.block.header.hash.toString();
        } else if (currentSession > session) {
            return null;
        }

        await context.waitBlock(1, "Tanssi");
    }
}

export function extractFeeAuthor(events: EventRecord[] = [], feePayer: string) {
    const filtered = filterAndApply(
        events,
        "balances",
        ["Withdraw"],
        ({ event }: EventRecord) => event.data as unknown as { who: AccountId32; amount: u128 }
    );
    const extractFeeFromAuthor = filtered.filter(({ who }) => who.toString() === feePayer);
    return extractFeeFromAuthor[0];
}

export function fetchRewardAuthorOrchestrator(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "inflationRewards",
        ["RewardedOrchestrator"],
        ({ event }: EventRecord) => event.data as unknown as { accountId: AccountId32; balance: u128 }
    );

    return filtered[0];
}

export function filterRewardStakingCollator(events: EventRecord[] = [], author: string) {
    const stakignRewardEvents = fetchRewardStakingCollators(events);
    for (const index in stakignRewardEvents) {
        if (stakignRewardEvents[index].collator.toString() === author) {
            return {
                manualRewards: stakignRewardEvents[index].manualClaimRewards.toBigInt(),
                autoCompoundingRewards: stakignRewardEvents[index].autoCompoundingRewards.toBigInt(),
            };
        }
    }

    return {
        manualRewards: 0n,
        autoCompoundingRewards: 0n,
    };
}

export function filterRewardStakingDelegators(events: EventRecord[] = [], author: string) {
    const stakignRewardEvents = fetchRewardStakingDelegators(events);
    for (const index in stakignRewardEvents) {
        if (stakignRewardEvents[index].collator.toString() === author) {
            return {
                manualRewards: stakignRewardEvents[index].manualClaimRewards.toBigInt(),
                autoCompoundingRewards: stakignRewardEvents[index].autoCompoundingRewards.toBigInt(),
            };
        }
    }

    return {
        manualRewards: 0n,
        autoCompoundingRewards: 0n,
    };
}

export function fetchRewardStakingDelegators(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "pooledStaking",
        ["RewardedDelegators"],
        ({ event }: EventRecord) =>
            event.data as unknown as { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
    );

    return filtered;
}

export function fetchRewardStakingCollators(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "pooledStaking",
        ["RewardedCollator"],
        ({ event }: EventRecord) =>
            event.data as unknown as { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
    );

    return filtered;
}

export function fetchRewardAuthorContainers(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "inflationRewards",
        ["RewardedContainer"],
        ({ event }: EventRecord) => event.data as unknown as { accountId: AccountId32; paraId: ParaId; balance: u128 }
    );

    return filtered;
}

export function fetchRandomnessEvent(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "collatorAssignment",
        ["NewPendingAssignment"],
        ({ event }: EventRecord) =>
            event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
    );

    return filtered[0];
}

export function fetchIssuance(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "balances",
        ["Issued"],
        ({ event }: EventRecord) => event.data as unknown as { amount: u128 }
    );

    return filtered[0];
}

export function fetchCollatorAssignmentTip(events: EventRecord[] = []) {
    const filtered = filterAndApply(
        events,
        "servicesPayment",
        ["CollatorAssignmentTipCollected"],
        ({ event }: EventRecord) => event.data as unknown as { paraId: ParaId; payer: AccountId32; tip: u128 }
    );

    return filtered[0];
}

export function filterRewardFromOrchestratorWithFailure(events: EventRecord[] = [], author: string) {
    const reward = fetchRewardAuthorOrchestrator(events);
    expect(reward, `orchestrator rewards event not found`).not.toBe(undefined);
    expect(
        reward.accountId.toString() === author,
        `orchestrator author  ${reward.accountId.toString()} does not match expected author  ${author}`
    ).to.be.true;
    return reward.balance.toBigInt();
}

export function filterRewardFromOrchestrator(events: EventRecord[] = [], author: string) {
    const reward = fetchRewardAuthorOrchestrator(events);
    if (reward === undefined || reward.accountId.toString() !== author) {
        return 0n;
    } else {
        return reward.balance.toBigInt();
    }
}

export function filterRewardFromContainer(events: EventRecord[] = [], feePayer: string, paraId: ParaId) {
    const rewardEvents = fetchRewardAuthorContainers(events);
    for (const index in rewardEvents) {
        if (
            rewardEvents[index].accountId.toString() === feePayer &&
            rewardEvents[index].paraId.toString() === paraId.toString()
        ) {
            return rewardEvents[index].balance.toBigInt();
        }
    }
    return 0n;
}

/// Same as tx.signAndSend(account), except that it waits for the transaction to be included in a block:
///
/// ```
/// const txHash = await tx.signAndSend(alice);
/// // We don't know if the transaction has been included in a block or not
/// const { txHash, blockHash } = await signAndSendAndInclude(tx, alice);
/// // We know the blockHash of the block that includes this transaction
/// ```
export function signAndSendAndInclude(tx, account): Promise<{ txHash; blockHash; status }> {
    return new Promise((resolve) => {
        tx.signAndSend(account, ({ status, txHash }) => {
            if (status.isFinalized) {
                resolve({
                    txHash,
                    blockHash: status.asFinalized,
                    status,
                });
            }
        });
    });
}

export function initializeCustomCreateBlock(context): any {
    if (!context.hasModifiedCreateBlockThatChecksExtrinsics) {
        const originalCreateBlock = context.createBlock;
        // Alternative implementation of context.createBlock that checks that the extrinsics have
        // actually been included in the created block.
        const createBlockAndCheckExtrinsics = async (tx, opt) => {
            if (tx === undefined) {
                return await originalCreateBlock(tx, opt);
            } else {
                const res = await originalCreateBlock(tx, opt);
                // Ensure that all the extrinsics have been included
                const txs = Array.isArray(tx) ? tx : [tx];
                const expectedTxHashes = txs.map((x) => x.hash.toString());
                const block = await context.polkadotJs().rpc.chain.getBlock(res.block.hash);
                const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
                // Note, the block may include some additional extrinsics
                expectedTxHashes.forEach((a) => {
                    expect(includedTxHashes).toContain(a);
                });
                return res;
            }
        };
        context.createBlock = createBlockAndCheckExtrinsics;
        context.hasModifiedCreateBlockThatChecksExtrinsics = true;
    }
}
