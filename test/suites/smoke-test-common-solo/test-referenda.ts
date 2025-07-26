import "@tanssi/api-augment/dancelight";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";

const AMOUNT_RECENT_PROPOSALS_FOR_CHECKING = 10;

/*
 * The goal of this smoke test is to ensure that the decision deposit for referenda proposals is set.
 * If the deposit is not set, the proposal will be in Preparing state until UndecidingTimeout passed (14 days).
 * Then it will be rejected with event: referenda.TimedOut.
 */
describeSuite({
    id: "SMOK17",
    title: "Smoke tests for referenda",
    foundationMethods: "read_only",
    testCases: ({ it, context, log }) => {
        let api: ApiPromise;

        beforeAll(async () => {
            api = context.polkadotJs();
        });

        it({
            id: "C01",
            title: "Referenda deposit does exist for recent proposals",
            test: async () => {
                const referendumCount = (await api.query.referenda.referendumCount()).toNumber();
                const startIndex = Math.max(0, referendumCount - AMOUNT_RECENT_PROPOSALS_FOR_CHECKING);

                for (let i = startIndex; i < referendumCount; i++) {
                    const infoOpt = await api.query.referenda.referendumInfoFor(i);
                    if (infoOpt.isSome) {
                        const info = infoOpt.unwrap();
                        if (info.isOngoing) {
                            const decisionDeposit = info.asOngoing.decisionDeposit;
                            const unwrappedDecisionDeposit = decisionDeposit.unwrapOr(null);

                            expect(
                                unwrappedDecisionDeposit,
                                `Expecting decision deposit is not null for proposal with index ${i}`
                            ).not.toBeNull();
                        }
                    }
                }
            },
        });
    },
});
