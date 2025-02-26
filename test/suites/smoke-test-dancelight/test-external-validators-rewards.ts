import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

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
    },
});
