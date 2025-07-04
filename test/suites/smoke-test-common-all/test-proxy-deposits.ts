import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

import { totalForProxies } from "../../utils/proxies.ts";
import { type BlockData, getBlocksDataForPeriodMs } from "../../utils";
import type { u128 } from "@polkadot/types-codec";

const timePeriod = process.env.TIME_PERIOD ? Number(process.env.TIME_PERIOD) : 1 * 60 * 60 * 1000;
const timeout = Math.max(Math.floor(timePeriod / 12), 5000);

describeSuite({
    id: "S09",
    title: "Verify proxies deposits",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let api: ApiPromise;
        let blocksData: BlockData[];

        beforeAll(async () => {
            api = context.polkadotJs();

            blocksData = await getBlocksDataForPeriodMs(api, timePeriod);
        }, timeout);

        it({
            id: "C01",
            title: "Add Proxy transaction holds the deposit",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const proxyAddedEvents = events
                            .filter((event) => event.event.method === "ProxyAdded" && event.event.section === "proxy")
                            .map((event) => event.event.data as unknown as { delegator: string });

                        if (!proxyAddedEvents.length) {
                            continue;
                        }

                        for (const proxyAddedEvent of proxyAddedEvents) {
                            log(`Found "ProxyAdded" event for block: ${blockToCheck.blockNum}. Checking...`);

                            // Gel proxies at previous block
                            const prevBlockHash = await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1);
                            const apiAtPrevBlock = await api.at(prevBlockHash);

                            const proxies = await apiAtPrevBlock.query.proxy.proxies(proxyAddedEvent.delegator);
                            const proxiesLength = proxies.toJSON()[0].length;
                            const alreadyInProxyReserve = proxies[1].toBigInt();

                            const expectedAmount = totalForProxies(api, proxiesLength + 1);

                            const reserved = events
                                .filter(
                                    (event) => event.event.method === "Reserved" && event.event.section === "balances"
                                )
                                .map((event) => event.event.data as unknown as { amount: u128 });

                            expect(reserved.length).toBeGreaterThan(0);
                            const actuallyReserved = reserved[0].amount.toBigInt();
                            expect(
                                actuallyReserved,
                                `Block #${blockToCheck.blockNum}. Expecting actuallyReserved: ${actuallyReserved} to equal expectedAmount - alreadyInProxyReserve: ${expectedAmount - alreadyInProxyReserve}`
                            ).toEqual(expectedAmount - alreadyInProxyReserve);
                        }
                    }
                }
            },
        });

        it({
            id: "C02",
            title: "Remove Proxy transaction unholds the deposit",
            test: async () => {
                for (const blockToCheck of blocksData) {
                    for (const [index, extrinsic] of blockToCheck.extrinsics.entries()) {
                        // Skip unsigned extrinsics, since no commission is paid
                        if (!extrinsic.isSigned) {
                            continue;
                        }

                        const events = blockToCheck.extrinsicIndexToEventsMap.get(`${index}`) || [];

                        // Get all fee paid events for the current extrinsic
                        const proxyRemovedEvents = events
                            .filter((event) => event.event.method === "ProxyRemoved" && event.event.section === "proxy")
                            .map((event) => event.event.data as unknown as { delegator: string });

                        if (!proxyRemovedEvents.length) {
                            continue;
                        }

                        for (const proxyRemovedEvent of proxyRemovedEvents) {
                            log(`Found "ProxyRemoved" event for block: ${blockToCheck.blockNum}. Checking...`);

                            // Gel proxies at previous block
                            const prevBlockHash = await api.rpc.chain.getBlockHash(blockToCheck.blockNum - 1);
                            const apiAtPrevBlock = await api.at(prevBlockHash);

                            const proxies = await apiAtPrevBlock.query.proxy.proxies(proxyRemovedEvent.delegator);
                            const proxiesLength = proxies.toJSON()[0].length;
                            const alreadyInProxyReserve = proxies[1].toBigInt();

                            const expectedAmount = totalForProxies(api, proxiesLength - 1);

                            const unreserved = events
                                .filter(
                                    (event) => event.event.method === "Unreserved" && event.event.section === "balances"
                                )
                                .map((event) => event.event.data as unknown as { amount: u128 });

                            expect(unreserved.length).toBeGreaterThan(0);
                            const actuallyUnreserved = unreserved[0].amount.toBigInt();
                            expect(
                                actuallyUnreserved,
                                `Block #${blockToCheck.blockNum}. Expecting actuallyUnreserved: ${actuallyUnreserved} to equal alreadyInProxyReserve - expectedAmount: ${alreadyInProxyReserve - expectedAmount}`
                            ).toEqual(alreadyInProxyReserve - expectedAmount);
                        }
                    }
                }
            },
        });
    },
});
