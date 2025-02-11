import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import type { ApiPromise } from "@polkadot/api";
import { readFileSync } from "node:fs";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DEVT0402",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let initialSlot: string;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            const initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );
            initialSlot = initialCheckpoint.header.slot.toString();
            const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
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
