import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";

import {
    ETHEREUM_NETWORK_TESTNET,
    generateEventLog,
    generateUpdate,
    signAndSendAndInclude,
    signAndSendAndIncludeMany,
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
                const erc20AssetId = 24;
                const tanssiAssetId = 42;
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
                console.log("Registering foreign tokens");

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

                const registerErc20AssetTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethTokenLocation,
                        erc20AssetId,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );

                // Add funds in sovereign account for tanssi and foreign tokens.

                const relayTokenLocation = {
                    parents: 1,
                    interior: "Here",
                };

                const registerTanssiAssetTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        relayTokenLocation,
                        tanssiAssetId,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );

                await signAndSendAndIncludeMany(
                    containerChainPolkadotJs,
                    [registerErc20AssetTx, registerTanssiAssetTx],
                    aliceFrontier
                );

                console.log("Creating asset rate for tanssi token");
                const assetRateTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.assetRate.create(tanssiAssetId, 50_000_000_000_000_000_000n)
                );
                await signAndSendAndInclude(assetRateTx, aliceFrontier);

                console.log("Minting tanssi token on foreignAssets");
                const transferRelayToken = containerChainPolkadotJs.tx.foreignAssets.mint(
                    tanssiAssetId,
                    ethereumSovereignAccountAddress,
                    20000000000000000000000n
                );

                await signAndSendAndInclude(transferRelayToken, aliceFrontier);

                console.log("ethereumSovereignAccountAddress: ", ethereumSovereignAccountAddress);

                const ethereumSovereignContainerTanssiBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        tanssiAssetId,
                        ethereumSovereignAccountAddress
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log(
                    "ethereumSovereignContainerTanssiBalanceBefore: ",
                    ethereumSovereignContainerTanssiBalanceBefore
                );

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

                const ethereumSovereignContainerTanssiBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        tanssiAssetId,
                        ethereumSovereignAccountAddress
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log(
                    "ethereumSovereignContainerTanssiBalanceAfter: ",
                    ethereumSovereignContainerTanssiBalanceAfter
                );

                const receiverForeignContainerBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(erc20AssetId, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                console.log("receiverNativeContainerBalanceAfter: ", receiverForeignContainerBalanceAfter);

                // Check that fees were deducted in native token from the ETH sovereign account
                expect(ethereumSovereignContainerTanssiBalanceAfter).to.be.lt(
                    Number(ethereumSovereignContainerTanssiBalanceBefore)
                );
                expect(
                    ethereumSovereignContainerTanssiBalanceAfter - ethereumSovereignContainerTanssiBalanceBefore
                ).to.be.lte(Number(containerFee));

                // Check that the foreign token amount was deposited into the receiver account.
                expect(receiverForeignContainerBalanceAfter).to.be.eq(
                    receiverForeignContainerBalanceBefore + transferAmount
                );
            },
        });
    },
});
