import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { findEraBlockUsingBinarySearch, PRIMARY_GOVERNANCE_CHANNEL_ID } from "../../utils";
import { getBlockArray } from "@moonwall/util";
import Bottleneck from "bottleneck";
import type { FrameSystemEventRecord } from "@polkadot/types/lookup";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);

describeSuite({
    id: "SMOK10",
    title: "Inbound channel validator reception",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let blocksData: { blockNum: number; events: FrameSystemEventRecord[] }[];

        beforeAll(async () => {
            api = context.polkadotJs();

            const blockNumbersArray = await getBlockArray(api, timePeriod);

            const limiter = new Bottleneck({ maxConcurrent: 5, minTime: 100 });

            const start = performance.now();
            blocksData = await Promise.all(
                blockNumbersArray.map((num) => limiter.schedule(() => getBlockData(api, num)))
            );
            const end = performance.now();

            log(`Blocks data fetching took: ${(end - start).toFixed(2)} ms. Fetched: ${blocksData.length} blocks.`);
        }, timeout);

        it({
            id: "C01",
            title: "External index is correctly updated",
            test: async () => {
                const bondedEras = (
                    await api.query.externalValidatorSlashes.bondedEras()
                ).toJSON() as BondedEraParams[];
                // Let's check 2 recent eras
                for (const bondedEra of bondedEras.slice(-2)) {
                    const result = await findEraBlockUsingBinarySearch(api, bondedEra[0]);

                    if (!result.found) {
                        continue;
                    }

                    const startEraBlockNumber = result.blockNumber;
                    const lastBlockNumberPreviousEra = startEraBlockNumber - 1;

                    const apiAtLastBlockPreviousEra = await api.at(
                        await api.rpc.chain.getBlockHash(lastBlockNumberPreviousEra)
                    );

                    const apiAtStartBlock = await api.at(await api.rpc.chain.getBlockHash(startEraBlockNumber));

                    const startBlockEra = (await apiAtStartBlock.query.externalValidators.activeEra())
                        .unwrap()
                        .index.toNumber();

                    const prevBlockEra = (await apiAtLastBlockPreviousEra.query.externalValidators.activeEra())
                        .unwrap()
                        .index.toNumber();

                    console.log(`
StartEraBlockNumber: ${startEraBlockNumber}
PrevBlockNumber: ${startEraBlockNumber - 1}
Era at start block number: ${startBlockEra}
Era at prev block number: ${prevBlockEra}
Pending External index: ${(await apiAtLastBlockPreviousEra.query.externalValidators.pendingExternalIndex()).toNumber()}    
Bonded era: ${JSON.stringify(bondedEra)}
                    `);

                    expect(
                        (await apiAtLastBlockPreviousEra.query.externalValidators.pendingExternalIndex()).toNumber()
                    ).toEqual(bondedEra[2]);
                }
            },
        });

        it({
            id: "C02",
            title: "Inbound primary channel should have increased the nonce",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    const event = blockToCheck.events.find((event) => event.event.method === "ExternalValidatorsSet");

                    if (event) {
                        const [apiAtPreviousBlock, apiAtCurrentBlock] = await Promise.all([
                            api.at(await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1)),
                            api.at(await api.rpc.chain.getBlockHash(blockToCheck.blockNum)),
                        ]);

                        const [previousBlockNonce, currentBlockNonce] = await Promise.all([
                            await apiAtPreviousBlock.query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID),
                            await apiAtCurrentBlock.query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID),
                        ]);

                        expect(
                            previousBlockNonce.toBigInt() + 1n,
                            `Nonce for block: ${blockToCheck.blockNum} should increment by 1, because event with method: ${event.event.method} was emitted`
                        ).toEqual(currentBlockNonce.toBigInt());
                    }
                }
            },
        });
    },
});

const getBlockData = async (api: ApiPromise, blockNum: number) => {
    const blockHash = await api.rpc.chain.getBlockHash(blockNum);
    const apiAt = await api.at(blockHash);

    return {
        blockNum: blockNum,
        events: await apiAt.query.system.events(),
    };
};

type BondedEraParams = [number, number, number];
