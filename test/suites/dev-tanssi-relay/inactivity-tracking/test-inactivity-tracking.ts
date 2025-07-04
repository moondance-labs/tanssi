import "@tanssi/api-augment";
import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession, mockAndInsertHeadData } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_INACTIVITY_TRACKING } from "helpers";

describeSuite({
    id: "DEVT2001",
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
            title: "Pallet should correctly update collators' activity records with active chains",
            test: async () => {
                if (shouldSkipStarlightIT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    // TODO: once the pallet is in starlight, check the calls are filtered,
                    // in case we don't want them for a specific runtime version.
                    return;
                }
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                const paraId = polkadotJs.createType("ParaId", 2000);
                await jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // No container chains has produced blocks yet so activity tracking storages for current session should
                // be empty
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.isEmpty).to.be.true;
                const activeChainsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionBeforeNoting.isEmpty).to.be.true;

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session and the chains will be added to chain activity tracking storage
                // for the current session
                await mockAndInsertHeadData(context, paraId, 2, 2, alice);
                await context.createBlock();
                const activeCollatorsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterNoting.size).to.be.equal(1);
                const activeCollatorAddress = activeCollatorsForSessionAfterNoting.toHuman()[0];
                const activeChainsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterNoting.toHuman()).to.deep.eq([paraId.toHuman()]);

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator and the chain again.
                await mockAndInsertHeadData(context, paraId, 3, 2, alice);
                await context.createBlock();
                const activeCollatorsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterSecondNoting.toHuman()).to.deep.eq([activeCollatorAddress]);
                const activeChainsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterSecondNoting).to.deep.eq(activeChainsForSessionAfterNoting);

                // Check that no collators are added to the inactivity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityWindow.isEmpty).to.be.true;

                // Check that the activeCollatorAddress is not added to the inactivity tracking storage for the current session
                // after the end of the session and the current session tracking storages are empty
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityWindow.isEmpty).to.be.false;
                expect(inactiveCollatorsRecordWithinActivityWindow.toHuman()).not.to.contain(activeCollatorAddress);
                const activeCollatorsForSessionOnNewSession =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionOnNewSession.isEmpty).to.be.true;
                const activeChainsForSessionOnNewSession =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionOnNewSession.isEmpty).to.be.true;

                // Since chain 2000 has been inactive for startSession + 1, collators assigned to it  should
                // not be added to the inactivity tracking storage after its end
                await jumpToSession(context, startSession + 2);
                const inactiveCollatorsRecordWithInactiveChain =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession + 1);
                expect(inactiveCollatorsRecordWithInactiveChain.isEmpty).to.be.true;

                // After the end of activity period, the inactivity tracking storage for startSession should be empty
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const inactiveCollatorsRecordAfterActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordAfterActivityWindow.isEmpty).to.be.true;
            },
        });
        it({
            id: "E02",
            title: "Pallet should correctly update collators' activity records with inactive chain",
            test: async () => {
                if (shouldSkipStarlightIT) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    // TODO: once the pallet is in starlight, check the calls are filtered,
                    // in case we don't want them for a specific runtime version.
                    return;
                }
                const paraId = polkadotJs.createType("ParaId", 2000);
                await jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // Disable chain 2000 from producing blocks and we want to disable that for this test
                const excludedChains = polkadotJs.createType("Vec<ParaId>", [paraId]);
                await customDevRpcRequest("mock_excludeContainerChains", [excludedChains]);
                // No container chains has produced blocks yet so activity tracking storages for current session should
                // be empty
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.isEmpty).to.be.true;
                const activeChainsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionBeforeNoting.isEmpty).to.be.true;

                // Since chain 2000 is disabled, the collators should not be added to the activity tracking storage
                // the chain will not be added to chain activity tracking storage for the current session
                await mockAndInsertHeadData(context, paraId, 2, 2, alice);
                await context.createBlock();
                const activeCollatorsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterNoting.isEmpty).to.be.true;
                const activeChainsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterNoting.isEmpty).to.be.true;

                // Check again if chain 2000 is disabled
                await mockAndInsertHeadData(context, paraId, 3, 2, alice);
                await context.createBlock();
                const activeCollatorsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterNoting.isEmpty).to.be.true;
                const activeChainsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterSecondNoting.isEmpty).to.be.true;

                // Check that no collators are added to the inactivity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityWindow.isEmpty).to.be.true;

                // Since chain 2000 is disabled, the collators should not be added to the inactivity tracking storage
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityWindow =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityWindow.isEmpty).to.be.true;
            },
        });
    },
});
