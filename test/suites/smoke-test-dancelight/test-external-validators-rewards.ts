import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import {
    HOLESKY_SOVEREIGN_ACCOUNT_ADDRESS,
    PRIMARY_GOVERNANCE_CHANNEL_ID,
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    getAccountBalance,
    getCurrentEraStartBlock,
} from "utils";

describeSuite({
    id: "SMOK05",
    title: "Smoke tests for external validators rewards pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let runtimeVersion: number;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
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
                // Checkpoint B: the block number of current epoch start
                const blockNumberCheckpointB = await getCurrentEraStartBlock(api);
                // Checkpoint A: the block number before Checkpoint B
                const blockNumberCheckpointA = blockNumberCheckpointB - 1;

                const apiAtCheckpointA = await api.at(await api.rpc.chain.getBlockHash(blockNumberCheckpointA));
                const apiAtCheckpointB = await api.at(await api.rpc.chain.getBlockHash(blockNumberCheckpointB));

                const sovereignAccount =
                    runtimeVersion > 1101 ? SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS : HOLESKY_SOVEREIGN_ACCOUNT_ADDRESS;

                const sovereignBalanceCheckpointB = await getAccountBalance(apiAtCheckpointB, sovereignAccount);
                const sovereignBalanceCheckpointA = await getAccountBalance(apiAtCheckpointA, sovereignAccount);

                const event = (await apiAtCheckpointB.query.system.events()).find(
                    (event) => event.event.method === "RewardsMessageSent"
                );

                const checkpointAPrimaryChannelNonce =
                    await apiAtCheckpointA.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                const checkpointBPrimaryChannelNonce =
                    await api.query.ethereumOutboundQueue.nonce(PRIMARY_GOVERNANCE_CHANNEL_ID);

                const nonceDiff = checkpointBPrimaryChannelNonce.toNumber() - checkpointAPrimaryChannelNonce.toNumber();

                // The event is triggered, nonce should be incremented
                if (event) {
                    expect(nonceDiff).toEqual(1);
                    expect(sovereignBalanceCheckpointA.toNumber()).to.be.lessThan(
                        sovereignBalanceCheckpointB.toNumber()
                    );

                    // The event is not triggered, nonce should be the same
                } else {
                    expect(nonceDiff).toEqual(0);
                }
            },
        });

        it({
            id: "C03",
            title: "Check if RewardPointsForEra expires after HistoryDepth",
            test: async ({ skip }) => {
                const historyDepth = api.consts.externalValidatorsRewards.historyDepth.toNumber();
                const currentEra = (await api.query.externalValidators.activeEra()).unwrap().index.toNumber();

                // Checkpoint A: current era index - historyDepth
                const eraIndexCheckpointA = currentEra - historyDepth;
                // Checkpoint B: eraIndexCheckpointA + 1
                const eraIndexCheckpointB = eraIndexCheckpointA + 1;

                if (eraIndexCheckpointA < 0) {
                    log("Current era is less than historyDepth, skipping the test");
                    skip();
                }

                // The mapping only contains the keys that are in `externalValidatorsRewards`
                const rewardMappingKeys = (await api.query.externalValidatorsRewards.rewardPointsForEra.keys()).map(
                    (key) => key.args[0].toNumber()
                );
                expect(rewardMappingKeys.includes(eraIndexCheckpointB)).toBe(true);
                expect(rewardMappingKeys.includes(eraIndexCheckpointA)).toBe(false);
            },
        });
    },
});
