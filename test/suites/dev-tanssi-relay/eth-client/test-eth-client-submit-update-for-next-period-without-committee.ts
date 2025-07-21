import "@tanssi/api-augment";

import { readFileSync } from "node:fs";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT0402",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let initialSlot: string;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlighEC: boolean;

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
            id: "E02",
            title: "Ethreum client should not be able to receive an update for the next period without the next sync committee",
            test: async () => {
                const nextPeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/next-finalized-header-update.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.submit(nextPeriodUpdate);
                const signedTx = await tx.signAsync(alice);

                if (shouldSkipStarlighEC) {
                    console.log(`Skipping E02 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, signedTx);
                    return;
                }

                const { result } = await context.createBlock([signedTx]);

                expect(result[0].successful).to.be.false;
                expect(result[0].error.section).to.eq("ethereumBeaconClient");
                expect(result[0].error.name).to.eq("SkippedSyncCommitteePeriod");

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                // The update did not go through, so the slot is the same as the initial one
                expect(latestFinalizedSlot.toHuman().slot).to.equal(initialSlot);
            },
        });
    },
});
