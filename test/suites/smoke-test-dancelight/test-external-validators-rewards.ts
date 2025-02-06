import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "SMOK05",
    title: "Smoke tests for external validators rewards pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Total points matches sum of individual points",
            test: async () => {
                const entries = await api.query.externalValidatorsRewards.rewardPointsForEra.entries();

                for (const [key, entry] of entries) {
                    let sum = 0;
                    for (const [, points] of entry.individual.entries()) {
                        sum += points.toNumber();
                    }
                    expect(sum).to.be.eq(entry.total.toNumber(), `inconsistency at key ${key}`);
                }
            },
        });
    },
});
