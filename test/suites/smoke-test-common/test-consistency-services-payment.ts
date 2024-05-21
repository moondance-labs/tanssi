import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { hasEnoughCredits } from "util/payment";

describeSuite({
    id: "S09",
    title: "Check services payment consistency",
    foundationMethods: "read_only",
    testCases: ({ it, context }) => {
        let api: ApiPromise;
        let runtimeVersion;
        const costPerSession = 100_000_000n;
        const costPerBlock = 1_000_000n;
        let blocksPerSession;

        beforeAll(async () => {
            api = context.polkadotJs();
            runtimeVersion = api.runtimeVersion.specVersion.toNumber();
            const chain = api.consts.system.version.specName.toString();
            blocksPerSession = chain == "Dancebox" ? 600n : 50n;
        });

        it({
            id: "C01",
            title: "All scheduled parachains should be able to pay for at least 2 sessions",
            test: async function () {
                if (runtimeVersion < 500) {
                    return;
                }

                // If they have collators scheduled, they should have at least enough money to pay
                let pending = await api.query.collatorAssignment.pendingCollatorContainerChain();
                if (pending.isNone) {
                    pending = await api.query.collatorAssignment.collatorContainerChain();
                }
                if (pending["containerChains"] != undefined) {
                    for (const container of Object.keys(pending.toJSON()["containerChains"])) {
                        expect(
                            await hasEnoughCredits(api, container, blocksPerSession, 2n, costPerSession, costPerBlock),
                            `Container chain ${container} was assigned collators without having a way to pay for it`
                        ).toBe(true);
                    }
                }
            },
        });
    },
});
