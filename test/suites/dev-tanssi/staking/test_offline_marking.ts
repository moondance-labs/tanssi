import "@tanssi/api-augment";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { jumpToSession } from "utils";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_INACTIVITY_TRACKING } from "helpers";
import type { AccountId32 } from "@polkadot/types/interfaces";
import { numberToHex } from "@polkadot/util";

describeSuite({
    id: "DEV0807",
    title: "Offline marking extriniscs test suite",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let bob: KeyringPair;
        let bobAccountId: AccountId32;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightIT: boolean;

        async function isAddressInAssignedCollators(address: string) {
            const assignedCollatorsRecords = await polkadotJs.query.collatorAssignment.collatorContainerChain();
            for (const collator of assignedCollatorsRecords.orchestratorChain.entries()) {
                if (collator.toString() === address) {
                    return true;
                }
            }
            for (const [_containerChain, collators] of assignedCollatorsRecords.containerChains.entries()) {
                for (const collator of collators) {
                    if (collator.toString() === address) {
                        return true;
                    }
                }
            }
            return false;
        }

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            // Check if the runtime is Starlight and set the spec version
            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightIT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_INACTIVITY_TRACKING.includes(specVersion);
            if (shouldSkipStarlightIT) {
                return;
            }

            alice = context.keyring.alice;
            bob = context.keyring.bob;
            bobAccountId = polkadotJs.createType("AccountId32", bob.publicKey);

            // Add Alice and Bob keys to pallet session. In dancebox they are already there in genesis.
            const newKey1 = await polkadotJs.rpc.author.rotateKeys();
            const newKey2 = await polkadotJs.rpc.author.rotateKeys();

            await context.createBlock([
                await polkadotJs.tx.session.setKeys(newKey1, []).signAsync(alice),
                await polkadotJs.tx.session.setKeys(newKey2, []).signAsync(bob),
            ]);

            // Jump to Session 2 so Alice and Bob are assigned to container chain
            await jumpToSession(context, 2);
            // Ensure that Bob is not invulnerable
            const removeBobFromInvulnerablesTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.invulnerables.removeInvulnerable(bob.address)
            );
            await context.createBlock([await removeBobFromInvulnerablesTx.signAsync(alice)]);

            // Ensure that offline marking is enabled
            const tx = polkadotJs.tx.sudo.sudo(polkadotJs.tx.inactivityTracking.enableOfflineMarking(true));
            await context.createBlock([await tx.signAsync(alice)]);

            // Making sure that Bob is a non-invulnerable collator assigned to a container chain and eligible for collating
            const addBobToSortedEligibleCollatorsTx = polkadotJs.tx.pooledStaking.requestDelegate(
                bob.address,
                "AutoCompounding",
                10000000000000000n
            );
            await context.createBlock([await addBobToSortedEligibleCollatorsTx.signAsync(bob)]);
            const stakingCandidates = await polkadotJs.query.pooledStaking.sortedEligibleCandidates();
            expect(stakingCandidates.toJSON()).to.deep.equal([
                {
                    candidate: bob.address,
                    stake: numberToHex(10000000000000000, 128),
                },
            ]);
            expect((await polkadotJs.query.invulnerables.invulnerables()).includes(bobAccountId)).to.be.false;
            expect(await isAddressInAssignedCollators(bob.address)).to.be.true;
        });

        it({
            id: "E01",
            title: "Setting a collator offline prevents it from being selected for collating in the next session",
            test: async () => {
                if (shouldSkipStarlightIT) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    // TODO: once the pallet is in starlight, check the calls are filtered,
                    // in case we don't want them for a specific runtime version.
                    return;
                }
                const bobOfflineStatusBeforeMarking = await polkadotJs.query.inactivityTracking.offlineCollators(
                    bob.address
                );
                if (bobOfflineStatusBeforeMarking.isTrue) {
                    console.log("BOB is marked as offline before the test starts.");
                    expect(true).to.be.false; // Fail the test if BOB is not offline
                }

                const setBobOfflineTx = polkadotJs.tx.pooledStaking.setOffline();
                await context.createBlock([await setBobOfflineTx.signAsync(bob)]);
                const bobOfflineStatusAfterMarking = await polkadotJs.query.inactivityTracking.offlineCollators(
                    bob.address
                );
                if (bobOfflineStatusAfterMarking.isFalse) {
                    console.log("BOB is not marked as offline.");
                    expect(false).to.be.true; // Fail the test if BOB is not offline
                }
                const currentSession = (await polkadotJs.query.session.currentIndex()).toNumber();
                await jumpToSession(context, currentSession + 2);

                // Check that BOB is:
                // - not part of the sorted eligible candidates list
                // - not selected for collating
                const stakingCandidatesAfterOfflineMarking =
                    await polkadotJs.query.pooledStaking.sortedEligibleCandidates();
                expect(stakingCandidatesAfterOfflineMarking.toJSON()).to.deep.equal([]);
                const isBobAssignedCollatorAfterOfflineMarking = await isAddressInAssignedCollators(bob.address);
                expect(isBobAssignedCollatorAfterOfflineMarking).to.be.false;
            },
        });
    },
});
