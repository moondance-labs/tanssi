import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import {
    generateOutboundMessageAcceptedLog,
    generateUpdate,
    signAndSendAndInclude,
    ETHEREUM_NETWORK_TESTNET,
    waitSessions,
} from "utils";

describeSuite({
    id: "ZOMBIETANSSICONTNATCONT",
    title: "Container native token transfer from Ethereum to container chain (via Snowbridge V2)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;

        // Constants matching Rust tests
        const RELAY_NATIVE_TOKEN_ASSET_ID = 42;
        const CONTAINER_TOKEN_TRANSFER_AMOUNT = 25_000_000_000_000n; // Amount of container native token to transfer
        const DEST_FEE_AMOUNT = 5_000_000_000_000n; // Relay tokens for container execution fees
        const TANSSI_FEE_AMOUNT = 10_000_000_000_000n; // Total Tanssi tokens for relay + container fees

        beforeAll(async () => {
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
        });

        it({
            id: "T01",
            title: "Should transfer container native token from Ethereum to Frontier container chain via V2",
            timeout: 300000,
            test: async () => {
                const containerChainPolkadotJs = context.polkadotJs("Container2001");
                const chain = containerChainPolkadotJs.consts.system.version.specName.toString();
                const aliceFrontier = chain === "frontier-template" ? alith : aliceRelay;
                const containerParaId = 2001;

                // Token receiver - AccountKey20 for Frontier
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

                // Get Balances pallet index for container native token location
                const containerMetadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = containerMetadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();

                // Container native token location from relay perspective
                const containerNativeTokenLocationRelay = {
                    parents: 0,
                    interior: {
                        X2: [{ Parachain: containerParaId }, { PalletInstance: balancesPalletIndex }],
                    },
                };

                // Get Ethereum sovereign account on container chain
                const ethereumSovereignContainer =
                    await containerChainPolkadotJs.call.locationToAccountApi.convertLocation({
                        V3: { parents: 2, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                    });
                const ethereumSovereignContainerAddress = ethereumSovereignContainer.asOk.toHuman();

                // Get Ethereum sovereign account on relay
                const ethereumSovereignRelay = await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: { parents: 1, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                });
                const ethereumSovereignRelayAddress = ethereumSovereignRelay.asOk.toHuman();

                // === Setup on Relay Chain ===

                // 1. Register Tanssi native token in EthereumSystemV2 (for fees)
                const tanssiTokenLocation = { parents: 0, interior: "Here" };
                const versionedTanssiLocation = { V3: tanssiTokenLocation };
                const tanssiMetadata = { name: "relay", symbol: "relay", decimals: 12 };

                const registerTanssiTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumSystemV2.registerToken(
                        versionedTanssiLocation,
                        versionedTanssiLocation,
                        tanssiMetadata,
                        1
                    )
                );
                await signAndSendAndInclude(registerTanssiTx, aliceRelay);

                // 2. Register container native token in EthereumSystemV2
                const versionedContainerLocation = { V3: containerNativeTokenLocationRelay };
                const containerMetadataToken = { name: "container", symbol: "CTR", decimals: 18 };

                const registerContainerTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumSystemV2.registerToken(
                        versionedContainerLocation,
                        versionedContainerLocation,
                        containerMetadataToken,
                        2
                    )
                );
                await signAndSendAndInclude(registerContainerTx, aliceRelay);

                // Get token IDs
                const allEntries = await relayChainPolkadotJs.query.ethereumSystem.nativeToForeignId.entries();
                const tokenIds = allEntries.map(([, id]) => id.toHuman());
                const tanssiTokenId = tokenIds[0];
                const containerTokenId = tokenIds[1];

                // 3. Fund Ethereum sovereign account on relay
                const fundEthSovRelayTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.balances.forceSetBalance(
                        ethereumSovereignRelayAddress,
                        100_000_000_000_000_000_000n
                    )
                );
                await signAndSendAndInclude(fundEthSovRelayTx, aliceRelay);

                // 4. Force XCM version
                const forceXcmVersionTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(forceXcmVersionTx, aliceRelay);

                // === Setup on Container Chain ===

                // 1. Register relay token (Tanssi) as foreign asset for fees
                const relayNativeTokenLocation = { parents: 1, interior: "Here" };

                const registerRelayTokenTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        relayNativeTokenLocation,
                        RELAY_NATIVE_TOKEN_ASSET_ID,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(registerRelayTokenTx, aliceFrontier);

                // 2. Create asset rate for relay token in container
                const assetRateTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.assetRate.create(
                        RELAY_NATIVE_TOKEN_ASSET_ID,
                        2_000_000_000_000_000_000_000n
                    )
                );
                await signAndSendAndInclude(assetRateTx, aliceFrontier);

                // 3. Fund Ethereum sovereign account ON THE CONTAINER with native balance
                // This simulates the container native tokens that were bridged to Ethereum and are now being sent back
                const fundEthSovContainerTx = containerChainPolkadotJs.tx.balances.transferKeepAlive(
                    ethereumSovereignContainerAddress,
                    CONTAINER_TOKEN_TRANSFER_AMOUNT * 2n
                );
                await signAndSendAndInclude(fundEthSovContainerTx, aliceFrontier);

                // === Check balances before ===
                const receiverNativeBalanceBefore = (
                    await containerChainPolkadotJs.query.system.account(tokenReceiver)
                ).data.free.toBigInt();

                // === Generate and submit V2 message ===
                // Fee asset for execution on container (relay token)
                const feeAsset = {
                    id: { parents: 0, interior: "Here" },
                    fun: { Fungible: DEST_FEE_AMOUNT },
                };

                // Container native token reanchored to container's perspective (becomes PalletInstance only)
                const containerNativeReanchored = {
                    parents: 0,
                    interior: { X1: [{ PalletInstance: balancesPalletIndex }] },
                };

                const containerAsset = {
                    id: containerNativeReanchored,
                    fun: { Fungible: CONTAINER_TOKEN_TRANSFER_AMOUNT },
                };

                // Build the XCM following native_container_tokens_processor pattern
                // InitiateTransfer with empty assets, preserve_origin: true, and remote_xcm does WithdrawAsset + DepositAsset
                const instructions = [
                    {
                        InitiateTransfer: {
                            destination: { parents: 0, interior: { X1: [{ Parachain: containerParaId }] } },
                            remoteFees: {
                                ReserveDeposit: { Definite: [feeAsset] },
                            },
                            preserveOrigin: true,
                            assets: [], // Empty assets - container native tokens are withdrawn on the container
                            remoteXcm: [
                                // Withdraw container native tokens from ETH sovereign account on container
                                {
                                    WithdrawAsset: [containerAsset],
                                },
                                // Deposit to receiver
                                {
                                    DepositAsset: {
                                        assets: { Definite: [containerAsset] },
                                        beneficiary: {
                                            parents: 0,
                                            interior: {
                                                X1: [{ AccountKey20: { network: null, key: tokenReceiver } }],
                                            },
                                        },
                                    },
                                },
                            ],
                        },
                    },
                ];

                // For container native transfers, we use foreignTokenParams for both:
                // - Tanssi fees (kind: 1)
                // - Container native token (kind: 1) - registered as foreign token on Ethereum
                const log = await generateOutboundMessageAcceptedLog(
                    relayChainPolkadotJs,
                    1, // nonce
                    0n, // ethValue (no native ETH)
                    instructions,
                    [], // nativeERC20Params (no ERC20)
                    [
                        { tokenId: tanssiTokenId.toString(), value: TANSSI_FEE_AMOUNT }, // Tanssi for fees
                        { tokenId: containerTokenId.toString(), value: CONTAINER_TOKEN_TRANSFER_AMOUNT }, // Container native token
                    ]
                );

                const { checkpointUpdate, messageExtrinsics } = await generateUpdate(relayChainPolkadotJs, [log]);

                // Force checkpoint
                const checkpointTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumBeaconClient.forceCheckpoint(checkpointUpdate)
                );
                await signAndSendAndInclude(checkpointTx, aliceRelay);

                // Submit V2 message
                const submitTx = relayChainPolkadotJs.tx.ethereumInboundQueueV2.submit(messageExtrinsics[0]);
                await signAndSendAndInclude(submitTx, aliceRelay);

                // Wait for XCM message to reach container
                await waitSessions(context, relayChainPolkadotJs, 1, null, "Tanssi-relay");

                // === Verify on Container ===
                const receiverNativeBalanceAfter = (
                    await containerChainPolkadotJs.query.system.account(tokenReceiver)
                ).data.free.toBigInt();

                // Container native balance should increase by transfer amount
                expect(receiverNativeBalanceAfter).to.be.greaterThan(Number(receiverNativeBalanceBefore));
                expect(receiverNativeBalanceAfter - receiverNativeBalanceBefore).to.be.eq(
                    CONTAINER_TOKEN_TRANSFER_AMOUNT
                );
            },
        });
    },
});
