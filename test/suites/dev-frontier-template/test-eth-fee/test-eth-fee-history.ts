import { describeSuite, expect } from "@moonwall/cli";
import { ALITH_ADDRESS, alith } from "@moonwall/util";
import { customWeb3Request } from "@moonwall/util";
import { hexToNumber, numberToHex } from "@polkadot/util";
import { getCompiled } from "utils";
import { parseGwei } from "viem";

// We use ethers library in this test as apparently web3js's types are not fully EIP-1559
// compliant yet.
describeSuite({
    id: "DE0501",
    title: "Fee History",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        interface FeeHistory {
            oldestBlock: string;
            baseFeePerGas: string[];
            gasUsedRatio: number[];
            reward: string[][];
        }

        async function createBlocks(
            block_count: number,
            reward_percentiles: number[],
            priority_fees: number[],
            max_fee_per_gas: string
        ) {
            let nonce = await context.viem("public").getTransactionCount({ address: ALITH_ADDRESS });
            const contractData = getCompiled("MultiplyBy7");
            for (let b = 0; b < block_count; b++) {
                for (let p = 0; p < priority_fees.length; p++) {
                    await context.ethers().sendTransaction({
                        from: alith.address,
                        data: contractData.byteCode,
                        value: "0x00",
                        maxFeePerGas: max_fee_per_gas,
                        maxPriorityFeePerGas: numberToHex(priority_fees[p]),
                        accessList: [],
                        nonce: nonce,
                        gasLimit: "0x100000",
                        chainId: 1281,
                    });
                    nonce++;
                }
                await context.createBlock();
            }
        }

        function get_percentile(percentile: number, array: number[]) {
            array.sort((a, b) => a - b);
            const index = (percentile / 100) * array.length - 1;
            if (Math.floor(index) === index) {
                return array[index];
            }
            return Math.ceil((array[Math.floor(index)] + array[Math.ceil(index)]) / 2);
        }

        it({
            id: "T01",
            title: "result length should match spec",
            timeout: 30000,
            test: async () => {
                const block_count = 2;
                const reward_percentiles = [20, 50, 70];
                const priority_fees = [1, 2, 3];
                const startingBlock = await context.viem("public").getBlockNumber();

                const feeHistory = new Promise<FeeHistory>((resolve) => {
                    const unwatch = context.viem("public").watchBlocks({
                        onBlock: async (block) => {
                            if (Number(block.number - startingBlock) === block_count) {
                                const { result } = (await customWeb3Request(context.web3(), "eth_feeHistory", [
                                    "0x2",
                                    "latest",
                                    reward_percentiles,
                                ])) as {
                                    jsonrpc: string;
                                    id: number;
                                    result: FeeHistory;
                                };
                                unwatch();
                                resolve(result);
                            }
                        },
                    });
                });

                await createBlocks(block_count, reward_percentiles, priority_fees, parseGwei("10").toString());
                const feeResults = await feeHistory;
                expect(
                    feeResults.baseFeePerGas.length,
                    "baseFeePerGas should always the requested block range + 1 (the next derived base fee)"
                ).toBe(block_count + 1);
                expect(
                    feeResults.reward.length,
                    "should return two-dimensional reward list for the requested block range"
                ).to.be.eq(block_count);

                const failures = feeResults.reward.filter((item) => {
                    item.length !== reward_percentiles.length;
                });
                expect(
                    failures.length,
                    "each block has a reward list which's size is the requested percentile list"
                ).toBe(0);
            },
        });

        it({
            id: "T02",
            title: "should calculate percentiles",
            timeout: 60000,
            test: async () => {
                const max_fee_per_gas = parseGwei("10").toString();
                const block_count = 11;
                const reward_percentiles = [20, 50, 70, 85, 100];
                const priority_fees = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
                const startingBlock = await context.viem("public").getBlockNumber();

                const feeHistory = new Promise<FeeHistory>((resolve) => {
                    const unwatch = context.viem("public").watchBlocks({
                        onBlock: async (block) => {
                            if (Number(block.number - startingBlock) === block_count) {
                                const { result } = (await customWeb3Request(context.web3(), "eth_feeHistory", [
                                    "0xA",
                                    "latest",
                                    reward_percentiles,
                                ])) as {
                                    jsonrpc: string;
                                    id: number;
                                    result: FeeHistory;
                                };

                                unwatch();
                                resolve(result);
                            }
                        },
                    });
                });

                await createBlocks(block_count, reward_percentiles, priority_fees, max_fee_per_gas);

                const feeResults = await feeHistory;
                const localRewards = reward_percentiles
                    .map((percentile) => get_percentile(percentile, priority_fees))
                    .map((reward) => numberToHex(reward));
                // We only test if BaseFee update is enabled.
                //
                // If BaseFee is a constant 1GWEI, that means that there is no effective reward using
                // the specs formula MIN(tx.maxPriorityFeePerGas, tx.maxFeePerGas-block.baseFee).
                //
                // In other words, for this tip oracle there would be no need to provide a priority fee
                // when the block fullness is considered ideal in an EIP-1559 chain.
                const failures = feeResults.reward.filter(
                    (item, index) =>
                        hexToNumber(max_fee_per_gas) - hexToNumber(feeResults.baseFeePerGas[index]) > 0 &&
                        (item.length !== localRewards.length ||
                            !item.every((val, idx) => BigInt(val) === BigInt(localRewards[idx])))
                );

                expect(failures.length, "each block should have rewards matching the requested percentile list").toBe(
                    0
                );
            },
        });
    },
});
