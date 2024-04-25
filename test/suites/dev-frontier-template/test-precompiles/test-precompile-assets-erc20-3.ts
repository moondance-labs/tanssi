import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import {
    ALITH_ADDRESS,
    BALTATHAR_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
    CHARLETH_ADDRESS,
    alith,
    createEthersTransaction,
    createViemTransaction,
} from "@moonwall/util";
import { u16 } from "@polkadot/types-codec";
import { mockAssetCreation, RELAY_SOURCE_LOCATION, relayAssetMetadata } from "../../../helpers/assets";

import { Abi, encodeFunctionData } from "viem";

describeSuite({
    id: "DF1103",
    title: "Precompiles - Assets-ERC20",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        let erc20Abi: Abi;
        const ASSET_ID = 15n;
        const ADDRESS_ERC20 = "0xfFfFFFffFffFFFFffFFfFfffFfFFFFFfffFF000f" as `0x${string}`;
        const SELECTORS = {
            balanceOf: "70a08231",
            totalSupply: "18160ddd",
            approve: "095ea7b3",
            allowance: "dd62ed3e",
            transfer: "a9059cbb",
            transferFrom: "23b872dd",
            logApprove: "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            logTransfer: "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        };

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
            title: "allows to approve transfer and use transferFrom",
            test: async function () {
                const rawSigned = await createEthersTransaction(context, {
                    to: ADDRESS_ERC20,
                    data: encodeFunctionData({
                        abi: erc20Abi,
                        functionName: "approve",
                        args: [BALTATHAR_ADDRESS, 1000],
                    }),
                });
                await context.createBlock(rawSigned);

                const approvals = await context
                    .polkadotJs()
                    .query.foreignAssets.approvals(assetId.toU8a(), ALITH_ADDRESS, BALTATHAR_ADDRESS);

                expect(approvals.unwrap().amount.toBigInt()).to.equal(1000n);
                // We are gonna spend 1000 from alith to send it to charleth
                const rawSigned2 = await createViemTransaction(context, {
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    to: ADDRESS_ERC20,
                    data: encodeFunctionData({
                        abi: erc20Abi,
                        functionName: "transferFrom",
                        args: [ALITH_ADDRESS, CHARLETH_ADDRESS, 1000],
                    }),
                });

                const { result } = await context.createBlock(rawSigned2);
                const receipt = await context.viem().getTransactionReceipt({
                    hash: result?.hash as `0x${string}`,
                });

                expect(receipt.logs.length).to.eq(1);
                expect(receipt.logs[0].address.toLowerCase()).to.eq(ADDRESS_ERC20.toLowerCase());
                expect(receipt.logs[0].topics.length).to.eq(3);
                expect(receipt.logs[0].topics[0]).to.eq(SELECTORS.logTransfer);
                expect(receipt.status).to.equal("success");

                await new Promise((resolve) => setTimeout(resolve, 1000));
                // Approve amount is null now
                const newApprovals = await context
                    .polkadotJs()
                    .query.foreignAssets.approvals(assetId.toU8a(), ALITH_ADDRESS, BALTATHAR_ADDRESS);

                expect(newApprovals.isNone).to.eq(true);

                // Charleth balance is 1000
                const charletBalance = await context
                    .polkadotJs()
                    .query.foreignAssets.account(assetId.toU8a(), CHARLETH_ADDRESS);
                expect(charletBalance.unwrap().balance.toBigInt()).to.equal(1000n);
            },
        });
    },
});
