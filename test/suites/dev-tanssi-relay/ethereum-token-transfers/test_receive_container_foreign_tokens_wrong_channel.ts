import "@tanssi/api-augment";

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS, expectEventCount } from "helpers";
import {
    ETHEREUM_NETWORK_MAINNET,
    ETHEREUM_NETWORK_TESTNET,
    generateEventLog,
    generateUpdate,
    mockAndInsertHeadData,
} from "utils";

describeSuite({
    id: "DTR1810",
    title: "EthTokensLocalProcessor: reception of container foreign tokens (wrong channel)",
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
            title: "Receive Container foreign token from Ethereum in Tanssi chain and forward to container",
            test: async () => {
                if (shouldSkipStarlightCTT) {
                    console.log(
                        `Skipping E01 test for Starlight version ${specVersion}: Container foreign token transfers not available yet`
                    );
                    return;
                }

                const paraIdForChannel = 2000;
                const containerParaId = 2001;
                const assetId = 42;
                const tokenAddrHex = "1111111111111111111111111111111111111111";

                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    polkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    // Wrong channel id
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000008", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    // Payload with the following shape:
                    // let payload = VersionedXcmMessage::V1(MessageV1 {
                    //     chain_id: 1,
                    //     command: Command::SendToken {
                    //         token: 0x1111111111111111111111111111111111111111,
                    //         destination: Destination::ForeignAccountId32 {
                    //             para_id: 2001,
                    //             id: [5u8; 32],
                    //             fee: 500000000000000,
                    //         },
                    //         amount: 100000000,
                    //         1500000000000000,
                    //     },
                    // })
                    new Uint8Array([
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                        17, 17, 17, 17, 1, 209, 7, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 64, 99, 82, 191, 198, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225,
                        245, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 41, 247, 61, 84, 5, 0, 0, 0, 0, 0, 0, 0, 0,
                        0,
                    ])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(polkadotJs, [log]);

                const tx = polkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                const signedTx = await polkadotJs.tx.sudo.sudo(tx).signAsync(alice);
                await context.createBlock([signedTx], { allowFailures: false });

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";

                const tx1 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                            newChannelId,
                            newAgentId,
                            paraIdForChannel
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx1], { allowFailures: false });

                const ethereumNetwork = isStarlight ? ETHEREUM_NETWORK_MAINNET : ETHEREUM_NETWORK_TESTNET;

                const ethTokenLocation = {
                    parents: 1,
                    interior: {
                        X2: [
                            {
                                GlobalConsensus: ethereumNetwork,
                            },
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: hexToU8a(tokenAddrHex),
                                },
                            },
                        ],
                    },
                };

                // Register token on foreignAssetsCreator.
                const tx2 = await polkadotJs.tx.sudo
                    .sudo(
                        polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                            ethTokenLocation,
                            assetId,
                            alice.address,
                            true,
                            1
                        )
                    )
                    .signAsync(alice);
                await context.createBlock([tx2], { allowFailures: false });

                const paraId = polkadotJs.createType("ParaId", containerParaId);
                await mockAndInsertHeadData(context, paraId, 2, 2, alice);

                const nonceBefore = await polkadotJs.query.ethereumInboundQueue.nonce(newChannelId);

                // Submit the message
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);

                // Since the channel is wrong, execution should fail in can_process_message.
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
