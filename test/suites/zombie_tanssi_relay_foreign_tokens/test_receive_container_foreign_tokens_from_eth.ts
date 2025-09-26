import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";

import {
    ETHEREUM_NETWORK_TESTNET,
    SNOWBRIDGE_FEES_ACCOUNT,
    generateEventLog,
    generateUpdate,
    signAndSendAndInclude,
    waitSessions,
} from "utils";

describeSuite({
    id: "ZOMBIETANSS04",
    title: "Container foreign tokens transfer from Ethereum to container (via Tanssi)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let aliceFrontier: KeyringPair;
        let aliceRelay: KeyringPair;
        let chain: string;

        beforeAll(async () => {
            containerChainPolkadotJs = context.polkadotJs("Container2001");
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            chain = containerChainPolkadotJs.consts.system.version.specName.toString();
            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
            aliceFrontier = chain === "frontier-template" ? alith : aliceRelay;
        });

        it({
            id: "T01",
            title: "Should receive container foreign tokens from Ethereum and forward them to container",
            timeout: 600000,
            test: async () => {
                // Amount of native container tokens to transfer.
                const transferAmount = BigInt(100_000_000);

                // Create token receiver account
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

                const paraIdForChannel = 2000;
                const relayNativeTokenAssetId = 42;
                const erc20AssetId = 24;
                const tokenAddrHex = "0x1111111111111111111111111111111111111111";

                // Hard-coding payload as we do not have scale encoding-decoding
                const log = await generateEventLog(
                    relayChainPolkadotJs,
                    Uint8Array.from(Buffer.from("eda338e4dc46038493b885327842fd3e301cab39", "hex")),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000004", "hex")
                    ),
                    Uint8Array.from(
                        Buffer.from("0000000000000000000000000000000000000000000000000000000000000000", "hex")
                    ),
                    1,
                    new Uint8Array([
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
                        17, 17, 17, 17, 2, 209, 7, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0,
                        64, 99, 82, 191, 198, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 192, 41, 247, 61, 84, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(relayChainPolkadotJs, [log]);

                console.log("Forcing checkpoint");
                const tx = relayChainPolkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);
                await signAndSendAndInclude(relayChainPolkadotJs.tx.sudo.sudo(tx), aliceRelay);

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";

                console.log("Creating channel");
                const tx1 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                        newChannelId,
                        newAgentId,
                        paraIdForChannel
                    )
                );
                await signAndSendAndInclude(tx1, aliceRelay);

                console.log("Force default XCM version");
                const tx2 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(tx2, aliceRelay);

                // Register token on foreignAssetsCreator.
                console.log("Registering erc20 foreign token container");

                const ethTokenLocationFromContainerViewpoint = {
                    parents: 2,
                    interior: {
                        X2: [
                            {
                                GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                            },
                            {
                                AccountKey20: {
                                    network: ETHEREUM_NETWORK_TESTNET,
                                    key: hexToU8a(tokenAddrHex),
                                },
                            },
                        ],
                    },
                };

                const registerErc20ContainerViewpointAssetTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethTokenLocationFromContainerViewpoint,
                        erc20AssetId,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(registerErc20ContainerViewpointAssetTx, aliceFrontier);

                console.log("Registering native relay token in container");

                const relayNativeTokenLocation = {
                    parents: 1,
                    interior: "Here",
                };

                const registerRelayNativeTokenLocation = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        relayNativeTokenLocation,
                        relayNativeTokenAssetId,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(registerRelayNativeTokenLocation, aliceFrontier);

                console.log("Creating asset rate for tanssi token");
                const assetRateTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.assetRate.create(relayNativeTokenAssetId, 50_000_000_000_000_000_000n)
                );
                await signAndSendAndInclude(assetRateTx, aliceFrontier);

                console.log("Registering erc20 foreign token relay");

                const ethTokenLocationFromRelayViewpoint = {
                    parents: 1,
                    interior: {
                        X2: [
                            {
                                GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                            },
                            {
                                AccountKey20: {
                                    network: ETHEREUM_NETWORK_TESTNET,
                                    key: hexToU8a(tokenAddrHex),
                                },
                            },
                        ],
                    },
                };

                const registerErc20RelayViewpointAssetTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethTokenLocationFromRelayViewpoint,
                        erc20AssetId,
                        aliceRelay.address,
                        true,
                        1
                    )
                );

                await signAndSendAndInclude(registerErc20RelayViewpointAssetTx, aliceRelay);

                // Add funds in relay fees account
                const transferFeesAccountTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.balances.forceSetBalance(SNOWBRIDGE_FEES_ACCOUNT, 500_000_000_000_000_000n)
                );
                await signAndSendAndInclude(transferFeesAccountTx, aliceRelay);

                const snowbridgeFeesAccountBalanceBefore = (
                    await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                console.log("snowbridgeFeesAccountBalanceBefore: ", snowbridgeFeesAccountBalanceBefore);

                const receiverForeignContainerBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(erc20AssetId, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log("receiverNativeContainerBalanceBefore: ", receiverForeignContainerBalanceBefore);

                const tx5 = relayChainPolkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]);
                await signAndSendAndInclude(tx5, aliceRelay);

                // Wait for the XCM message to reach the container chain
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                const receiverForeignContainerBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(erc20AssetId, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log("receiverNativeContainerBalanceAfter: ", receiverForeignContainerBalanceAfter);

                const snowbridgeFeesAccountBalanceAfter = (
                    await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                console.log("snowbridgeFeesAccountBalanceAfter: ", snowbridgeFeesAccountBalanceAfter);

                // Check that the foreign token amount was deposited into the receiver account.
                expect(receiverForeignContainerBalanceAfter).to.be.eq(
                    receiverForeignContainerBalanceBefore + transferAmount
                );
            },
        });
    },
});
