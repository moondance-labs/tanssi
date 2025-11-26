// @ts-nocheck

import { beforeAll, customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { type KeyringPair, generateKeyringPair, filterAndApply } from "@moonwall/util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import {
    type RawXcmMessage,
    XcmFragment,
    injectUmpMessageAndSeal,
    isStarlightRuntime,
    jumpToSession,
    TESTNET_ETHEREUM_NETWORK_ID,
    SNOWBRIDGE_FEES_ACCOUNT,
    sleep,
    DANCELIGHT_GENESIS_HASH,
} from "utils";
import {
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS,
    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2,
} from "helpers";
import type { EventRecord } from "@polkadot/types/interfaces";
import { hexToU8a, u8aToHex } from "@polkadot/util";

describeSuite({
    id: "DTR2004",
    title: "Snowbridge V2 - Succeeds sending container token to eth from para",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let random: KeyringPair;
        let transferredBalance: bigint;
        let isStarlight: boolean;
        let specVersion: number;
        let shouldSkipStarlightContainerExport: boolean;
        let containerAsset: any;
        let tokenTransferChannel: any;
        let containerTokenLocation;
        let ethLocation;
        let ethereumNetwork;
        let tokenAddress;
        let sovAddress;
        let sovAddressHex;
        let assetId: number;

        beforeAll(async () => {
            polkadotJs = context.polkadotJs();
            alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice", {
                name: "Alice default",
            });

            random = generateKeyringPair("sr25519");

            isStarlight = isStarlightRuntime(polkadotJs);
            specVersion = polkadotJs.consts.system.version.specVersion.toNumber();
            shouldSkipStarlightContainerExport =
                isStarlight &&
                (STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_CONTAINER_EXPORTS.includes(specVersion) ||
                    STARLIGHT_VERSIONS_TO_EXCLUDE_FROM_SNOWBRIDGE_V2);

            if (shouldSkipStarlightContainerExport) {
                console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                return;
            }

            assetId = 42;

            transferredBalance = 100_000_000_000_000_000n;

            const location = {
                V3: {
                    parents: 0,
                    interior: { X1: { Parachain: 2000 } },
                },
            };

            const locationToAccountResult = await polkadotJs.call.locationToAccountApi.convertLocation(location);
            expect(locationToAccountResult.isOk);

            sovAddress = locationToAccountResult.asOk.toJSON();
            sovAddressHex = "0x70617261d0070000000000000000000000000000000000000000000000000000";

            let aliceNonce = (await polkadotJs.query.system.account(alice.address)).nonce.toNumber();

            // Send some tokens to the sovereign account of para 2000
            const txSigned = polkadotJs.tx.balances.transferAllowDeath(sovAddress, transferredBalance);
            await context.createBlock(await txSigned.signAsync(alice, { nonce: aliceNonce++ }), {
                allowFailures: false,
            });

            const balanceSigned = (await polkadotJs.query.system.account(sovAddress)).data.free.toBigInt();
            expect(balanceSigned).to.eq(transferredBalance);
            ethereumNetwork = isStarlight ? { Ethereum: { chainId: 1 } } : { Ethereum: { chainId: 11155111 } };
            tokenAddress = hexToU8a("deadbeefdeadbeefdeadbeefdeadbeefdeadbeef");

            // Specify ethereum destination with global consensus
            ethLocation = {
                parents: 1,
                interior: {
                    X1: [
                        {
                            GlobalConsensus: ethereumNetwork,
                        },
                    ],
                },
            };
            // Register container token on EthereumSystem.
            containerTokenLocation = {
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
                                Parachain: 2000,
                            },
                            {
                                PalletInstance: 10,
                            },
                        ],
                    },
                },
            };
            const initialBalance = 100_000_000_000_000n;

            const containerTokenMetadata = {
                name: "para2001",
                symbol: "para2001",
                decimals: 12,
            };

            // Register container token on EthereumSystem.
            const registerContainerTokenTx = await polkadotJs.tx.sudo
                .sudo(polkadotJs.tx.ethereumSystem.registerToken(containerTokenLocation, containerTokenMetadata))
                .signAsync(alice);

            await context.createBlock([registerContainerTokenTx], { allowFailures: false });

            const setBalanceTx = await polkadotJs.tx.sudo
                .sudo(polkadotJs.tx.balances.forceSetBalance(sovAddress, initialBalance))
                .signAsync(alice);

            await context.createBlock([setBalanceTx], { allowFailures: false });
        });

        it({
            id: "T01",
            title: "Should fail sending container token to eth if export fee is not in holding",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const transferAmount = 100_000n;
                const exportFeeAmount = 500_000_000n;
                const beneficiaryOnDest = {
                    parents: 0,
                    interior: {
                        X1: [
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: "0x1111111111111111111111111111111111111111",
                                },
                            },
                        ],
                    },
                };

                const exportFeeTokenLocation = {
                    parents: 1,
                    interior: {
                        X1: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: DANCELIGHT_GENESIS_HASH,
                                },
                            },
                        ],
                    },
                };

                const exportFeeAssetToWithdraw = {
                    id: exportFeeTokenLocation,
                    fun: { Fungible: exportFeeAmount },
                };

                const containerTokenLocationFromParent = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { PalletInstance: 10 }] },
                };

                const containerTokenToDeposit = {
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
                                    Parachain: 2000,
                                },
                                { PalletInstance: 10 },
                            ],
                        },
                    },
                    fun: { Fungible: transferAmount },
                };

                const aliceContainerOrigin = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { AccountId32: { id: u8aToHex(alice.addressRaw) } }] },
                };

                const exportMessage = [
                    {
                        WithdrawAsset: [exportFeeAssetToWithdraw],
                    },
                    {
                        PayFees: {
                            asset: exportFeeAssetToWithdraw,
                        },
                    },
                    {
                        ReserveAssetDeposited: [containerTokenToDeposit],
                    },
                    {
                        AliasOrigin: aliceContainerOrigin,
                    },
                    {
                        DepositAsset: {
                            assets: { Definite: [containerTokenToDeposit] },
                            beneficiary: beneficiaryOnDest,
                        },
                    },
                    {
                        SetTopic: [
                            57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121, 88, 234, 173,
                            250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                        ],
                    },
                ];

                const feeTokenLocation = {
                    parents: 0,
                    interior: { Here: null },
                };

                // This fee only covers XCM execution on Tanssi.
                // It doesn't cover the export fee, so the outbound sending should fail.
                const overalFeeAmount = 70_000_000n;

                const feeAssetToWithdraw = {
                    id: feeTokenLocation,
                    fun: { Fungible: overalFeeAmount },
                };

                const xcmMessage = {
                    V5: [
                        {
                            WithdrawAsset: [feeAssetToWithdraw],
                        },
                        {
                            BuyExecution: {
                                fees: feeAssetToWithdraw,
                                weight_limit: "Unlimited",
                            },
                        },
                        {
                            SetAppendix: [
                                {
                                    DepositAsset: {
                                        assets: { Wild: { AllCounted: 1 } },
                                        beneficiary: {
                                            parents: 0,
                                            interior: { X1: [{ AccountId32: { id: sovAddressHex } }] },
                                        },
                                    },
                                },
                            ],
                        },
                        {
                            ExportMessage: {
                                network: ethereumNetwork,
                                destination: "Here",
                                xcm: exportMessage,
                            },
                        },
                    ],
                };

                await context.createBlock();

                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                const tokenTransferNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceBefore = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                // Wait until message is processed
                // we need to wait until session 3 for sure so that paras produce blocks
                await jumpToSession(context, 3);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();
                await context.createBlock();

                // Things to verify:
                // 1. ethereumOutboundQueueV2 nonce should not change
                // 2. container sov account pays only the XCM fee on Tanssi, but not the export fee.
                // 3. There's no pending order for such nonce.
                const tokenTransferNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceAfter = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                const pendingOrder =
                    await polkadotJs.query.ethereumOutboundQueueV2.pendingOrders(tokenTransferNonceAfter);

                expect(tokenTransferNonceAfter.toNumber()).to.be.equal(tokenTransferNonceBefore.toNumber());
                expect(snowbridgeFeesAccountBalanceAfter).to.be.eq(snowbridgeFeesAccountBalanceBefore);
                expect(
                    containerSovereignAccountBalanceBefore - containerSovereignAccountBalanceAfter
                ).to.be.lessThanOrEqual(overalFeeAmount);

                expect(pendingOrder.toHuman()).to.be.null;
            },
        });

        it({
            id: "T02",
            title: "Should fail sending container token to eth if export fee is lower than the minimum",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const transferAmount = 100_000n;
                // Minimum export fee is 1, so we put 0 to test the failure.
                const exportFeeAmount = 0n;
                const beneficiaryOnDest = {
                    parents: 0,
                    interior: {
                        X1: [
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: "0x1111111111111111111111111111111111111111",
                                },
                            },
                        ],
                    },
                };

                const exportFeeTokenLocation = {
                    parents: 1,
                    interior: {
                        X1: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: DANCELIGHT_GENESIS_HASH,
                                },
                            },
                        ],
                    },
                };

                const exportFeeAssetToWithdraw = {
                    id: exportFeeTokenLocation,
                    fun: { Fungible: exportFeeAmount },
                };

                const containerTokenLocationFromParent = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { PalletInstance: 10 }] },
                };

                const containerTokenToDeposit = {
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
                                    Parachain: 2000,
                                },
                                { PalletInstance: 10 },
                            ],
                        },
                    },
                    fun: { Fungible: transferAmount },
                };

                const aliceContainerOrigin = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { AccountId32: { id: u8aToHex(alice.addressRaw) } }] },
                };

                const exportMessage = [
                    {
                        WithdrawAsset: [exportFeeAssetToWithdraw],
                    },
                    {
                        PayFees: {
                            asset: exportFeeAssetToWithdraw,
                        },
                    },
                    {
                        ReserveAssetDeposited: [containerTokenToDeposit],
                    },
                    {
                        AliasOrigin: aliceContainerOrigin,
                    },
                    {
                        DepositAsset: {
                            assets: { Definite: [containerTokenToDeposit] },
                            beneficiary: beneficiaryOnDest,
                        },
                    },
                    {
                        SetTopic: [
                            57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121, 88, 234, 173,
                            250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                        ],
                    },
                ];

                const feeTokenLocation = {
                    parents: 0,
                    interior: { Here: null },
                };

                const overalFeeAmount = 500_000_000n;

                const feeAssetToWithdraw = {
                    id: feeTokenLocation,
                    fun: { Fungible: overalFeeAmount },
                };

                const xcmMessage = {
                    V5: [
                        {
                            WithdrawAsset: [feeAssetToWithdraw],
                        },
                        {
                            BuyExecution: {
                                fees: feeAssetToWithdraw,
                                weight_limit: "Unlimited",
                            },
                        },
                        {
                            SetAppendix: [
                                {
                                    DepositAsset: {
                                        assets: { Wild: { AllCounted: 1 } },
                                        beneficiary: {
                                            parents: 0,
                                            interior: { X1: [{ AccountId32: { id: sovAddressHex } }] },
                                        },
                                    },
                                },
                            ],
                        },
                        {
                            ExportMessage: {
                                network: ethereumNetwork,
                                destination: "Here",
                                xcm: exportMessage,
                            },
                        },
                    ],
                };

                await context.createBlock();

                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                const tokenTransferNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceBefore = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                // Wait until message is processed
                // we need to wait until session 3 for sure so that paras produce blocks
                await jumpToSession(context, 3);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();
                await context.createBlock();

                // Things to verify:
                // 1. ethereumOutboundQueueV2 nonce should not change
                // 2. container sov account pays only the XCM fee on Tanssi, but not the export fee.
                // 3. There's no pending order for such nonce.
                const tokenTransferNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceAfter = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                const pendingOrder =
                    await polkadotJs.query.ethereumOutboundQueueV2.pendingOrders(tokenTransferNonceAfter);

                expect(tokenTransferNonceAfter.toNumber()).to.be.equal(tokenTransferNonceBefore.toNumber());
                expect(snowbridgeFeesAccountBalanceAfter).to.be.eq(snowbridgeFeesAccountBalanceBefore);
                expect(
                    containerSovereignAccountBalanceBefore - containerSovereignAccountBalanceAfter
                ).to.be.lessThanOrEqual(overalFeeAmount);

                expect(pendingOrder.toHuman()).to.be.null;
            },
        });

        it({
            id: "T03",
            title: "Should succeed sending container token to eth from container 2000",
            test: async () => {
                if (shouldSkipStarlightContainerExport) {
                    console.log(`Skipping XCM tests for Starlight version ${specVersion}`);
                    return;
                }

                const transferAmount = 100_000n;
                const exportFeeAmount = 500_000_000n;
                const beneficiaryOnDest = {
                    parents: 0,
                    interior: {
                        X1: [
                            {
                                AccountKey20: {
                                    network: ethereumNetwork,
                                    key: "0x1111111111111111111111111111111111111111",
                                },
                            },
                        ],
                    },
                };

                const exportFeeTokenLocation = {
                    parents: 1,
                    interior: {
                        X1: [
                            {
                                GlobalConsensus: {
                                    ByGenesis: DANCELIGHT_GENESIS_HASH,
                                },
                            },
                        ],
                    },
                };

                const exportFeeAssetToWithdraw = {
                    id: exportFeeTokenLocation,
                    fun: { Fungible: exportFeeAmount },
                };

                const containerTokenLocationFromParent = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { PalletInstance: 10 }] },
                };

                const containerTokenToDeposit = {
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
                                    Parachain: 2000,
                                },
                                { PalletInstance: 10 },
                            ],
                        },
                    },
                    fun: { Fungible: transferAmount },
                };

                const aliceContainerOrigin = {
                    parents: 0,
                    interior: { X2: [{ Parachain: 2000 }, { AccountId32: { id: u8aToHex(alice.addressRaw) } }] },
                };

                const exportMessage = [
                    {
                        WithdrawAsset: [exportFeeAssetToWithdraw],
                    },
                    {
                        PayFees: {
                            asset: exportFeeAssetToWithdraw,
                        },
                    },
                    {
                        ReserveAssetDeposited: [containerTokenToDeposit],
                    },
                    {
                        AliasOrigin: aliceContainerOrigin,
                    },
                    {
                        DepositAsset: {
                            assets: { Definite: [containerTokenToDeposit] },
                            beneficiary: beneficiaryOnDest,
                        },
                    },
                    {
                        SetTopic: [
                            57, 238, 159, 80, 83, 113, 184, 105, 108, 164, 73, 6, 134, 160, 7, 234, 121, 88, 234, 173,
                            250, 136, 18, 29, 1, 204, 109, 70, 45, 3, 160, 251,
                        ],
                    },
                ];

                const feeTokenLocation = {
                    parents: 0,
                    interior: { Here: null },
                };
                const overalFeeAmount = 2_700_000_000_000n;

                const feeAssetToWithdraw = {
                    id: feeTokenLocation,
                    fun: { Fungible: overalFeeAmount },
                };

                const xcmMessage = {
                    V5: [
                        {
                            WithdrawAsset: [feeAssetToWithdraw],
                        },
                        {
                            BuyExecution: {
                                fees: feeAssetToWithdraw,
                                weight_limit: "Unlimited",
                            },
                        },
                        {
                            SetAppendix: [
                                {
                                    DepositAsset: {
                                        assets: { Wild: { AllCounted: 1 } },
                                        beneficiary: {
                                            parents: 0,
                                            interior: { X1: [{ AccountId32: { id: sovAddressHex } }] },
                                        },
                                    },
                                },
                            ],
                        },
                        {
                            ExportMessage: {
                                network: ethereumNetwork,
                                destination: "Here",
                                xcm: exportMessage,
                            },
                        },
                    ],
                };

                await context.createBlock();

                // Send RPC call to enable para inherent candidate generation
                await customDevRpcRequest("mock_enableParaInherentCandidate", []);

                const tokenTransferNonceBefore = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceBefore = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceBefore = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                // Wait until message is processed
                // we need to wait until session 3 for sure so that paras produce blocks
                await jumpToSession(context, 3);

                // Send ump message
                await injectUmpMessageAndSeal(context, {
                    type: "XcmVersionedXcm",
                    payload: xcmMessage,
                } as RawXcmMessage);

                await context.createBlock();
                await context.createBlock();

                // Things to verify:
                // 1. ethereumOutboundQueueV2 increases the nonce
                // 2. reward goes to snowbridge fees account
                // 3. container sov account pays all the fees (export fee + execution fee on Tanssi)
                // 4. a pending order exists for such nonce, with the fee=reward
                const tokenTransferNonceAfter = await polkadotJs.query.ethereumOutboundQueueV2.nonce();
                const snowbridgeFeesAccountBalanceAfter = (
                    await polkadotJs.query.system.account(SNOWBRIDGE_FEES_ACCOUNT)
                ).data.free.toBigInt();

                const containerSovereignAccountBalanceAfter = (
                    await polkadotJs.query.system.account(sovAddress)
                ).data.free.toBigInt();

                const pendingOrder =
                    await polkadotJs.query.ethereumOutboundQueueV2.pendingOrders(tokenTransferNonceAfter);

                expect(tokenTransferNonceAfter.toNumber()).to.be.equal(tokenTransferNonceBefore.toNumber() + 1);
                expect(snowbridgeFeesAccountBalanceAfter).to.be.eq(
                    snowbridgeFeesAccountBalanceBefore + exportFeeAmount
                );
                const roundingNonExporterFees = 80_000_000n;
                expect(containerSovereignAccountBalanceAfter).toBeGreaterThan(
                    containerSovereignAccountBalanceBefore - exportFeeAmount - roundingNonExporterFees
                );
                expect(pendingOrder.unwrap().fee.toBigInt()).to.be.equal(exportFeeAmount);
            },
        });
    },
});
