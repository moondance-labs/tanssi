import "@tanssi/api-augment";

import { beforeAll, describeSuite } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import { STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_TRANSFERS, expectEventCount } from "helpers";
import {
    ETHEREUM_NETWORK_MAINNET,
    ETHEREUM_NETWORK_TESTNET,
    SNOWBRIDGE_FEES_ACCOUNT,
    generateEventLog,
    generateUpdate,
    mockAndInsertHeadData,
} from "utils";

describeSuite({
    id: "DTR1807",
    title: "EthTokensLocalProcessor: receive and forward container foreign tokens from Ethereum",
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

                // Add funds in relay fees account
                const transferFeesAccountTx = await polkadotJs.tx.sudo
                    .sudo(polkadotJs.tx.balances.forceSetBalance(SNOWBRIDGE_FEES_ACCOUNT, 500_000_000_000_000_000n))
                    .signAsync(alice);
                await context.createBlock([transferFeesAccountTx], { allowFailures: false });

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

                // Submit the message
                const tx3 = await polkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]).signAsync(alice);
                await context.createBlock([tx3], { allowFailures: false });

                // Check for the XCM Sent event
                await expectEventCount(polkadotJs, {
                    Sent: 1,
                });
            },
        });
    },
});
