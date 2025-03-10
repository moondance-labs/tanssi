import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S22",
    title: "Smoke tests for external validators rewards pallet",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion: number;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
        });

        it({
            id: "C01",
            title: "Total points matches sum of individual points",
            test: async function () {
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
