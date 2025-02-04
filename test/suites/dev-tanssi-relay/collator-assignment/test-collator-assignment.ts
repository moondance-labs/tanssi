import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { jumpBlocks, jumpSessions, jumpToSession } from "util/block";
import { filterAndApply } from "@moonwall/util";
import type { EventRecord } from "@polkadot/types/interfaces";
import type { bool, u32, u8, Vec } from "@polkadot/types-codec";

describeSuite({
    id: "DEVT0301",
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
                const fullRotationPeriod = (await context.polkadotJs().query.collatorConfiguration.activeConfig())[
                    "fullRotationPeriod"
                ].toString();
                const sessionIndex = (await polkadotJs.query.session.currentIndex()).toNumber();
                // Calculate the remaining sessions for next full rotation
                // This is a workaround for running moonwall in run mode
                // as it runs all tests in the same chain instance
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
                const rotationEndAssignment = (
                    await polkadotJs.query.tanssiCollatorAssignment.collatorContainerChain()
                ).toJSON();

                expect((await polkadotJs.query.tanssiCollatorAssignment.pendingCollatorContainerChain()).isSome);
                // Assignment shouldn't have changed yet
                expect(initialAssignment.containerChains[2000].toSorted()).to.deep.eq(
                    rotationEndAssignment.containerChains[2000].toSorted()
                );

                // As randomness isn't deterministic in dancelight we can't be
                // 100% certain that the assignation will indeed change. So the
                // best we can do is verify that the pending rotation event for
                // next session is emitted and is a full rotation as expected
                const events = await polkadotJs.query.system.events();
                const filteredEvents = filterAndApply(
                    events,
                    "tanssiCollatorAssignment",
                    ["NewPendingAssignment"],
                    ({ event }: EventRecord) =>
                        event.data as unknown as { randomSeed: Vec<u8>; fullRotation: bool; targetSession: u32 }
                );
                expect(filteredEvents[0].fullRotation.toJSON()).toBe(true);

                // Check that the randomness is set in CollatorAssignment the
                // block previous to the full rotation
                const sessionDuration = await polkadotJs.consts.babe.epochDuration.toNumber();
                await jumpBlocks(context, sessionDuration - 1);
                const assignmentRandomness = await polkadotJs.query.tanssiCollatorAssignment.randomness();
                expect(assignmentRandomness.isEmpty).toBe(false);
            },
        });
    },
});
