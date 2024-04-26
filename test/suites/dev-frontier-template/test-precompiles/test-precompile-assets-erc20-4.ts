import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, alith, createViemTransaction } from "@moonwall/util";
import { u16 } from "@polkadot/types-codec";
import { Abi, encodeFunctionData } from "viem";
import { mockAssetCreation, RELAY_SOURCE_LOCATION, relayAssetMetadata } from "../../../helpers/assets";

describeSuite({
    id: "DF1104",
    title: "Precompiles - Assets-ERC20",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        let erc20Abi: Abi;
        const ASSET_ID = 15n;
        const ADDRESS_ERC20 = "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF000f" as `0x${string}`;

        beforeAll(async () => {
            assetId = context.polkadotJs().createType("u16", ASSET_ID);

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
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, 100000000000000n)
            );

            const { abi } = await deployCreateCompiledContract(context, "ERC20Instance");
            erc20Abi = abi;
        });

        it({
            id: "T01",
            title: "allows to transfer",
            test: async function () {
                const { result } = await context.createBlock(
                    createViemTransaction(context, {
                        to: ADDRESS_ERC20,
                        data: encodeFunctionData({
                            functionName: "transfer",
                            args: [BALTATHAR_ADDRESS, 1000],
                            abi: erc20Abi,
                        }),
                    })
                );

                // const receipt = await context.web3.eth.getTransactionReceipt(result.hash);
                const receipt = await context.viem().getTransactionReceipt({ hash: result?.hash as `0x${string}` });
                expect(receipt.status).to.equal("success");

                // Baltathar balance is 1000
                const baltatharBalance = await context
                    .polkadotJs()
                    .query.foreignAssets.account(assetId.toU8a(), BALTATHAR_ADDRESS);
                expect(baltatharBalance.unwrap().balance.toBigInt()).to.equal(1000n);
            },
        });
    },
});
