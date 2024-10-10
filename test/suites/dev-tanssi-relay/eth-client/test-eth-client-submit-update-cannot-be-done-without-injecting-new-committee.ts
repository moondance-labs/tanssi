import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { readFileSync } from "fs";
import { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1204",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let initialSlot;
        String;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );
            initialSlot = initialCheckpoint["header"]["slot"].toString();
            const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
            const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
            await context.createBlock([signedTx]);
        });

        it({
            id: "E01",
            title: "Ethreum client should not be able to receive an update for the next period without pushing the following sync committee",
            test: async function () {
                // Next sync committee shold give us the default values
                const nextSyncCommitteeBeforeUpdate = await polkadotJs.query.ethereumBeaconClient.nextSyncCommittee();
                expect(nextSyncCommitteeBeforeUpdate.root.toHuman()).to.be.eq(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );

                const thisPeriodNextSyncCommitteeUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/sync-committee-update.json").toString()
                );
                await context.createBlock([
                    await polkadotJs.tx.ethereumBeaconClient.submit(thisPeriodNextSyncCommitteeUpdate).signAsync(alice),
                ]);

                // Now the next sync committee should have been populated
                const nextSyncCommittee = await polkadotJs.query.ethereumBeaconClient.nextSyncCommittee();
                expect(nextSyncCommittee.root.toHuman()).to.not.be.eq(
                    "0x0000000000000000000000000000000000000000000000000000000000000000"
                );

                // Now we are injecting an update for the next period, but without specifying who the next committee is.
                // this will fail, if you push an update in a new period, you always need to push the new sync committee
                const nextPeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/next-finalized-header-update.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.submit(nextPeriodUpdate);
                const signedTx = await tx.signAsync(alice);
                const { result } = await context.createBlock([signedTx]);

                expect(result[0].successful).to.be.false;
                expect(result[0].error.section).to.eq("ethereumBeaconClient");
                expect(result[0].error.name).to.eq("SyncCommitteeUpdateRequired");

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot = await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(
                    latestFinalizedBlockRoot
                );

                // The update did not go through, so the slot is the same as the latest one we pushed
                // The sync committee update has a a finalized slot lower than the initial, so we keep the
                // initial
                expect(latestFinalizedSlot.toHuman().slot).to.equal(initialSlot);
            },
        });
    },
});
