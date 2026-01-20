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
    id: "ZOMBIETANSSIERC20CONT",
    title: "ERC20 token transfer from Ethereum to container chains (via Snowbridge V2)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let relayChainPolkadotJs: ApiPromise;
        let aliceRelay: KeyringPair;

        // Constants matching Rust tests
        const RELAY_NATIVE_TOKEN_ASSET_ID = 42;
        const ERC20_ASSET_ID = 24;
        const TRANSFER_AMOUNT = 100_000_000n;
        const TOKEN_ADDRESS = "0x1111111111111111111111111111111111111111";
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
            title: "Should transfer ERC20 token from Ethereum to Frontier container chain via V2",
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

                // 3. Register ERC20 foreign token on relay
                const erc20AssetLocationRelay = {
                    parents: 1,
                    interior: {
                        X2: [
                            { GlobalConsensus: ETHEREUM_NETWORK_TESTNET },
                            { AccountKey20: { network: ETHEREUM_NETWORK_TESTNET, key: TOKEN_ADDRESS } },
                        ],
                    },
                };

                const createErc20OnRelayTx = relayChainPolkadotJs.tx.sudo.sudo(
                    relayChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        erc20AssetLocationRelay,
                        ERC20_ASSET_ID,
                        aliceRelay.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(createErc20OnRelayTx, aliceRelay);

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

                // 3. Register ERC20 foreign token on container (with parents: 2)
                const erc20AssetLocationContainer = {
                    parents: 2,
                    interior: {
                        X2: [
                            { GlobalConsensus: ETHEREUM_NETWORK_TESTNET },
                            { AccountKey20: { network: ETHEREUM_NETWORK_TESTNET, key: TOKEN_ADDRESS } },
                        ],
                    },
                };

                const createErc20OnContainerTx = containerChainPolkadotJs.tx.sudo.sudo(
                    containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                        erc20AssetLocationContainer,
                        ERC20_ASSET_ID,
                        aliceFrontier.address,
                        true,
                        1
                    )
                );
                await signAndSendAndInclude(createErc20OnContainerTx, aliceFrontier);

                // === Check balances before ===
                const receiverErc20BalanceBefore = (
                    await containerChainPolkadotJs.query.foreignAssets.account(ERC20_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // === Generate and submit V2 message ===
                const feeAsset = {
                    id: { parents: 0, interior: "Here" },
                    fun: { Fungible: DEST_FEE_AMOUNT },
                };

                const erc20AssetForTransfer = {
                    id: {
                        parents: 1,
                        interior: {
                            X2: [
                                { GlobalConsensus: ETHEREUM_NETWORK_TESTNET },
                                { AccountKey20: { network: ETHEREUM_NETWORK_TESTNET, key: TOKEN_ADDRESS } },
                            ],
                        },
                    },
                    fun: { Fungible: TRANSFER_AMOUNT },
                };

                const instructions = [
                    {
                        InitiateTransfer: {
                            destination: { parents: 0, interior: { X1: [{ Parachain: containerParaId }] } },
                            remoteFees: {
                                ReserveDeposit: { Definite: [feeAsset] },
                            },
                            preserveOrigin: false,
                            assets: [{ ReserveDeposit: { Definite: [erc20AssetForTransfer] } }],
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

                const log = await generateOutboundMessageAcceptedLog(
                    relayChainPolkadotJs,
                    1, // nonce
                    0n, // ethValue (no native ETH)
                    instructions,
                    [{ tokenAddress: TOKEN_ADDRESS, value: TRANSFER_AMOUNT }], // nativeERC20Params (kind: 0)
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
                const receiverErc20BalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(ERC20_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                const receiverTanssiBalanceAfter = (
                    await containerChainPolkadotJs.query.foreignAssets.account(RELAY_NATIVE_TOKEN_ASSET_ID, tokenReceiver)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();

                // ERC20 balance should increase by transfer amount
                expect(receiverErc20BalanceAfter).to.be.eq(receiverErc20BalanceBefore + TRANSFER_AMOUNT);

                // Receiver should have some Tanssi tokens from refunded surplus
                expect(receiverTanssiBalanceAfter).toBeGreaterThan(0n);
                expect(receiverTanssiBalanceAfter).toBeLessThan(DEST_FEE_AMOUNT);
            },
        });
    },
});
