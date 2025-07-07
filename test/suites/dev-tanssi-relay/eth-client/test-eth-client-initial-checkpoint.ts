import "@tanssi/api-augment";

import { readFileSync } from "node:fs";
import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { ApiPromise } from "@polkadot/api";

import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT, checkCallIsFiltered } from "helpers";

describeSuite({
    id: "DEVT0401",
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

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlighEC = isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_ETH_CLIENT.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Ethreum client should accept an intiial checkpoint",
            test: async () => {
                const initialCheckpoint = JSON.parse(
                    readFileSync("tmp/ethereum_client_test/initial-checkpoint.json").toString()
                );
                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(initialCheckpoint);

                if (shouldSkipStarlighEC) {
                    console.log(`Skipping E01 test for Starlight version ${specVersion}`);
                    await checkCallIsFiltered(context, polkadotJs, await tx.signAsync(alice));
                    return;
                }

                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx]);
                const checkpointRoot = await polkadotJs.query.ethereumBeaconClient.validatorsRoot();
                expect(checkpointRoot.toHuman()).to.equal(initialCheckpoint.validators_root);

                const latestFinalizedBlockRoot = await polkadotJs.query.ethereumBeaconClient.latestFinalizedBlockRoot();
                const latestFinalizedSlot =
                    await polkadotJs.query.ethereumBeaconClient.finalizedBeaconState(latestFinalizedBlockRoot);

                expect(latestFinalizedSlot.toHuman().slot).to.equal(initialCheckpoint.header.slot.toString());
            },
        });
    },
});
