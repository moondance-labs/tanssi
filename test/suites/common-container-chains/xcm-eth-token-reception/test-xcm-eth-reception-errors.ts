// @ts-nocheck

import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, alith } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";

import {
    type RawXcmMessage,
    XcmFragment,
    injectDmpMessageAndSeal,
    SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_FRONTIER,
    SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_SUBSTRATE,
    ETHEREUM_NETWORK_TESTNET,
} from "utils";

describeSuite({
    id: "COM0201",
    title: "Mock XCM - Fails receiving container-chain token back from ethereum",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let transferredBalance: bigint;
        let alice: KeyringPair;
        let chain: any;
        let ethereumSovereignAddress: any;
        let balancesPalletIndex: number;
        let containerAssetFee: bigint;
        let bridgeLocation: any;
        let relayNativeTokenLocation: any;
        let relayNativeTokenAssetId: number;
        let containerTokenLocation: any;
        let aliceLocation: any;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            chain = polkadotJs.consts.system.version.specName.toString();

            // Get Pallet balances index
            const metadata = await context.polkadotJs().rpc.state.getMetadata();
            const foundPallet = metadata.asLatest.pallets.find((pallet) => pallet.name.toString() === "Balances");

            if (!foundPallet || !foundPallet.index) {
                throw new Error("Balances pallet or its index not found");
            }

            balancesPalletIndex = foundPallet.index.toNumber();

            // since in the future is likely that we are going to add this to containers, I leave it here
            alice =
                chain === "frontier-template"
                    ? alith
                    : new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                          name: "Alice default",
                      });

            aliceLocation =
                chain === "frontier-template"
                    ? { AccountKey20: { network: "Any", key: alice.address } }
                    : { AccountId32: { network: "Any", id: u8aToHex(alice.addressRaw) } };

            transferredBalance = context.isEthereumChain ? 10_000_000_000_000_000_000n : 10_000_000_000_000n;
            containerAssetFee = 500_000_000_000_000n;

            bridgeLocation = {
                parents: 2,
                interior: {
                    X1: { GlobalConsensus: ETHEREUM_NETWORK_TESTNET },
                },
            };

            ethereumSovereignAddress = context.isEthereumChain
                ? SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_FRONTIER
                : SEPOLIA_CONTAINER_SOVEREIGN_ADDRESS_SUBSTRATE;

            // We transfer double to the sovereign account just in case
            // this is to avoid ED errors
            const txSigned = polkadotJs.tx.balances.transferKeepAlive(
                ethereumSovereignAddress,
                transferredBalance * 2n
            );

            await context.createBlock(await txSigned.signAsync(alice), {
                allowFailures: false,
            });

            containerTokenLocation = {
                parents: 0,
                interior: {
                    X1: { PalletInstance: balancesPalletIndex },
                },
            };

            // Register relay token as foreign in container
            const relayNativeTokenLocation = {
                parents: 1,
                interior: "Here",
            };

            relayNativeTokenAssetId = 42;

            const registerRelayNativeTokenTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.foreignAssetsCreator.createForeignAsset(
                    relayNativeTokenLocation,
                    42,
                    alice.address,
                    true,
                    1
                )
            );

            await context.createBlock(await registerRelayNativeTokenTx.signAsync(alice), {
                allowFailures: false,
            });

            // Create asset rate for tanssi token in container
            const assetRateTx = polkadotJs.tx.sudo.sudo(
                polkadotJs.tx.assetRate.create(
                    42,
                    // this defines how much the asset costs with respect to the
                    // new asset
                    // in this case, asset*2=native
                    // that means that we will charge 0.5 of the native balance
                    2000000000000000000n
                )
            );

            await context.createBlock(await assetRateTx.signAsync(alice), {
                allowFailures: false,
            });
        });

        it({
            id: "T01",
            title: "Should fail receiving tokens with a non-supported aliaser",
            test: async () => {
                const aliceBalanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free;
                const ethSovereignBalanceBefore = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;
                const ethSovereignRelayTokenBalanceBefore = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceBefore).to.be.eq(0n);

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                ReserveAssetDeposited(vec![container_asset_fee.clone()].into()),
                BuyExecution {
                    fees: container_asset_fee,
                    weight_limit: Unlimited,
                },
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                UniversalOrigin(GlobalConsensus(network)),
                WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset_to_deposit.into()),
                    beneficiary,
                },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: Wild(AllOf {
                        id: Location::parent().into(),
                        fun: WildFungibility::Fungible,
                    }),
                    beneficiary: bridge_location,
                }])),
                */
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: relayNativeTokenLocation,
                            fungible: containerAssetFee,
                        },
                    ],
                })
                    .push_any({
                        ReserveAssetDeposited: [
                            {
                                id: {
                                    Concrete: relayNativeTokenLocation,
                                },
                                fun: { Fungible: containerAssetFee },
                            },
                        ],
                    })
                    .buy_execution() // fee index 0
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                PalletInstance: 24,
                            },
                        },
                    })
                    .push_any({
                        UniversalOrigin: {
                            GlobalConsensus: { Ethereum: { chainId: 11155112 } }, // wrong chainId
                        },
                    })
                    .push_any({
                        WithdrawAsset: [
                            {
                                id: {
                                    Concrete: containerTokenLocation,
                                },
                                fun: { Fungible: transferredBalance },
                            },
                        ],
                    })
                    .push_any({
                        DepositAsset: {
                            assets: {
                                Definite: [
                                    {
                                        id: {
                                            Concrete: containerTokenLocation,
                                        },
                                        fun: { Fungible: transferredBalance },
                                    },
                                ],
                            },
                            beneficiary: {
                                parents: 0,
                                interior: { X1: aliceLocation },
                            },
                        },
                    })
                    .push_any({
                        SetAppendix: [
                            {
                                DepositAsset: {
                                    assets: {
                                        Wild: {
                                            AllOf: {
                                                id: { Concrete: relayNativeTokenLocation },
                                                fun: "Fungible",
                                            },
                                        },
                                    },
                                    beneficiary: bridgeLocation,
                                },
                            },
                        ],
                    })
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const aliceBalanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free;

                const ethSovereignBalanceAfter = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // Sovereign balance (relay token) should not have changed
                const ethSovereignRelayTokenBalanceAfter = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceAfter).to.be.eq(ethSovereignRelayTokenBalanceBefore);

                // alice balance should not have changed
                expect(aliceBalanceAfter.toBigInt()).to.be.eq(aliceBalanceBefore.toBigInt());

                // neither the eth sovereign
                expect(ethSovereignBalanceBefore.toBigInt()).to.be.eq(ethSovereignBalanceAfter.toBigInt());
            },
        });
        it({
            id: "T02",
            title: "Should fail receiving tokens from somewhere that is not the proper pallet",
            test: async () => {
                const aliceBalanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free;
                const ethSovereignBalanceBefore = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;
                const ethSovereignRelayTokenBalanceBefore = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceBefore).to.be.eq(0n);

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                ReserveAssetDeposited(vec![container_asset_fee.clone()].into()),
                BuyExecution {
                    fees: container_asset_fee,
                    weight_limit: Unlimited,
                },
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                UniversalOrigin(GlobalConsensus(network)),
                WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset_to_deposit.into()),
                    beneficiary,
                },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: Wild(AllOf {
                        id: Location::parent().into(),
                        fun: WildFungibility::Fungible,
                    }),
                    beneficiary: bridge_location,
                }])),
                */
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: relayNativeTokenLocation,
                            fungible: containerAssetFee,
                        },
                    ],
                })
                    .push_any({
                        ReserveAssetDeposited: [
                            {
                                id: {
                                    Concrete: relayNativeTokenLocation,
                                },
                                fun: { Fungible: containerAssetFee },
                            },
                        ],
                    })
                    .buy_execution() // fee index 0
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                PalletInstance: 25, // wrong pallet index
                            },
                        },
                    })
                    .push_any({
                        UniversalOrigin: {
                            GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                        },
                    })
                    .push_any({
                        WithdrawAsset: [
                            {
                                id: {
                                    Concrete: containerTokenLocation,
                                },
                                fun: { Fungible: transferredBalance },
                            },
                        ],
                    })
                    .push_any({
                        DepositAsset: {
                            assets: {
                                Definite: [
                                    {
                                        id: {
                                            Concrete: containerTokenLocation,
                                        },
                                        fun: { Fungible: transferredBalance },
                                    },
                                ],
                            },
                            beneficiary: {
                                parents: 0,
                                interior: { X1: aliceLocation },
                            },
                        },
                    })
                    .push_any({
                        SetAppendix: [
                            {
                                DepositAsset: {
                                    assets: {
                                        Wild: {
                                            AllOf: {
                                                id: { Concrete: relayNativeTokenLocation },
                                                fun: "Fungible",
                                            },
                                        },
                                    },
                                    beneficiary: bridgeLocation,
                                },
                            },
                        ],
                    })
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const aliceBalanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free;

                const ethSovereignBalanceAfter = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // Sovereign balance (relay token) should not have changed
                const ethSovereignRelayTokenBalanceAfter = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceAfter).to.be.eq(ethSovereignRelayTokenBalanceBefore);

                // alice balance should not have changed
                expect(aliceBalanceAfter.toBigInt()).to.be.eq(aliceBalanceBefore.toBigInt());

                // neither the eth sovereign
                expect(ethSovereignBalanceBefore.toBigInt()).to.be.eq(ethSovereignBalanceAfter.toBigInt());
            },
        });
        it({
            id: "T03",
            title: "Should fail receiving tokens if we request to withdraw more than what the sov account has",
            test: async () => {
                const aliceBalanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free;
                const ethSovereignBalanceBefore = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;
                const ethSovereignRelayTokenBalanceBefore = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceBefore).to.be.eq(0n);

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                ReserveAssetDeposited(vec![container_asset_fee.clone()].into()),
                BuyExecution {
                    fees: container_asset_fee,
                    weight_limit: Unlimited,
                },
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                UniversalOrigin(GlobalConsensus(network)),
                WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset_to_deposit.into()),
                    beneficiary,
                },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: Wild(AllOf {
                        id: Location::parent().into(),
                        fun: WildFungibility::Fungible,
                    }),
                    beneficiary: bridge_location,
                }])),
                */
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: relayNativeTokenLocation,
                            fungible: containerAssetFee,
                        },
                    ],
                })
                    .push_any({
                        ReserveAssetDeposited: [
                            {
                                id: {
                                    Concrete: relayNativeTokenLocation,
                                },
                                fun: { Fungible: containerAssetFee },
                            },
                        ],
                    })
                    .buy_execution() // fee index 0
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                PalletInstance: 24,
                            },
                        },
                    })
                    .push_any({
                        UniversalOrigin: {
                            GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                        },
                    })
                    .push_any({
                        WithdrawAsset: [
                            {
                                id: {
                                    Concrete: containerTokenLocation,
                                },
                                fun: { Fungible: transferredBalance * 10n }, // more than the sovereign account has
                            },
                        ],
                    })
                    .push_any({
                        DepositAsset: {
                            assets: {
                                Definite: [
                                    {
                                        id: {
                                            Concrete: containerTokenLocation,
                                        },
                                        fun: { Fungible: transferredBalance },
                                    },
                                ],
                            },
                            beneficiary: {
                                parents: 0,
                                interior: { X1: aliceLocation },
                            },
                        },
                    })
                    .push_any({
                        SetAppendix: [
                            {
                                DepositAsset: {
                                    assets: {
                                        Wild: {
                                            AllOf: {
                                                id: { Concrete: relayNativeTokenLocation },
                                                fun: "Fungible",
                                            },
                                        },
                                    },
                                    beneficiary: bridgeLocation,
                                },
                            },
                        ],
                    })
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const aliceBalanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free;

                const ethSovereignBalanceAfter = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // Sovereign balance (relay token) should not have changed
                const ethSovereignRelayTokenBalanceAfter = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceAfter).to.be.eq(ethSovereignRelayTokenBalanceBefore);

                // alice balance should not have changed
                expect(aliceBalanceAfter.toBigInt()).to.be.eq(aliceBalanceBefore.toBigInt());

                // neither the sovereign accounbt one
                expect(ethSovereignBalanceBefore.toBigInt()).to.be.eq(ethSovereignBalanceAfter.toBigInt());
            },
        });
        it({
            id: "T04",
            title: "Should fail receiving tokens if we dont have to tokens to pay for execution",
            test: async () => {
                const aliceBalanceBefore = (await polkadotJs.query.system.account(alice.address)).data.free;
                const ethSovereignBalanceBefore = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;
                const ethSovereignRelayTokenBalanceBefore = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceBefore).to.be.eq(0n);

                // Send an XCM and create block to execute it
                // This is composed of
                /*
                ReserveAssetDeposited(vec![container_asset_fee.clone()].into()),
                BuyExecution {
                    fees: container_asset_fee,
                    weight_limit: Unlimited,
                },
                DescendOrigin(PalletInstance(inbound_queue_pallet_index).into()),
                UniversalOrigin(GlobalConsensus(network)),
                WithdrawAsset(vec![container_asset_to_withdraw.clone()].into()),
                DepositAsset {
                    assets: Definite(container_asset_to_deposit.into()),
                    beneficiary,
                },
                SetAppendix(Xcm(vec![DepositAsset {
                    assets: Wild(AllOf {
                        id: Location::parent().into(),
                        fun: WildFungibility::Fungible,
                    }),
                    beneficiary: bridge_location,
                }])),
                */
                const xcmMessage = new XcmFragment({
                    assets: [
                        {
                            multilocation: relayNativeTokenLocation,
                            fungible: 1n, // not enough to pay for execution
                        },
                    ],
                })
                    .push_any({
                        ReserveAssetDeposited: [
                            {
                                id: {
                                    Concrete: relayNativeTokenLocation,
                                },
                                fun: { Fungible: containerAssetFee },
                            },
                        ],
                    })
                    .buy_execution() // fee index 0
                    .push_any({
                        DescendOrigin: {
                            X1: {
                                PalletInstance: 24,
                            },
                        },
                    })
                    .push_any({
                        UniversalOrigin: {
                            GlobalConsensus: ETHEREUM_NETWORK_TESTNET,
                        },
                    })
                    .push_any({
                        WithdrawAsset: [
                            {
                                id: {
                                    Concrete: containerTokenLocation,
                                },
                                fun: { Fungible: transferredBalance },
                            },
                        ],
                    })
                    .push_any({
                        DepositAsset: {
                            assets: {
                                Definite: [
                                    {
                                        id: {
                                            Concrete: containerTokenLocation,
                                        },
                                        fun: { Fungible: transferredBalance },
                                    },
                                ],
                            },
                            beneficiary: {
                                parents: 0,
                                interior: { X1: aliceLocation },
                            },
                        },
                    })
                    .push_any({
                        SetAppendix: [
                            {
                                DepositAsset: {
                                    assets: {
                                        Wild: {
                                            AllOf: {
                                                id: { Concrete: relayNativeTokenLocation },
                                                fun: "Fungible",
                                            },
                                        },
                                    },
                                    beneficiary: bridgeLocation,
                                },
                            },
                        ],
                    })
                    .as_v3();

                // Send an XCM and create block to execute it
                await injectDmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                // Create a block in which the XCM will be executed
                await context.createBlock();

                const aliceBalanceAfter = (await polkadotJs.query.system.account(alice.address)).data.free;

                const ethSovereignBalanceAfter = (await polkadotJs.query.system.account(ethereumSovereignAddress)).data
                    .free;

                // Sovereign balance (relay token) should not have changed
                const ethSovereignRelayTokenBalanceAfter = (
                    await polkadotJs.query.foreignAssets.account(relayNativeTokenAssetId, ethereumSovereignAddress)
                )
                    .unwrapOrDefault()
                    .balance.toBigInt();
                expect(ethSovereignRelayTokenBalanceAfter).to.be.eq(ethSovereignRelayTokenBalanceBefore);

                // alice balance should not have changed
                expect(aliceBalanceAfter.toBigInt()).to.be.eq(aliceBalanceBefore.toBigInt());

                // Since we pay fees with the relay token, the sovereign balance should not have changed either
                expect(ethSovereignBalanceBefore.toBigInt()).to.be.eq(ethSovereignBalanceAfter.toBigInt());
            },
        });
    },
});
