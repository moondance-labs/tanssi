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
    id: "ZOMBIETANSSIETHCONT",
    title: "Native ETH transfer from Ethereum to container chains (via Snowbridge V2)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;

        // Constants matching Rust tests
        const RELAY_NATIVE_TOKEN_ASSET_ID = 42;
        const ETH_ASSET_ID = 99;
        const ETH_TRANSFER_AMOUNT = 50_000_000_000n;
        const TANSSI_FEE_AMOUNT = 10_000_000_000_000n;
        const DEST_FEE_AMOUNT = 5_000_000_000_000n; // Half of TANSSI_FEE_AMOUNT

        beforeAll(async () => {
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
        });

        it({
            id: "T01",
            title: "Should transfer native ETH from Ethereum to Frontier container chain via V2",
            timeout: 300000,
            test: async () => {
                const containerChainPolkadotJs = context.polkadotJs("Container2001");
                const chain = containerChainPolkadotJs.consts.system.version.specName.toString();
                const aliceFrontier = chain === "frontier-template" ? alith : aliceRelay;
                const containerParaId = 2001;

                // Token receiver - AccountKey20 for Frontier
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

                // Get Ethereum sovereign account on relay
                const ethereumSovereignRelay = await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: { parents: 1, interior: { X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET } } },
                });
                const ethereumSovereignRelayAddress = ethereumSovereignRelay.asOk.toHuman();

                // === Setup on Relay Chain ===

                // 1. Register Tanssi native token in EthereumSystemV2
                const tokenLocation = { parents: 0, interior: "Here" };
                const versionedLocation = { V3: tokenLocation };
                const metadata = { name: "relay", symbol: "relay", decimals: 12 };

                const registerTokenTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.ethereumSystemV2.registerToken(
                        versionedLocation,
                        versionedLocation,
                        metadata,
                        1
                    )
                );
                await signAndSendAndInclude(registerTokenTx, aliceRelay);

                // Get Tanssi token ID
                const allEntries = await relayChainPolkadotJs.query.ethereumSystem.nativeToForeignId.entries();
                const tanssiTokenId = allEntries.map(([, id]) => id.toHuman())[0];

                // 2. Fund Ethereum sovereign account on relay
                const fundEthSovTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.balances.forceSetBalance(
                        ethereumSovereignRelayAddress,
                        10_000_000_000_000_000_000n
                    )
                );
                await signAndSendAndInclude(fundEthSovTx, aliceRelay);

                // 3. Register native ETH foreign token on relay
                // ETH asset location has only GlobalConsensus (no AccountKey20 like ERC20)
                const ethAssetLocationRelay = {
                    parents: 1,
                    interior: {
                        X1: [{ GlobalConsensus: ETHEREUM_NETWORK_TESTNET }],
                    },
                };

                const createEthOnRelayTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethAssetLocationRelay,
                        ETH_ASSET_ID,
                        aliceRelay.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(createEthOnRelayTx, aliceRelay);

                // 4. Force XCM version
                const forceXcmVersionTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(forceXcmVersionTx, aliceRelay);

                // === Setup on Container Chain ===

                // 1. Register relay token as foreign in container
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

                // 3. Register native ETH foreign token on container (with parents: 2)
                // ETH asset location has only GlobalConsensus (no AccountKey20 like ERC20)
                const ethAssetLocationContainer = {
                    parents: 2,
                    interior: {
                        X1: [{ GlobalConsensus: ETHEREUM_NETWORK_TESTNET }],
                    },
                };

                const createEthOnContainerTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        ethAssetLocationContainer,
                        ETH_ASSET_ID,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(createEthOnContainerTx, aliceFrontier);

                // === Check balances before ===
                const receiverEthBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(ETH_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // === Generate and submit V2 message ===
                const feeAsset = {
                    id: { parents: 0, interior: "Here" },
                    fun: { Fungible: DEST_FEE_AMOUNT },
                };

                // ETH asset location from relay's perspective (parents: 1)
                const ethAssetForTransfer = {
                    id: {
                        parents: 1,
                        interior: {
                            X1: [{ GlobalConsensus: ETHEREUM_NETWORK_TESTNET }],
                        },
                    },
                    fun: { Fungible: ETH_TRANSFER_AMOUNT },
                };

                const instructions = [
                    {
                        InitiateTransfer: {
                            destination: { parents: 0, interior: { X1: [{ Parachain: containerParaId }] } },
                            remoteFees: {
                                ReserveDeposit: { Definite: [feeAsset] },
                            },
                            preserveOrigin: false,
                            assets: [{ ReserveDeposit: { Definite: [ethAssetForTransfer] } }],
                            remoteXcm: [
                                { RefundSurplus: null },
                                {
                                    DepositAsset: {
                                        assets: { Wild: { AllCounted: 2 } },
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

                // For ETH transfers, we use ethValue instead of nativeERC20Params
                const log = await generateOutboundMessageAcceptedLog(
                    relayChainPolkadotJs,
                    1, // nonce
                    ETH_TRANSFER_AMOUNT, // ethValue - native ETH amount
                    instructions,
                    [], // nativeERC20Params - empty for ETH transfers
                    [{ tokenId: tanssiTokenId.toString(), value: TANSSI_FEE_AMOUNT }] // foreignTokenParams (kind: 1) for Tanssi fees
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
                const receiverEthBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(ETH_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                const receiverTanssiBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(RELAY_NATIVE_TOKEN_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // ETH balance should increase by transfer amount
                expect(receiverEthBalanceAfter).to.be.eq(receiverEthBalanceBefore + ETH_TRANSFER_AMOUNT);

                // Receiver should have some Tanssi tokens from refunded surplus
                expect(receiverTanssiBalanceAfter).toBeGreaterThan(0n);
                expect(receiverTanssiBalanceAfter).toBeLessThan(DEST_FEE_AMOUNT);
            },
        });
    },
});
