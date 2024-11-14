import "@tanssi/api-augment";
import { ApiDecoration } from "@polkadot/api/types";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "S16",
    title: "Verify data preservers consistency",
    foundationMethods: "read_only",
    testCases: ({ context, it, log }) => {
        let atBlockNumber: number = 0;
        let apiAt: ApiDecoration<"promise">;
        let paraApi: ApiPromise;

        beforeAll(async function () {
            paraApi = context.polkadotJs("para");
            atBlockNumber = (await paraApi.rpc.chain.getHeader()).number.toNumber();
            apiAt = await paraApi.at(await paraApi.rpc.chain.getBlockHash(atBlockNumber));
        });

        it({
            id: "C01",
            title: "all profiles should have a deposit of either 0 or value fixed in the runtime",
            test: async function () {
                // Add more if we change ProfileDeposit value. Keep previous values for profiles
                // created before the change.
                const validDeposits = [0, 11330000000000];

                const entries = await paraApi.query.dataPreservers.profiles.entries();

                for (const [key, entry] of entries) {
                    expect(validDeposits.includes(entry.deposit));
                }
            },
        });
    },
});
