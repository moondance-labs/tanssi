import "@moonbeam-network/api-augment";
import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_PRIVATE_KEY, CHARLETH_ADDRESS, alith, createViemTransaction } from "@moonwall/util";
import { u16 } from "@polkadot/types-codec";
import { Abi, encodeFunctionData } from "viem";
import { mockAssetCreation, RELAY_SOURCE_LOCATION, relayAssetMetadata } from "../../../helpers/assets";

describeSuite({
    id: "DF1106",
    title: "Precompiles - Assets-ERC20",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        let erc20Abi: Abi;
        let erc20InstanceAddress: `0x${string}`;
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

            const { contractAddress, abi } = await deployCreateCompiledContract(context, "ERC20Instance");
            erc20InstanceAddress = contractAddress;
            erc20Abi = abi;

            await mockAssetCreation(
                context,
                alith,
                assetId,
                ALITH_ADDRESS,
                RELAY_SOURCE_LOCATION,
                relayAssetMetadata,
                true
            );

            // We fund Alith
            await context.createBlock(
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, 100000000000000n)
            );
        });

        it({
            id: "T01",
            title: "Bob approves contract and use transferFrom from contract calls",
            test: async function () {
                const tx = await createViemTransaction(context, {
                    to: ADDRESS_ERC20,
                    data: encodeFunctionData({
                        functionName: "approve",
                        args: [erc20InstanceAddress, 1000],
                        abi: erc20Abi,
                    }),
                });

                const { result } = await context.createBlock(tx);
                const receipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: result?.hash as `0x${string}` });

                expect(receipt.status).to.equal("success");
                expect(receipt.logs.length).to.eq(1);
                expect(receipt.logs[0].address).to.eq(ADDRESS_ERC20.toLowerCase());
                expect(receipt.logs[0].topics.length).to.eq(3);
                expect(receipt.logs[0].topics[0]).to.eq(SELECTORS.logApprove);

                const approvals = await context
                    .polkadotJs()
                    .query.foreignAssets.approvals(assetId.toU8a(), ALITH_ADDRESS, erc20InstanceAddress);

                expect(approvals.unwrap().amount.toBigInt()).to.equal(1000n);
                // We are gonna spend 1000 from ALITH_ADDRESS to send it to charleth from contract address
                // even if Bob calls, msg.sender will become the contract with regular calls
                const blockBaltathar = await context.createBlock(
                    createViemTransaction(context, {
                        privateKey: BALTATHAR_PRIVATE_KEY,
                        to: erc20InstanceAddress,
                        data: encodeFunctionData({
                            functionName: "transferFrom",
                            args: [ALITH_ADDRESS, CHARLETH_ADDRESS, 1000],
                            abi: erc20Abi,
                        }),
                    })
                );

                const receiptBaltathar = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: blockBaltathar.result?.hash as `0x${string}` });
                expect(receiptBaltathar.logs.length).to.eq(1);
                expect(receiptBaltathar.logs[0].address).to.eq(ADDRESS_ERC20.toLowerCase());
                expect(receiptBaltathar.logs[0].topics.length).to.eq(3);
                expect(receiptBaltathar.logs[0].topics[0]).to.eq(SELECTORS.logTransfer);
                expect(receiptBaltathar.status).to.equal("success");

                // Approve amount is null now
                const approvalBaltathar = await context
                    .polkadotJs()
                    .query.foreignAssets.approvals(assetId.toU8a(), ALITH_ADDRESS, erc20InstanceAddress);
                expect(approvalBaltathar.isNone).to.eq(true);

                // Charleth balance is 1000
                const charletBalance = await context
                    .polkadotJs()
                    .query.foreignAssets.account(assetId.toU8a(), CHARLETH_ADDRESS);
                expect(charletBalance.unwrap().balance.toBigInt()).to.equal(1000n);
            },
        });
    },
});
