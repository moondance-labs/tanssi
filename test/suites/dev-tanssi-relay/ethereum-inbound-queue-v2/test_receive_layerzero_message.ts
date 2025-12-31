// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    generateLayerZeroOutboundLog,
    sendCallAsChildPara,
    sovereignAccountOfChildForAddress32,
} from "utils";
import type { KeyringPair } from "@moonwall/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2 } from "../../../helpers";
import { hexToU8a } from "@polkadot/util";
import * as console from "node:console";

describeSuite({
    id: "ETHINBV2LZOK",
    title: "Receive LayerZero message from Ethereum - Success",
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
            const paraSovereignAccount = sovereignAccountOfChildForAddress32(context, destinationChain);
            const fundTx = polkadotJs.tx.balances.transferAllowDeath(paraSovereignAccount, fundAmount);
            await context.createBlock(await fundTx.signAsync(alice), { allowFailures: false });
        });

        it({
            id: "E01",
            title: "LayerZero message succeeds when sender is whitelisted",
            test: async () => {
                if (shouldSkipStarlightSnV2TT) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const lzSourceAddress = hexToU8a("0x0000000000000000000000001234567890abcdef1234567890abcdef12345678");
                const lzSourceEndpoint = 30101;

                // Set up a forwarding config with the SAME sender address via XCM from para 2000
                const configCall = polkadotJs.tx.lzRouter.updateRoutingConfig({
                    whitelistedSenders: [[lzSourceEndpoint, Array.from(lzSourceAddress)]],
                    notificationDestination: [100, 0], // pallet_index, call_index
                });

                // Send XCM Transact from para 2000 to set up the config via UMP
                await sendCallAsChildPara(configCall, destinationChain, context, fundAmount / 10n);

                // Now send a LayerZero message from the whitelisted sender
                const message = new Uint8Array([0x01, 0x02, 0x03, 0x04]);

                const log = await generateLayerZeroOutboundLog(polkadotJs, 1, {
                    lzSourceAddress, // Same as whitelisted sender
                    lzSourceEndpoint,
                    destinationChain,
                    message,
                });

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                const events = await polkadotJs.query.system.events();

                // Verify ExtrinsicSuccess event exists
                const successEvent = events.find((a) => {
                    return a.event.method === "ExtrinsicSuccess";
                });
                expect(!!successEvent, "ExtrinsicSuccess event should exist").to.equal(true);

                // Verify MessageReceived event from ethereumInboundQueueV2
                const messageReceivedEvent = events.find((a) => {
                    return a.event.section === "ethereumInboundQueueV2" && a.event.method === "MessageReceived";
                });
                expect(!!messageReceivedEvent, "MessageReceived event should exist").to.equal(true);

                console.log("LayerZero message successfully processed with whitelisted sender.");
            },
        });
    },
});
