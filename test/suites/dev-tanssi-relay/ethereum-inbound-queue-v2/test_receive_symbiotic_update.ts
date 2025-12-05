// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    ETHEREUM_NETWORK_TESTNET,
    generateOutboundMessageAcceptedLog,
    ETHEREUM_NETWORK_MAINNET,
} from "utils";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "ETHINBV2SYMB",
    title: "Receive Symbiotic update from Ethereum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let ethNetworkId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            ethNetworkId = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;
        });

        it({
            id: "E01",
            title: "Receive Symbiotic update from Ethereum in Tanssi chain",
            test: async () => {
                if (isStarlight) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const keyring = new Keyring({ type: "sr25519" });
                const validators = [
                    keyring.addFromUri("//Charlie").address,
                    keyring.addFromUri("//Ferdie").address,
                    keyring.addFromUri("//Bob//stash").address,
                ];

                const log = await generateOutboundMessageAcceptedLog(polkadotJs, 1, 0, null, [], [], validators);

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const externalValidatorsList = (
                    await polkadotJs.query.externalValidators.externalValidators()
                ).toJSON();

                expect(externalValidatorsList, validators);
            },
        });
    },
});
