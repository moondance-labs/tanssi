import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { isStarlightRuntime } from "../../../utils/runtime.ts";

describeSuite({
    id: "DEVT1204",
    title: "Services payment RPC",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });
        it({
            id: "E01",
            title: "Services payment RPC",
            test: async () => {
                const isStarlight = isStarlightRuntime(polkadotJs);
                const costPerBlock = isStarlight ? 30_000_000_000n : 1_000_000n;
                const costPerSession = isStarlight ? 50_000_000_000_000n : 100_000_000n;

                const chainBlockCost = BigInt((await polkadotJs.call.servicesPaymentApi.blockCost(1000)).toString());
                const chainSessionCost = BigInt(
                    (await polkadotJs.call.servicesPaymentApi.collatorAssignmentCost(1000)).toString()
                );
                expect(costPerBlock).eq(chainBlockCost);
                expect(chainSessionCost).eq(costPerSession);
            },
        });
    },
});
