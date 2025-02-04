import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { readFileSync } from "fs";
import { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1201",
    title: "Ethereum Beacon Client tests",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });

        it({
            id: "E01",
            title: "Ethreum client should accept an intiial checkpoint",
            test: async function () {
                const initialCheckpoint = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx]);
                const checkpointRoot = await polkadotJs.query.ethereumBeaconClient.validatorsRoot();
                expect(checkpointRoot.toHuman()).to.equal(initialCheckpoint["validators_root"]);

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                expect(latestFinalizedSlot.toHuman().slot).to.equal(initialCheckpoint["header"]["slot"].toString());
            },
        });
    },
});
