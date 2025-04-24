import "@tanssi/api-augment";

import { readFileSync } from "node:fs";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT0404",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let initialSlot: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlighEC: boolean;
        String;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );
            initialSlot = initialCheckpoint.header.slot.toString();

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
            title: "Ethreum client should not be able to receive an update for the next period without pushing the following sync committee",
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

                // Now we are injecting an update for the next period, but without specifying who the next committee is.
                // this will fail, if you push an update in a new period, you always need to push the new sync committee
                const nextPeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/next-finalized-header-update.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.submit(nextPeriodUpdate);
                const signedTx1 = await tx.signAsync(alice);
                const { result } = await context.createBlock([signedTx1]);

                expect(result[0].successful).to.be.false;
                expect(result[0].error.section).to.eq("ethereumBeaconClient");
                expect(result[0].error.name).to.eq("InvalidFinalizedHeaderGap");

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                // The update did not go through, so the slot is the same as the latest one we pushed
                // The sync committee update has a a finalized slot lower than the initial, so we keep the
                // initial
                expect(latestFinalizedSlot.toHuman().slot).to.equal(initialSlot);
            },
        });
    },
});
