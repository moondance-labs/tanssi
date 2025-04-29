import "@tanssi/api-augment";

import { readFileSync } from "node:fs";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT0405",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlighEC: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlighEC = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT.includes(specVersion);

            const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);

            if (shouldSkipStarlighEC) {
                console.log(`Skipping ETH client test for Starlight version ${specVersion}`);
                await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                return;
            }

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

                const signedTx = await polkadotJs.tx.ethereumBeaconClient
                    .submit(thisPeriodNextSyncCommitteeUpdate)
                    .signAsync(alice);

                if (shouldSkipStarlighEC) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                await context.createBlock([signedTx]);

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
