import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { readFileSync } from "fs";
import { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1203",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;

            let initialCheckpoint = JSON.parse(
                readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
            );
            const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
            const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
            await context.createBlock([signedTx]);
        });

        it({
            id: "E02",
            title: "Ethreum client should be able to receive an update within the same period by same committee",
            test: async function () {
                let samePeriodUpdate = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/finalized-header-update.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.submit(samePeriodUpdate);
                const signedTx = await tx.signAsync(alice);
                await context.createBlock([signedTx]);

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot = await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(
                    latestFinalizedBlockRoot
                );

                expect(latestFinalizedSlot.toHuman().slot).to.equal(
                    samePeriodUpdate["finalized_header"]["slot"].toString()
                );
            },
        });
    },
});
