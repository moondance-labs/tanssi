import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { fetchIssuance } from "util/block";

describeSuite({
    id: "CT0302",
    title: "Issuance reward test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });
        it({
            id: "E01",
            title: "Issuance is the correct percentage",
            test: async function () {
                const supplyBefore = (await polkadotJs.query.balances.totalIssuance()).toBigInt();
                await context.createBlock();

                const events = await polkadotJs.query.system.events();

                const issuance = await fetchIssuance(events).amount.toBigInt();

                const supplyAfter = (await polkadotJs.query.balances.totalIssuance()).toBigInt();

                // in dev mode is 1%
                const expectedIssuanceIncrement = supplyBefore / 100n;
                expect(issuance).to.equal(expectedIssuanceIncrement);
                expect(supplyAfter).to.equal(supplyBefore + expectedIssuanceIncrement);
            },
        });
    },
});
