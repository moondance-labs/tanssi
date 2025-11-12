import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { generateEventLog, generateUpdate, SNOWBRIDGE_FEES_ACCOUNT } from "utils";
import { expectEventCount, STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS } from "helpers";
import type { KeyringPair } from "@moonwall/util";

describeSuite({
    id: "DTR1803",
    title: "NativeContainerTokensProcessor: reception of container native tokens (token not registered)",
    foundationMethods: "dev",

    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightCTT: boolean;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            const keyring = new Keyring({ type: "sr25519" });
            alice = keyring.addFromUri("//Alice", { name: "Alice default" });

            const runtimeName = polkadotJs.runtimeVersion.specName.toString();
            isStarlight = runtimeName === "starlight";
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightCTT =
                isStarlight && STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS.includes(specVersion);
        });

        it({
            id: "E01",
            title: "Receive Container native token from Ethereum in Tanssi chain and forward to container",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Container native token transfers not available yet`
                    );
                    return;
                }
                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    // Payload with the following shape:
                    // let payload = VersionedXcmMessage::V1(MessageV1 {
                    //     chain_id: 1,
                    //     command: Command::SendNativeToken {
                    //         token_id: 0x485f805cb9de38b4324485447c664e16035aa9d28e8723df192fa02ad3530889,
                    //         destination: Destination::ForeignAccountId20 {
                    //             para_id: 2001,
                    //             id: [5u; 20],
                    //             fee: 500_000_000_000_000,
                    //         },
                    //         amount: 100_000_000,
                    //         fee: 1_500_000_000_000_000,
                    //     },
                    // });
                    new Uint8Array([
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 72, 95, 128, 92, 185, 222, 56, 180, 50, 68, 133, 68, 124, 102, 78,
                        22, 3, 90, 169, 210, 142, 135, 35, 223, 25, 47, 160, 42, 211, 83, 8, 137, 2, 209, 7, 0, 0, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 64, 99, 82, 191, 198, 1, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 41, 247, 61, 84, 5, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            newParaId
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                // Add funds to snowbridge fees account
                const transferFeesAccountTx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.balances.forceSetBalance(SNOWBRIDGE_FEES_ACCOUNT, 500_000_000_000_000_000n))
                    .signAsync(alice);
                await context.createBlock([transferFeesAccountTx], { allowFailures: false });

                const nonceBefore = await polkadotJs.query.ethereumInboundQueue.nonce(newChannelId);

                // Submit the message
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);

                // Since the token is not registered, execution should fail due to can_process_message returns
                // false for NativeContainerTokensProcessor and no other processor can process the message either.
                const { result } = await context.createBlock([tx3]);
                expect(result[0].successful).to.be.false;

                const nonceAfter = await polkadotJs.query.ethereumInboundQueue.nonce(newChannelId);

                // Nonce should stay the same
                expect(nonceAfter.toNumber()).to.be.equal(nonceBefore.toNumber());

                // XCM Sent event should not be emitted
                await expectEventCount(polkadotJs, {
                    Sent: 0,
                });
            },
        });
    },
});
