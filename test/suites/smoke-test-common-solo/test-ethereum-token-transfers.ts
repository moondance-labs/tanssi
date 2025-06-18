import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS, SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS } from "utils";

const SS58_FORMAT = 42;

let BLOCKS_AMOUNT_TO_CHECK = 100;
// For debug purposes only, specify block here to check it
const BLOCK_NUMBER_TO_DEBUG = undefined;

describeSuite({
    id: "SMOK15",
    title: "Ethereum token transfers smoke tests",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let sovereignAccount: string;

        beforeAll(async () => {
            api = context.polkadotJs();
            const runtimeName = api.runtimeVersion.specName.toString();
            const isStarlight = runtimeName === "starlight";
            sovereignAccount = isStarlight
                ? ETHEREUM_MAINNET_SOVEREIGN_ACCOUNT_ADDRESS
                : SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;

            sovereignAccount = encodeAddress(hexToU8a(sovereignAccount), SS58_FORMAT);
        });

        it({
            id: "C01",
            title: "Token transfer channels exists",
            test: async () => {
                const channels = await api.query.ethereumSystem.channels.entries();
                expect(channels.length).toBeGreaterThan(0);
            },
        });

        it({
            id: "C02",
            title: "Sovereign account collects amount when native token is transferred",
            test: async () => {
                // Go through the last BLOCKS_AMOUNT_TO_CHECK blocks and check if the sovereign account is collecting
                // the amount for each native token transfer event.
                let currentBlock = (await api.rpc.chain.getBlock()).block.header.number.toNumber();

                if (BLOCK_NUMBER_TO_DEBUG !== undefined) {
                    BLOCKS_AMOUNT_TO_CHECK = 1;
                    currentBlock = BLOCK_NUMBER_TO_DEBUG + 1;
                }

                for (let i = 1; i <= BLOCKS_AMOUNT_TO_CHECK; i++) {
                    const blockNumber = currentBlock - i;
                    process.stdout.write(`\rProcessing block [${blockNumber}]: ${i}/${BLOCKS_AMOUNT_TO_CHECK}`);

                    const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
                    const events = await api.query.system.events.at(blockHash);

                    const tokenTransferEvents = events.filter(({ event }) =>
                        api.events.ethereumTokenTransfers.NativeTokenTransferred.is(event)
                    );

                    const balanceTransferEvents = events.filter(
                        ({ event }) =>
                            api.events.balances?.Transfer?.is(event) || api.events.currencies?.Transferred?.is(event)
                    );

                    // Check sovereign is collecting the amount for each native token transfer event
                    for (const { event } of tokenTransferEvents) {
                        const nativeTokenTransferEvent = event.data;

                        const recipientReceived = balanceTransferEvents.some(({ event }) => {
                            const [from, to, amount] = event.data;
                            return (
                                from?.toString() === nativeTokenTransferEvent.source.toString() &&
                                to?.toString() === sovereignAccount &&
                                amount?.toString() === nativeTokenTransferEvent.amount.toString()
                            );
                        });

                        expect(
                            recipientReceived,
                            `Expected transfer to ${sovereignAccount} not found in block ${blockNumber}`
                        ).toBe(true);
                    }
                }
            },
        });
    },
});
