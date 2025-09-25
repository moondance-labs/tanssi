import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { isStarlightRuntime } from "../../utils";

describeSuite({
    id: "S11",
    title: "Verify payment costs",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C100",
            title: "should have value > 0",
            test: async () => {
                const isStarlight = isStarlightRuntime(api);
                const runtimeVersion = api.runtimeVersion.specVersion.toNumber();
                const costPerBlock = isStarlight && runtimeVersion >= 1500 ? 30_000_000_000n : 1_000_000n;
                const costPerSession = isStarlight && runtimeVersion >= 1500 ? 50_000_000_000_000n : 100_000_000n;

                const chainBlockCost = BigInt((await api.call.servicesPaymentApi.blockCost(1000)).toString());
                const chainSessionCost = BigInt(
                    (await api.call.servicesPaymentApi.collatorAssignmentCost(1000)).toString()
                );
                expect(costPerBlock).eq(chainBlockCost);
                expect(chainSessionCost).eq(costPerSession);
            },
        });
    },
});
