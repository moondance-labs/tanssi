import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import { type ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT2001",
    title: "Filter calls test",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Filter Balances call",
            test: async () => {
                const specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
                if (specVersion > 1300) {
                    // TODO: check balances call are filtered
                }
            },
        });
    },
});
