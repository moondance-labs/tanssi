import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { readFileSync } from "node:fs";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DEVT0405",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );
            const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
            const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
            await context.createBlock([signedTx]);
        });

        it({
            id: "E01",
            title: "Ethreum client should be able to receive an update for the next period when pushing all committee info",
            test: async () => {
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

                // Now we are injecting the first update of the next period
                // this should contain the next sync committee
                const nextPeriodSyncCommitteeUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/next-sync-committee-update.json").toString()
                );
                await context.createBlock([
                    await polkadotJs.tx.ethereumBeaconClient.submit(nextPeriodSyncCommitteeUpdate).signAsync(alice),
                ]);

                // Now we are injecting an update for the period 'intial period +1' for which
                // we have already pushed the sync committee update. Since this needs to be done
                // only once per period, we shoudl be good
                const nextPeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/next-finalized-header-update.json").toString()
                );

                await context.createBlock([
                    await polkadotJs.tx.ethereumBeaconClient.submit(nextPeriodUpdate).signAsync(alice),
                ]);

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                // The update did go through, we should have the latest state
                const expectedSlot = nextPeriodUpdate.finalized_header.slot;
                expect(latestFinalizedSlot.toHuman().slot.replace(/,/g, "")).to.equal(expectedSlot.toString());
            },
        });
    },
});
