import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "utils";

describeSuite({
    id: "DEV1001",
    title: "Dancebox: Inactivity tracking test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            await context.createBlock(polkadotJs.tx.configuration.setMaxOrchestratorCollators(1).signAsync(alice));
        });
        it({
            id: "E01",
            title: "Pallet should correctly update collators' activity records",
            test: async () => {
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // record orchestrator collators
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.size).to.be.equal(1);
                expect(activeCollatorsForSessionBeforeNoting.toHuman()[0]).to.be.equal(alice.address);

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterNoting.size).to.be.equal(3);
                expect(activeCollatorsForSession2AfterNoting.toHuman()).to.deep.eq([
                    context.keyring.bob.address,
                    context.keyring.charlie.address,
                    alice.address,
                ]);

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator again
                await context.createBlock();
                const activeCollatorsForSession2AfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterSecondNoting.size).to.be.equal(3);
                expect(activeCollatorsForSession2AfterSecondNoting).to.deep.eq(activeCollatorsForSession2AfterNoting);

                // Check that the collators are not added to the activity tracking storage for the current session
                // before the end of the session
                const activeCollatorsRecordBeforeActivityPeriod =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordBeforeActivityPeriod.isEmpty).to.be.true;

                // Check that the collators are added to the activity tracking storage for the current session
                // after the end of the session
                await jumpToSession(context, startSession + 1);
                const activeCollatorsRecordWithinActivityPeriod =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordWithinActivityPeriod.size).to.be.equal(3);
                expect(activeCollatorsRecordWithinActivityPeriod).to.deep.eq(activeCollatorsForSession2AfterNoting);

                // After the end of activity period, the collators should be removed from the activity tracking storage
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const activeCollatorsRecordAfterActivityPeriod =
                    await polkadotJs.query.inactivityTracking.activeCollators(startSession);
                expect(activeCollatorsRecordAfterActivityPeriod.isEmpty).to.be.true;
            },
        });
    },
});
