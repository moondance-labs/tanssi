import { describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import {
    BALTATHAR_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
    CONTRACT_PROXY_TYPE_ANY,
    DOROTHY_ADDRESS,
    DOROTHY_PRIVATE_KEY,
    FAITH_ADDRESS,
} from "@moonwall/util";
import { parseEther } from "ethers";
import { expectEVMResult } from "helpers/eth-transactions";
import { encodeFunctionData } from "viem";

const PRECOMPILE_NATIVE_ERC20_ADDRESS = "0x0000000000000000000000000000000000000800";
const PRECOMPILE_PROXY_ADDRESS = "0x0000000000000000000000000000000000000805";

describeSuite({
    id: "DE1504",
    title: "Storage growth limit - Precompiles",
    foundationMethods: "dev",
    testCases: ({ context, it, log }) => {
        const newAccount = "0x1ced798a66b803d0dbb665680283980a939a6432";
        // The tx can create an account, so record 148 bytes of storage growth
        // Storage growth ratio is 1464
        // expected_gas = 148 * 1464 = 216672
        const expectedGas = 216672n;

        it({
            id: "T01",
            title: "should fail transfer due to insufficient gas required to cover the storage growth",
            test: async () => {
                const { abi: ierc20Abi } = fetchCompiledContract("IERC20");

                const rawTxn = await context.writeContract?.({
                    contractAddress: PRECOMPILE_PROXY_ADDRESS,
                    contractName: "Proxy",
                    functionName: "addProxy",
                    args: [BALTATHAR_ADDRESS, CONTRACT_PROXY_TYPE_ANY, 0],
                    privateKey: DOROTHY_PRIVATE_KEY,
                    rawTxOnly: true,
                });
                const { result } = await context.createBlock(rawTxn);
                expectEVMResult(result?.events, "Succeed");

                const balBefore = await context.viem().getBalance({ address: FAITH_ADDRESS });
                const rawTxn2 = await context.writeContract?.({
                    contractName: "Proxy",
                    functionName: "proxy",
                    contractAddress: PRECOMPILE_PROXY_ADDRESS,
                    args: [
                        DOROTHY_ADDRESS,
                        PRECOMPILE_NATIVE_ERC20_ADDRESS,
                        encodeFunctionData({
                            abi: ierc20Abi,
                            functionName: "transfer",
                            args: [newAccount, parseEther("5")],
                        }),
                    ],
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    rawTxOnly: true,
                    gas: 216672n - 10n, // Not enough gas to cover the storage growth
                });

                const { result: result2 } = await context.createBlock(rawTxn2);
                // Check that the transaction failed with an out of gas error
                expectEVMResult(result2.events, "Error", "OutOfGas");

                const balAfter = await context.viem().getBalance({ address: FAITH_ADDRESS });
                expect(balBefore - balAfter).to.equal(0n);
            },
        });

        it({
            id: "T02",
            title: "should transfer correctly with the required gas to cover the storage growth",
            test: async () => {
                const balanceBefore = await context.viem().getBalance({ address: DOROTHY_ADDRESS });
                const { abi: ierc20Abi } = fetchCompiledContract("IERC20");
                const { abi: proxyAbi } = fetchCompiledContract("Proxy");

                const proxyProxyEstimatedGas = await context.viem().estimateGas({
                    account: BALTATHAR_ADDRESS,
                    to: PRECOMPILE_PROXY_ADDRESS,
                    data: encodeFunctionData({
                        abi: proxyAbi,
                        functionName: "proxy",
                        args: [
                            DOROTHY_ADDRESS,
                            PRECOMPILE_NATIVE_ERC20_ADDRESS,
                            encodeFunctionData({
                                abi: ierc20Abi,
                                functionName: "transfer",
                                args: [newAccount, parseEther("5")],
                            }),
                        ],
                    }),
                });

                const rawTxn2 = await context.writeContract?.({
                    contractName: "Proxy",
                    functionName: "proxy",
                    contractAddress: PRECOMPILE_PROXY_ADDRESS,
                    args: [
                        DOROTHY_ADDRESS,
                        PRECOMPILE_NATIVE_ERC20_ADDRESS,
                        encodeFunctionData({
                            abi: ierc20Abi,
                            functionName: "transfer",
                            args: [newAccount, parseEther("5")],
                        }),
                    ],
                    privateKey: BALTATHAR_PRIVATE_KEY,
                    rawTxOnly: true,
                });

                const { result } = await context.createBlock(rawTxn2);
                expectEVMResult(result?.events, "Succeed");

                const { gasUsed } = await context.viem().getTransactionReceipt({ hash: result?.hash as `0x${string}` });
                expect(gasUsed).to.equal(expectedGas);

                const balanceAfter = await context.viem().getBalance({ address: DOROTHY_ADDRESS });
                expect(balanceBefore - balanceAfter).to.equal(parseEther("5"));
            },
        });
    },
});
