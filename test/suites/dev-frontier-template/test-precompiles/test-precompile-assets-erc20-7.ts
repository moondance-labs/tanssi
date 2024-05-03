import "@moonbeam-network/api-augment";
import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, alith, createViemTransaction } from "@moonwall/util";
import { u16 } from "@polkadot/types-codec";
import { Abi, encodeFunctionData } from "viem";
import { mockAssetCreation, RELAY_SOURCE_LOCATION, relayAssetMetadata } from "../../../helpers/assets";

describeSuite({
    id: "DF1107",
    title: "Precompiles - Assets-ERC20",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        let erc20Abi: Abi;
        let erc20InstanceAddress: `0x${string}`;
        const ASSET_ID = 15n;

        beforeAll(async () => {
            assetId = context.polkadotJs().createType("u16", ASSET_ID);

            const { abi, contractAddress } = await deployCreateCompiledContract(context, "ERC20Instance");
            erc20Abi = abi;
            erc20InstanceAddress = contractAddress;

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
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, 200000000000000n)
            );

            // We fund the contract address
            await context.createBlock(
                context
                    .polkadotJs()
                    .tx.foreignAssets.forceTransfer(
                        assetId.toU8a(),
                        ALITH_ADDRESS,
                        erc20InstanceAddress,
                        100000000000000n
                    )
            );
        });

        it({
            id: "T01",
            title: "allows to transfer through call from SC ",
            test: async function () {
                // Create approval
                const { result } = await context.createBlock(
                    createViemTransaction(context, {
                        to: erc20InstanceAddress,
                        data: encodeFunctionData({
                            abi: erc20Abi,
                            functionName: "transfer",
                            args: [BALTATHAR_ADDRESS, 1000],
                        }),
                    })
                );

                const receipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: result?.hash as `0x${string}` });
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
