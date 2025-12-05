// @ts-nocheck

import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    generateUpdate,
    ETHEREUM_NETWORK_TESTNET,
    ETHEREUM_NETWORK_MAINNET,
    encodeRawPayload,
    PayloadEnum,
} from "utils";
import type { KeyringPair } from "@moonwall/util";
import { hexToU8a } from "@polkadot/util";
import { getBytes } from "ethers/utils";
import { AbiCoder } from "ethers/abi";

describeSuite({
    id: "ETHINBV2SYMBFAIL",
    title: "Receive Symbiotic update from Ethereum is failing",
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
            title: "Receive Symbiotic update should fail because incorrect payload",
            test: async () => {
                if (isStarlight) {
                    console.log("Skipping test for Starlight runtime");
                    return;
                }

                const gatewayHex = "EDa338E4dC46038493b885327842fD3E301CaB39";
                const origin = `0x${gatewayHex}`;
                const gatewayAddress = Uint8Array.from(Buffer.from(gatewayHex, "hex"));

                const signature = hexToU8a("0x550e2067494b1736ea5573f2d19cdc0ac95b410fff161bf16f11c6229655ec9c");
                const topics = [signature];

                const xcmEncoded = [
                    0,
                    encodeRawPayload(
                        polkadotJs,
                        polkadotJs
                            .createType("Bytes", "0x1234") // <-- some random bytes
                            .toU8a(),
                        PayloadEnum.SYMBIOTIC
                    ),
                ];

                const defaultAbiCoder = AbiCoder.defaultAbiCoder();
                const encodedDataString = defaultAbiCoder.encode(
                    ["uint64", "tuple(address,tuple(uint8,bytes)[],tuple(uint8,bytes),bytes,uint128,uint128,uint128)"],
                    [1, [origin, [], xcmEncoded, "0x", 0n, 0n, 0n]]
                );

                const encodedData = getBytes(encodedDataString);

                const log = polkadotJs.createType<SnowbridgeVerificationPrimitivesLog>(
                    "SnowbridgeVerificationPrimitivesLog",
                    {
                        address: gatewayAddress,
                        topics,
                        data: [].slice.call(encodedData),
                    }
                );

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                const tx3 = await polkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: true }); // <-- Allowing failure here to catch error event later

                const events = await polkadotJs.query.system.events();

                const failedEvent = events.find((a) => {
                    return a.event.method === "ExtrinsicFailed";
                });

                expect(!!failedEvent).to.equal(true);
            },
        });
    },
});
