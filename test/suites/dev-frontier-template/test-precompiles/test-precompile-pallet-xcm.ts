import "@moonbeam-network/api-augment";
import { beforeAll, describeSuite, fetchCompiledContract, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, alith, createEthersTransaction } from "@moonwall/util";
import type { u16 } from "@polkadot/types-codec";
import { encodeFunctionData } from "viem";
import { expectEVMResult } from "helpers";
import { mockAssetCreation, relayAssetMetadata } from "../../../helpers/assets.ts";
import { RELAY_SOURCE_LOCATION } from "../../../util/constants.ts";

const PRECOMPILE_PALLET_XCM_ADDRESS: `0x${string}` = "0x0000000000000000000000000000000000000804";

describeSuite({
    id: "DE1312",
    title: "Precompiles - PalletXcm",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        const ADDRESS_ERC20 = "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF000f";
        const ASSET_ID = 15n;
        const amountToSend = 100n;

        beforeAll(async () => {
            assetId = context.polkadotJs().createType("u16", ASSET_ID);
            const balance = 200000000000000n;
            await mockAssetCreation(
                context,
                alith,
                assetId,
                ALITH_ADDRESS,
                RELAY_SOURCE_LOCATION,
                relayAssetMetadata,
                true
            );

            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, balance)
            );
        });

        it({
            id: "T01",
            title: "allows to call transferAssetsLocation function",
            test: async () => {
                const { abi: xcmInterface } = fetchCompiledContract("XCM");
                const assetBalanceBefore = (
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();

                const dest: [number, any[]] = [1, []];

                const destinationAddress = "0101010101010101010101010101010101010101010101010101010101010101";
                const destinationNetworkId = "00";
                const beneficiary: [number, any[]] = [
                    0,
                    // junction: AccountId32 enum (01) + the 32 byte account + Any network selector(00)
                    ["0x01" + destinationAddress + destinationNetworkId],
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
                expectEVMResult(result.result!.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                expectEVMResult(result.result!.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                expectEVMResult(result.result!.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
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
                expectEVMResult(result.result!.events, "Succeed");

                const assetBalanceAfter = (
                    await context.polkadotJs().query.foreignAssets.account(assetId.toU8a(), ALITH_ADDRESS)
                )
                    .unwrap()
                    .balance.toBigInt();
                expect(assetBalanceAfter).to.equal(assetBalanceBefore - amountToSend);
            },
        });
    },
});
