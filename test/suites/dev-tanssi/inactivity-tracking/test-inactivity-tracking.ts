import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { jumpToSession } from "utils";
import { u8aToHex } from "@polkadot/util";

describeSuite({
    id: "DEV1001",
    title: "Dancebox: Inactivity tracking test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let keyring: Keyring;
        let collatorNimbusKey: KeyringPair;
        let collatorAccountKey: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
            keyring = new Keyring({ type: "sr25519" });
            collatorNimbusKey = keyring.addFromUri("//" + "COLLATOR_NIMBUS", { name: "COLLATOR" + " NIMBUS" });
            // Collator key of Dave
            collatorAccountKey = keyring.addFromUri("//" + "Dave", { name: "COLLATOR" + " ACCOUNT" });
        });
        it({
            id: "E01",
            title: "Pallet should correctly update collators' activity records with no inactive collators",
            test: async () => {
                await context.createBlock(polkadotJs.tx.configuration.setMaxOrchestratorCollators(1).signAsync(alice));
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                jumpToSession(context, 2);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // record orchestrator collators
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.toHuman()).to.deep.eq([alice.address]);

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
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
                expect(activeCollatorsForSession2AfterSecondNoting).to.deep.eq(activeCollatorsForSession2AfterNoting);

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
            title: "Pallet should correctly update collators' activity records with one inactive collators",
            test: async () => {
                const maxInactiveSessions = polkadotJs.consts.inactivityTracking.maxInactiveSessions.toNumber();
                const nimbusPublicKey = collatorNimbusKey.publicKey;
                const collatorAccountId = context.polkadotJs().createType("AccountId", collatorAccountKey.publicKey);
                await polkadotJs.tx.session.setKeys(u8aToHex(nimbusPublicKey), []).signAndSend(collatorAccountKey);
                context.createBlock();
                const addInvulnerablesTx = polkadotJs.tx.sudo.sudo(
                    polkadotJs.tx.invulnerables.addInvulnerable(collatorAccountId)
                );
                await context.createBlock([await addInvulnerablesTx.signAsync(alice)]);

                jumpToSession(context, 3);
                const startSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                const collators = await polkadotJs.query.collatorAssignment.collatorContainerChain();
                expect(collators.toJSON().containerChains["2001"]).to.contain(collatorAccountId.toHuman());
                // No container chains has produced blocks yet so activity tracking storage for current session should
                // record orchestrator collators
                const activeCollatorsForSessionBeforeNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
                expect(activeCollatorsForSessionBeforeNoting.toHuman()).to.deep.eq([alice.address]);

                // After noting the first block, the collators should be added to the activity tracking storage
                // for the current session
                await context.createBlock();
                const activeCollatorsForSession2AfterNoting =
                    await polkadotJs.query.inactivityTracking.activeCollatorsForCurrentSession();
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
                expect(activeCollatorsForSession2AfterSecondNoting).to.deep.eq(activeCollatorsForSession2AfterNoting);

                // Check that the collators are not added to the activity tracking storage for the current session
                // before the end of the session
                const inactiveCollatorsRecordBeforeActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordBeforeActivityPeriod.isEmpty).to.be.true;

                // Check that the active collators are not added to the inactivity tracking storage for the current session
                // after the end of the session. Storge must contain only collatorAccountId because it is inactive
                await jumpToSession(context, startSession + 1);
                const inactiveCollatorsRecordWithinActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordWithinActivityPeriod.toHuman()).to.deep.eq([collatorAccountId]);

                // After the end of activity period, the inactivity tracking storage for startSession should be empty
                await jumpToSession(context, maxInactiveSessions + startSession + 1);
                const inactiveCollatorsRecordAfterActivityPeriod =
                    await polkadotJs.query.inactivityTracking.inactiveCollators(startSession);
                expect(inactiveCollatorsRecordAfterActivityPeriod.isEmpty).to.be.true;
            },
        });
    },
});
