import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { getBlockTime } from "@moonwall/util";

import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S03",
    title: "Sample suite that only runs on Dancebox chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Parachain blocks should be finalized",
            test: async function () {
                const head = await api.rpc.chain.getFinalizedHead();
                const block = await api.rpc.chain.getBlock(head);
                const diff = Date.now() - getBlockTime(block);
                log(`Last finalized block was ${diff / 1000} seconds ago`);
                expect(diff).to.be.lessThanOrEqual(10 * 60 * 1000); // 10 minutes in milliseconds
                expect(api.consts.system.version.specVersion.toNumber()).to.be.greaterThan(0);
            },
        });
    },
});
