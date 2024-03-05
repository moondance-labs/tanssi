import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { paraIdTank } from "util/payment";

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
            blocksPerSession = chain == "Dancebox" ? 300n : 5n;
        });

        it({
            id: "C01",
            title: "All scheduled parachains should be able to pay for at least 2 sessions",
            test: async function () {
                if (runtimeVersion < 500) {
                    return;
                }
                const existentialDeposit = await api.consts.balances.existentialDeposit.toBigInt();

                // If they have collators scheduled, they should have at least enough money to pay
                let pending = await api.query.collatorAssignment.pendingCollatorContainerChain();
                if (pending.isNone) {
                    pending = await api.query.collatorAssignment.collatorContainerChain();
                }
                if (pending["containerChains"] != undefined) {
                    for (const container of Object.keys(pending.toJSON()["containerChains"])) {
                        const freeBlockCredits = (await api.query.servicesPayment.blockProductionCredits(container))
                            .unwrap()
                            .toBigInt();

                        const freeSessionCredits = (
                            await api.query.servicesPayment.collatorAssignmentCredits(container)
                        )
                            .unwrap()
                            .toBigInt();

                        // We need, combined, at least credits for 2 session coverage + blocks
                        const neededBlockPaymentAfterCredits =
                            2n * blocksPerSession - freeBlockCredits < 0n
                                ? 0n
                                : 2n * blocksPerSession - freeBlockCredits;
                        const neededCollatorAssignmentPaymentAfterCredits =
                            2n - freeSessionCredits < 0n ? 0n : 2n - freeSessionCredits;

                        if (neededBlockPaymentAfterCredits > 0n || neededCollatorAssignmentPaymentAfterCredits > 0n) {
                            const neededTankMoney =
                                existentialDeposit +
                                neededCollatorAssignmentPaymentAfterCredits * costPerSession +
                                neededBlockPaymentAfterCredits * costPerBlock;
                            const tankBalance = (
                                await api.query.system.account(paraIdTank(container))
                            ).data.free.toBigInt();

                            expect(
                                tankBalance,
                                `Container chain ${container} was assigned collators without having a way to pay for it`
                            ).toBeGreaterThanOrEqual(neededTankMoney);
                        }
                    }
                }
            },
        });
    },
});
