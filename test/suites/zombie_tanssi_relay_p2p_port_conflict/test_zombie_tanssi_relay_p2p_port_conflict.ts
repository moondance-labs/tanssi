// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "ZOMBIETANSS01",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it, context }) => {
        let relayApi: ApiPromise;

        beforeAll(async () => {
            relayApi = context.polkadotJs("Tanssi-relay");
        });

        it({
            id: "T01",
            title: "Test relay rpc (alice) has more than 0 peers",
            test: async () => {
                const peers = (await relayApi.rpc.system.peers()).toJSON();
                console.log(peers);
                expect(peers.length, "validators cannot connect to each other").toBeGreaterThan(0);
            },
        });
    },
});
