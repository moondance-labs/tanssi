import "@tanssi/api-augment/dancelight";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { PRIMARY_GOVERNANCE_CHANNEL_ID } from "../../util/constants.ts";

describeSuite({
    id: "SMOK05",
    title: "Smoke tests for external validators rewards pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Total points matches sum of individual points",
            test: async () => {
                const entries = await api.query.externalValidatorsRewards.rewardPointsForEra.entries();

                log(`Checking rewards for ${entries.length} validators...`);
                const failures = entries
                    .map(([key, entry]) => {
                        const sum = [...entry.individual.entries()].reduce(
                            (acc, [key, points]) => acc + points.toNumber(),
                            0
                        );
                        const failed = sum !== entry.total.toNumber();
                        return { failed, key: key.toHex() };
                    })
                    .filter(({ failed }) => failed);

                for (const failed of failures) {
                    console.error(`inconsistency at key ${failed.key}`);
                }

                expect(failures.length).to.be.eq(0);
            },
        });

        it({
            id: "C02",
            title: "Check if message with rewards is sent in the end of the era and nonce is incremented",
            test: async () => {
                // Checkpoint B - the block number of current epoch start
                const currentEpochStartBlockNumber = (await api.query.babe.epochStart())[0].toNumber();
                // Checkpoint A - the block number before Checkpoint B
                const beforeCurrentEpochStartBlockNumber = currentEpochStartBlockNumber - 1;

                const apiAtCheckpointA = await api.at(await api.rpc.chain.getBlockHash(beforeCurrentEpochStartBlockNumber));
                const apiAtCheckpointB = await api.at(await api.rpc.chain.getBlockHash(currentEpochStartBlockNumber));

                const event = (await apiAtCheckpointB.query.system.events()).find(event => event.event.method === 'RewardsMessageSent');

                const checkpointAPrimaryChannelNonce =
                    await apiAtCheckpointA.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                const checkpointBPrimaryChannelNonce =
                    await api.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                const nonceDiff = checkpointBPrimaryChannelNonce.toNumber()  - checkpointAPrimaryChannelNonce.toNumber();

                // The event is triggered, nonce should be incremented
                if (event) {
                    expect(nonceDiff).toEqual(1);

                // The event is not triggered, nonce should be the same
                } else {
                    expect(nonceDiff).toEqual(0);
                }
            },
        });
    },
});
