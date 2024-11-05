import { beforeAll, describeSuite, expect } from "@moonwall/cli";

import { ApiPromise } from "@polkadot/api";
import { fetchWithdrawnAmount, fetchDepositedAmount } from "util/block";

describeSuite({
    id: "S05",
    title: "Sample suite that runs on Dancebox and Dancelight chains",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;

        beforeAll(() => {
            api = context.polkadotJs();
        });

        it({
            id: "C03",
            title: "Supply variance is correct",
            test: async function () {
                const latestBlock = await api.rpc.chain.getBlock();

                const latestBlockHash = latestBlock.block.hash;
                const latestParentBlockHash = latestBlock.block.header.parentHash;
                const apiAtIssuanceAfter = await api.at(latestBlockHash);
                const apiAtIssuanceBefore = await api.at(latestParentBlockHash);

                const supplyBefore = (await apiAtIssuanceBefore.query.balances.totalIssuance()).toBigInt();

                const events = await apiAtIssuanceAfter.query.system.events();

                const withdrawnAmount = await fetchWithdrawnAmount(events);
                const depositAmount = await fetchDepositedAmount(events);

                const supplyAfter = (await apiAtIssuanceAfter.query.balances.totalIssuance()).toBigInt();
                expect(supplyAfter).to.equal(supplyBefore + depositAmount - withdrawnAmount);
            },
        });
    },
});
