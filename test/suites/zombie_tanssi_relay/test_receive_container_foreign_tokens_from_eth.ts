import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";

import { ETHEREUM_NETWORK_TESTNET, generateEventLog, generateUpdate, signAndSendAndInclude, waitSessions } from "utils";

describeSuite({
    id: "ZOMBIETANSS03",
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
            timeout: 300000,
            test: async () => {
                const ethereumSovereignAccount =
                    await containerChainPolkadotJs.call.locationToAccountApi.convertLocation({
                        V3: { parents: 2, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                    });

                const ethereumSovereignAccountAddress = ethereumSovereignAccount.asOk.toHuman();

                // Amount of native container tokens to transfer.
                const transferAmount = BigInt(100_000_000);

                // Amount in native container tokens to charge on destination.
                const containerFee = BigInt(500_000_000_000_000);

                // Create token receiver account
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

                const paraIdForChannel = 2000;
                const containerParaId = 2001;
                const assetId = 42;
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

                const ethTokenLocation = {
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

                // Register token on foreignAssetsCreator.
                const tx2 = await containerChainPolkadotJs.tx.sudo
                    .sudo(
                        containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                            ethTokenLocation,
                            assetId,
                            aliceFrontier.address,
                            true,
                            1
                        )
                    )
                    .signAsync(aliceFrontier);
                await signAndSendAndInclude(tx2, aliceFrontier);

                const tx4 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(tx4, aliceRelay);

                // Add funds in sovereign account for native and foreign tokens.

                console.log("ethereumSovereignAccountAddress: ", ethereumSovereignAccountAddress);

                const transferContainerToken = containerChainPolkadotJs.tx.balances.transferKeepAlive(
                    ethereumSovereignAccountAddress,
                    20000000000000000n
                );
                await signAndSendAndInclude(transferContainerToken, aliceFrontier);

                const ethereumSovereignContainerNativeBalanceBefore = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();

                console.log(
                    "ethereumSovereignContainerNativeBalanceBefore: ",
                    ethereumSovereignContainerNativeBalanceBefore
                );

                const receiverForeignContainerBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log("receiverNativeContainerBalanceBefore: ", receiverForeignContainerBalanceBefore);

                const tx5 = relayChainPolkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]);
                await signAndSendAndInclude(tx5, aliceRelay);

                // Wait for the XCM message to reach the container chain
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                const ethereumSovereignContainerNativeBalanceAfter = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();

                console.log(
                    "ethereumSovereignContainerNativeBalanceAfter: ",
                    ethereumSovereignContainerNativeBalanceAfter
                );

                const receiverForeignContainerBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(assetId, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log("receiverNativeContainerBalanceAfter: ", receiverForeignContainerBalanceAfter);

                // Check that fees were deducted in native token from the ETH sovereign account
                expect(ethereumSovereignContainerNativeBalanceAfter).to.be.eq(
                    ethereumSovereignContainerNativeBalanceBefore - containerFee
                );

                // Check that the foreign token amount was deposited into the receiver account.
                expect(receiverForeignContainerBalanceAfter).to.be.eq(
                    receiverForeignContainerBalanceBefore + transferAmount
                );
            },
        });
    },
});
