import { describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import {
    ALITH_ADDRESS,
    BALTATHAR_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
    CHARLETH_ADDRESS,
    createViemTransaction,
    sendRawTransaction,
} from "@moonwall/util";
import { expectEVMResult } from "helpers";
import { getSignatureParameters } from "utils";
import { encodeFunctionData, fromHex } from "viem";

const PRECOMPILE_BATCH_ADDRESS = "0x0000000000000000000000000000000000000801";
const PRECOMPILE_CALL_PERMIT_ADDRESS = "0x0000000000000000000000000000000000000802";

describeSuite({
    id: "DE1309",
    title: "Batch - All functions should consume the same gas",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "should consume the same gas",
            test: async () => {
                const { abi: batchInterface } = fetchCompiledContract("Batch");

                let aliceNonce = (await context.polkadotJs().query.system.account(ALITH_ADDRESS)).nonce.toNumber();

                // each tx have a different gas limit to ensure it doesn't impact gas used
                const batchAllTx = await createViemTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    gas: 1114112n,
                    nonce: aliceNonce++,
                    data: encodeFunctionData({
                        abi: batchInterface,
                        functionName: "batchAll",
                        args: [
                            [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
                            ["1000000000000000000", "2000000000000000000"],
                            [],
                            [],
                        ],
                    }),
                });

                const batchSomeTx = await createViemTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    gas: 1179648n,
                    nonce: aliceNonce++,
                    data: encodeFunctionData({
                        abi: batchInterface,
                        functionName: "batchSome",
                        args: [
                            [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
                            ["1000000000000000000", "2000000000000000000"],
                            [],
                            [],
                        ],
                    }),
                });

                const batchSomeUntilFailureTx = await createViemTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    gas: 1245184n,
                    nonce: aliceNonce++,
                    data: encodeFunctionData({
                        abi: batchInterface,
                        functionName: "batchSomeUntilFailure",
                        args: [
                            [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
                            ["1000000000000000000", "2000000000000000000"],
                            [],
                            [],
                        ],
                    }),
                });

                const batchAllResult = await sendRawTransaction(context, batchAllTx);
                const batchSomeResult = await sendRawTransaction(context, batchSomeTx);
                const batchSomeUntilFailureResult = await sendRawTransaction(context, batchSomeUntilFailureTx);

                await context.createBlock();

                const batchAllReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: batchAllResult as `0x${string}` });
                const batchSomeReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: batchSomeResult as `0x${string}` });
                const batchSomeUntilFailureReceipt = await context
                    .viem("public")
                    .getTransactionReceipt({ hash: batchSomeUntilFailureResult as `0x${string}` });

                expect(batchAllReceipt.gasUsed).to.equal(44_932n);
                expect(batchSomeReceipt.gasUsed).to.equal(44_932n);
                expect(batchSomeUntilFailureReceipt.gasUsed).to.equal(44_932n);
            },
        });

        it({
            id: "T02",
            title: "should be able to call itself",
            test: async () => {
                const { abi: batchInterface } = fetchCompiledContract("Batch");

                const batchAll = await context.writeContract({
                    contractAddress: PRECOMPILE_BATCH_ADDRESS,
                    contractName: "Batch",
                    functionName: "batchAll",
                    args: [
                        [PRECOMPILE_BATCH_ADDRESS],
                        [],
                        [
                            encodeFunctionData({
                                abi: batchInterface,
                                functionName: "batchAll",
                                args: [
                                    [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
                                    ["1000000000000000000", "2000000000000000000"],
                                    [],
                                    [],
                                ],
                            }),
                        ],
                        [],
                    ],
                    rawTxOnly: true,
                });

                const { result } = await context.createBlock(batchAll);
                expectEVMResult(result?.events, "Succeed");
            },
        });

        it({
            id: "T03",
            title: "should be able to be called from call permit",
            test: async () => {
                const { abi: batchInterface } = fetchCompiledContract("Batch");
                const { abi: callPermitAbi } = fetchCompiledContract("CallPermit");

                const alithNonceResult = (
                    await context.viem().call({
                        to: PRECOMPILE_CALL_PERMIT_ADDRESS,
                        data: encodeFunctionData({
                            abi: callPermitAbi,
                            functionName: "nonces",
                            args: [ALITH_ADDRESS],
                        }),
                    })
                ).data;

                const batchData = encodeFunctionData({
                    abi: batchInterface,
                    functionName: "batchAll",
                    args: [
                        [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
                        ["1000000000000000000", "2000000000000000000"],
                        [],
                        [],
                    ],
                });

                const signature = await context.viem().signTypedData({
                    types: {
                        EIP712Domain: [
                            {
                                name: "name",
                                type: "string",
                            },
                            {
                                name: "version",
                                type: "string",
                            },
                            {
                                name: "chainId",
                                type: "uint256",
                            },
                            {
                                name: "verifyingContract",
                                type: "address",
                            },
                        ],
                        CallPermit: [
                            {
                                name: "from",
                                type: "address",
                            },
                            {
                                name: "to",
                                type: "address",
                            },
                            {
                                name: "value",
                                type: "uint256",
                            },
                            {
                                name: "data",
                                type: "bytes",
                            },
                            {
                                name: "gaslimit",
                                type: "uint64",
                            },
                            {
                                name: "nonce",
                                type: "uint256",
                            },
                            {
                                name: "deadline",
                                type: "uint256",
                            },
                        ],
                    },
                    primaryType: "CallPermit",
                    domain: {
                        name: "Call Permit Precompile",
                        version: "1",
                        chainId: 1281n,
                        verifyingContract: PRECOMPILE_CALL_PERMIT_ADDRESS,
                    },
                    message: {
                        from: ALITH_ADDRESS,
                        to: PRECOMPILE_BATCH_ADDRESS,
                        value: 0n,
                        data: batchData,
                        gaslimit: 200_000n,
                        nonce: fromHex(alithNonceResult, "bigint"),
                        deadline: 9999999999n,
                    },
                });
                const { v, r, s } = getSignatureParameters(signature);

                const { result: baltatharForAlithResult } = await context.createBlock(
                    await createViemTransaction(context, {
                        privateKey: BALTATHAR_PRIVATE_KEY,
                        to: PRECOMPILE_CALL_PERMIT_ADDRESS,
                        data: encodeFunctionData({
                            abi: callPermitAbi,
                            functionName: "dispatch",
                            args: [ALITH_ADDRESS, PRECOMPILE_BATCH_ADDRESS, 0, batchData, 200_000, 9999999999, v, r, s],
                        }),
                    })
                );
                expectEVMResult(baltatharForAlithResult?.events, "Succeed");
            },
        });
    },
});
