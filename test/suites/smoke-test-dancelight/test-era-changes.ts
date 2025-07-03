import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { getCurrentEraStartBlock, getPastEraStartBlock } from "utils/block";

describeSuite({
    id: "SMOKD02",
    title: "Era changes suit that only runs on Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;
        let currentEraStartBlock: number;
        let pastEraStartBlock: number;

        beforeAll(async () => {
            api = context.polkadotJs();
            currentEraStartBlock = await getCurrentEraStartBlock(api);
            pastEraStartBlock = await getPastEraStartBlock(api, currentEraStartBlock - 1);
        });

        it({
            id: "C01",
            title: "Era changes are happening as expected and information is correct",
            test: async () => {
                const sessionsPerEra = api.consts.externalValidators.sessionsPerEra.toNumber();
                const apiAtCurrentEraStart = await api.at(await api.rpc.chain.getBlockHash(currentEraStartBlock));
                const currentEraStartSessionIndex = (
                    await apiAtCurrentEraStart.query.session.currentIndex()
                ).toNumber();
                const apiAtPreviousEraStart = await api.at(await api.rpc.chain.getBlockHash(pastEraStartBlock));
                const previousEraStartSessionIndex = (
                    await apiAtPreviousEraStart.query.session.currentIndex()
                ).toNumber();

                expect(previousEraStartSessionIndex + sessionsPerEra).toEqual(currentEraStartSessionIndex);
            },
        });
    },
});
