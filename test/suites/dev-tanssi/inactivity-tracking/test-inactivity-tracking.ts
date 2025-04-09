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
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.size).to.be.equal(1);

                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterNoting.size).to.be.equal(3);

                await context.createBlock();
                const activeCollatorsForSession2AfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSession2AfterSecondNoting.size).to.be.equal(3);

                const activeCollatorsRecordBeforeActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(2);
                expect(activeCollatorsRecordBeforeActivityWindow.isEmpty).to.be.true;
                await jumpToSession(context, 3);
                const activeCollatorsRecordWithinActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(2);
                expect(activeCollatorsRecordWithinActivityWindow.size).to.be.equal(3);

                await jumpToSession(context, maxInactiveSessions + 3);
                const activeCollatorsRecordAfterActivityWindow =
                    await polkadotJs.query.inactivityTracking.activeCollators(2);
                expect(activeCollatorsRecordAfterActivityWindow.isEmpty).to.be.true;
            },
        });
    },
});
