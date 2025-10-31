import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, alith, createEthersTransaction, generateKeyringPair } from "@moonwall/util";
import type { u16 } from "@polkadot/types-codec";
import { expectEVMResult } from "helpers";
import { RELAY_SOURCE_LOCATION, SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS, TESTNET_ETHEREUM_NETWORK_ID } from "utils";
import { encodeFunctionData } from "viem";
import { type AssetMetadata, mockAssetCreation, relayAssetMetadata } from "../../../helpers/assets.ts";
import { numberToHex } from "@polkadot/util";

const PRECOMPILE_PALLET_XCM_ADDRESS: `0x${string}` = "0x0000000000000000000000000000000000000804";

describeSuite({
    id: "DE1312",
    title: "Precompiles - PalletXcm",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let relayAssetId: u16;
        let erc20AssetId: u16;

        const ADDRESS_ERC20 = "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF000f";
        const RELAY_ASSET_ID = 15n;
        const ERC20_RELAY_ASSET_ID = 16n;
        const amountToSend = 100n;
        const holdingAccount = SEPOLIA_SOVEREIGN_ACCOUNT_ADDRESS;
        const tokenToTransfer = 123_321_000_000_000n;

        beforeAll(async () => {
            relayAssetId = context.polkadotJs().createType("u16", RELAY_ASSET_ID);
            erc20AssetId = context.polkadotJs().createType("u16", ERC20_RELAY_ASSET_ID);

            const relayAssetBalance = 200000000000000n;
            await mockAssetCreation(
                context,
                alith,
                relayAssetId,
                ALITH_ADDRESS,
                RELAY_SOURCE_LOCATION,
                relayAssetMetadata,
                true
            );
            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(relayAssetId.toU8a(), ALITH_ADDRESS, relayAssetBalance)
            );

            const erc20AssetMetadata: AssetMetadata = {
                name: "erc20",
                symbol: "erc20",
                decimals: 18n,
                isFrozen: false,
            };
            const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
            const erc20Location = {
                parents: 2,
                interior: {
                    X2: [
                        { GlobalConsensus: ethereumNetwork },
                        {
                            AccountKey20: {
                                network: ethereumNetwork,
                                key: ADDRESS_ERC20,
                            },
                        },
                    ],
                },
            };
            await mockAssetCreation(
                context,
                alith,
                erc20AssetId,
                ALITH_ADDRESS,
                erc20Location,
                erc20AssetMetadata,
                true
            );

            const erc20AssetBalance = 123_321_000_000_000_001n; // Adding 1 extra for the check
            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(erc20AssetId.toU8a(), ALITH_ADDRESS, erc20AssetBalance)
            );
        });

        it({
            id: "T01",
            title: "allows to call transferAssetsLocation function",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const assetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                const dest: [number, any[]] = [1, []];

                const destinationAddress = "0101010101010101010101010101010101010101010101010101010101010101";
                const destinationNetworkId = "00";
                const beneficiary: [number, any[]] = [
                    0,
                    // junction: AccountId32 enum (01) + the 32 byte account + Any network selector(00)
                    [`0x01${destinationAddress}${destinationNetworkId}`],
                ];

                const assetLocation: [number, any[]] = [1, []];
                const assetLocationInfo = [[assetLocation, amountToSend]];

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        args: [dest, beneficiary, assetLocationInfo, 0],
                        functionName: "transferAssetsLocation",
                    }),
                    gasLimit: 20_000_000n,
                });

                const result = await context.createBlock(rawTxn);
                expectEVMResult(result.result?.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(assetBalanceAfter).to.equal(assetBalanceBefore - amountToSend);
            },
        });

        it({
            id: "T02",
            title: "allows to call transferAssetsToPara20 function",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const assetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                const paraId = 1000n;
                const assetAddressInfo = [[ADDRESS_ERC20, amountToSend]];

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        args: [paraId, BALTATHAR_ADDRESS, assetAddressInfo, 0],
                        functionName: "transferAssetsToPara20",
                    }),
                    gasLimit: 20_000_000n,
                });

                const result = await context.createBlock(rawTxn);
                expectEVMResult(result.result?.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(assetBalanceAfter).to.equal(assetBalanceBefore - amountToSend);
            },
        });

        it({
            id: "T03",
            title: "allows to call transferAssetsToPara32 function",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const assetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                const paraId = 1000n;
                const assetAddressInfo = [[ADDRESS_ERC20, amountToSend]];
                const beneficiaryAddress = "01010101010101010101010101010101";

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        args: [paraId, beneficiaryAddress, assetAddressInfo, 0],
                        functionName: "transferAssetsToPara32",
                    }),
                    gasLimit: 20_000_000n,
                });

                const result = await context.createBlock(rawTxn);
                expectEVMResult(result.result?.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(assetBalanceAfter).to.equal(assetBalanceBefore - amountToSend);
            },
        });

        it({
            id: "T04",
            title: "allows to call transferAssetsToRelay function",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const assetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                const assetAddressInfo = [[ADDRESS_ERC20, amountToSend]];
                const beneficiaryAddress = "01010101010101010101010101010101";

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        args: [beneficiaryAddress, assetAddressInfo, 0],
                        functionName: "transferAssetsToRelay",
                    }),
                    gasLimit: 20_000_000n,
                });

                const result = await context.createBlock(rawTxn);
                expectEVMResult(result.result?.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(assetBalanceAfter).to.equal(assetBalanceBefore - amountToSend);
            },
        });

        it({
            id: "T05",
            title: "allows to call transferAssetsLocation precompile for exporting assets (container chain native token) to Ethereum",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");

                const chainIdHex = numberToHex(TESTNET_ETHEREUM_NETWORK_ID, 64);

                const balanceBefore = (
                    await context.polkadotJs().query.system.account(holdingAccount)
                ).data.free.toBigInt();

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        functionName: "transferAssetsLocation",
                        // args: [dest, beneficiary, assets, feeAssetItem],
                        args: [
                            // junction: globalConsensus + ethereum + chainId
                            [2, [["0x", "09", "08", chainIdHex.slice(2)].join("")]],
                            [
                                0,
                                // junction: AccountId32 enum (01) + the 32 byte account + Network Any - 00
                                [
                                    [
                                        "0x01",
                                        "0101010101010101010101010101010101010101010101010101010101010101",
                                        "00",
                                    ].join(""),
                                ],
                            ],
                            [[[0, ["0x040a"]], tokenToTransfer]],
                            0,
                        ],
                    }),
                });

                type EventType = { event: { section: string; method: string; data: any } };
                const result = await context.createBlock(rawTxn);
                const sentXcmEvent = result.result?.events
                    .map((e) => e.toHuman())
                    .find((e) => {
                        const event = e as unknown as EventType;
                        return event.event.section === "polkadotXcm" && event.event.method === "Sent";
                    }) as unknown as EventType;

                const balanceAfter = (
                    await context.polkadotJs().query.system.account(holdingAccount)
                ).data.free.toBigInt();
                expect(balanceAfter - balanceBefore).toEqual(tokenToTransfer);

                expect(sentXcmEvent.event.data).toEqual({
                    origin: {
                        parents: "0",
                        interior: {
                            X1: [
                                {
                                    AccountKey20: {
                                        network: {
                                            ByGenesis:
                                                "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
                                        },
                                        key: "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac",
                                    },
                                },
                            ],
                        },
                    },
                    destination: {
                        parents: "2",
                        interior: {
                            X1: [
                                {
                                    GlobalConsensus: {
                                        Ethereum: {
                                            chainId: "11,155,111",
                                        },
                                    },
                                },
                            ],
                        },
                    },
                    message: [
                        {
                            ReserveAssetDeposited: [
                                {
                                    id: {
                                        parents: "1",
                                        interior: {
                                            X3: [
                                                {
                                                    GlobalConsensus: {
                                                        ByGenesis:
                                                            "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
                                                    },
                                                },
                                                {
                                                    Parachain: "2,000",
                                                },
                                                {
                                                    PalletInstance: "10",
                                                },
                                            ],
                                        },
                                    },
                                    fun: {
                                        Fungible: "123,321,000,000,000",
                                    },
                                },
                            ],
                        },
                        "ClearOrigin",
                        {
                            BuyExecution: {
                                fees: {
                                    id: {
                                        parents: "1",
                                        interior: {
                                            X3: [
                                                {
                                                    GlobalConsensus: {
                                                        ByGenesis:
                                                            "0x983a1a72503d6cc3636776747ec627172b51272bf45e50a355348facb67a820a",
                                                    },
                                                },
                                                {
                                                    Parachain: "2,000",
                                                },
                                                {
                                                    PalletInstance: "10",
                                                },
                                            ],
                                        },
                                    },
                                    fun: {
                                        Fungible: "123,321,000,000,000",
                                    },
                                },
                                weightLimit: "Unlimited",
                            },
                        },
                        {
                            DepositAsset: {
                                assets: {
                                    Wild: {
                                        AllCounted: "1",
                                    },
                                },
                                beneficiary: {
                                    parents: "0",
                                    interior: {
                                        X1: [
                                            {
                                                AccountId32: {
                                                    network: null,
                                                    id: "0x0101010101010101010101010101010101010101010101010101010101010101",
                                                },
                                            },
                                        ],
                                    },
                                },
                            },
                        },
                    ],
                    messageId: expect.any(String),
                });

                expectEVMResult(result.result?.events, "Succeed");
            },
        });

        it({
            id: "T06",
            title: "allows to call transferAssetsLocation precompile for exporting assets (container chain ERC20 token) to Ethereum",
            test: async () => {
                const ERC20_ASSET_AMOUNT = 123_321_000_000_000_000n;
                const RELAY_ASSET_FEE_AMOUNT = 3_500_000_000_000n;

                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const dest = [
                    // one parents
                    1,
                    // Here
                    [],
                ];
                const ethereumNetwork = { Ethereum: { chainId: TESTNET_ETHEREUM_NETWORK_ID } };
                const accountKey20Interior = {
                    AccountKey20: {
                        network: ethereumNetwork,
                        key: ADDRESS_ERC20,
                    },
                };
                const globalConsensusEthereumInterior = { GlobalConsensus: ethereumNetwork };
                const erc20AssetIdForRelayContext = {
                    parents: 1,
                    interior: { X2: [globalConsensusEthereumInterior, accountKey20Interior] },
                };

                // DestinationReserve
                const assetsAndFeesTransferType = 2;

                const erc20AssetIdForEthereumContext = {
                    parents: 0,
                    interior: {
                        X1: accountKey20Interior,
                    },
                };
                const erc20AssetReceiverAddress = generateKeyringPair("ethereum").address;
                const beneficiary = {
                    parents: 0,
                    interior: {
                        X1: {
                            AccountKey20: {
                                network: ethereumNetwork,
                                key: erc20AssetReceiverAddress,
                            },
                        },
                    },
                };
                const xcmOnDest = context.polkadotJs().createType("XcmVersionedXcm", {
                    V3: [
                        {
                            InitiateReserveWithdraw: {
                                assets: {
                                    Definite: [
                                        {
                                            id: {
                                                Concrete: erc20AssetIdForRelayContext,
                                            },
                                            fun: { Fungible: ERC20_ASSET_AMOUNT },
                                        },
                                    ],
                                },
                                reserve: {
                                    parents: 1,
                                    interior: { X1: globalConsensusEthereumInterior },
                                },
                                xcm: [
                                    {
                                        DepositAsset: {
                                            assets: {
                                                Definite: [
                                                    {
                                                        id: {
                                                            Concrete: erc20AssetIdForEthereumContext,
                                                        },
                                                        fun: { Fungible: ERC20_ASSET_AMOUNT },
                                                    },
                                                ],
                                            },
                                            beneficiary,
                                        },
                                    },
                                ],
                            },
                        },
                    ],
                });
                const chainIdHex = numberToHex(TESTNET_ETHEREUM_NETWORK_ID, 64);

                const rawTxn = await createEthersTransaction(context, {
                    to: PRECOMPILE_PALLET_XCM_ADDRESS,
                    data: encodeFunctionData({
                        abi: xcmInterface,
                        // args: [dest, assets, assetsTransferType, remoteFeesId, feesTransferType, customXcmOnDest],
                        args: [
                            dest,
                            [
                                [
                                    [
                                        // parents: 1
                                        1,
                                        // Here
                                        [],
                                    ],
                                    RELAY_ASSET_FEE_AMOUNT,
                                ],
                                [
                                    [
                                        // parents = 2
                                        2,
                                        [
                                            // X2 -> 0: GlobalConsensus + Ethereum + networkId
                                            ["0x", "09", "08", chainIdHex.slice(2)].join(""),
                                            //X2 -> 1: AccountId20 + key + network + Ethereum + networkId
                                            ["0x", "03", ADDRESS_ERC20.slice(2), "08", chainIdHex.slice(2)].join(""),
                                        ],
                                    ],
                                    ERC20_ASSET_AMOUNT,
                                ],
                            ],
                            assetsAndFeesTransferType,
                            0n,
                            assetsAndFeesTransferType,
                            xcmOnDest.toHex(),
                        ],
                        functionName: "transferAssetsUsingTypeAndThenLocation",
                    }),
                    gasLimit: 500_000n,
                });

                const erc20AssetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(erc20AssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                const relayAssetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                type EventType = { event: { section: string; method: string; data: any } };
                const result = await context.createBlock(rawTxn);
                const sentXcmEvent = result.result?.events
                    .map((e) => e.toHuman())
                    .find((e) => {
                        const event = e as unknown as EventType;
                        return event.event.section === "polkadotXcm" && event.event.method === "Sent";
                    }) as unknown as EventType;

                expect(!!sentXcmEvent).toEqual(true); // Event exists

                const erc20AssetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(erc20AssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                const relayAssetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(relayAssetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                // Check that ERC20 asset was transferred to the destination address
                expect(erc20AssetBalanceBefore - erc20AssetBalanceAfter).toEqual(ERC20_ASSET_AMOUNT);
                expect(relayAssetBalanceBefore - relayAssetBalanceAfter).toEqual(RELAY_ASSET_FEE_AMOUNT);

                expect(erc20AssetBalanceAfter).to.equal(1n);

                expectEVMResult(result.result?.events, "Succeed");
            },
        });
    },
});
