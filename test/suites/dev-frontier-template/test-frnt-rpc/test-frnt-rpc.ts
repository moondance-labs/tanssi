import { customDevRpcRequest, describeSuite, expect } from "@moonwall/cli";
import { BALTATHAR_ADDRESS, createViemTransaction } from "@moonwall/util";

const DEFAULT_TXN_MAX_BASE_FEE = 10_000_000_000;

describeSuite({
    id: "DE1001",
    title: "Frontier RPC Methods - frnt_isBlockFinalized ",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "should return as finalized when true",
            test: async () => {
                const blockHash = (await context.createBlock([], { finalize: true })).block.hash;
                const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
                expect(resp, "Block finalization status mismatch").toBe(true);
            },
        });

        it({
            id: "T02",
            title: "should return as unfinalized when false",
            test: async () => {
                const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
                const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
                expect(resp, "Block finalization status mismatch").toBe(false);
            },
        });

        it({
            id: "T03",
            title: "should return as unfinalized when block not found",
            test: async () => {
                const blockHash = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
                const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
                expect(resp, "Block finalization status mismatch").toBe(false);
            },
        });

        it({
            id: "T04",
            title: "should return as finalized when new block is true",
            test: async () => {
                const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
                await context.createBlock([], { finalize: true });
                const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
                expect(resp, "Block finalization status mismatch").toBe(true);
            },
        });

        it({
            id: "T05",
            title: "should return as finalized when new block reorg happens",
            test: async () => {
                const blockHash = (await context.createBlock([], { finalize: false })).block.hash;
                await context.createBlock([], { finalize: false });
                await context.createBlock([], { finalize: true, parentHash: blockHash });

                const resp = await customDevRpcRequest("frnt_isBlockFinalized", [blockHash]);
                expect(resp, "Block finalization status mismatch").toBe(true);
            },
        });

        it({
            id: "T06",
            title: "should return as finalized when true",
            test: async () => {
                await context.createBlock(
                    await createViemTransaction(context, {
                        to: BALTATHAR_ADDRESS,
                        gas: 12_000_000n,
                        gasPrice: BigInt(DEFAULT_TXN_MAX_BASE_FEE),
                        value: 1_000_000n,
                    }),
                    { finalize: true }
                );

                const block = await context.viem().getBlock();
                const resp = await customDevRpcRequest("frnt_isTxFinalized", [block.transactions[0]]);
                expect(resp, "Transaction finalization status mismatch").toBe(true);
            },
        });

        it({
            id: "T07",
            title: "should return as unfinalized when false",
            test: async () => {
                await context.createBlock(
                    await createViemTransaction(context, {
                        to: BALTATHAR_ADDRESS,
                        gas: 12_000_000n,
                        gasPrice: BigInt(DEFAULT_TXN_MAX_BASE_FEE),
                        value: 1_000_000n,
                    }),
                    { finalize: false }
                );

                const block = await context.viem().getBlock();
                const resp = await customDevRpcRequest("frnt_isTxFinalized", [block.transactions[0]]);
                expect(resp, "Transaction finalization status mismatch").toBe(false);
            },
        });

        it({
            id: "T08",
            title: "should return as unfinalized when txn not found",
            test: async () => {
                const txnHash = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
                const resp = await customDevRpcRequest("frnt_isTxFinalized", [txnHash]);
                expect(resp, "Transaction finalization status mismatch").toBe(false);
            },
        });

        it({
            id: "T09",
            title: "should return as finalized when new block is true",
            test: async () => {
                await context.createBlock(
                    await createViemTransaction(context, {
                        to: BALTATHAR_ADDRESS,
                        gas: 12_000_000n,
                        gasPrice: BigInt(DEFAULT_TXN_MAX_BASE_FEE),
                        value: 1_000_000n,
                    }),
                    { finalize: false }
                );
                const block = await context.viem().getBlock();
                await context.createBlock([], { finalize: true });
                const resp = await customDevRpcRequest("frnt_isTxFinalized", [block.transactions[0]]);
                expect(resp, "Transaction finalization status mismatch").toBe(true);
            },
        });

        it({
            id: "T10",
            title: "should return as finalized when new block reorg happens",
            test: async () => {
                const blockHash = (
                    await context.createBlock(
                        await createViemTransaction(context, {
                            to: BALTATHAR_ADDRESS,
                            gas: 12_000_000n,
                            gasPrice: BigInt(DEFAULT_TXN_MAX_BASE_FEE),
                            value: 1_000_000n,
                        }),
                        { finalize: false }
                    )
                ).block.hash;

                const block = await context.viem().getBlock();
                await context.createBlock([], { finalize: false });
                await context.createBlock([], { finalize: true, parentHash: blockHash });
                const resp = await customDevRpcRequest("frnt_isTxFinalized", [block.transactions[0]]);
                expect(resp, "Transaction finalization status mismatch").toBe(true);
            },
        });
    },
});
