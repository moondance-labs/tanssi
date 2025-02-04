import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "util/block";

describeSuite({
    id: "DT0802",
    title: "Collator assignment tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
        });

        it({
            id: "E01",
            title: "Collator should rotate",
            test: async () => {
                const fullRotationPeriod = (await context.polkadotJs().query.configuration.activeConfig())[
                    "fullRotationPeriod"
                ].toString();
                const maxCollators = (await context.polkadotJs().query.configuration.activeConfig())[
                    "maxCollators"
                ].toNumber();
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                // Calculate the remaining sessions for next full rotation
                // This is a workaround for running moonwall in run mode
                // as it runs all tests in the same chain instance
                const remainingSessionsForRotation =
                    sessionIndex > fullRotationPeriod ? sessionIndex % fullRotationPeriod : fullRotationPeriod;

                await jumpToSession(context, remainingSessionsForRotation - 2);

                const initialAssignment = (await polkadotJs.query.collatorAssignment.collatorContainerChain()).toJSON();
                // Count the number of assigned collators
                const countAssignedCollators = (obj): number => {
                    let totalLength = obj.orchestratorChain.length;

                    for (const chainArray of Object.values(obj.containerChains)) {
                        totalLength += chainArray.length;
                    }

                    return totalLength;
                };
                const assignedCollators = countAssignedCollators(initialAssignment);
                // Convert Perbill to number, divinding by 10**9
                const collatorFullness =
                    (await polkadotJs.query.collatorAssignment.collatorFullnessRatio()).toJSON() / 10 ** 9;
                // Test float equality allowing for a small error
                const expectedRatio = assignedCollators / maxCollators;
                const epsilon = 0.000001;
                expect(collatorFullness).to.be.within(expectedRatio - epsilon, expectedRatio + epsilon);
            },
        });
    },
});
