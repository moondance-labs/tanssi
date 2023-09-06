import { DevModeContext, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

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

export async function waitSessions(context, paraApi: ApiPromise, count: number): Promise<string | null> {
    const session = (await paraApi.query.session.currentIndex()).addn(count.valueOf()).toNumber();

    return waitToSession(context, paraApi, session);
}

export async function waitToSession(context, paraApi: ApiPromise, session: number): Promise<string | null> {
    for (;;) {
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
            if (status.isInBlock) {
                resolve({
                    txHash,
                    blockHash: status.asInBlock,
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
                const expectedTxHashes = tx.map((x) => x.hash.toString());
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
