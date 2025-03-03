import "@tanssi/api-augment";

import { readFileSync } from "node:fs";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

describeSuite({
    id: "DEVT0403",
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
            id: "E02",
            title: "Ethreum client should be able to receive an update within the same period by same committee",
            test: async () => {
                const samePeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/finalized-header-update.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.submit(samePeriodUpdate);
                const signedTx = await tx.signAsync(alice);
                await context.createBlock([signedTx]);

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                expect(latestFinalizedSlot.unwrap().slot.toString()).to.equal(
                    samePeriodUpdate.finalized_header.slot.toString()
                );
            },
        });
    },
});
