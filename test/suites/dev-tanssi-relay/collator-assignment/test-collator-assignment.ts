import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions, jumpToSession } from "util/block";

describeSuite({
    id: "DTR0301",
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
            test: async function () {
                const fullRotationPeriod = (await context.polkadotJs().query.collatorConfiguration.activeConfig())[
                    "fullRotationPeriod"
                ].toString();
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                const remainingSessionsForRotation =
                    sessionIndex > fullRotationPeriod ? sessionIndex % fullRotationPeriod : fullRotationPeriod;

                await jumpToSession(context, remainingSessionsForRotation - 2);

                const initialAssignment = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();

                expect(initialAssignment.containerChains[2000].length).to.eq(2);
                expect((await polkadotJs.query.tanssiCollatorAssignment.pendingCollatorContainerChain()).isNone);

                // remainingSessionsForRotation - 1
                await jumpSessions(context, 1);
                const minusOneAssignment = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();

                expect((await polkadotJs.query.tanssiCollatorAssignment.pendingCollatorContainerChain()).isSome);
                // Assignment shouldn't have changed yet
                expect(initialAssignment.containerChains[2000].toSorted()).to.deep.eq(
                    minusOneAssignment.containerChains[2000].toSorted()
                );

                // remainingSessionsForRotation
                await jumpSessions(context, 1);

                const finalAssignment = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();
                // Assignment should have changed
                expect(initialAssignment.containerChains[2000].toSorted()).to.not.deep.eq(
                    finalAssignment.containerChains[2000].toSorted()
                );
            },
        });
    },
});
