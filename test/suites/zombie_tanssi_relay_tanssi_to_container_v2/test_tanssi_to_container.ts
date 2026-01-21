import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";

import { generateOutboundMessageAcceptedLog, generateUpdate, signAndSendAndInclude, waitSessions } from "utils";

describeSuite({
    id: "ZOMBIETANSSITANSSICONT",
    title: "Tanssi native token transfer from Ethereum to container chains (via Snowbridge V2)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;

        // Constants matching Rust tests
        const RELAY_NATIVE_TOKEN_ASSET_ID = 42;
        // Tanssi amounts matching Rust test values
        const TANSSI_WITHDRAW_AMOUNT = 50_000_000_000_000n; // 50 Tanssi tokens
        const TANSSI_TRANSFER_AMOUNT = 43_000_000_000_000n; // 43 Tanssi tokens to transfer
        const DEST_FEE_AMOUNT = 5_000_000_000_000n; // Fees for execution on container

        beforeAll(async () => {
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            aliceRelay = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
        });

        it({
            id: "T01",
            title: "Should transfer Tanssi native token from Ethereum to Frontier container chain via V2",
            timeout: 300000,
            test: async () => {
                const containerChainPolkadotJs = context.polkadotJs("Container2001");
                const chain = containerChainPolkadotJs.consts.system.version.specName.toString();
                const aliceFrontier = chain === "frontier-template" ? alith : aliceRelay;
                const containerParaId = 2001;

                // Token receiver - AccountKey20 for Frontier
                const tokenReceiver = "0x0505050505050505050505050505050505050505";

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
                const ethereumSovereignRelay = await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: {
                        parents: 1,
                        interior: {
                            X1: {
                                GlobalConsensus: {
                                    Ethereum: { chainId: 11155111 },
                                },
                            },
                        },
                    },
                });
                const ethereumSovereignRelayAddress = ethereumSovereignRelay.asOk.toHuman();

                const fundEthSovTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.balances.forceSetBalance(
                        ethereumSovereignRelayAddress,
                        10_000_000_000_000_000_000n
                    )
                );
                await signAndSendAndInclude(fundEthSovTx, aliceRelay);

                // 3. Force XCM version
                const forceXcmVersionTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.xcmPallet.forceDefaultXcmVersion(5)
                );
                await signAndSendAndInclude(forceXcmVersionTx, aliceRelay);

                // === Setup on Container Chain ===

                // 1. Register relay token (Tanssi) as foreign asset in container
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

                // === Check balances before ===
                const receiverTanssiBalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        RELAY_NATIVE_TOKEN_ASSET_ID,
                        tokenReceiver
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // === Generate and submit V2 message ===
                // Both fee and transfer assets are Tanssi (Location::here() from relay perspective)
                const feeAsset = {
                    id: { parents: 0, interior: "Here" },
                    fun: { Fungible: DEST_FEE_AMOUNT },
                };

                const tanssiTransferAsset = {
                    id: { parents: 0, interior: "Here" },
                    fun: { Fungible: TANSSI_TRANSFER_AMOUNT },
                };

                const instructions = [
                    {
                        InitiateTransfer: {
                            destination: { parents: 0, interior: { X1: [{ Parachain: containerParaId }] } },
                            remoteFees: {
                                ReserveDeposit: { Definite: [feeAsset] },
                            },
                            preserveOrigin: false,
                            assets: [{ ReserveDeposit: { Definite: [tanssiTransferAsset] } }],
                            remoteXcm: [
                                // No RefundSurplus - we want to test exact transfer amount as in Rust test
                                {
                                    DepositAsset: {
                                        assets: { Wild: { AllCounted: 1 } },
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

                // For Tanssi transfers, we use foreignTokenParams with the full withdraw amount
                // The XCM handles splitting between fees and transfer
                const log = await generateOutboundMessageAcceptedLog(
                    relayChainPolkadotJs,
                    1, // nonce
                    0n, // ethValue (no native ETH)
                    instructions,
                    [], // nativeERC20Params (no ERC20)
                    [{ tokenId: tanssiTokenId.toString(), value: TANSSI_WITHDRAW_AMOUNT }] // foreignTokenParams (kind: 1) for Tanssi
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
                const receiverTanssiBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(
                        RELAY_NATIVE_TOKEN_ASSET_ID,
                        tokenReceiver
                    )
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // Tanssi balance should increase by transfer amount
                expect(receiverTanssiBalanceAfter).to.be.eq(receiverTanssiBalanceBefore + TANSSI_TRANSFER_AMOUNT);
            },
        });
    },
});
