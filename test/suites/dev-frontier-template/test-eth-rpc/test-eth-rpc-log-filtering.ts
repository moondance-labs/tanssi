import { beforeAll, deployCreateCompiledContract, describeSuite, expect } from "@moonwall/cli";
import { customWeb3Request } from "@moonwall/util";
import type { TransactionReceipt } from "viem";
import type { RpcResponse } from "../../../types/rpc-response.type.ts";

describeSuite({
    id: "DE0703",
    title: "Ethereum RPC - Filtering non-matching logs",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let nonMatchingCases: ReturnType<typeof getNonMatchingCases>;

        const getNonMatchingCases = (receipt: TransactionReceipt) => {
            return [
                // Non-existant address.
                {
                    fromBlock: "0x0",
                    toBlock: "latest",
                    address: "0x0000000000000000000000000000000000000000",
                },
                // Non-existant topic.
                {
                    fromBlock: "0x0",
                    toBlock: "latest",
                    topics: ["0x0000000000000000000000000000000000000000000000000000000000000000"],
                },
                // Existant address + non-existant topic.
                {
                    fromBlock: "0x0",
                    toBlock: "latest",
                    address: receipt.contractAddress,
                    topics: ["0x0000000000000000000000000000000000000000000000000000000000000000"],
                },
                // Non-existant address + existant topic.
                {
                    fromBlock: "0x0",
                    toBlock: "latest",
                    address: "0x0000000000000000000000000000000000000000",
                    topics: receipt.logs[0].topics,
                },
            ];
        };

        beforeAll(async () => {
            const { hash } = await deployCreateCompiledContract(context, "EventEmitter");
            const receipt = await context.viem("public").getTransactionReceipt({ hash });
            nonMatchingCases = getNonMatchingCases(receipt);
        });

        it({
            id: "T01",
            title: "EthFilterApi::getFilterLogs - should filter out non-matching cases.",
            test: async () => {
                let create_filter: any;
                for (const item of nonMatchingCases) {
                    create_filter = await customWeb3Request(context.web3(), "eth_newFilter", [item]);
                    const poll = (await customWeb3Request(context.web3(), "eth_getFilterLogs", [
                        create_filter.result,
                    ])) as any;
                    expect(poll.result.length).to.be.eq(0);
                }
            },
        });
        it({
            id: "T02",
            title: "EthApi::getLogs - should filter out non-matching cases.",
            test: async () => {
                for (const item of nonMatchingCases) {
                    const request = (await customWeb3Request(context.web3(), "eth_getLogs", [item])) as any;
                    expect(request.result.length).to.be.eq(0);
                }
            },
        });

        it({
            id: "T03",
            title: "Validate eth_getLogs block range limit",
            test: async () => {
                let blocksToCreate = 1025;
                for (; blocksToCreate > 0; blocksToCreate--) {
                    await context.createBlock();
                }

                const result = (await customWeb3Request(context.web3(), "eth_getLogs", [
                    {
                        fromBlock: "0x0",
                        toBlock: "latest",
                        topics: [],
                    },
                ])) as RpcResponse;

                if ("error" in result) {
                    expect(result.error.message).toEqual("block range is too wide (maximum 1024)");
                } else {
                    throw new Error("Unexpected response, failing the test");
                }
            },
        });
    },
});
