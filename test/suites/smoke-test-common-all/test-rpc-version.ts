import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S16",
    title: "Ensure RPC version is not a dev version",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C100",
            title: "should not contain dev version",
            test: async () => {
                const rpcVersion = (await api.rpc.system.version()).toString();

                expect(rpcVersion).to.not.contain("-dev");
                // Version should be something like 0.14.0-77200a65234
                expect(rpcVersion).to.match(/^\d+\.\d+\.\d+-[a-z0-9]+$/);
            },
        });
    },
});
