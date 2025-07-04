import "@tanssi/api-augment";
import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession, jumpSessions } from "utils";

describeSuite({
    id: "DEV1001",
    title: "Dancebox: Inactivity tracking test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let keyring: Keyring;
        let daveAccountKey: KeyringPair;
        let ferdieAccountKey: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            keyring = new Keyring({ type: "sr25519" });
            daveAccountKey = keyring.addFromUri("//" + "Dave", { name: "COLLATOR" + " ACCOUNT" + "DAVE" });
            ferdieAccountKey = keyring.addFromUri("//" + "Ferdie", { name: "COLLATOR" + " ACCOUNT" + "FERDIE" });
        });
        it({
            id: "E01",
            title: "Pallet should correctly update collators' activity records with no inactive collators",
            test: async () => {
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                await jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // record orchestrator collators and one of the container chain's collators
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.size).to.be.equal(2);
                expect(activeCollatorsForSessionBeforeNoting.toHuman()).to.contain(alice.address);
                expect(activeCollatorsForSessionBeforeNoting.toHuman()).to.contain(context.keyring.charlie.address);

                // After noting the first block, the 2 container chain collators should be added to the activity tracking storage
                // for the current session
                await context.createBlock();
                const activeCollatorsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterNoting.toHuman()).to.deep.eq([
                    context.keyring.bob.address,
                    context.keyring.charlie.address,
                    alice.address,
                ]);
                const activeChainsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterNoting.toHuman()).to.deep.eq(["2,000"]);

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator and the chain again
                await context.createBlock();
                const activeCollatorsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterSecondNoting).to.deep.eq(activeCollatorsForSessionAfterNoting);
                const activeChainsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterSecondNoting).to.deep.eq(activeChainsForSessionAfterNoting);

                // Check that the collators are not added to the activity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityPeriod.isEmpty).to.be.true;

                // Check that the active collators are not added to the inactivity tracking storage for the current session
                // after the end of the session. Storage must be empty because all collators are active
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityPeriod.isEmpty).to.be.true;

                // After the end of activity period, the inactivity tracking storage for startSession should be empty
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const inactiveCollatorsRecordAfterActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordAfterActivityPeriod.isEmpty).to.be.true;
            },
        });

        it({
            id: "E02",
            title: "Pallet should correctly update collators' activity records with inactive chain",
            test: async () => {
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                const daveAccountId = polkadotJs.createType("AccountId", daveAccountKey.publicKey);
                const ferdieAccountId = polkadotJs.createType("AccountId", ferdieAccountKey.publicKey);

                // Registering 2 new collators so they appear as collators for chain 2001
                await jumpSessions(context, 4);
                const daveKey = await polkadotJs.rpc.author.rotateKeys();
                const ferdieKey = await polkadotJs.rpc.author.rotateKeys();
                await polkadotJs.tx.session.setKeys(daveKey, []).signAndSend(daveAccountKey);
                context.createBlock();
                await polkadotJs.tx.session.setKeys(ferdieKey, []).signAndSend(ferdieAccountKey);
                context.createBlock();

                await jumpSessions(context, 2);
                const addInvulnerablesDaveTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.invulnerables.addInvulnerable(daveAccountId)
                );
                const addInvulnerablesFerdieTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.invulnerables.addInvulnerable(ferdieAccountId)
                );
                await context.createBlock([await addInvulnerablesDaveTx.signAsync(alice)]);
                await context.createBlock([await addInvulnerablesFerdieTx.signAsync(alice)]);

                // Chain 2001 will be producing blocks and we want to disable that for this test
                const excludedChains = polkadotJs.createType("Vec<ParaId>", [2001]);
                await customDevRpcRequest("mock_excludeContainerChains", [excludedChains]);

                await jumpSessions(context, 3);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(collators.toJSON().containerChains["2001"]).to.deep.eq([
                    daveAccountKey.address,
                    ferdieAccountKey.address,
                ]);
                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await context.createBlock();
                const activeCollatorsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterNoting.toHuman()).to.deep.eq([
                    context.keyring.bob.address,
                    context.keyring.charlie.address,
                    alice.address,
                ]);
                const activeChainsForSessionAfterNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterNoting.toHuman()).to.deep.eq(["2,000"]);

                // If the same collator produces more than 1 block, the activity tracking storage
                // for the current session should not add the collator again
                await context.createBlock();
                const activeCollatorsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionAfterSecondNoting).to.deep.eq(activeCollatorsForSessionAfterNoting);
                const activeChainsForSessionAfterSecondNoting =
                    await polkadotJs.query.inactivityTracking.activeContainerChainsForCurrentSession();
                expect(activeChainsForSessionAfterSecondNoting).to.deep.eq(activeChainsForSessionAfterNoting);

                // Check that the collators are not added to the activity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityPeriod.isEmpty).to.be.true;

                // Check that the active collators are not added to the inactivity tracking storage after the end of the session.
                // Storge must be empty because all collators for chain 2000 are active and chain 2001 is inactive
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityPeriod.isEmpty).to.be.true;

                // After the end of activity period, the inactivity tracking storage for startSession should be empty
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const inactiveCollatorsRecordAfterActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordAfterActivityPeriod.isEmpty).to.be.true;
            },
        });
    },
});
