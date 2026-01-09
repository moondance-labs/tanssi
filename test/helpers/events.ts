// @ts-nocheck

import { expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import type { SpRuntimeDispatchError } from "@polkadot/types/lookup";

export async function expectEventCount(polkadotJs: ApiPromise, eventCounts: Record<string, number>): Promise<void> {
    const events = await polkadotJs.query.system.events();

    for (const [eventMethod, expectedCount] of Object.entries(eventCounts)) {
        const matchingEvents = events.filter(({ event }) => event.method === eventMethod);

        expect(
            matchingEvents.length,
            `Expected ${expectedCount} occurrences of event '${eventMethod}', but found ${matchingEvents.length}`
        ).to.equal(expectedCount);
    }
}

export async function checkCallIsFiltered(context: any, polkadotJs: ApiPromise, tx: any) {
    try {
        await context.createBlock(await tx, { allowFailures: false });
        expect.fail("Expected call to be filtered, but it was not.");
    } catch {
        const events = await polkadotJs.query.system.events();
        const errors = events
            .filter(({ event }) => polkadotJs.events.system.ExtrinsicFailed.is(event))
            .map(
                ({
                    event: {
                        data: [error],
                    },
                }) => {
                    const dispatchError = error as SpRuntimeDispatchError;
                    if (dispatchError.isModule) {
                        const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                        const { method } = decoded;

                        return `${method}`;
                    }
                    return error.toString();
                }
            );

        expect(errors.length).to.be.eq(1);
        expect(errors[0]).to.be.eq("CallFiltered");
    }
}

export async function retrieveDispatchErrors(polkadotJs: ApiPromise) {
    const events = await polkadotJs.query.system.events();
    const errors = events
        .filter(({ event }) => polkadotJs.events.system.ExtrinsicFailed.is(event))
        .map(
            ({
                event: {
                    data: [error],
                },
            }) => {
                const dispatchError = error as SpRuntimeDispatchError;
                if (dispatchError.isModule) {
                    const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                    const { method } = decoded;

                    return `${method}`;
                }
                return error.toString();
            }
        );
    return errors;
}

export async function retrieveSudoDispatchErrors(polkadotJs: ApiPromise) {
    const events = await polkadotJs.query.system.events();

    const sudoErrors = events
        .filter(({ event }) => event.section === "sudo" && event.method === "Sudid")
        .map(({ event }) => {
            const result = event.data[0];
            if (result.isErr) {
                const dispatchError = result.asErr as DispatchError;

                // Decode the error (module errors)
                if (dispatchError.isModule) {
                    const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                    const { method, section } = decoded;
                    return {
                        section,
                        method,
                    };
                }
            }
            return null;
        })
        .filter((err) => err !== null);

    return sudoErrors;
}

export async function retrieveBatchDispatchErrors(polkadotJs: ApiPromise) {
    const events = await polkadotJs.query.system.events();

    const batchErrors = events
        .filter(({ event }) => event.section === "utility" && event.method === "BatchInterrupted")
        .map(({ event }) => {
            const dispatchError = event.data[1] as DispatchError;

            if (dispatchError.isModule) {
                const decoded = polkadotJs.registry.findMetaError(dispatchError.asModule);
                const { section, method } = decoded;
                return {
                    section,
                    method,
                };
            }
        })
        .filter((err) => err !== null);

    return batchErrors;
}

/**
 * Search for an event across a range of blocks
 *
 * @param api - The Polkadot API instance
 * @param startBlockNumber - The block number to start searching from (inclusive)
 * @param endBlockNumber - The block number to stop searching at (inclusive)
 * @param predicate - Function to test if an event record matches the desired event
 * @returns Object with blockNum and event if found, or null if not found
 */
export async function findEventInBlockRange(
    api: ApiPromise,
    startBlockNumber: number,
    endBlockNumber: number,
    predicate: (record: any) => boolean
): Promise<{ blockNum: number; event: any } | null> {
    for (let blockNum = startBlockNumber; blockNum <= endBlockNumber; blockNum++) {
        const blockHash = await api.rpc.chain.getBlockHash(blockNum);
        const apiAt = await api.at(blockHash);
        const events = await apiAt.query.system.events();

        const matchingEvent = events.find(predicate);

        if (matchingEvent) {
            return { blockNum, event: matchingEvent };
        }
    }

    return null;
}

/**
 * Search for an event in recent blocks (from current block backwards)
 *
 * @param api - The Polkadot API instance
 * @param predicate - Function to test if an event record matches the desired event
 * @param maxBlocksBack - Maximum number of blocks to search backwards (default: 20)
 * @returns Object with blockNum and event if found, or null if not found
 */
export async function findEventInRecentBlocks(
    api: ApiPromise,
    predicate: (record: any) => boolean,
    maxBlocksBack = 20
): Promise<{ blockNum: number; event: any } | null> {
    const currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();
    const startBlock = Math.max(1, currentBlock - maxBlocksBack);

    return findEventInBlockRange(api, startBlock, currentBlock, predicate);
}
