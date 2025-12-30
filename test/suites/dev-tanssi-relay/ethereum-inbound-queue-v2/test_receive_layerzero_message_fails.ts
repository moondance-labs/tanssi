// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { generateUpdate, generateLayerZeroOutboundLog, sendCallAsChildPara, getChildParaSovereignAccount } from "utils";
import type { KeyringPair } from "@moonwall/util";
import { retrieveDispatchErrors, STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "../../../helpers";
import { hexToU8a } from "@polkadot/util";
import * as console from "node:console";

describeSuite({
    id: "ETHINBV2LZ",
    title: "Receive LayerZero message from Ethereum",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let shouldSkipStarlightSnV2TT: boolean;
        let specVersion: number;

        const destinationChain = 2000; // Container chain para_id
        const fundAmount = 100_000_000_000_000_000n;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";

            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightSnV2TT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2.includes(specVersion);

            if (shouldSkipStarlightSnV2TT) {
                console.log("Skipping test for Starlight runtime");
                return;
            }

            // Fund the sovereign account of para 2000 (child parachain on relay)
            const paraSovereignAccount = getChildParaSovereignAccount(context, destinationChain);
            const fundTx = polkadotJs.tx.balances.transferAllowDeath(paraSovereignAccount, fundAmount);
            await context.createBlock(await fundTx.signAsync(alice), { allowFailures: false });
        });

        it({
            id: "E01",
            title: "LayerZero message fails with NoForwardingConfig when no config is set",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                // Create a simple LayerZero message
                const lzSourceAddress = hexToU8a("0x0000000000000000000000001234567890abcdef1234567890abcdef12345678");
                const lzSourceEndpoint = 30101; // Ethereum mainnet LayerZero endpoint ID
                const message = new Uint8Array([0x01, 0x02, 0x03, 0x04]); // Some arbitrary message

                const log = await generateLayerZeroOutboundLog(polkadotJs, 1, {
                    lzSourceAddress,
                    lzSourceEndpoint,
                    destinationChain,
                    message,
                });

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                // Submit the LayerZero message - it should fail because no forwarding config exists
                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: true });

                const errors = await retrieveDispatchErrors(polkadotJs);

                expect(errors).to.include("NoForwardingConfig");
            },
        });

        it({
            id: "E02",
            title: "LayerZero message fails with NotWhitelistedSender when sender is not whitelisted",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const lzSourceAddress = hexToU8a("0x0000000000000000000000001234567890abcdef1234567890abcdef12345678");
                const lzSourceEndpoint = 30101;

                // Set up a forwarding config with a DIFFERENT whitelisted sender via XCM from para 2000
                const differentSenderAddress = hexToU8a(
                    "0x000000000000000000000000abcdefabcdefabcdefabcdefabcdefabcdefabcd"
                );
                const configCall = polkadotJs.tx.lzRouter.updateMessageForwardingConfig({
                    whitelistedSenders: [[lzSourceEndpoint, Array.from(differentSenderAddress)]],
                    notificationDestination: [100, 0], // pallet_index, call_index
                });

                // Send XCM Transact from para 2000 to set up the config via UMP
                await sendCallAsChildPara(configCall, destinationChain, context, fundAmount / 10n);

                // Now send a LayerZero message from a non-whitelisted sender
                const message = new Uint8Array([0x01, 0x02, 0x03, 0x04]);

                const log = await generateLayerZeroOutboundLog(polkadotJs, 2, {
                    lzSourceAddress, // This is different from differentSenderAddress
                    lzSourceEndpoint,
                    destinationChain,
                    message,
                });

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: true });

                const errors = await retrieveDispatchErrors(polkadotJs);

                expect(errors).to.include("NotWhitelistedSender");
            },
        });
    },
});
