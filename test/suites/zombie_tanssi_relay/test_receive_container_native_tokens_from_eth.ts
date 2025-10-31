import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import {
    generateEventLog,
    generateUpdate,
    signAndSendAndInclude,
    ETHEREUM_NETWORK_TESTNET,
    waitSessions,
    SNOWBRIDGE_FEES_ACCOUNT,
} from "utils";

describeSuite({
    id: "ZOMBIETANSS03",
    title: "Container native tokens transfer from Ethereum to container (via Tanssi)",
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
            title: "Should receive container native tokens from Ethereum and forward them to container",
            timeout: 300000,
            test: async () => {
                const ethereumSovereignAccount =
                    await containerChainPolkadotJs.call.locationToAccountApi.convertLocation({
                        V3: { parents: 2, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                    });

                const ethereumSovereignAccountAddress = ethereumSovereignAccount.asOk.toHuman();

                // Amount of native container tokens to transfer.
                const transferAmount = BigInt(100_000_000);

                // Amount in tanssi tokens to charge on destination.
                const containerFee = BigInt(500_000_000_000_000);

                // Create token receiver account
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

                const relayNativeTokenAssetId = 42;

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
                        0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 72, 95, 128, 92, 185, 222, 56, 180, 50, 68, 133, 68, 124, 102, 78,
                        22, 3, 90, 169, 210, 142, 135, 35, 223, 25, 47, 160, 42, 211, 83, 8, 137, 2, 209, 7, 0, 0, 5, 5,
                        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 64, 99, 82, 191, 198, 1, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 41, 247, 61, 84, 5, 0,
                        0, 0, 0, 0, 0, 0, 0, 0,
                    ])
                );
                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(relayChainPolkadotJs, [log]);

                const tx = relayChainPolkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate);

                await signAndSendAndInclude(relayChainPolkadotJs.tx.sudo.sudo(tx), aliceRelay);

                // Create EthereumTokenTransfers channel to validate when receiving the tokens.
                const newChannelId = "0x0000000000000000000000000000000000000000000000000000000000000004";
                const newAgentId = "0x0000000000000000000000000000000000000000000000000000000000000005";
                const newParaId = 500;

                const tx1 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumTokenTransfers.setTokenTransferChannel(
                        newChannelId,
                        newAgentId,
                        newParaId
                    )
                );
                await signAndSendAndInclude(tx1, aliceRelay);

                const containerMetadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = containerMetadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();

                const tokenLocation = {
                    parents: 0,
                    interior: {
                        X2: [
                            {
                                Parachain: 2001,
                            },
                            {
                                PalletInstance: balancesPalletIndex,
                            },
                        ],
                    },
                };
                const versionedLocation = {
                    V3: tokenLocation,
                };

                const metadata = {
                    name: "para2001",
                    symbol: "para2001",
                    decimals: 12,
                };

                // Register token on EthereumSystem.
                const tx2 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumSystem.registerToken(versionedLocation, metadata)
                );

                await signAndSendAndInclude(tx2, aliceRelay);

                const tx4 = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(tx4, aliceRelay);

                // Simulate previous native container token reception from Ethereum.
                const transferContainerToken = containerChainPolkadotJs.tx.balances.transferKeepAlive(
                    ethereumSovereignAccountAddress,
                    20000000000000000n
                );
                await signAndSendAndInclude(transferContainerToken, aliceFrontier);

                // Register relay token as foreign in container
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

                // Create asset rate for tanssi token in container
                const assetRateTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.assetRate.create(
                        relayNativeTokenAssetId,
                        // this defines how much the asset costs with respect to the
                        // new asset
                        // in this case, asset*2=native
                        // that means that we will charge 0.5 of the native balance
                        2000000000000000000n
                    )
                );
                await signAndSendAndInclude(assetRateTx, aliceFrontier);

                // Add funds to snowbridge fees account
                const transferFeesAccountTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.balances.forceSetBalance(SNOWBRIDGE_FEES_ACCOUNT, 500_000_000_000_000_000n)
                );
                await signAndSendAndInclude(transferFeesAccountTx, aliceRelay);

                const ethereumSovereignRelayTokenBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        relayNativeTokenAssetId,
                        ethereumSovereignAccountAddress
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethereumSovereignRelayTokenBalanceBefore).to.be.eq(0n);

                const ethereumSovereignContainerBalanceBefore = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();

                const receiverNativeContainerBalanceBefore = (
                    await containerChainPolkadotJs.query.system.account(tokenReceiver)
                ).data.free.toBigInt();

                const tx5 = relayChainPolkadotJs.tx.ethereumInboundQueue.submit(messageExtrinsics[0]);
                await signAndSendAndInclude(tx5, aliceRelay);

                // Wait for the XCM message to reach the container chain
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                const ethereumSovereignRelayTokenBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        relayNativeTokenAssetId,
                        ethereumSovereignAccountAddress
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                expect(ethereumSovereignRelayTokenBalanceAfter).toBeGreaterThan(0n);
                expect(ethereumSovereignRelayTokenBalanceAfter).toBeLessThan(containerFee);

                const ethereumSovereignContainerBalanceAfter = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();

                const receiverNativeContainerBalanceAfter = (
                    await containerChainPolkadotJs.query.system.account(tokenReceiver)
                ).data.free.toBigInt();

                // Check that container tokens amount was deducted from the ETH sovereign account
                expect(ethereumSovereignContainerBalanceAfter).to.be.eq(
                    ethereumSovereignContainerBalanceBefore - transferAmount
                );

                // Check that the native container token amount was deposited into the receiver account.
                expect(receiverNativeContainerBalanceAfter).to.be.eq(
                    receiverNativeContainerBalanceBefore + transferAmount
                );
            },
        });
    },
});
