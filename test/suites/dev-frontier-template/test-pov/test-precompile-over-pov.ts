import "@tanssi/api-augment";

import { beforeAll, deployCreateCompiledContract, describeSuite, expect, fetchCompiledContract } from "@moonwall/cli";
import { type HeavyContract, deployHeavyContracts, expectEVMResult } from "../../../helpers";

import { ALITH_ADDRESS, createEthersTransaction } from "@moonwall/util";
import { type Abi, encodeFunctionData } from "viem";

const PRECOMPILE_BATCH_ADDRESS = "0x0000000000000000000000000000000000000801";

describeSuite({
    id: "DE1203",
    title: "PoV precompile test - gasLimit",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        let contracts: HeavyContract[];
        const MAX_CONTRACTS = 50;
        const EXPECTED_POV_ROUGH = 52_800; // bytes
        let batchAbi: Abi;
        let proxyAbi: Abi;
        let proxyAddress: `0x${string}`;
        let callData: `0x${string}`;

        beforeAll(async () => {
            const { contractAddress: contractAdd1, abi } = await deployCreateCompiledContract(
                context,
                "CallForwarder",
                { gas: 20000000n }
            );
            proxyAddress = contractAdd1;
            proxyAbi = abi;
            contracts = await deployHeavyContracts(context, 6000, 6000 + MAX_CONTRACTS);

            // Get the interface for Batch precompile
            batchAbi = fetchCompiledContract("Batch").abi;

            callData = encodeFunctionData({
                abi: batchAbi,
                functionName: "batchAll",
                args: [
                    [proxyAddress],
                    [],
                    [
                        encodeFunctionData({
                            abi: proxyAbi,
                            functionName: "callRange",
                            args: [contracts[0].account, contracts[MAX_CONTRACTS].account],
                        }),
                    ],
                    [],
                ],
            });
        });

        it({
            id: "T01",
            title: "gas cost should have increased with POV",
            test: async () => {
                // Previously this tx cost was ~500K gas -> now it is about 5M due to POV.
                // We pass 1M, so it should fail.
                const rawSigned = await createEthersTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                    gasLimit: 1_000_000,
                    txnType: "eip1559",
                });

                const { result, block } = await context.createBlock(rawSigned);

                // With 1M gas we are allowed to use ~62kb of POV, so verify the range.
                // The tx is still included in the block because it contains the failed tx,
                // so POV is included in the block as well.
                expect(block.proofSize).to.be.at.least(34_000);
                expect(block.proofSize).to.be.at.most(70_000);
                expect(result?.successful).to.equal(true);
                expectEVMResult(result?.events, "Error", "OutOfGas");
            },
        });

        it({
            id: "T02",
            title: "should be able to create a block using the estimated amount of gas",
            test: async () => {
                const gasEstimate = await context.viem().estimateGas({
                    account: ALITH_ADDRESS,
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                });

                const rawSigned = await createEthersTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                    gasLimit: gasEstimate,
                    txnType: "eip1559",
                });

                const { result, block } = await context.createBlock(rawSigned);
                expect(block.proofSize).to.be.at.least(EXPECTED_POV_ROUGH / 1.3);
                expect(block.proofSize).to.be.at.most(EXPECTED_POV_ROUGH * 1.3);
                expect(result?.successful).to.equal(true);
                expectEVMResult(result?.events, "Succeed", "Returned");
            },
        });

        it({
            id: "T03",
            title: "should allow to call a precompile tx with enough gas limit to cover PoV",
            test: async () => {
                const rawSigned = await createEthersTransaction(context, {
                    to: PRECOMPILE_BATCH_ADDRESS,
                    data: callData,
                    gasLimit: 24_000_000,
                    txnType: "eip1559",
                });

                const { result, block } = await context.createBlock(rawSigned);
                expect(block.proofSize).to.be.at.least(EXPECTED_POV_ROUGH / 1.3);
                expect(block.proofSize).to.be.at.most(EXPECTED_POV_ROUGH * 1.3);
                expect(result?.successful).to.equal(true);
                expectEVMResult(result?.events, "Succeed", "Returned");
            },
        });
    },
});
