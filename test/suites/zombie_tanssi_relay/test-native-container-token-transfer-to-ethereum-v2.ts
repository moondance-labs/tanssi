import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import {
    SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS,
    sleep,
    TESTNET_ETHEREUM_NETWORK_ID,
    waitEventUntilTimeout,
    SNOWBRIDGE_FEES_ACCOUNT,
    DANCELIGHT_GENESIS_HASH,
} from "utils";

describeSuite({
    id: "ZOMBIETANSS05",
    title: "XCM transfer to Ethereum (Snowbridge V2)",
    foundationMethods: "zombie",
    testCases: ({ context, it }) => {
        let containerChainPolkadotJs: ApiPromise;
        let relayChainPolkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceAccount32: KeyringPair;
        let chain: string;

        // Random ETH destination that we send asset to
        const destinationAddress = "0x1234567890abcdef1234567890abcdef12345678";
        const ethereumSovereignAccountAddress = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        const tokenToTransfer = 123_321_000_000_000_000n;

        beforeAll(async () => {
            containerChainPolkadotJs = context.polkadotJs("Container2001");
            relayChainPolkadotJs = context.polkadotJs("Tanssi-relay");
            chain = containerChainPolkadotJs.consts.system.version.specName.toString();
            aliceAccount32 = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });
            alice = chain === "frontier-template" ? alith : aliceAccount32;
        });

        it({
            id: "T01",
            title: "Should allow sending asset to Ethereum",
            test: async () => {
                // Register relay token as foreign in container
                const relayNativeTokenAssetId = 42;
                const relayNativeTokenLocation = {
                    parents: 1,
                    interior: "Here",
                };

                const containerBatchTx = await containerChainPolkadotJs.tx.utility
                    .batchAll([
                        containerChainPolkadotJs.tx.sudo.sudo(
                            containerChainPolkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                                relayNativeTokenLocation,
                                relayNativeTokenAssetId,
                                u8aToHex(alice.addressRaw),
                                true,
                                1
                            )
                        ),
                        containerChainPolkadotJs.tx.foreignAssets.mint(
                            relayNativeTokenAssetId,
                            u8aToHex(alice.addressRaw),
                            200000000000000000000n
                        ),
                        containerChainPolkadotJs.tx.sudo.sudo(
                            containerChainPolkadotJs.tx.assetRate.create(
                                relayNativeTokenAssetId,
                                // this defines how much the asset costs with respect to the
                                // new asset
                                // in this case, asset*2=native
                                // that means that we will charge 0.5 of the native balance
                                2000000000000000000n
                            )
                        ),
                    ])
                    .signAndSend(alice);

                expect(!!containerBatchTx.toHuman()).to.be.true;

                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
                const convertLocation = await relayChainPolkadotJs.call.locationToAccountApi.convertLocation({
                    V3: { parents: 0, interior: { X1: { Parachain: 2001 } } },
                });
                const containerSovereignAccountAddress = convertLocation.asOk.toHuman();

                console.log("Container sovereign account address:", containerSovereignAccountAddress);

                const versionedLocation = {
                    V5: {
                        parents: 1,
                        interior: {
                            X3: [
                                {
                                    GlobalConsensus: {
                                        ByGenesis: DANCELIGHT_GENESIS_HASH,
                                    },
                                },
                                {
                                    Parachain: 2001,
                                },
                                {
                                    PalletInstance: 10,
                                },
                            ],
                        },
                    },
                };

                const containerTokenMetadata = {
                    name: "para2001",
                    symbol: "para2001",
                    decimals: 12,
                };

                const initialBalance = 100_000_000_000_000n;
                const txHash = await relayChainPolkadotJs.tx.utility
                    .batch([
                        relayChainPolkadotJs.tx.sudo.sudo(
                            relayChainPolkadotJs.tx.balances.forceSetBalance(
                                containerSovereignAccountAddress,
                                initialBalance
                            )
                        ),
                        relayChainPolkadotJs.tx.sudo.sudo(
                            relayChainPolkadotJs.tx.ethereumSystem.registerToken(
                                versionedLocation,
                                containerTokenMetadata
                            )
                        ),
                    ])
                    .signAndSend(aliceAccount32);

                expect(!!txHash.toHuman()).to.be.true;

                await sleep(24000);

                // Check balance before transfer
                const ethereumSovereignAccountBalanceBefore = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();
                const beneficiaryOnDest = {
                    parents: 0,
                    interior: {
                        X1: [
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: destinationAddress,
                                },
                            },
                        ],
                    },
                };
                const metadata = await containerChainPolkadotJs.rpc.state.getMetadata();
                const balancesPalletIndex = metadata.asLatest.pallets
                    .find(({ name }) => name.toString() === "Balances")
                    .index.toNumber();

                const assetToTransferNative = {
                    id: {
                        parents: 0,
                        interior: {
                            X1: [{ PalletInstance: balancesPalletIndex }],
                        },
                    },
                    fun: { Fungible: tokenToTransfer },
                };

                const assetToTransferNativeReanchored = {
                    id: {
                        parents: 1,
                        interior: {
                            X3: [
                                {
                                    GlobalConsensus: {
                                        ByGenesis: DANCELIGHT_GENESIS_HASH,
                                    },
                                },
                                {
                                    Parachain: 2001,
                                },
                                { PalletInstance: balancesPalletIndex },
                            ],
                        },
                    },
                    fun: { Fungible: tokenToTransfer },
                };

                const nativeAssetToWithdraw = {
                    id: {
                        parents: 0,
                        interior: {
                            X1: [{ PalletInstance: balancesPalletIndex }],
                        },
                    },
                    fun: { Fungible: tokenToTransfer * 1000n },
                };

                // Specify ethereum destination with global consensus
                const dest = {
                    parents: 2,
                    interior: {
                        X1: [
                            {
                                GlobalConsensus: ethereumNetwork,
                            },
                        ],
                    },
                };

                const v2NonceBefore = await relayChainPolkadotJs.query.ethereumOutboundQueueV2.nonce();

                const feesAccountBalanceBefore = (
                    await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const relayFee = 500_000_000n;

                const relayAssetFeeToWithdraw = {
                    id: relayNativeTokenLocation,
                    fun: { Fungible: relayFee },
                };

                const xcmMessage = {
                    V5: [
                        {
                            WithdrawAsset: [nativeAssetToWithdraw, relayAssetFeeToWithdraw],
                        },
                        {
                            InitiateTransfer: {
                                destination: dest,
                                remoteFees: {
                                    ReserveWithdraw: {
                                        Definite: [
                                            {
                                                id: relayNativeTokenLocation,
                                                fun: { Fungible: relayFee },
                                            },
                                        ],
                                    },
                                },
                                preserveOrigin: true,
                                assets: [
                                    {
                                        ReserveDeposit: {
                                            Definite: [assetToTransferNative],
                                        },
                                    },
                                ],
                                remoteXcm: [
                                    {
                                        DepositAsset: {
                                            assets: { Definite: [assetToTransferNativeReanchored] },
                                            beneficiary: beneficiaryOnDest,
                                        },
                                    },
                                ],
                            },
                        },
                    ],
                };

                await sleep(24000);

                const weight = await containerChainPolkadotJs.call.xcmPaymentApi.queryXcmWeight(xcmMessage as any);
                console.log("Weight: ", weight.toHuman());

                await containerChainPolkadotJs.tx.polkadotXcm
                    .execute(xcmMessage as any, {
                        refTime: 100000000000,
                        proofSize: 100000,
                    })
                    .signAndSend(alice);

                await waitEventUntilTimeout(relayChainPolkadotJs, "ethereumOutboundQueueV2.MessageAccepted", 90000);

                // Wait a few blocks until nonce has been increased
                await sleep(24000);

                const ethereumSovereignAccountBalanceAfter = (
                    await containerChainPolkadotJs.query.system.account(ethereumSovereignAccountAddress)
                ).data.free.toBigInt();

                expect(ethereumSovereignAccountBalanceAfter - ethereumSovereignAccountBalanceBefore).toEqual(
                    tokenToTransfer
                );

                const v2NonceAfter = await relayChainPolkadotJs.query.ethereumOutboundQueueV2.nonce();

                // Wait a few blocks until fees are collected
                await sleep(24000);

                // Fees are collected on Tanssi
                const feesAccountBalanceAfter = (
                    await relayChainPolkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();
                expect(feesAccountBalanceAfter).toBeGreaterThan(feesAccountBalanceBefore);

                // Check that the container chain sovereign account balance (in Tanssi) has been reduced
                const containerSovereignAccountBalance = (
                    await relayChainPolkadotJs.query.system.account(containerSovereignAccountAddress)
                ).data.free.toBigInt();
                expect(containerSovereignAccountBalance).toBeLessThan(initialBalance);

                // Check we are in range
                const exporterFees = feesAccountBalanceAfter - feesAccountBalanceBefore;
                const roundingNonExporterFees = 80_000_000n;
                expect(containerSovereignAccountBalance).toBeGreaterThan(
                    initialBalance - exporterFees - roundingNonExporterFees
                );

                // Check that nonce has changed
                expect(Number(v2NonceAfter.toHuman()) - Number(v2NonceBefore.toHuman())).toEqual(1);
            },
        });
    },
});
