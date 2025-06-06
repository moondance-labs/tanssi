import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession, mockAndInsertHeadData } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_INACTIVITY_TRACKING } from "helpers";

describeSuite({
    id: "DEVT2201",
    title: "Dancelight: Inactivity tracking test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightIT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightIT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_INACTIVITY_TRACKING.includes(specVersion);
        });
        it({
            id: "E01",
            title: "Pallet should correctly update collators' activity records",
            test: async () => {
                if (shouldSkipStarlightIT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    // TODO: once the pallet is in starlight, check the calls are filtered,
                    // in case we don't want them for a specific runtime version.
                    return;
                }
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                await jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // be empty
                const activeCollatorsForSession2BeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2BeforeNoting.isEmpty).to.be.true;

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await mockAndInsertHeadData(context, 2000, 2, 2, alice);
                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterNoting.size).to.be.equal(1);
                const activeCollatorAddress = activeCollatorsForSession2AfterNoting.toHuman()[0];

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator again
                await mockAndInsertHeadData(context, 2000, 3, 2, alice);
                await context.createBlock();
                const activeCollatorsForSession2AfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterSecondNoting.toHuman()).to.deep.eq([activeCollatorAddress]);

                // Check that no collators are added to the inactivity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityWindow.isEmpty).to.be.true;

                // Check that the activeCollatorAddress is not added to the inactivity tracking storage for the current session
                // after the end of the session.
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityWindow.isEmpty).to.be.false;
                expect(inactiveCollatorsRecordWithinActivityWindow.toHuman()).not.to.contain(activeCollatorAddress);

                // After the end of activity period, the inactivity tracking storage for startSession should be empty
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const inactiveCollatorsRecordAfterActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordAfterActivityWindow.isEmpty).to.be.true;
            },
        });
    },
});
