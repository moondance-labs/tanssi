import { type DevModeContext, type ZombieContext, expect, type ChopsticksContext } from "@moonwall/cli";
import { filterAndApply, getBlockArray, type KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import type { SubmittableExtrinsic } from "@polkadot/api/types";
import { TypeRegistry } from "@polkadot/types";
import type { Vec, bool, u8, u32, u128 } from "@polkadot/types-codec";
import type {
    AccountId32,
    BlockHash,
    Digest,
    DigestItem,
    EventRecord,
    HeadData,
    Header,
    ParaId,
    Slot,
} from "@polkadot/types/interfaces";
import { stringToHex } from "@polkadot/util";
import Bottleneck from "bottleneck";
import { globalStorageGet, globalStorageHas, globalStorageSet } from "./global-storage.ts";

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
        }
        if (currentSession > session) {
            return null;
        }

        lastBlockHash = (await context.createBlock()).block.hash.toString();
    }
}

export async function jumpBlocks(context: DevModeContext, blockCount: number) {
    let count = blockCount;
    while (count > 0) {
        await context.createBlock();
        count--;
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
    earlyExit?: () => Promise<boolean> | boolean,
    connectionName?: string
): Promise<string | null> {
    const session = (await paraApi.query.session.currentIndex()).addn(count.valueOf()).toNumber();

    return waitToSession(context, paraApi, session, earlyExit, connectionName);
}

export async function waitToSession(
    context,
    paraApi: ApiPromise,
    session: number,
    earlyExit?: () => Promise<boolean> | boolean,
    connectionName?: string
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
        }
        if (currentSession > session) {
            return null;
        }

        await context.waitBlock(1, connectionName ? connectionName : "Tanssi");
    }
}

export function extractFeeAuthor(events: EventRecord[], feePayer: string) {
    const filtered = filterAndApply(
        events,
        "balances",
        ["Withdraw"],
        ({ event }: EventRecord) => event.data as unknown as { who: AccountId32; amount: u128 }
    );
    const extractFeeFromAuthor = filtered.filter(({ who }) => who.toString() === feePayer);
    return extractFeeFromAuthor[0];
}

export function fetchRewardAuthorOrchestrator(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "inflationRewards",
        ["RewardedOrchestrator"],
        ({ event }: EventRecord) => event.data as unknown as { accountId: AccountId32; balance: u128 }
    );

    return filtered[0];
}

export function filterRewardStakingCollator(events: EventRecord[], author: string) {
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

export function filterRewardStakingDelegators(events: EventRecord[], author: string) {
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

export function fetchRewardStakingDelegators(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "pooledStaking",
        ["RewardedDelegators"],
        ({ event }: EventRecord) =>
            event.data as unknown as { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
    );

    return filtered;
}

export function fetchRewardStakingCollators(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "pooledStaking",
        ["RewardedCollator"],
        ({ event }: EventRecord) =>
            event.data as unknown as { collator: AccountId32; autoCompoundingRewards: u128; manualClaimRewards: u128 }
    );

    return filtered;
}

export function fetchRewardAuthorContainers(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "inflationRewards",
        ["RewardedContainer"],
        ({ event }: EventRecord) => event.data as unknown as { accountId: AccountId32; paraId: ParaId; balance: u128 }
    );

    return filtered;
}

export function fetchRandomnessEvent(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "collatorAssignment",
        ["NewPendingAssignment"],
        ({ event }: EventRecord) =>
            event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
    );

    return filtered[0];
}

export function fetchRandomnessEventTanssiSolo(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "tanssiCollatorAssignment",
        ["NewPendingAssignment"],
        ({ event }: EventRecord) =>
            event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
    );

    return filtered[0];
}

export function fetchIssuance(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "balances",
        ["Issued"],
        ({ event }: EventRecord) => event.data as unknown as { amount: u128 }
    );

    if (filtered.length === 0) {
        return { amount: new TypeRegistry().createType("u128", 0) };
    }
    return filtered[0];
}

export function fetchWithdrawnAmount(events: EventRecord[]) {
    let withdrawnAmount = 0n;
    const filtered = filterAndApply(
        events,
        "balances",
        ["Withdraw"],
        ({ event }: EventRecord) => event.data as unknown as { amount: u128 }
    );

    for (const event of filtered) {
        withdrawnAmount += event.amount.toBigInt();
    }
    return withdrawnAmount;
}

export function fetchDepositedAmount(events: EventRecord[]) {
    let depositAmount = 0n;
    const filtered = filterAndApply(
        events,
        "balances",
        ["Deposit"],
        ({ event }: EventRecord) => event.data as unknown as { amount: u128 }
    );

    for (const event of filtered) {
        depositAmount += event.amount.toBigInt();
    }
    return depositAmount;
}

export function fetchCollatorAssignmentTip(events: EventRecord[]) {
    const filtered = filterAndApply(
        events,
        "servicesPayment",
        ["CollatorAssignmentTipCollected"],
        ({ event }: EventRecord) => event.data as unknown as { paraId: ParaId; payer: AccountId32; tip: u128 }
    );

    return filtered[0];
}

export function filterRewardFromOrchestratorWithFailure(events: EventRecord[], author: string) {
    const reward = fetchRewardAuthorOrchestrator(events);
    expect(reward, "orchestrator rewards event not found").not.toBe(undefined);
    expect(
        reward.accountId.toString() === author,
        `orchestrator author  ${reward.accountId.toString()} does not match expected author  ${author}`
    ).to.be.true;
    return reward.balance.toBigInt();
}

export function filterRewardFromOrchestrator(events: EventRecord[], author: string) {
    const reward = fetchRewardAuthorOrchestrator(events);
    if (reward === undefined || reward.accountId.toString() !== author) {
        return 0n;
    }
    return reward.balance.toBigInt();
}

export function filterRewardFromContainer(events: EventRecord[], feePayer: string, paraId: ParaId) {
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

// Same as tx.signAndSend(account), except that it waits for the transaction to be included in a block:
//
// ```
// const txHash = await tx.signAndSend(alice);
// // We don't know if the transaction has been included in a block or not
// const { txHash, blockHash } = await signAndSendAndInclude(tx, alice);
// // We know the blockHash of the block that includes this transaction
// ```
//
// @param tx - The SubmittableExtrinsic to send.
// @param account - The account (keypair or address) used to sign the transaction.
// @param timeout - The timeout in milliseconds, or null for no timeout. Defaults to 5 minutes.
// @returns A Promise resolving with the transaction hash, block hash, and the full status object.
export async function signAndSendAndInclude(tx, account, timeout: number | null = 3 * 60 * 1000) {
    // Inner function that doesn't handle timeout
    const signAndSendAndIncludeInner = (tx, account) => {
        return new Promise((resolve, reject) => {
            tx.signAndSend(account, (result) => {
                const { status, txHash } = result;

                // Resolve once the transaction is finalized
                if (status.isFinalized) {
                    resolve({
                        txHash,
                        blockHash: status.asFinalized,
                        status: result,
                    });
                }
            }).catch((error) => {
                reject(error);
            });
        });
    };

    // If no timeout is specified, directly call the no-timeout version
    if (timeout === null) {
        return signAndSendAndIncludeInner(tx, account);
    }

    // Otherwise, create our own promise that sets/rejects on timeout
    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            console.log("Transaction timed out");
            console.log(tx.toJSON());
            reject(new Error("Transaction timed out"));
        }, timeout);

        signAndSendAndIncludeInner(tx, account)
            .then((result) => {
                clearTimeout(timer);
                resolve(result);
            })
            .catch((error) => {
                clearTimeout(timer);
                reject(error);
            });
    });
}

// Same as `signAndSendAndInclude` but support sending multiple transactions at once.
// By default the nonce is read from the API, an optional nonce parameter can be passed to override this.
export async function signAndSendAndIncludeMany(
    api,
    txs: SubmittableExtrinsic<"promise">[],
    account,
    nonce: number | null = null,
    timeout: number | null = 3 * 60 * 1000
) {
    // Inner function that doesn't handle timeout
    const signAndSendAndIncludeInner = async (txs: SubmittableExtrinsic<"promise">[], account) => {
        let nextNonce = nonce;
        if (nextNonce === null || nextNonce === undefined) {
            nextNonce = (await api.query.system.account(account.address)).nonce.toNumber();
        }

        // Get all transactions except last one
        const txs1 = txs.slice(0, -1);
        // Send them but don't wait for inclusion in a block
        for (const tx of txs1) {
            await tx.signAndSend(account, { nonce: nextNonce++ });
        }

        // Get last transaction
        const tx: SubmittableExtrinsic<"promise"> = txs[txs.length - 1];

        // Only wait for the last transaction to be included, because it cannot be included if the previous transactions
        // were not included, because of the nonce.
        return new Promise((resolve, reject) => {
            tx.signAndSend(account, { nonce: nextNonce++ }, (result) => {
                const { status, txHash } = result;

                // Resolve once the transaction is finalized
                if (status.isFinalized) {
                    resolve({
                        txHash,
                        blockHash: status.asFinalized,
                        status: result,
                    });
                }
            }).catch((error) => {
                reject(error);
            });
        });
    };

    // If no timeout is specified, directly call the no-timeout version
    if (timeout === null) {
        return signAndSendAndIncludeInner(txs, account);
    }

    // Otherwise, create our own promise that sets/rejects on timeout
    return new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            console.log("Transaction timed out");
            reject(new Error("Transaction timed out"));
        }, timeout);

        signAndSendAndIncludeInner(txs, account)
            .then((result) => {
                clearTimeout(timer);
                resolve(result);
            })
            .catch((error) => {
                clearTimeout(timer);
                reject(error);
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
            }
            const res = await originalCreateBlock(tx, opt);
            // Ensure that all the extrinsics have been included
            const txs = Array.isArray(tx) ? tx : [tx];
            const expectedTxHashes = txs.map((x) => x.hash.toString());
            const block = await context.polkadotJs().rpc.chain.getBlock(res.block.hash);
            const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
            // Note, the block may include some additional extrinsics

            for (const a of expectedTxHashes) {
                expect(includedTxHashes).toContain(a);
            }
            return res;
        };
        context.createBlock = createBlockAndCheckExtrinsics;
        context.hasModifiedCreateBlockThatChecksExtrinsics = true;
    }
}

// Creating a relay storage proof in dev tests is not possible, because there is no relay chain, but we can re-use the
// proof from setValidationData, which includes the registrar->paras entries because we added them.
export async function fetchStorageProofFromValidationData(polkadotJs) {
    const block = await polkadotJs.rpc.chain.getBlock();

    // Find parachainSystem.setValidationData extrinsic
    const ex = block.block.extrinsics.find((ex) => {
        const {
            method: { method, section },
        } = ex;
        return section === "parachainSystem" && method === "setValidationData";
    });
    // Error handling if not found
    if (!ex) {
        throw new Error("parachainSystem.setValidationData extrinsic not found");
    }
    const {
        method: { args },
    } = ex;
    const arg = args[0].toJSON();
    const relayProofBlockNumber = arg.validationData.relayParentNumber;
    const relayStorageProof = arg.relayChainState;

    return {
        relayProofBlockNumber,
        relayStorageProof,
    };
}

export async function isEventEmittedInTheNextBlocks(
    context: ZombieContext,
    api: ApiPromise,
    blockCount: number,
    chainName: string,
    eventName: string
) {
    let count = blockCount;
    while (count > 0) {
        await context.waitBlock(1, chainName);
        const currentBlockEvents = await api.query.system.events();
        const filteredEvents = currentBlockEvents.filter((a) => {
            return a.event.method === eventName;
        });
        if (filteredEvents.length > 0) {
            return true;
        }
        count--;
    }
    return false;
}

// we should always use active era
// currentEra is used for scheduling an era change in the next session
// active era informs about the era that is now active
// for checking rewards and slashes, we should always use active
export const getCurrentEraStartBlock = async (api: ApiPromise): Promise<number> => {
    const currentEra = await api.query.externalValidators.activeEra();
    if (currentEra.isNone) {
        expect.fail("No external validators found");
    }

    let epochStartBlock = (await api.query.babe.epochStart())[1].toNumber();

    let apiAtPreviousEpochBlock = await api.at(await api.rpc.chain.getBlockHash(epochStartBlock - 1));

    while (
        currentEra.unwrap().index.toNumber() ===
        (await apiAtPreviousEpochBlock.query.externalValidators.activeEra()).unwrap().index.toNumber()
    ) {
        epochStartBlock = (await apiAtPreviousEpochBlock.query.babe.epochStart()).toJSON()[1];
        apiAtPreviousEpochBlock = await api.at(await api.rpc.chain.getBlockHash(epochStartBlock - 1));
    }

    return epochStartBlock;
};

// Same as getCurrentEraStartBlock, but using and api at a certain block height
// the block that you pass is the block from which we will get the active era
// and from which we will get the era start block number
export const getPastEraStartBlock = async (currentApi: ApiPromise, block: number): Promise<number> => {
    const apiAtCheckpointForEra = await currentApi.at(await currentApi.rpc.chain.getBlockHash(block));

    const currentEra = await apiAtCheckpointForEra.query.externalValidators.activeEra();
    if (currentEra.isNone) {
        expect.fail("No external validators found");
    }

    if (currentEra.unwrap().index.toNumber() === 0) {
        throw new Error("There is no past era start block, current era is 0");
    }

    let epochStartBlock = (await apiAtCheckpointForEra.query.babe.epochStart())[1].toNumber();

    let apiAtPreviousEpochBlock = await currentApi.at(await currentApi.rpc.chain.getBlockHash(epochStartBlock - 1));

    while (
        currentEra.unwrap().index.toNumber() ===
        (await apiAtPreviousEpochBlock.query.externalValidators.activeEra()).unwrap().index.toNumber()
    ) {
        epochStartBlock = (await apiAtPreviousEpochBlock.query.babe.epochStart()).toJSON()[1];
        apiAtPreviousEpochBlock = await currentApi.at(await currentApi.rpc.chain.getBlockHash(epochStartBlock - 1));
    }

    return epochStartBlock;
};

export const getEraIndexForBlock = async (api: ApiPromise, blockNumber: number): Promise<number> => {
    const apiAtBlock = await api.at(await api.rpc.chain.getBlockHash(blockNumber));
    const eraAtBlock = await apiAtBlock.query.externalValidators.activeEra();
    return eraAtBlock.unwrap().index.toNumber();
};

export const getBlockNumberAtWhichEraStarted = async (api: ApiPromise): Promise<number> => {
    const chain = (await api.rpc.system.chain()).toString();
    const FIRST_BLOCK_WITH_ERA_PRESENT_FOR_STAGELIGHT: number = 734672;
    if (chain === "Stagelight") {
        return FIRST_BLOCK_WITH_ERA_PRESENT_FOR_STAGELIGHT;
    }
    // For Dancelight ExternalValidators was present from the beginning
    return 1;
};

export const findEraBlockUsingBinarySearch = async (
    api: ApiPromise,
    eraIndex: number
): Promise<{ found: boolean; blockHash: BlockHash; blockNumber: number }> => {
    const sessionsPerEra = api.consts.externalValidators.sessionsPerEra.toNumber();
    const blocksPerSession = api.consts.babe.epochDuration.toNumber();
    const approximateBlockForEra = (eraIndex + 1) * sessionsPerEra * blocksPerSession;
    let currentEraIndex = 0;

    const runtimeUpgradedToSupportEraAt = await getBlockNumberAtWhichEraStarted(api);
    console.log("Runtime upgrade to support era at:", runtimeUpgradedToSupportEraAt);

    // Approximated block for era can be different than in reality in case of downtime, in that case there are no block produced in that era or the era
    // only consist of earlier blocks than approximated block.
    // In other words, if there is a downtime in between we will get later era index for the approximated block compared to what we expected.
    let currentMax = runtimeUpgradedToSupportEraAt + approximateBlockForEra;
    // In case if currentMax exceeded the real max block number
    currentMax = Math.min((await api.rpc.chain.getHeader()).number.toNumber(), currentMax);
    // In worst case, it could be possible that chain has skipped all eras till this one
    let currentMin = runtimeUpgradedToSupportEraAt;
    let currentBlock = currentMax;

    // Most of the time the static calculation of era block matches the reality, so check that first and if that is true bypass it.
    if ((await getEraIndexForBlock(api, currentMax)) !== eraIndex) {
        while (currentMax >= currentMin) {
            currentBlock = Math.floor((currentMax + currentMin) / 2);
            currentEraIndex = await getEraIndexForBlock(api, currentBlock);
            if (currentEraIndex > eraIndex) {
                currentMax = currentBlock - 1;
            } else if (currentEraIndex < eraIndex) {
                currentMin = currentBlock + 1;
            } else {
                break;
            }
        }
    }

    // We did not find any block for this era
    if (currentMax < currentMin) {
        return {
            found: false,
            blockHash: undefined,
            blockNumber: undefined,
        };
    }

    const eraStartBlock = await getPastEraStartBlock(api, currentBlock);
    const eraStartBlockHash = await api.rpc.chain.getBlockHash(eraStartBlock);

    return {
        found: true,
        blockHash: eraStartBlockHash,
        blockNumber: eraStartBlock,
    };
};

// Helper function to make rewards work for a specific block and slot.
// We need to mock a proper HeadData object for AuthorNoting inherent to work, and thus
// rewards take place.
//
// Basically, if we don't call this function before testing the rewards given
// to collators in a block, the HeadData object mocked in genesis will not be decoded properly
// and the AuthorNoting inherent will fail.
export async function mockAndInsertHeadData(
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

// This function creates chopsticks blocks until a given tx is included
export async function chopsticksWaitTillIncluded(
    context: ChopsticksContext,
    api: ApiPromise,
    sender: KeyringPair,
    tx: SubmittableExtrinsic<"promise">,
    maxTries: number | null = 5
) {
    let tries = 0;

    while (tries < maxTries) {
        const txHash = await tx.signAndSend(sender);
        const result = await context.createBlock({ count: 1 });

        const block = await api.rpc.chain.getBlock(result.result);
        const includedTxHashes = block.block.extrinsics.map((x) => x.hash.toString());
        if (includedTxHashes.includes(txHash.toString())) {
            break;
        }
        tries++;
    }
}

export async function getLastSessionEndBlock(api: ApiPromise, lastSessionIndex: number): Promise<number> {
    let blockNumber = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
    let currentSessionIndex = (await api.query.session.currentIndex()).toNumber();
    while (currentSessionIndex > lastSessionIndex) {
        blockNumber -= 1;
        const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
        const apiAtBlock = await api.at(blockHash);
        currentSessionIndex = (await apiAtBlock.query.session.currentIndex()).toNumber();
    }
    return blockNumber;
}

export type EventType = {
    phase: "Initialization" | "Finalization" | { ApplyExtrinsic: string };
    event: { method: string; section: string; index: "string"; data: unknown };
    topics: string[];
};

export type ExtrinsicType = {
    isSigned: boolean;
    method: { method: string; section: string; args: unknown };
};

export type BlockData = {
    blockNum: number;
    blockHash: string;
    events: EventType[];
    extrinsics: ExtrinsicType[];
    extrinsicIndexToEventsMap: Map<string, EventType[]>;
};

export const getBlocksDataForPeriodMs = async (api: ApiPromise, timePeriodMs: number): Promise<BlockData[]> => {
    const runtimeName = api.runtimeVersion.specName.toString();
    const key = `blocks_data_key::${runtimeName}::${timePeriodMs}`;

    if (globalStorageHas(key)) {
        return globalStorageGet<BlockData[]>(key);
    }

    const blockNumbersArray = await getBlockArray(api, timePeriodMs);

    const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });

    console.log(
        `About to start blocks data fetching (#${blockNumbersArray[0]} ... #${blockNumbersArray[blockNumbersArray.length - 1]}). Total: ${blockNumbersArray.length}`
    );
    const start = performance.now();
    const blocksData: BlockData[] = await Promise.all(
        blockNumbersArray.map((num) => limiter.schedule(() => getBlockData(api, num)))
    );
    const end = performance.now();

    console.log(`Blocks data fetching took: ${(end - start).toFixed(2)} ms. Fetched: ${blocksData.length} blocks.`);
    globalStorageSet(key, blocksData);

    return blocksData;
};

export const getBlockData = async (api: ApiPromise, blockNum: number): Promise<BlockData> => {
    const blockHash = (await api.rpc.chain.getBlockHash(blockNum)) as unknown as string;
    const apiAt = await api.at(blockHash);
    const events = (await apiAt.query.system.events()).map((event) => event.toHuman() as unknown as EventType);
    const block = await api.rpc.chain.getBlock(blockHash);
    const extrinsics = block.block.extrinsics.map((extrinsic) => extrinsic.toHuman() as unknown as ExtrinsicType);

    const extrinsicIndexToEventsMap = new Map<string, EventType[]>();

    for (const eventRecord of events) {
        const phase = eventRecord.phase;

        if (typeof phase !== "object" || phase.ApplyExtrinsic === undefined) {
            continue;
        }

        const extrinsicIndex = phase.ApplyExtrinsic;
        if (!extrinsicIndexToEventsMap.has(extrinsicIndex)) {
            extrinsicIndexToEventsMap.set(extrinsicIndex, []);
        }

        const events = extrinsicIndexToEventsMap.get(extrinsicIndex);
        events.push(eventRecord);

        extrinsicIndexToEventsMap.set(extrinsicIndex, events);
    }

    return {
        blockNum,
        blockHash,
        events,
        extrinsics,
        extrinsicIndexToEventsMap,
    };
};
