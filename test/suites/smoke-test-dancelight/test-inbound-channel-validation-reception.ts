import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { findEraBlockUsingBinarySearch, PRIMARY_GOVERNANCE_CHANNEL_ID } from "../../utils";

describeSuite({
    id: "SMOK10",
    title: "Inbound channel validator reception",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "External index is correctly updated",
            test: async () => {
                const bondedEras = (await api.query.externalValidatorSlashes.bondedEras()).toJSON() as [
                    [number, number, number],
                ];
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
                    expect(
                        (await apiAtLastBlockPreviousEra.query.externalValidators.currentExternalIndex()).toNumber()
                    ).toEqual(bondedEra[2]);
                }
            },
        });

        it({
            id: "C02",
            title: "Inbound primary channel should have increased the nonce",
            test: async () => {
                const blockDurationSec = 6;
                const minutesToCheck = 15; // Probably 60 minutes - too much, each block check takes around 500ms
                const oneHourDurationBlocksAmount = (minutesToCheck * 60) / blockDurationSec;
                const currentBlockNumber = (await api.rpc.chain.getHeader()).number.toNumber();

                for (let i = 0; i < oneHourDurationBlocksAmount; i++) {
                    const start = performance.now();
                    const blockToCheck = currentBlockNumber - i;
                    const apiAtBlock = await api.at(await api.rpc.chain.getBlockHash(blockToCheck));
                    const event = (await apiAtBlock.query.system.events()).find(
                        (event) => event.event.method === "ExternalValidatorsSet"
                    );

                    if (event) {
                        const apiAtPreviousBlock = await api.at(await api.rpc.chain.getBlockHash(blockToCheck - 1));
                        const previousBlockNonce =
                            await apiAtPreviousBlock.query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                        const currentBlockNonce =
                            await api.query.ethereumInboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                        // TODO: Find the block with this event and test
                        expect(previousBlockNonce.toBigInt() + 1n).toEqual(currentBlockNonce.toBigInt());
                    }
                }
            },
        });
    },
});
