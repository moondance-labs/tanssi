import { TransactionTypes, beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, BALTATHAR_ADDRESS, CHARLETH_ADDRESS, alith, createEthersTransaction } from "@moonwall/util";
import { u16 } from "@polkadot/types";
import { nToHex } from "@polkadot/util";
import { Abi, encodeFunctionData } from "viem";
import { mockAssetCreation, relayAssetMetadata, RELAY_SOURCE_LOCATION } from "../../../helpers/assets";

describeSuite({
    id: "DF1108",
    title: "Precompiles - Low Level Transactions",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let assetId: u16;
        let contractInstanceAddress: `0x${string}`;
        let contractAbi: Abi;

        const ASSET_ID = 15n;
        const MAX_SUPPLY = 100000000000000n;

        beforeAll(async function () {
            assetId = context.polkadotJs().createType("u16", ASSET_ID);

            const { contractAddress, abi } = await deployCreateCompiledContract(context, "ERC20Instance");
            contractInstanceAddress = contractAddress;
            contractAbi = abi;

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
                context.polkadotJs().tx.foreignAssets.mint(assetId.toU8a(), ALITH_ADDRESS, MAX_SUPPLY)
            );
        });

        let testCounter = 2;

        it({
            id: "T01",
            title: "can make static calls to view functions",
            test: async function () {
                const callResult = await context.viem().call({
                    account: ALITH_ADDRESS,
                    to: contractInstanceAddress,
                    data: encodeFunctionData({
                        abi: contractAbi,
                        functionName: "totalSupply_static",
                    }),
                });

                expect(callResult.data).equals(nToHex(MAX_SUPPLY, { bitLength: 256 }));
            },
        });

        for (const txnType of TransactionTypes) {
            it({
                id: `T${testCounter < 10 ? "0" : ""}${testCounter++}`,
                title: `can make static calls to view functions and transact ${txnType}`,
                test: async function () {
                    await context.createBlock(
                        await createEthersTransaction(context, {
                            to: contractInstanceAddress,
                            data: encodeFunctionData({
                                abi: contractAbi,
                                functionName: "approve_max_supply",
                                args: [CHARLETH_ADDRESS],
                            }),
                            txnType: "eip1559",
                        })
                    );

                    const approvals = await context
                        .polkadotJs()
                        .query.foreignAssets.approvals(assetId.toU8a(), contractInstanceAddress, CHARLETH_ADDRESS);

                    expect(approvals.unwrap().amount.toBigInt()).to.equal(MAX_SUPPLY);
                },
            });

            it({
                id: `T${testCounter < 10 ? "0" : ""}${testCounter++}`,
                title: `has unchanged state when submitting static call ${txnType}`,
                test: async function () {
                    const { result } = await context.createBlock(
                        await createEthersTransaction(context, {
                            to: contractInstanceAddress,
                            data: encodeFunctionData({
                                abi: contractAbi,
                                functionName: "approve_static",
                                args: [BALTATHAR_ADDRESS, 1000],
                            }),
                        })
                    );

                    const approvals = await context
                        .polkadotJs()
                        .query.foreignAssets.approvals(assetId.toU8a(), contractInstanceAddress, BALTATHAR_ADDRESS);

                    expect(result?.successful, "Call unsuccessful").to.be.true;
                    expect(approvals.isNone).to.be.true;
                },
            });

            it({
                id: `T${testCounter < 10 ? "0" : ""}${testCounter++}`,
                title: `visibility preserved for static calls ${txnType}`,
                test: async function () {
                    const { result } = await context.createBlock(
                        await createEthersTransaction(context, {
                            to: contractInstanceAddress,
                            data: encodeFunctionData({
                                abi: contractAbi,
                                functionName: "approve_ext_static",
                                args: [BALTATHAR_ADDRESS, 1000],
                            }),
                        })
                    );

                    const approvals = await context
                        .polkadotJs()
                        .query.foreignAssets.approvals(assetId.toU8a(), contractInstanceAddress, BALTATHAR_ADDRESS);

                    expect(result?.successful, "Call unsuccessful").to.be.true;
                    expect(approvals.isNone).to.be.true;
                },
            });

            it({
                id: `T${testCounter < 10 ? "0" : ""}${testCounter++}`,
                title: `visibility preserved for delegate->static calls ${txnType}`,
                test: async function () {
                    const { result } = await context.createBlock(
                        await createEthersTransaction(context, {
                            to: contractInstanceAddress,
                            data: encodeFunctionData({
                                abi: contractAbi,
                                functionName: "approve_delegate_to_static",
                                args: [BALTATHAR_ADDRESS, 1000],
                            }),
                        })
                    );

                    const approvals = await context
                        .polkadotJs()
                        .query.foreignAssets.approvals(assetId.toU8a(), contractInstanceAddress, BALTATHAR_ADDRESS);

                    expect(result?.successful, "Call unsuccessful").to.be.true;
                    expect(approvals.isNone).to.be.true;
                },
            });

            it({
                id: `T${testCounter < 10 ? "0" : ""}${testCounter++}`,
                title: `visibility preserved for static->delegate calls ${txnType}`,
                test: async function () {
                    const { result } = await context.createBlock(
                        await createEthersTransaction(context, {
                            to: contractInstanceAddress,
                            data: encodeFunctionData({
                                abi: contractAbi,
                                functionName: "approve_static_to_delegate",
                                args: [BALTATHAR_ADDRESS, 1000],
                            }),
                        })
                    );

                    const approvals = await context
                        .polkadotJs()
                        .query.foreignAssets.approvals(assetId.toU8a(), contractInstanceAddress, BALTATHAR_ADDRESS);

                    expect(result?.successful, "Call unsuccessful").to.be.true;
                    expect(approvals.isNone).to.be.true;
                },
            });
        }
    },
});
